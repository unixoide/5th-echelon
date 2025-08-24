/// This module handles the RMC (Remote Method Call) protocol.
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

/// An error that can occur during RMC operations.
#[derive(Debug, Display, DeriveError, From)]
pub enum Error {
    /// Not enough data to parse a message.
    #[display("Not enough data. Expected {_0} bytes, got {_1}")]
    MissingData(#[error(not(source))] usize, #[error(not(source))] usize),

    /// An error occurred while parsing a message.
    ParsingError,
    /// The requested protocol is unknown.
    UnknownProtocol,
    /// The requested method is unknown.
    UnknownMethod,
    /// The requested method is not implemented.
    UnimplementedMethod,
    /// The packet type is invalid.
    InvalidPacketType,
    /// An internal error occurred.
    InternalError,
    /// Access to the requested resource is denied.
    AccessDenied,

    /// An I/O error occurred.
    IO(#[error(source)] std::io::Error),
    /// An error occurred while reading from a stream.
    FromStream(#[error(source)] basic::FromStreamError),
}

impl Error {
    /// Converts the error to an error code.
    #[must_use]
    pub fn to_error_code(&self) -> u32 {
        // https://github.com/kinnay/NintendoClients/blob/13a5bdc3723bcc6cd5d0c8bb106250efbce7c165/nintendo/nex/errors.py
        let code = match self {
            Error::UnknownProtocol | Error::UnknownMethod => 0x0001_0001,
            Error::UnimplementedMethod => 0x0001_0002,
            Error::AccessDenied => 0x0001_0006,
            Error::MissingData(_, _) => 0x0001_0009,
            Error::ParsingError | Error::InvalidPacketType | Error::IO(_) | Error::FromStream(_) => 0x0001_000A,
            Error::InternalError => 0x0001_0012,
        };
        code | 0x8000_0000
    }

    /// Creates an error from an error code.
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

/// A result type for RMC operations.
pub type Result<T> = std::result::Result<T, Error>;

/// An RMC request.
#[derive(Debug)]
pub struct Request {
    /// The protocol ID.
    pub protocol_id: u16,
    /// The call ID.
    pub call_id: u32,
    /// The method ID.
    pub method_id: u32,
    /// The parameters for the method call.
    pub parameters: Vec<u8>,
}

impl Request {
    /// Creates a `Request` from a byte buffer.
    /// This function deserializes an RMC request from a byte stream.
    /// It handles the length prefix, protocol ID (which can be 1-byte or 2-byte),
    /// call ID, method ID, and the raw parameters.
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

    /// Converts the request to a byte vector.
    /// This function serializes an RMC request into a byte stream.
    /// It includes the length prefix, handles 1-byte or 2-byte protocol IDs,
    /// and appends the call ID, method ID, and parameters.
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

/// The data of an RMC response.
#[derive(Debug)]
pub struct ResponseData {
    /// The call ID.
    pub call_id: u32,
    /// The method ID.
    pub method_id: u32,
    /// The data of the response.
    pub data: Vec<u8>,
}

impl ResponseData {
    /// Creates a `ResponseData` from a reader.
    /// This function deserializes the data portion of an RMC response.
    /// It reads the call ID, method ID, and the raw response data.
    pub fn from_reader<R: Read>(rdr: &mut R) -> Result<Self> {
        let call_id = rdr.read_u32::<LittleEndian>()?;
        let method_id = rdr.read_u32::<LittleEndian>()? & !0x8000;

        let mut data = Vec::new();
        rdr.read_to_end(&mut data)?;

        Ok(Self { call_id, method_id, data })
    }

    /// Converts the response data to a byte vector.
    /// This function serializes the data portion of an RMC response.
    /// It includes the call ID, method ID (with a flag set), and the raw data.
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut data = vec![];
        data.extend_from_slice(&self.call_id.to_le_bytes());
        data.extend_from_slice(&(self.method_id | 0x8000).to_le_bytes());
        data.extend_from_slice(&self.data);
        data
    }
}

/// An RMC response error.
#[derive(Debug)]
pub struct ResponseError {
    /// The error code.
    pub error_code: u32,
    /// The call ID.
    pub call_id: u32,
}

impl ResponseError {
    /// Creates a `ResponseError` from a reader.
    pub fn from_reader<R: Read>(rdr: &mut R) -> Result<Self> {
        let error_code = rdr.read_u32::<LittleEndian>()?;
        let call_id = rdr.read_u32::<LittleEndian>()?;
        Ok(Self { error_code, call_id })
    }

    /// Converts the response error to a byte vector.
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut data = vec![];
        data.extend_from_slice(&self.error_code.to_le_bytes());
        data.extend_from_slice(&self.call_id.to_le_bytes());
        data
    }
}

/// An RMC response.
#[derive(Debug)]
pub struct Response {
    /// The protocol ID.
    pub protocol_id: u16,
    /// The result of the method call.
    pub result: std::result::Result<ResponseData, ResponseError>,
}

impl Response {
    /// Creates a `Response` from a byte buffer.
    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        let mut rdr = Cursor::new(data);

        let _size = rdr.read_u32::<LittleEndian>()?;

        let protocol_id = rdr.read_u8()?;
        let protocol_id = if protocol_id == 0x7f { rdr.read_u16::<LittleEndian>()? } else { u16::from(protocol_id) };
        let status = rdr.read_u8()?;
        let result = match status {
            0 => Err(ResponseError::from_reader(&mut rdr)?),
            1 => Ok(ResponseData::from_reader(&mut rdr)?),
            _v => return Err(Error::InvalidPacketType),
        };

        Ok(Self { protocol_id, result })
    }

    /// Converts the response to a byte vector.
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

/// An RMC packet.
#[derive(Debug)]
pub enum Packet {
    /// An RMC request.
    Request(Request),
    /// An RMC response.
    Response(Response),
}

impl Packet {
    /// Creates a `Packet` from a byte buffer.
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

    /// Converts the packet to a byte vector.
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Packet::Request(r) => r.to_bytes(),
            Packet::Response(r) => r.to_bytes(),
        }
    }
}

/// A trait for RMC protocols.
pub trait Protocol<T> {
    /// Returns the protocol ID.
    fn id(&self) -> u16;
    /// Returns the protocol name.
    fn name(&self) -> String;
    /// Returns the number of methods in the protocol.
    fn num_methods(&self) -> u32;
    /// Handles an RMC request.
    fn handle(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<T>,
        request: &Request,
        client_registry: &ClientRegistry<T>,
        socket: &std::net::UdpSocket,
    ) -> std::result::Result<Vec<u8>, Error>;
    /// Returns the name of a method.
    fn method_name(&self, method_id: u32) -> Option<String>;
}

/// A trait for RMC client protocols.
pub trait ClientProtocol<T> {
    /// Returns the protocol ID.
    fn id(&self) -> u16;
    /// Returns the protocol name.
    fn name(&self) -> String;
    /// Returns the number of methods in the protocol.
    fn num_methods(&self) -> u32;
    /// Returns the name of a method.
    fn method_name(&self, method_id: u32) -> Option<String>;

    /// Sends an RMC request.
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

/// A handler for RVSec packets.
pub struct RVSecHandler<T> {
    logger: slog::Logger,
    rmc_registry: HashMap<u16, Box<dyn Protocol<T>>>,
}

impl<T> RVSecHandler<T> {
    /// Creates a new `RVSecHandler`.
    #[must_use]
    pub fn new(logger: slog::Logger) -> Self {
        Self {
            logger,
            rmc_registry: HashMap::default(),
        }
    }

    /// Registers a protocol handler.
    pub fn register_protocol(&mut self, protocol: Box<dyn Protocol<T>>) {
        debug!(self.logger, "Registering handler for protocol {} ({})", protocol.id(), protocol.name(),);
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

        info!(logger, "Looking for protocol {}, method {}", rmc_packet.protocol_id, rmc_packet.method_id);

        let logger = logger.new(o!(
            "protocol_id" => rmc_packet.protocol_id,
            "method_id" => rmc_packet.method_id,
            "call" => rmc_packet.call_id
        ));

        let protocol = self.rmc_registry.get(&rmc_packet.protocol_id);

        let maybe_protocol = if let Some(protocol) = protocol {
            info!(logger, "Calling {}.{}", protocol.name(), protocol.method_name(rmc_packet.method_id).unwrap_or_default(),);

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
                result: Err(ResponseError { call_id: 2, error_code: 3 }),
            }
            .to_bytes(),
            b"\x0a\x00\x00\x00\x01\x00\x03\x00\x00\x00\x02\x00\x00\x00".to_vec()
        );
    }

    #[test]
    fn test_request() {
        let data = [
            0x48, 0x00, 0x00, 0x00, 0x8a, 0x08, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x03, 0x00, 0x77, 0x76, 0x00, 0x21, 0x00, 0x55, 0x62, 0x69, 0x41, 0x75, 0x74, 0x68, 0x65,
            0x6e, 0x74, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x4c, 0x6f, 0x67, 0x69, 0x6e, 0x43, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x44, 0x61, 0x74, 0x61, 0x00, 0x13, 0x00, 0x00,
            0x00, 0x0f, 0x00, 0x00, 0x00, 0x03, 0x00, 0x77, 0x76, 0x00, 0x01, 0x00, 0x00, 0x05, 0x00, 0x74, 0x65, 0x73, 0x74, 0x00,
        ];

        let _req = dbg!(Request::from_bytes(&data)).unwrap();
    }
}
