/// This module defines the structure and handling of PRUDP packets.
use std::collections::HashMap;
use std::convert::TryFrom;
use std::io::Cursor;
use std::io::Read;
use std::io::Seek;
use std::iter;
use std::num::Wrapping;

use byteorder::LittleEndian;
use byteorder::ReadBytesExt;
use derive_more::Display;
use derive_more::Error as DeriveError;
use derive_more::From;
use enumflags2::bitflags;
use enumflags2::BitFlags;
use hmac::Hmac;
use hmac::Mac;
use md5::Digest;
use md5::Md5;
use miniz_oxide::deflate::compress_to_vec_zlib;
use miniz_oxide::inflate::decompress_to_vec_zlib;
use miniz_oxide::inflate::DecompressError;
use num_enum::IntoPrimitive;
use num_enum::TryFromPrimitive;
use slog::Logger;

use super::ClientRegistry;
use crate::ClientInfo;
use crate::Context;

/// Errors that can occur during packet processing.
#[derive(Debug, Display, DeriveError, From)]
pub enum Error {
    /// An invalid flag was encountered.
    #[display("Invalid Flag {_0}")]
    #[from(ignore)]
    InvalidFlag(#[error(ignore)] u8),

    /// The packet checksum is invalid.
    #[display("Invalid checksum")]
    InvalidChecksum,

    /// Decompression of the payload failed.
    #[display("Decompression failed {_0:?}")]
    DecompressFailed(#[error(ignore)] DecompressError),

    /// An invalid packet type was encountered.
    #[display("Invalid Packet Type {_0}")]
    #[from(ignore)]
    InvalidPacketType(#[error(ignore)] u8),

    /// An invalid stream type was encountered.
    #[display("Invalid Stream Type {_0}")]
    #[from(ignore)]
    InvalidStreamType(#[error(ignore)] u8),

    /// An I/O error occurred.
    #[display("I/O error {_0}")]
    IO(#[error(source)] std::io::Error),

    /// An error occurred in a stream handler.
    #[display("Stream handler error {_0}")]
    #[from(ignore)]
    StreamHandler(#[error(ignore)] Box<dyn std::error::Error + Send>),

    /// The requested operation is unimplemented.
    Unimplemented,
}

/// Represents the type of stream.
#[derive(Debug, TryFromPrimitive, IntoPrimitive, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(u8)]
pub enum StreamType {
    #[default]
    DO = 1,
    RV = 2,
    RVSec = 3,
    SBMGMT = 4,
    NAT = 5,
    SessionDiscovery = 6,
    NATEcho = 7,
    Routing = 8,
}

/// Trait for handling PRUDP streams.
pub trait StreamHandler<T> {
    /// Handles an incoming packet.
    fn handle(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<T>,
        data: &[u8],
        client_registry: &ClientRegistry<T>,
        socket: &std::net::UdpSocket,
    ) -> Result<Vec<u8>, Error>;
}

/// A registry for stream handlers.
pub struct StreamHandlerRegistry<T> {
    logger: slog::Logger,
    handlers: HashMap<VPort, Box<dyn StreamHandler<T>>>,
}

impl<T> StreamHandlerRegistry<T> {
    /// Creates a new `StreamHandlerRegistry`.
    #[must_use]
    pub fn new(logger: slog::Logger) -> Self {
        Self {
            logger,
            handlers: HashMap::default(),
        }
    }

    /// Registers a stream handler for a given virtual port.
    pub fn register(&mut self, port: VPort, handler: Box<dyn StreamHandler<T>>) {
        debug!(self.logger, "Registering handler for stream type {:?}", port);
        self.handlers.insert(port, handler);
    }

    /// Handles an incoming packet by dispatching it to the appropriate handler.
    #[allow(clippy::too_many_arguments)]
    pub fn handle_packet(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<T>,
        dest: &VPort,
        data: &[u8],
        client_registry: &ClientRegistry<T>,
        socket: &std::net::UdpSocket,
    ) -> Option<Result<Vec<u8>, Error>> {
        self.handlers.get(dest).map(|h| h.handle(logger, ctx, ci, data, client_registry, socket))
    }
}

/// Represents a virtual port, consisting of a port number and a stream type.
#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
pub struct VPort {
    /// The port number.
    pub port: u8,
    /// The stream type.
    pub stream_type: StreamType,
}

impl VPort {
    /// Creates a `VPort` from a reader.
    pub fn from_reader<R: Read>(rdr: &mut R) -> Result<Self, Error> {
        let val = rdr.read_u8()?;
        Ok(Self {
            port: val & 0xF,
            stream_type: StreamType::try_from(val >> 4).map_err(|e| Error::InvalidStreamType(e.number))?,
        })
    }

    /// Converts the `VPort` to a byte vector.
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        let st: u8 = self.stream_type.into();
        vec![self.port | (st << 4)]
    }
}

/// Represents the type of a PRUDP packet.
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, TryFromPrimitive, IntoPrimitive, Clone, Copy, Default, PartialEq, Eq)]
#[repr(u8)]
pub enum PacketType {
    #[default]
    Syn = 0,
    Connect = 1,
    Data = 2,
    Disconnect = 3,
    Ping = 4,
    User = 5,
    Route = 6,
    Raw = 7,
}

/// Flags that can be set on a PRUDP packet.
#[allow(clippy::module_name_repetitions)]
#[bitflags]
#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum PacketFlag {
    /// Acknowledgment flag.
    Ack = 0b0001,      // 1
    /// Reliable delivery flag.
    Reliable = 0b0010, // 2
    /// Acknowledgment is required.
    NeedAck = 0b0100,  // 4
    /// The packet has a size field.
    HasSize = 0b1000,  // 8
}

/// Represents a PRUDP packet.
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct QPacket {
    /// Source virtual port.
    pub source: VPort,
    /// Destination virtual port.
    pub destination: VPort,
    /// Type of the packet.
    pub packet_type: PacketType,
    /// Packet flags.
    pub flags: BitFlags<PacketFlag>,
    /// Session ID.
    pub session_id: u8,
    /// Packet signature.
    pub signature: u32,
    /// Fragment ID, if the packet is fragmented.
    pub fragment_id: Option<u8>,
    /// Connection signature, used during connection establishment.
    pub conn_signature: Option<u32>,
    /// Sequence number of the packet.
    pub sequence: u16,
    /// Payload of the packet.
    pub payload: Vec<u8>,
    /// Checksum of the packet.
    pub checksum: u8,
    /// Indicates if compression is used for the payload.
    pub use_compression: bool,
}

impl QPacket {
    /// Creates a `QPacket` from a byte slice.
    pub fn from_bytes(ctx: &Context, data: &[u8]) -> Result<(Self, u64), Error> {
        let mut rdr = Cursor::new(data);
        let start = rdr.stream_position()?;
        let p = Self::from_reader(ctx, &mut rdr)?;
        let end = rdr.stream_position()?;
        Ok((p, end - start))
    }

    /// Creates a `QPacket` from a reader.
    /// Creates a `QPacket` from a reader.
    /// This function parses the raw bytes of a PRUDP packet into the `QPacket` struct.
    /// It handles various fields, flags, optional fields (fragment ID, connection signature),
    /// and performs decompression/encryption if applicable.
    pub fn from_reader<R>(ctx: &Context, rdr: &mut R) -> Result<Self, Error>
    where
        R: ReadBytesExt + std::io::Seek,
    {
        #![allow(clippy::cast_possible_truncation)]

        let source = VPort::from_reader(rdr)?;
        let destination = VPort::from_reader(rdr)?;
        let type_flag = rdr.read_u8()?;
        let packet_type = PacketType::try_from(type_flag & 0x7).map_err(|e| Error::InvalidPacketType(e.number))?;
        let flags = BitFlags::from_bits(type_flag >> 3).map_err(|e| Error::InvalidFlag(e.invalid_bits()))?;
        let session_id = rdr.read_u8()?;
        let signature = rdr.read_u32::<LittleEndian>()?;
        let sequence = rdr.read_u16::<LittleEndian>()?;

        let conn_signature = if matches!(packet_type, PacketType::Syn | PacketType::Connect) {
            Some(rdr.read_u32::<LittleEndian>()?)
        } else {
            None
        };

        let fragment_id = if matches!(packet_type, PacketType::Data) { Some(rdr.read_u8()?) } else { None };

        let payload_size = if flags.contains(PacketFlag::HasSize) {
            rdr.read_u16::<LittleEndian>()? as usize
        } else {
            let l = rdr.stream_len();
            let p = rdr.stream_position();
            l.and_then(|l| p.map(|p| l - p - 1)).expect("getting length and position from buffer should never fail") as usize
        };

        let mut payload = vec![0u8; payload_size];
        rdr.read_exact(&mut payload)?;

        let use_compression = if !matches!(packet_type, PacketType::Syn) && matches!(source.stream_type, StreamType::RVSec) {
            // Decrypt the payload if it's an RVSec stream and not a Syn packet
            payload = crypt(ctx, &payload);
            let use_compression = !payload.is_empty() && payload[0] != 0;
            if use_compression {
                // Decompress the payload if the first byte indicates compression
                payload = decompress_to_vec_zlib(&payload.as_slice()[1..]).map_err(Error::DecompressFailed)?;
            } else if !payload.is_empty() {
                // If not compressed, remove the first byte (which was the compression flag)
                payload.remove(0);
            }
            use_compression
        } else {
            false
        };
        let checksum = rdr.read_u8()?;

        Ok(Self {
            source,
            destination,
            packet_type,
            flags,
            session_id,
            signature,
            fragment_id,
            conn_signature,
            sequence,
            payload,
            checksum,
            use_compression,
        })
    }

    /// Calculates the checksum of the packet.
    #[must_use]
    pub(crate) fn calc_checksum(&self, ctx: &Context) -> u8 {
        calc_checksum_from_data(ctx.key(self.destination.stream_type), &self.to_data_bytes(ctx))
    }

    /// Calculates the signature of the packet.
    #[must_use]
    pub(crate) fn calc_signature(&self, ctx: &Context, payload: &[u8]) -> u32 {
        match self.packet_type {
            PacketType::Data => {
                if payload.is_empty() {
                    0x1234_5678
                } else {
                    let key = Md5::digest(&ctx.access_key);
                    let mut mac = Hmac::<Md5>::new_from_slice(&key).expect("Key size is constant");
                    mac.update(payload);
                    let result = mac.finalize();
                    let mut rdr = Cursor::new(result.into_bytes());
                    rdr.read_u32::<LittleEndian>().expect("convert digest to u32")
                }
            }
            PacketType::Syn => todo!(),
            PacketType::Connect => todo!(),
            PacketType::Disconnect => todo!(),
            PacketType::Ping => todo!(),
            PacketType::User => todo!(),
            PacketType::Route => todo!(),
            PacketType::Raw => todo!(),
        }
    }

    /// Converts the packet data to a byte vector.
    /// Converts the packet data to a byte vector.
    /// This function serializes the `QPacket` struct into raw bytes.
    /// It handles various fields, flags, optional fields, and performs
    /// compression and encryption if applicable.
    fn to_data_bytes(&self, ctx: &Context) -> Vec<u8> {
        let mut data = vec![];
        data.append(&mut self.source.to_bytes());
        data.append(&mut self.destination.to_bytes());

        let pt: u8 = self.packet_type.into();
        data.push(pt | (self.flags.bits() << 3));
        data.push(self.session_id);
        data.extend_from_slice(&self.signature.to_le_bytes());
        data.extend_from_slice(&self.sequence.to_le_bytes());

        match self.packet_type {
            PacketType::Syn | PacketType::Connect => {
                data.extend_from_slice(&self.conn_signature.expect("connection signature required").to_le_bytes());
            }
            PacketType::Data => {
                data.push(self.fragment_id.expect("fragment id required"));
            }
            PacketType::Disconnect | PacketType::Ping | PacketType::User | PacketType::Route | PacketType::Raw => {} // No additional fields for these types
        }

        let payload = if self.payload.is_empty() {
            vec![]
        } else if self.use_compression {
            // Compress the payload and prepend a byte indicating compression ratio
            let mut tmp = compress_to_vec_zlib(&self.payload, 6);
            #[allow(clippy::cast_possible_truncation)]
            tmp.insert(0, (self.payload.len() / tmp.len() + 1) as u8);
            tmp
        } else {
            // If not compressed, prepend a zero byte
            let mut tmp = vec![0u8; 1];
            tmp.extend_from_slice(&self.payload);
            tmp
        };

        let mut payload = if !matches!(self.packet_type, PacketType::Syn) && matches!(self.source.stream_type, StreamType::RVSec) {
            // Encrypt the payload if it's an RVSec stream and not a Syn packet
            crypt(ctx, &payload)
        } else {
            payload
        };

        if self.flags.contains(PacketFlag::HasSize) {
            #[allow(clippy::cast_possible_truncation)]
            let s = payload.len() as u16;
            data.extend_from_slice(&s.to_le_bytes());
        }

        data.append(&mut payload);
        data
    }

    /// Converts the packet to a byte vector.
    #[must_use]
    pub fn to_bytes(&self, ctx: &Context) -> Vec<u8> {
        let mut data = self.to_data_bytes(ctx);
        data.push(calc_checksum_from_data(ctx.key(self.destination.stream_type), &data));
        data
    }

    /// Validates the packet's checksum.
    pub fn validate(&self, ctx: &Context, data: &[u8]) -> Result<(), Error> {
        if self.checksum != calc_checksum_from_data(ctx.key(self.destination.stream_type), &data[..data.len() - 1]) {
            return Err(Error::InvalidChecksum);
        }
        Ok(())
    }
}

/// Calculates the checksum from a given key and data.
/// The checksum is calculated by summing `u32` chunks of the data (little-endian),
/// then summing the bytes of that `u32` sum, and finally adding the remaining
/// bytes of the data and a key byte.
fn calc_checksum_from_data(key: u32, data: &[u8]) -> u8 {
    let l = data.len();
    let l = l - (l % 4);
    let mut rdr = Cursor::new(&data[..l]);
    let tmp: u32 = iter::from_fn(|| rdr.read_u32::<LittleEndian>().ok()).fold(Wrapping(0u32), |acc, x| acc + Wrapping(x)).0;

    let data_sum = tmp.to_le_bytes().iter().fold(Wrapping(0u8), |acc, x| acc + Wrapping(*x));

    let trailer_sum = &data[l..].iter().fold(Wrapping(0u8), |acc, x| acc + Wrapping(*x));

    #[allow(clippy::cast_possible_truncation)]
    let res = data_sum + Wrapping(key as u8) + trailer_sum;
    res.0
}

/// Crypts the given data using the context's crypto key.
fn crypt(ctx: &Context, data: &[u8]) -> Vec<u8> {
    crypt_key(&ctx.crypto_key, data)
}

/// Crypts the given data using the provided key.
/// This function applies the RC4 cipher to the data using the given key.
#[must_use]
pub(crate) fn crypt_key(key: &[u8], data: &[u8]) -> Vec<u8> {
    let rc4 = Rc4::new(key);
    rc4.zip(data).map(|(a, b)| a ^ b).collect()
}

/// Implements the RC4 cipher.
#[derive(Clone)]
pub(crate) struct Rc4 {
    i: u8,
    j: u8,
    state: [u8; 256],
}

impl Rc4 {
    /// Creates a new RC4 cipher with the given key.
    /// This initializes the S-box and the internal state (i, j) based on the key-scheduling algorithm (KSA).
    #[must_use]
    pub fn new(key: &[u8]) -> Rc4 {
        #![allow(clippy::cast_possible_truncation)]

        assert!(!key.is_empty() && key.len() <= 256);
        let mut rc4 = Rc4 { i: 0, j: 0, state: [0; 256] };
        for (i, x) in rc4.state.iter_mut().enumerate() {
            *x = i as u8;
        }
        let mut j: u8 = 0;
        for i in 0..256 {
            j = j.wrapping_add(rc4.state[i]).wrapping_add(key[i % key.len()]);
            rc4.state.swap(i, j as usize);
        }
        rc4
    }
}

impl Iterator for Rc4 {
    type Item = u8;

    /// Generates the next byte of the keystream using the pseudo-random generation algorithm (PRGA).
    fn next(&mut self) -> Option<Self::Item> {
        self.i = self.i.wrapping_add(1);
        self.j = self.j.wrapping_add(self.state[self.i as usize]);
        self.state.swap(self.i as usize, self.j as usize);
        let k = self.state[(self.state[self.i as usize].wrapping_add(self.state[self.j as usize])) as usize];
        Some(k)
    }
}

/// Parses a single packet from the given reader.
///
/// ```
/// use std::io::Cursor;
///
/// use quazal::prudp::packet::*;
/// use quazal::Context;
///
/// let packet = parse(
///     &Context {
///         access_key: b"yl4NG7qZ".to_vec(),
///         ..Default::default()
///     },
///     &mut Cursor::new([
///         0x3f, 0x31, 0x32, 0xa7, 0x91, 0x93, 0x92, 0xdd, 0x4, 0x0, 0x0, 0xf, 0x89, 0x44, 0xdb,
///         0x13, 0x58, 0x3a, 0x50, 0x5, 0xa2, 0x63, 0xfd, 0x2a, 0x16, 0xf1, 0xb1, 0x9b, 0x33,
///         0xe6, 0xe0,
///     ]),
/// )
/// .unwrap();
///
/// assert_eq!(
///     packet,
///     QPacket {
///         source: VPort {
///             port: 15,
///             stream_type: StreamType::RVSec
///         },
///         destination: VPort {
///             port: 1,
///             stream_type: StreamType::RVSec
///         },
///         packet_type: PacketType::Data,
///         flags: PacketFlag::Reliable | PacketFlag::NeedAck,
///         session_id: 167,
///         signature: 0xdd929391,
///         fragment_id: Some(0),
///         conn_signature: None,
///         sequence: 4,
///         payload: vec![
///             0xe, 0x0, 0x0, 0x0, 0xa7, 0x16, 0x0, 0x0, 0x0, 0x2, 0x0, 0x0, 0x0, 0x3, 0x0, 0x64,
///             0x65, 0x0
///         ],
///         checksum: 0xe0,
///         use_compression: false
///     }
/// );
/// ```
pub fn parse<R: std::io::Read + std::io::Seek>(ctx: &Context, rdr: &mut R) -> Result<QPacket, Error> {
    let start = rdr.stream_position()?;
    let pkt = QPacket::from_reader(ctx, rdr)?;
    let end = rdr.stream_position()?;
    rdr.seek(std::io::SeekFrom::Start(start))?;
    #[allow(clippy::cast_possible_truncation)]
    let mut data = vec![0u8; (end - start) as usize];
    rdr.read_exact(&mut data)?;
    pkt.validate(ctx, &data)?;
    Ok(pkt)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_syn() {
        let ctx = &Context::splinter_cell_blacklist();
        let data = [0x3f, 0x31, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40];

        let pkt = dbg!(QPacket::from_bytes(ctx, &data).unwrap()).0;
        assert_eq!(pkt.to_bytes(ctx), Vec::from(data));
        assert_eq!(pkt.checksum, pkt.calc_checksum(ctx));

        let data = [
            0x31, 0x3f, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x78, 0x56, 0x34, 0x12, 0x3c, // not verified
        ];

        let pkt = dbg!(QPacket::from_bytes(ctx, &data).unwrap()).0;
        assert_eq!(pkt.to_bytes(ctx), Vec::from(data));
        assert_eq!(pkt.checksum, pkt.calc_checksum(ctx));

        let data = *b"\x3e\x31\x31\x96\x60\x30\x0d\xd5\x01\x00\x37\x00\xba\xd2\x1c";

        let pkt = dbg!(QPacket::from_bytes(ctx, &data).unwrap()).0;
        assert_eq!(pkt.to_bytes(ctx), Vec::from(data));
        assert_eq!(pkt.checksum, pkt.calc_checksum(ctx));

        let data = *b"\x3f\x31\x32\x7c\x60\x30\x0d\xd5\x02\x00\x00\x0f\x93\x44\xdb\x13\x75\xe2\x50\x05\xa2\
\x60\xfd\x2a\x16\xfb\xb1\xab\x24\x87\x96\xfc\x3f\xcc\x7b\x5a\x7f";

        let pkt = dbg!(QPacket::from_bytes(ctx, &data).unwrap()).0;
        assert_eq!(pkt.to_bytes(ctx), Vec::from(data));
        assert_eq!(pkt.checksum, pkt.calc_checksum(ctx));
    }

    #[test]
    fn nat_packet() {
        #![allow(clippy::cast_possible_truncation)]

        let ctx = &Context::splinter_cell_blacklist();
        let data = *b"qq\x05\x00\x00\x00\x00\x00\x00\x00\x01\x053\x00\x00\x00\x00\xdcJ\x8d{\x80\x00\x01\x03\xd4";
        let (pkt, l) = dbg!(QPacket::from_bytes(ctx, &data).unwrap());
        assert!(pkt.validate(ctx, &data[..l as usize]).is_ok());

        assert!(parse(ctx, &mut Cursor::new(data)).is_ok());
    }
}