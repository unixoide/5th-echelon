#![feature(seek_stream_len, proc_macro_hygiene)]

#[macro_use]
extern crate slog;

#[macro_use]
extern crate quazal_macros;

use crate::prudp::packet::QPacket;
use crate::prudp::packet::{PacketFlag, PacketType};
use crate::rmc::Protocol;
use derive_more::{Display, Error as DeriveError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::{SocketAddr, UdpSocket};

pub mod config;
pub mod prudp;
pub mod rmc;
pub use crate::config::*;

#[derive(Debug, Display, DeriveError)]
pub enum Error {
    #[display(fmt = "Service {} not found", _0)]
    ServiceNotFound(#[error(not(source))] String),
}

#[derive(Debug, Default)]
pub struct ClientInfo {
    sequence_id: u16,
    client_signature: Option<u32>,
    server_signature: u32,
    client_session: u8,
    server_session: u8,
    username: Option<String>,
    ticket: Option<Vec<u8>>,
    packet_fragments: HashMap<u8, Vec<u8>>,
}

impl ClientInfo {
    fn new() -> ClientInfo {
        ClientInfo {
            server_signature: rand::random(),
            sequence_id: 1,
            ..Default::default()
        }
    }
}

pub struct Server {
    socket: Option<UdpSocket>,
    ctx: Context,
    clients: HashMap<SocketAddr, ClientInfo>,
    protocols: HashMap<u16, Box<dyn Protocol>>,
}

impl Server {
    pub fn new(ctx: Context) -> Server {
        Server {
            socket: None,
            ctx,
            clients: HashMap::new(),
            protocols: HashMap::new(),
        }
    }

    pub fn register_protocol(&mut self, protocol: Box<dyn Protocol>) {
        self.protocols.insert(protocol.id(), protocol);
    }

    #[allow(unreachable_code)]
    pub fn serve(&mut self, port: usize) -> Result<(), Box<dyn std::error::Error>> {
        self.socket = Some(UdpSocket::bind(format!("0.0.0.0:{}", port))?);

        let mut buf = [0; 8 * 1024];
        loop {
            let (size, src) = self.socket.as_ref().unwrap().recv_from(&mut buf)?;
            self.handle_client(src, &buf[..size])?;
        }
        Ok(())
    }

    fn handle_client(
        &mut self,
        src: SocketAddr,
        data: &[u8],
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("{} -> {:02x?}", src, data);
        let packet = dbg!(QPacket::from_bytes(&self.ctx, data))?.0;
        packet.validate(&self.ctx, data)?;

        if packet.flags.contains(PacketFlag::Ack) {
            return Ok(());
        }

        let mut resp = match packet.packet_type {
            PacketType::Syn => {
                let sig = 0x12345678;
                let client = ClientInfo {
                    server_signature: sig,
                    ..Default::default()
                };
                self.clients.insert(src, client);
                QPacket {
                    source: packet.destination,
                    destination: packet.source,
                    packet_type: PacketType::Syn,
                    flags: PacketFlag::Ack.into(),
                    conn_signature: Some(sig),
                    ..Default::default()
                }
            }
            PacketType::Connect => {
                let ci = self.clients.get_mut(&src).unwrap();
                ci.client_signature = packet.conn_signature;
                let payload = if !packet.payload.is_empty() {
                    let mut s = rmc::basic::ReadStream::from_bytes(&packet.payload);
                    let _ticket: Vec<u8> = s.read()?;
                    let request_data: Vec<u8> = s.read()?;
                    let data = dbg!(prudp::packet::crypt_key(
                        &[
                            0x9C, 0xB0, 0x1D, 0x7A, 0x2C, 0x5A, 0x6C, 0x5B, 0xED, 0x12, 0x68, 0x45,
                            0x69, 0xAE, 0x09, 0x0D,
                        ],
                        &request_data
                    ));
                    let mut s = rmc::basic::ReadStream::from_bytes(&data);
                    let _user_pid: u32 = s.read()?;
                    let _connection_id: u32 = s.read()?;
                    let check_value: u32 = s.read()?;
                    let mut p = (check_value + 1).to_le_bytes().to_vec();
                    p.insert(0, 0);
                    p.insert(0, 0);
                    p.insert(0, 0);
                    p.insert(0, 4);
                    p
                } else {
                    vec![]
                };
                QPacket {
                    source: packet.destination,
                    destination: packet.source,
                    packet_type: PacketType::Connect,
                    flags: PacketFlag::Ack.into(),
                    session_id: packet.session_id,
                    signature: packet.conn_signature.unwrap(),
                    conn_signature: Some(0x12345678),
                    payload,
                    ..Default::default()
                }
            }
            PacketType::Data => {
                if packet.flags.contains(PacketFlag::NeedAck) {
                    let ci = self.clients.get(&src).unwrap();
                    self.send_ack(&src, &packet, ci)?;
                }
                let ci = self.clients.get_mut(&src).unwrap();
                ci.sequence_id += 1;
                let rmcp = dbg!(rmc::Packet::from_bytes(&packet.payload))?;
                if let rmc::Packet::Request(request) = rmcp {
                    let resp = match self.protocols.get(&request.protocol_id) {
                        Some(proto) if proto.num_methods() >= request.method_id => {
                            println!(
                                "Executing Method {:?} on {}",
                                proto.method_name(request.method_id),
                                proto.name(),
                            );
                            panic!("foo");
                            // proto.handle(ci, &src, &packet, &request)
                        }
                        None => rmc::Response {
                            protocol_id: request.protocol_id,
                            result: Err(rmc::ResponseError {
                                error_code: 0x80010001,
                                call_id: request.call_id,
                            }),
                        },
                        _ => rmc::Response {
                            protocol_id: request.protocol_id,
                            result: Err(rmc::ResponseError {
                                error_code: 0x80010001,
                                call_id: request.call_id,
                            }),
                        },
                    };
                    QPacket {
                        source: packet.destination,
                        destination: packet.source,
                        packet_type: PacketType::Data,
                        flags: PacketFlag::NeedAck.into(),
                        session_id: packet.session_id,
                        payload: dbg!(resp).to_bytes(),
                        signature: ci.client_signature.unwrap(),
                        fragment_id: Some(0),
                        use_compression: true,
                        ..Default::default()
                    }
                } else {
                    return Ok(());
                }
            }
            PacketType::Disconnect => {
                println!("DISCONNECT");
                let ci = self.clients.get(&src).unwrap();
                QPacket {
                    source: packet.destination,
                    destination: packet.source,
                    packet_type: PacketType::Disconnect,
                    flags: PacketFlag::Ack.into(),
                    session_id: packet.session_id,
                    signature: ci.client_signature.unwrap(),
                    ..Default::default()
                }
            }
            PacketType::Ping => {
                let ci = self.clients.get(&src).unwrap();
                QPacket {
                    source: packet.destination,
                    destination: packet.source,
                    packet_type: PacketType::Ping,
                    flags: PacketFlag::Ack.into(),
                    session_id: packet.session_id,
                    signature: ci.client_signature.unwrap(),
                    ..Default::default()
                }
            }
        };

        let _client = self.clients.get_mut(&src);
        // resp.sequence = client.as_ref().map(|c| c.sequence_id).unwrap_or_default();
        // if let Some(client) = client {
        //     client.sequence_id += 1;
        // }
        resp.sequence = packet.sequence;
        self.send_packet(&src, resp)?;

        Ok(())
    }

    fn send_packet(
        &self,
        src: &SocketAddr,
        mut resp: QPacket,
    ) -> Result<usize, Box<dyn std::error::Error>> {
        if matches!(resp.packet_type, PacketType::Data) {
            resp.fragment_id = Some(0);
        }
        let data = &dbg!(resp).to_bytes(&self.ctx);
        println!("{} <- {:02x?}", src, data);
        Ok(self.socket.as_ref().unwrap().send_to(data, src)?)
    }

    fn send_ack(
        &self,
        src: &SocketAddr,
        packet: &QPacket,
        ci: &ClientInfo,
    ) -> Result<usize, Box<dyn std::error::Error>> {
        let mut resp = packet.clone();
        resp.source = packet.destination;
        resp.destination = packet.source;
        resp.flags = PacketFlag::Ack.into();
        resp.signature = ci.client_signature.unwrap();
        resp.payload.clear();
        resp.sequence = packet.sequence;
        self.send_packet(src, resp)
    }
}
