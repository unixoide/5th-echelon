pub mod packet;

use std::cell::RefCell;
use std::collections::HashMap;
use std::io;
use std::net::SocketAddr;
use std::net::UdpSocket;
use std::net::{self};
use std::sync::atomic::AtomicU32;
use std::time::Duration;
use std::time::Instant;

use slog::debug;
use slog::error;
use slog::o;
use slog::Logger;

use self::packet::crypt_key;
use self::packet::PacketFlag;
use self::packet::PacketType;
use self::packet::QPacket;
use self::packet::StreamHandler;
use self::packet::StreamHandlerRegistry;
use self::packet::VPort;
use crate::kerberos::KerberosTicketInternal;
use crate::rmc::basic::ReadStream;
use crate::rmc::basic::ToStream;
use crate::ClientInfo;
use crate::ConnectionID;
use crate::Context;
use crate::Signature;

const MAX_PAYLOAD_SIZE: usize = 1000;
const SESSION_TIMEOUT: Duration = Duration::from_secs(60);

#[derive(Default)]
pub struct ClientRegistry<T> {
    clients: HashMap<u32, RefCell<ClientInfo<T>>>,
    connection_id_session_ids: HashMap<ConnectionID, Signature>,
}

impl<T> ClientRegistry<T> {
    #[must_use]
    pub fn client_by_connection_id(&self, conn_id: ConnectionID) -> Option<&RefCell<ClientInfo<T>>> {
        self.connection_id_session_ids.get(&conn_id).and_then(|sig| self.clients.get(&sig.0))
    }
}

pub struct Server<'a, ECH, DH, T = ()>
where
    ECH: FnMut(ClientInfo<T>),
    DH: FnMut(ClientInfo<T>),
{
    logger: slog::Logger,
    registry: StreamHandlerRegistry<T>,
    socket: Option<net::UdpSocket>,
    ctx: &'a Context,
    new_clients: HashMap<u32, ClientInfo<T>>,
    client_registry: ClientRegistry<T>,
    pub user_handler: Option<fn(logger: &Logger, packet: QPacket, client: SocketAddr, sock: &net::UdpSocket)>,
    pub expired_client_handler: Option<ECH>,
    pub disconnect_handler: Option<DH>,
    next_conn_id: AtomicU32,
}

impl<ECH, DH, T> Server<'_, ECH, DH, T>
where
    T: Default,
    ECH: FnMut(ClientInfo<T>),
    DH: FnMut(ClientInfo<T>),
{
    #[must_use]
    pub fn new(logger: slog::Logger, ctx: &Context, registry: StreamHandlerRegistry<T>) -> Server<ECH, DH, T> {
        let client_registry = ClientRegistry {
            clients: HashMap::default(),
            connection_id_session_ids: HashMap::default(),
        };
        Server {
            logger,
            registry,
            socket: None,
            ctx,
            new_clients: HashMap::default(),
            client_registry,
            user_handler: None,
            expired_client_handler: None,
            disconnect_handler: None,
            next_conn_id: AtomicU32::new(0x3AAA_AAAA),
        }
    }

    pub fn register(&mut self, vport: VPort, protocol: Box<dyn StreamHandler<T>>) {
        self.registry.register(vport, protocol);
    }

    pub fn bind<A: net::ToSocketAddrs>(&mut self, addrs: A) -> io::Result<()> {
        self.socket = Some(net::UdpSocket::bind(addrs)?);
        info!(self.logger, "Listening on {}", self.socket.as_ref().unwrap().local_addr().unwrap());
        Ok(())
    }

    pub fn serve(mut self) {
        let socket = self.socket.as_ref().expect("UDP socket required").try_clone().expect("Couldn't clone socket");
        socket.set_read_timeout(Some(Duration::from_secs(1))).expect("error setting read timeout");
        let mut buf = vec![0u8; 1024];
        'outer: loop {
            let (nread, client) = match socket.recv_from(&mut buf) {
                Ok(x) => x,
                Err(e) => {
                    if e.kind() == std::io::ErrorKind::TimedOut || e.kind() == std::io::ErrorKind::WouldBlock {
                        self.clear_clients();
                    } else {
                        error!(self.logger, "recv_from failed: {}", e);
                    }
                    continue;
                }
            };
            let logger = self.logger.new(o!("client" => client));
            let mut data = &buf[..nread];

            while !data.is_empty() {
                let (packet, nparsed) = match QPacket::from_bytes(self.ctx, data) {
                    Ok(p) => p,
                    Err(e) => {
                        error!(logger, "Invalid packet received"; "error" =>  %e);
                        continue 'outer;
                    }
                };
                #[allow(clippy::cast_possible_truncation)]
                let (packet_data, next_data) = data.split_at(nparsed as usize);
                data = next_data;
                trace!(logger, "-> {:02x?}", packet_data);

                if let Err(e) = packet.validate(self.ctx, packet_data) {
                    error!(logger, "Invalid packet received: {:?}", packet; "error" =>  %e);
                    continue;
                };

                self.handle_packet(&logger.new(o!("seq" => packet.sequence, "session" => packet.session_id)), packet, client);
            }
        }
    }

    fn handle_packet(&mut self, logger: &Logger, packet: QPacket, client: SocketAddr) {
        debug!(logger, "packet: {:?}", packet);
        if packet.flags.contains(PacketFlag::Ack) {
            debug!(logger, "Received ACK");
            return;
        }
        match packet.packet_type {
            PacketType::Syn => self.handle_syn(logger, packet, client),
            PacketType::Connect => self.handle_connect(logger, packet, client),
            PacketType::Data => self.handle_data(logger, packet, client),
            PacketType::Disconnect => {
                let Some(ci) = self.client_registry.clients.remove(&packet.signature) else {
                    return;
                };
                info!(logger, "Client disconnected"; "signature" => packet.signature, "session" => packet.session_id);
                if self.send_ack(logger, &client, &packet, &ci.borrow(), false).is_err() {
                    // ignore
                }
                if let Some(handler) = self.disconnect_handler.as_mut() {
                    (handler)(ci.into_inner());
                }
            }
            PacketType::Ping => {
                let Some(ci) = self.client_registry.clients.get(&packet.signature) else {
                    return;
                };
                ci.borrow_mut().seen();
                if self.send_ack(logger, &client, &packet, &ci.borrow(), false).is_err() {
                    // ignore
                }
            }
            PacketType::User => {
                if self.user_handler.is_none() {
                    error!(logger, "unsupported user packet");
                } else {
                    (self.user_handler.as_ref().unwrap())(logger, packet, client, self.socket.as_ref().unwrap());
                }
            }
            PacketType::Route => todo!(),
            PacketType::Raw => todo!(),
        }
    }

    fn handle_data(&mut self, logger: &Logger, packet: QPacket, client: SocketAddr) {
        #![allow(clippy::cast_possible_truncation)]

        debug!(logger, "Handling data packet");
        let Some(ci) = self.client_registry.clients.get(&packet.signature) else {
            warn!(logger, "client is unknown!");
            return;
        };
        let logger = logger.new(o!("pid" => ci.borrow().user_id));
        let ci = &mut ci.borrow_mut();
        ci.seen();
        if let Err(e) = self.send_ack(&logger, &client, &packet, &*ci, false) {
            error!(logger, "Error sending ack"; "error" => %e);
        } else {
            debug!(logger, "Send ack");
        }
        let payload = if let Some(fid) = packet.fragment_id {
            if fid != 0 {
                info!(logger, "Caching fragment {}", fid);
                ci.packet_fragments.insert(fid, packet.payload);
                return;
            }
            let mut payload = vec![];
            if !ci.packet_fragments.is_empty() {
                for fid in 1..=ci.packet_fragments.len() as u8 {
                    let f = match ci.packet_fragments.get(&fid) {
                        None => {
                            error!(logger, "missing fragment {}", fid);
                            ci.packet_fragments.clear();
                            return;
                        }
                        Some(f) => f,
                    };
                    payload.extend(f.iter());
                    // debug!(logger, "Reassembled: {:?}", payload; "fid" => fid);
                }
                info!(logger, "Reassembled {} fragments", ci.packet_fragments.len() + 1);
                ci.packet_fragments.clear();
            }
            payload.extend(packet.payload);
            // debug!(logger, "Reassembled: {:?}", payload; "fid" => fid);
            payload
        } else {
            packet.payload
        };
        let resp = self
            .registry
            .handle_packet(&logger, self.ctx, ci, &packet.destination, &payload, &self.client_registry, self.socket.as_ref().unwrap());
        match resp {
            Some(Ok(payload)) => {
                let chunks = payload.chunks(MAX_PAYLOAD_SIZE);
                for (fid, chunk) in (0..chunks.len()).rev().zip(chunks) {
                    let resp = QPacket {
                        source: packet.destination,
                        destination: packet.source,
                        packet_type: PacketType::Data,
                        payload: chunk.to_vec(),
                        fragment_id: Some(fid as u8),
                        ..Default::default()
                    };
                    if let Err(e) = self.send_response(&logger, &client, resp, ci) {
                        error!(logger, "Error sending response"; "error" => %e);
                    } else {
                        trace!(logger, "Send response");
                    }
                }
            }
            None => {
                error!(logger, "No handler found");
            }
            Some(Err(_)) => {
                error!(logger, "Handler failed");
            }
        }
    }

    fn handle_syn(&mut self, logger: &Logger, mut packet: QPacket, client: SocketAddr) {
        debug!(logger, "Handling syn packet");
        let ci: ClientInfo<T> = ClientInfo::new(client);
        let sig = ci.server_signature;
        self.new_clients.insert(sig, ci);

        packet.conn_signature = Some(sig);

        let ci = self.new_clients.get(&sig).unwrap();

        if let Err(e) = self.send_ack(logger, &client, &packet, ci, false) {
            error!(logger, "Error sending syn ack packet"; "error" => %e);
        }
    }

    fn handle_connect(&mut self, logger: &Logger, mut packet: QPacket, client: SocketAddr) {
        debug!(logger, "Handling connect packet");
        let Some(signature) = packet.conn_signature else {
            todo!("deny connect. no signature");
        };

        let Some(mut ci) = self.new_clients.remove(&packet.signature) else {
            todo!("deny connect. no client");
        };
        ci.client_signature = Some(signature);
        ci.server_session = rand::random();
        ci.client_session = packet.session_id;

        /*
        let ci = if false {
            // this doesn't work with the borrow checker
            let ci = match self.clients.entry(packet.signature) {
                Entry::Occupied(mut e) => {
                    e.insert(ci);
                    e.into_mut()
                }
                Entry::Vacant(e) => e.insert(ci),
            };
            &*ci
        } else {
            self.client_registry.clients.insert(packet.signature, RefCell::new(ci));
            let ci = self.client_registry.clients.get(&packet.signature).unwrap();
            ci
        };
        */
        let ci = {
            self.client_registry.clients.insert(packet.signature, RefCell::new(ci));
            let ci = self.client_registry.clients.get(&packet.signature).unwrap();
            ci
        };

        if !packet.payload.is_empty() {
            let data = std::mem::take(&mut packet.payload);
            let mut s = ReadStream::from_bytes(&data);
            let ticket_key = &self.ctx.ticket_key;
            let cids = &mut self.client_registry.connection_id_session_ids;
            let next_conn_id = &mut self.next_conn_id;
            let res = move || -> Result<_, crate::rmc::basic::FromStreamError> {
                let ticket: Vec<u8> = s.read()?;
                let request_data: Vec<u8> = s.read()?;

                let ti = KerberosTicketInternal::open(&ticket, ticket_key)?;

                if ti.valid_until
                    < std::time::SystemTime::now()
                        .duration_since(std::time::SystemTime::UNIX_EPOCH)
                        .as_ref()
                        .map(std::time::Duration::as_secs)
                        .unwrap_or_default()
                {
                    return Ok(vec![]);
                }
                let id = next_conn_id.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
                ci.borrow_mut().user_id.replace(ti.principle_id);
                ci.borrow_mut().connection_id.replace(ConnectionID(id));
                // clear highest bit as the game sometimes uses signed ints instead of unsigned
                // .replace(ConnectionID(rand::random::<u16>() as u32 & 0x7FFF_FFFFu32));
                cids.insert(ci.borrow().connection_id.unwrap(), Signature(packet.signature));
                let data = crypt_key(ti.session_key.as_ref(), &request_data);

                #[allow(clippy::items_after_statements)]
                #[derive(FromStream, Debug)]
                struct ConnectData {
                    _user_pid: u32,
                    _connection_id: u32,
                    challenge: u32,
                }

                let cd: ConnectData = ReadStream::from_bytes(&data).read()?;

                let resp = cd.challenge + 1;
                let resp = resp.to_bytes();

                Ok(resp.to_bytes())
            }();

            match res {
                Ok(resp) => packet.payload = resp,
                Err(e) => error!(logger, "Error parsing ticket"; "error" => %e),
            }
        }

        packet.conn_signature = Some(0);

        if let Err(e) = self.send_ack(logger, &client, &packet, &ci.borrow(), !packet.payload.is_empty()) {
            error!(logger, "Error sending syn ack packet"; "error" => %e);
        }
        info!(logger, "New client connected"; "signature" => packet.signature, "session" => packet.session_id);
    }

    fn send_response(&self, logger: &Logger, src: &SocketAddr, resp: QPacket, ci: &mut ClientInfo<T>) -> Result<usize, Box<dyn std::error::Error>> {
        send_response(logger, self.ctx, src, self.socket.as_ref().unwrap(), resp, ci)
    }

    fn send_packet(&self, logger: &Logger, src: &SocketAddr, resp: QPacket) -> Result<usize, Box<dyn std::error::Error>> {
        send_packet(logger, self.ctx, src, self.socket.as_ref().unwrap(), resp)
    }

    fn send_ack(&self, logger: &Logger, src: &SocketAddr, packet: &QPacket, ci: &ClientInfo<T>, keep_payload: bool) -> Result<usize, Box<dyn std::error::Error>> {
        let mut resp = packet.clone();
        resp.source = packet.destination;
        resp.destination = packet.source;
        resp.flags = PacketFlag::Ack | PacketFlag::HasSize;
        resp.signature = ci.client_signature.unwrap_or_default();
        resp.session_id = ci.server_session;
        if !keep_payload {
            resp.payload.clear();
        }
        resp.sequence = packet.sequence;
        self.send_packet(logger, src, resp)
    }

    fn clear_clients(&mut self) {
        let now = Instant::now();
        for (_, ci) in self
            .client_registry
            .clients
            .extract_if(|_k, v| v.try_borrow().map(|ci| (now - ci.last_seen) > SESSION_TIMEOUT).unwrap_or(false))
        {
            if let Some(handler) = self.expired_client_handler.as_mut() {
                let ci = ci.into_inner();
                if let Some(conn_id) = ci.connection_id {
                    self.client_registry.connection_id_session_ids.remove(&conn_id);
                }
                (handler)(ci);
            }
        }
    }
}

pub fn send_response<T>(
    logger: &Logger,
    ctx: &Context,
    src: &SocketAddr,
    socket: &UdpSocket,
    mut resp: QPacket,
    ci: &mut ClientInfo<T>,
) -> Result<usize, Box<dyn std::error::Error>> {
    resp.sequence = ci.server_sequence_id;
    ci.server_sequence_id += 1;
    resp.flags.insert(PacketFlag::HasSize);
    resp.flags.insert(PacketFlag::NeedAck);
    resp.flags.insert(PacketFlag::Reliable); // ??
    resp.signature = ci.client_signature.unwrap_or_default();
    resp.session_id = ci.server_session;
    send_packet(logger, ctx, src, socket, resp)
}

pub fn send_request<T>(
    logger: &Logger,
    ctx: &Context,
    src: &SocketAddr,
    socket: &UdpSocket,
    mut req: QPacket,
    ci: &mut ClientInfo<T>,
) -> Result<usize, Box<dyn std::error::Error>> {
    req.sequence = ci.client_sequence_id;
    ci.client_sequence_id += 1;
    req.flags.insert(PacketFlag::HasSize);
    req.flags.insert(PacketFlag::NeedAck);
    req.flags.insert(PacketFlag::Reliable); // ??
    req.signature = ci.server_signature;
    req.session_id = ci.client_session;
    send_packet(logger, ctx, src, socket, req)
}

pub(crate) fn send_packet(logger: &Logger, ctx: &Context, src: &SocketAddr, socket: &UdpSocket, mut resp: QPacket) -> Result<usize, Box<dyn std::error::Error>> {
    if matches!(resp.packet_type, PacketType::Data) {
        resp.use_compression = true;
        if resp.fragment_id.is_none() {
            resp.fragment_id = Some(0);
        }
    }
    resp.flags.insert(PacketFlag::HasSize);
    trace!(logger, "<- {:?}", resp);
    let data = &resp.to_bytes(ctx);
    trace!(logger, "<- {:02x?}", data);
    let sz = socket.send_to(data, src)?;
    assert_eq!(sz, data.len());
    Ok(sz)
}
