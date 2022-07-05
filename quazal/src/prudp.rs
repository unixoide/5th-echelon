pub mod packet;

use crate::{
    kerberos::KerberosTicketInternal,
    rmc::basic::{ReadStream, ToStream},
    ClientInfo, Context,
};
use packet::*;
use slog::{debug, error, o, Logger};
use std::{
    cell::RefCell,
    collections::HashMap,
    io,
    net::{self, SocketAddr},
};

const MAX_PAYLOAD_SIZE: usize = 1000;

pub struct Server<'a, T = ()> {
    logger: slog::Logger,
    registry: StreamHandlerRegistry<T>,
    socket: Option<net::UdpSocket>,
    ctx: &'a Context,
    new_clients: HashMap<u32, ClientInfo<T>>,
    clients: HashMap<u32, RefCell<ClientInfo<T>>>,
    pub user_handler: Option<fn(packet: QPacket, client: SocketAddr, sock: &net::UdpSocket)>,
}

impl<'a, T> Server<'a, T>
where
    T: Default,
{
    pub fn new(
        logger: slog::Logger,
        ctx: &Context,
        registry: StreamHandlerRegistry<T>,
    ) -> Server<T> {
        Server {
            logger,
            registry,
            socket: None,
            ctx,
            new_clients: Default::default(),
            clients: Default::default(),
            user_handler: None,
        }
    }

    pub fn register(&mut self, vport: VPort, protocol: Box<dyn StreamHandler<T>>) {
        self.registry.register(vport, protocol);
    }

    pub fn bind<A: net::ToSocketAddrs>(&mut self, addrs: A) -> io::Result<()> {
        self.socket = Some(net::UdpSocket::bind(addrs)?);
        info!(
            self.logger,
            "Listening on {}",
            self.socket.as_ref().unwrap().local_addr().unwrap()
        );
        Ok(())
    }

    #[allow(unreachable_code)]
    pub fn serve(mut self) -> io::Result<()> {
        let socket = self
            .socket
            .as_ref()
            .expect("UDP socket required")
            .try_clone()
            .expect("Couldn't clone socket");
        let mut buf = vec![0u8; 1024];
        'outer: loop {
            let (nread, client) = socket.recv_from(&mut buf)?;
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
                let (pdata, ndata) = data.split_at(nparsed as usize);
                data = ndata;
                trace!(logger, "-> {:02x?}", pdata);

                if let Err(e) = packet.validate(self.ctx, pdata) {
                    error!(logger, "Invalid packet received: {:?}", packet; "error" =>  %e);
                    continue;
                };

                self.handle_packet(
                    logger.new(o!("seq" => packet.sequence, "session" => packet.session_id)),
                    packet,
                    client,
                );
            }
        }
        Ok(())
    }

    fn handle_packet(&mut self, logger: Logger, packet: QPacket, client: SocketAddr) {
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
                let ci = match self.clients.get(&packet.signature) {
                    Some(ci) => ci,
                    None => return,
                };
                info!(logger, "Client disconnected"; "signature" => packet.signature, "session" => packet.session_id);
                if self
                    .send_ack(&logger, &client, &packet, &ci.borrow(), false)
                    .is_err()
                {
                    // ignore
                }
            }
            PacketType::Ping => {
                let ci = match self.clients.get(&packet.signature) {
                    Some(ci) => ci,
                    None => return,
                };
                if self
                    .send_ack(&logger, &client, &packet, &ci.borrow(), false)
                    .is_err()
                {
                    // ignore
                }
            }
            PacketType::User => {
                if self.user_handler.is_none() {
                    error!(logger, "unsupported user packet");
                } else {
                    (self.user_handler.as_ref().unwrap())(
                        packet,
                        client,
                        self.socket.as_ref().unwrap(),
                    );
                }
            }
            PacketType::Route => todo!(),
            PacketType::Raw => todo!(),
        }
    }

    fn handle_data(&mut self, logger: Logger, packet: QPacket, client: SocketAddr) {
        debug!(logger, "Handling data packet");
        let ci = match self.clients.get(&packet.signature) {
            Some(ci) => ci,
            None => {
                warn!(logger, "client is unknown!");
                return;
            }
        };
        let logger = logger.new(o!("pid" => ci.borrow().user_id));
        let ci = &mut ci.borrow_mut();
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
                info!(
                    logger,
                    "Reassembled {} fragments",
                    ci.packet_fragments.len() + 1
                );
                ci.packet_fragments.clear();
            }
            payload.extend(packet.payload.into_iter());
            // debug!(logger, "Reassembled: {:?}", payload; "fid" => fid);
            payload
        } else {
            packet.payload
        };
        let resp =
            self.registry
                .handle_packet(&logger, self.ctx, ci, &packet.destination, &payload);
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

    fn handle_syn(&mut self, logger: Logger, mut packet: QPacket, client: SocketAddr) {
        debug!(logger, "Handling syn packet");
        let ci: ClientInfo<T> = ClientInfo::new(client);
        let sig = ci.server_signature;
        self.new_clients.insert(sig, ci);

        packet.conn_signature = Some(sig);

        let ci = self.new_clients.get(&sig).unwrap();

        if let Err(e) = self.send_ack(&logger, &client, &packet, ci, false) {
            error!(logger, "Error sending syn ack packet"; "error" => %e);
        }
    }

    fn handle_connect(&mut self, logger: Logger, mut packet: QPacket, client: SocketAddr) {
        debug!(logger, "Handling connect packet");
        let signature = match packet.conn_signature {
            Some(c) => c,
            None => todo!("deny connect. no signature"),
        };

        let mut ci = match self.new_clients.remove(&packet.signature) {
            Some(c) => c,
            None => todo!("deny connect. no client"),
        };
        ci.client_signature = Some(signature);
        ci.server_session = rand::random();
        ci.client_session = packet.session_id;

        let ci = /* if false {
            // this doesn't work with the borrow checker
            let ci = match self.clients.entry(packet.signature) {
                Entry::Occupied(mut e) => {
                    e.insert(ci);
                    e.into_mut()
                }
                Entry::Vacant(e) => e.insert(ci),
            };
            &*ci
        } else */ {
            self.clients.insert(packet.signature, RefCell::new(ci));
            let ci = self.clients.get(&packet.signature).unwrap();
            ci
        };

        if !packet.payload.is_empty() {
            let mut s = ReadStream::from_bytes(&packet.payload);
            let res = move || -> io::Result<_> {
                let ticket: Vec<u8> = s.read()?;
                let request_data: Vec<u8> = s.read()?;

                let ti = KerberosTicketInternal::open(&ticket)?;

                if ti.valid_until
                    < std::time::SystemTime::now()
                        .duration_since(std::time::SystemTime::UNIX_EPOCH)
                        .as_ref()
                        .map(std::time::Duration::as_secs)
                        .unwrap_or_default()
                {
                    return Ok(vec![]);
                }
                ci.borrow_mut().user_id.replace(ti.principle_id);
                let data = crypt_key(ti.session_key.as_ref(), &request_data);

                #[derive(FromStream, Debug)]
                struct ConnectData {
                    _user_pid: u32,
                    _connection_id: u32,
                    challenge: u32,
                }

                let cd: ConnectData = ReadStream::from_bytes(&data).read()?;

                let resp = cd.challenge + 1;
                let resp = resp.as_bytes();

                Ok(resp.as_bytes())
            }();

            match res {
                Ok(resp) => packet.payload = resp,
                Err(e) => error!(logger, "Error parsing ticket"; "error" => %e),
            }
        }

        packet.conn_signature = Some(0);

        if let Err(e) = self.send_ack(
            &logger,
            &client,
            &packet,
            &ci.borrow(),
            !packet.payload.is_empty(),
        ) {
            error!(logger, "Error sending syn ack packet"; "error" => %e);
        }
        info!(logger, "New client connected"; "signature" => packet.signature, "session" => packet.session_id);
    }

    fn send_response(
        &self,
        logger: &Logger,
        src: &SocketAddr,
        mut resp: QPacket,
        ci: &mut ClientInfo<T>,
    ) -> Result<usize, Box<dyn std::error::Error>> {
        resp.sequence = ci.sequence_id;
        ci.sequence_id += 1;
        resp.flags.insert(PacketFlag::HasSize);
        resp.flags.insert(PacketFlag::NeedAck);
        resp.flags.insert(PacketFlag::Reliable); // ??
        resp.signature = ci.client_signature.unwrap_or_default();
        resp.session_id = ci.server_session;
        self.send_packet(logger, src, resp)
    }

    fn send_packet(
        &self,
        logger: &Logger,
        src: &SocketAddr,
        mut resp: QPacket,
    ) -> Result<usize, Box<dyn std::error::Error>> {
        if matches!(resp.packet_type, PacketType::Data) {
            resp.use_compression = true;
            if resp.fragment_id.is_none() {
                resp.fragment_id = Some(0);
            }
        }
        resp.flags.insert(PacketFlag::HasSize);
        trace!(logger, "<- {:?}", resp);
        let data = &resp.to_bytes(self.ctx);
        trace!(logger, "<- {:02x?}", data);
        let sz = self.socket.as_ref().unwrap().send_to(data, src)?;
        assert_eq!(sz, data.len());
        Ok(sz)
    }

    fn send_ack(
        &self,
        logger: &Logger,
        src: &SocketAddr,
        packet: &QPacket,
        ci: &ClientInfo<T>,
        keep_payload: bool,
    ) -> Result<usize, Box<dyn std::error::Error>> {
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
}
