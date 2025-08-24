use std::collections::HashMap;
use std::io::Cursor;
use std::io::Read;

use byteorder::LittleEndian;
use byteorder::ReadBytesExt;
use derive_more::Display;
use derive_more::Error as DeriveError;
use derive_more::From;
use slog::Logger;

use crate::prudp::packet;
use crate::prudp::packet::QPacket;
use crate::prudp::packet::StreamHandler;
use crate::prudp::ClientRegistry;
use crate::ClientInfo;
use crate::Context;

pub mod basic;
pub mod result;
pub mod types;
// pub mod protocols;

#[derive(Debug, Display, DeriveError, From)]
pub enum Error {
    #[display("Not enough data. Expected {_0} bytes, got {_1}")]
    MissingData(#[error(not(source))] usize, #[error(not(source))] usize),

    ParsingError,
    UnknownProtocol,
    UnknownMethod,
    UnimplementedMethod,
    InvalidPacketType,
    InternalError,
    AccessDenied,

    IO(#[error(source)] std::io::Error),
    FromStream(#[error(source)] basic::FromStreamError),
}

impl Error {
    #[must_use]
    pub fn to_error_code(&self) -> u32 {
        let code = match self {
            // https://github.com/kinnay/NintendoClients/blob/13a5bdc3723bcc6cd5d0c8bb106250efbce7c165/nintendo/nex/errors.py
            Error::UnknownProtocol | Error::UnknownMethod => 0x0001_0001,
            Error::UnimplementedMethod => 0x0001_0002,
            Error::AccessDenied => 0x0001_0006,
            Error::MissingData(_, _) => 0x0001_0009,
            Error::ParsingError | Error::InvalidPacketType | Error::IO(_) | Error::FromStream(_) => 0x0001_000A,
            Error::InternalError => 0x0001_0012,
        };
        code | 0x8000_0000
    }

    pub fn from_error_code(code: u32) -> std::result::Result<Self, u32> {
        let code = if code & 0x8000_0000 != 0 {
            code & !0x8000_0000
        } else {
            return Err(code);
        };

        let err = match code {
            0x0001_0001 => Error::UnknownProtocol,
            0x0001_0002 => Error::UnimplementedMethod,
            0x0001_0006 => Error::AccessDenied,
            0x0001_0009 => Error::MissingData(0, 0),
            0x0001_000A => Error::ParsingError,
            0x0001_0012 => Error::InternalError,
            _ => return Err(code),
        };

        Ok(err)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Request {
    pub protocol_id: u16,
    pub call_id: u32,
    pub method_id: u32,
    pub parameters: Vec<u8>,
}

impl Request {
    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        let mut rdr = Cursor::new(data);
        let size = rdr.read_u32::<LittleEndian>()?;
        if (size as usize) < data.len() - 4 {
            return Err(Error::MissingData(size as usize, data.len() - 4));
        }
        let protocol_id = rdr.read_u8()?;
        let protocol_id = if protocol_id == 0xff {
            rdr.read_u16::<LittleEndian>()?
        } else {
            u16::from(protocol_id & !0x80)
        };
        let call_id = rdr.read_u32::<LittleEndian>()?;
        let method_id = rdr.read_u32::<LittleEndian>()?;
        let mut parameters = vec![];
        rdr.read_to_end(&mut parameters)?;
        Ok(Self {
            protocol_id,
            call_id,
            method_id,
            parameters,
        })
    }

    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut data = vec![];
        if self.protocol_id < 0xff {
            #[allow(clippy::cast_possible_truncation)]
            data.push(self.protocol_id as u8 | 0x80);
        } else {
            data.push(0xff);
            data.extend_from_slice(&self.protocol_id.to_le_bytes());
        }
        data.extend_from_slice(&self.call_id.to_le_bytes());
        data.extend_from_slice(&self.method_id.to_le_bytes());
        data.extend_from_slice(&self.parameters);
        let mut res = vec![];
        #[allow(clippy::cast_possible_truncation)]
        res.extend_from_slice(&(data.len() as u32).to_le_bytes());
        res.extend(data);
        res
    }
}

#[derive(Debug)]
pub struct ResponseData {
    pub call_id: u32,
    pub method_id: u32,
    pub data: Vec<u8>,
}

impl ResponseData {
    pub fn from_reader<R: Read>(rdr: &mut R) -> Result<Self> {
        let call_id = rdr.read_u32::<LittleEndian>()?;
        let method_id = rdr.read_u32::<LittleEndian>()? & !0x8000;

        let mut data = Vec::new();
        rdr.read_to_end(&mut data)?;

        Ok(Self {
            call_id,
            method_id,
            data,
        })
    }

    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut data = vec![];
        data.extend_from_slice(&self.call_id.to_le_bytes());
        data.extend_from_slice(&(self.method_id | 0x8000).to_le_bytes());
        data.extend_from_slice(&self.data);
        data
    }
}

#[derive(Debug)]
pub struct ResponseError {
    pub error_code: u32,
    pub call_id: u32,
}

impl ResponseError {
    pub fn from_reader<R: Read>(rdr: &mut R) -> Result<Self> {
        let error_code = rdr.read_u32::<LittleEndian>()?;
        let call_id = rdr.read_u32::<LittleEndian>()?;
        Ok(Self { error_code, call_id })
    }

    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut data = vec![];
        data.extend_from_slice(&self.error_code.to_le_bytes());
        data.extend_from_slice(&self.call_id.to_le_bytes());
        data
    }
}

#[derive(Debug)]
pub struct Response {
    pub protocol_id: u16,
    pub result: std::result::Result<ResponseData, ResponseError>,
}

impl Response {
    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        let mut rdr = Cursor::new(data);

        let _size = rdr.read_u32::<LittleEndian>()?;
        // assert_eq!(size as usize, data.len() - 4);

        let protocol_id = rdr.read_u8()?;
        let protocol_id = if protocol_id == 0x7f {
            rdr.read_u16::<LittleEndian>()?
        } else {
            u16::from(protocol_id)
        };
        let status = rdr.read_u8()?;
        let result = match status {
            0 => Err(ResponseError::from_reader(&mut rdr)?),
            1 => Ok(ResponseData::from_reader(&mut rdr)?),
            _v => return Err(Error::InvalidPacketType),
        };

        Ok(Self { protocol_id, result })
    }

    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut data = vec![0, 0, 0, 0];
        if self.protocol_id < 0x7f {
            #[allow(clippy::cast_possible_truncation)]
            data.push(self.protocol_id as u8);
        } else {
            data.push(0x7f);
            data.push((self.protocol_id & 0xff) as u8);
            data.push((self.protocol_id >> 8) as u8);
        }
        match self.result {
            Ok(ref rd) => {
                data.push(1);
                data.append(&mut rd.to_bytes());
            }
            Err(ref re) => {
                data.push(0);
                data.append(&mut re.to_bytes());
            }
        };
        #[allow(clippy::cast_possible_truncation)]
        let len = (data.len() - 4) as u32;
        let sz = &mut data[..4];
        sz.copy_from_slice(&len.to_le_bytes());
        data
    }
}

#[derive(Debug)]
pub enum Packet {
    Request(Request),
    Response(Response),
}

impl Packet {
    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        if data.len() < 5 {
            return Err(Error::MissingData(5, data.len()));
        }
        if data[4] & 0x80 == 0 {
            Ok(Packet::Response(Response::from_bytes(data)?))
        } else {
            Ok(Packet::Request(Request::from_bytes(data)?))
        }
    }

    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Packet::Request(r) => r.to_bytes(),
            Packet::Response(r) => r.to_bytes(),
        }
    }
}

pub trait Protocol<T> {
    fn id(&self) -> u16;
    fn name(&self) -> String;
    fn num_methods(&self) -> u32;
    fn handle(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<T>,
        request: &Request,
        client_registry: &ClientRegistry<T>,
        socket: &std::net::UdpSocket,
    ) -> std::result::Result<Vec<u8>, Error>;
    fn method_name(&self, method_id: u32) -> Option<String>;
}

pub trait ClientProtocol<T> {
    fn id(&self) -> u16;
    fn name(&self) -> String;
    fn num_methods(&self) -> u32;
    fn method_name(&self, method_id: u32) -> Option<String>;

    fn send(&self, logger: &Logger, ctx: &Context, ci: &mut ClientInfo<T>, method_id: u32, parameters: Vec<u8>) {
        let request = Request {
            protocol_id: self.id(),
            method_id,
            parameters,
            call_id: todo!(),
        };
        let req = QPacket {
            payload: request.to_bytes(),
            ..Default::default()
        };
        let _ = super::prudp::send_request(logger, ctx, todo!(), todo!(), req, ci);
    }
}

pub struct RVSecHandler<T> {
    logger: slog::Logger,
    rmc_registry: HashMap<u16, Box<dyn Protocol<T>>>,
}

impl<T> RVSecHandler<T> {
    #[must_use]
    pub fn new(logger: slog::Logger) -> Self {
        Self {
            logger,
            rmc_registry: HashMap::default(),
        }
    }

    pub fn register_protocol(&mut self, protocol: Box<dyn Protocol<T>>) {
        debug!(
            self.logger,
            "Registering handler for protocol {} ({})",
            protocol.id(),
            protocol.name(),
        );
        self.rmc_registry.insert(protocol.id(), protocol);
    }
}

impl<T> StreamHandler<T> for RVSecHandler<T> {
    fn handle(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<T>,
        data: &[u8],
        client_registry: &ClientRegistry<T>,
        socket: &std::net::UdpSocket,
        // packet: QPacket,
        // _client: &SocketAddr,
    ) -> std::result::Result<Vec<u8>, packet::Error> {
        let rmc_packet = Packet::from_bytes(data);

        let Ok(rmc_packet) = rmc_packet else {
            let err = rmc_packet.err().unwrap();
            error!(logger, "Parsing RMC packet failed"; "error" => %err);
            return Err(packet::Error::StreamHandler(Box::new(err)));
        };

        let rmc_packet = match rmc_packet {
            Packet::Request(r) => r,
            Packet::Response(_r) => {
                error!(logger, "RMC response not supported here. Data: {:#?}", data);
                return Err(packet::Error::Unimplemented);
            }
        };

        info!(
            logger,
            "Looking for protocol {}, method {}", rmc_packet.protocol_id, rmc_packet.method_id
        );

        let logger = logger
            .new(o!("protocol_id" => rmc_packet.protocol_id, "method_id" => rmc_packet.method_id, "call" => rmc_packet.call_id));

        let protocol = self.rmc_registry.get(&rmc_packet.protocol_id);

        let maybe_protocol = if let Some(protocol) = protocol {
            info!(
                logger,
                "Calling {}.{}",
                protocol.name(),
                protocol.method_name(rmc_packet.method_id).unwrap_or_default(),
            );

            protocol.handle(&logger, ctx, ci, &rmc_packet, client_registry, socket)
        } else {
            warn!(logger, "no handler available");
            Err(Error::UnknownProtocol)
        };

        let result = match maybe_protocol {
            Err(e) => {
                error!(logger, "handling request failed"; "error" => %e);
                Err(ResponseError {
                    error_code: e.to_error_code(),
                    call_id: rmc_packet.call_id,
                })
            }
            Ok(data) => Ok(ResponseData {
                call_id: rmc_packet.call_id,
                method_id: rmc_packet.method_id,
                data,
            }),
        };

        let resp = Response {
            protocol_id: rmc_packet.protocol_id,
            result,
        };
        trace!(logger, "<- {:?}", resp);
        Ok(resp.to_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response() {
        assert_eq!(
            Response {
                protocol_id: 1,
                result: Ok(ResponseData {
                    call_id: 2,
                    method_id: 3,
                    data: b"Hello".to_vec(),
                }),
            }
            .to_bytes(),
            b"\x0f\x00\x00\x00\x01\x01\x02\x00\x00\x00\x03\x80\x00\x00Hello".to_vec()
        );

        assert_eq!(
            Response {
                protocol_id: 1,
                result: Err(ResponseError {
                    call_id: 2,
                    error_code: 3,
                }),
            }
            .to_bytes(),
            b"\x0a\x00\x00\x00\x01\x00\x03\x00\x00\x00\x02\x00\x00\x00".to_vec()
        );
    }

    #[test]
    fn test_request() {
        let data = [
            0x48, 0x00, 0x00, 0x00, 0x8a, 0x08, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x03, 0x00, 0x77, 0x76, 0x00,
            0x21, 0x00, 0x55, 0x62, 0x69, 0x41, 0x75, 0x74, 0x68, 0x65, 0x6e, 0x74, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f,
            0x6e, 0x4c, 0x6f, 0x67, 0x69, 0x6e, 0x43, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x44, 0x61, 0x74, 0x61, 0x00, 0x13,
            0x00, 0x00, 0x00, 0x0f, 0x00, 0x00, 0x00, 0x03, 0x00, 0x77, 0x76, 0x00, 0x01, 0x00, 0x00, 0x05, 0x00, 0x74,
            0x65, 0x73, 0x74, 0x00,
        ];

        let _req = dbg!(Request::from_bytes(&data)).unwrap();
    }
}
