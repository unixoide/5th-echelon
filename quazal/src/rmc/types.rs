use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt::Debug;
use std::fmt::Display;
use std::io;
use std::marker::PhantomData;
use std::str::FromStr;

use byteorder::ReadBytesExt;
use byteorder::WriteBytesExt;
use serde::Deserialize;

use super::basic::FromStream;
use super::basic::FromStreamError;
use super::basic::ReadStream;
use super::basic::ToStream;
use super::basic::WriteStream;
use crate::Error as QuazalError;

#[derive(Debug, derive_more::Error, derive_more::Display, derive_more::From)]
pub enum Error {
    UnknownClass(#[error(not(source))] String),
    ParsingFailed(#[error(source)] FromStreamError),
}

#[derive(Debug, derive_more::Error, derive_more::Display, derive_more::From)]
pub enum StationURLParseError {
    MissingScheme,
    MissingAddress,
    MissingPort,
    InvalidParameters,
    InvalidPort(#[error(source)] std::num::ParseIntError),
}

#[derive(Debug, Default, Clone)]
pub struct StationURL {
    pub scheme: String,
    pub address: String,
    pub port: u16,
    pub params: HashMap<String, String>,
}

impl FromStr for StationURL {
    type Err = StationURLParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let Some((scheme, rest)) = value.split_once(":/") else {
            return Err(StationURLParseError::MissingScheme);
        };
        let scheme = scheme.to_owned();

        let mut params: HashMap<String, String> = rest
            .split(';')
            .map(|param| param.split_once('=').map(|(k, v)| (k.to_owned(), v.to_owned())))
            .collect::<Option<_>>()
            .ok_or(StationURLParseError::InvalidParameters)?;

        let address = params.remove("address").ok_or(StationURLParseError::MissingAddress)?;
        let port = params
            .remove("port")
            .ok_or(StationURLParseError::MissingPort)?
            .parse()?;

        Ok(Self {
            scheme,
            address,
            port,
            params,
        })
    }
}

impl Display for StationURL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params = [
            ("address", self.address.as_str()),
            ("port", self.port.to_string().as_str()),
        ]
        .into_iter()
        .chain(self.params.iter().map(|(k, v)| (k.as_str(), v.as_str())))
        .map(|(k, v)| format!("{k}={v}"))
        .collect::<Vec<_>>();
        write!(f, "{}:/{}", self.scheme, params.join(";"))
    }
}

impl ToStream for StationURL {
    fn to_stream<W>(&self, stream: &mut WriteStream<W>) -> io::Result<usize>
    where
        W: WriteBytesExt,
    {
        stream.write(&self.to_string())
    }
}

impl FromStream for StationURL {
    fn from_stream<R>(stream: &mut ReadStream<R>) -> Result<Self, FromStreamError>
    where
        R: ReadBytesExt,
    {
        Ok(stream.read::<String>()?.parse()?)
    }
}

#[derive(Debug)]
pub enum QResult {
    Ok,
    Error(super::result::Error),
    Unknown(u32),
}

impl ToStream for QResult {
    fn to_stream<W>(&self, stream: &mut WriteStream<W>) -> io::Result<usize>
    where
        W: WriteBytesExt,
    {
        let code: u32 = match self {
            QResult::Ok => 0x10001,
            QResult::Error(err) => u32::from(*err),
            QResult::Unknown(c) => *c,
        };
        stream.write(&code)
    }
}

impl FromStream for QResult {
    fn from_stream<R>(stream: &mut ReadStream<R>) -> Result<Self, FromStreamError>
    where
        R: ReadBytesExt,
    {
        let code: u32 = stream.read()?;

        Ok(match code {
            0x10001 => Self::Ok,
            c => super::result::Error::try_from(c).map_or_else(|_| Self::Unknown(c), Self::Error),
        })
    }
}

pub trait AnyClass: FromStream {
    fn as_any(&self) -> &dyn std::any::Any;
}

impl<T: std::any::Any + FromStream> AnyClass for T {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

pub type ClassPtr = Box<dyn AnyClass>;
pub type ClassFactory = fn(&[u8]) -> Result<ClassPtr, FromStreamError>;

#[derive(Default)]
pub struct ClassRegistry(HashMap<String, ClassFactory>);

impl ClassRegistry {
    pub fn register_class<T: AnyClass + 'static>(&mut self, name: impl Into<String>) {
        self.0.insert(name.into(), |data: &[u8]| {
            T::from_bytes(data).map(|v| Box::new(v) as ClassPtr)
        });
    }

    pub fn instantiate(&self, name: &str, data: &[u8]) -> Result<ClassPtr, Error> {
        let f = self.0.get(name).ok_or(Error::UnknownClass(name.to_owned()))?;
        Ok(f(data)?)
    }
}

#[derive(Debug)]
pub struct Any<V, K> {
    type_name: K,
    data: Vec<u8>,
    pd: PhantomData<V>,
}

impl<V, K> Any<V, K> {
    pub fn new(type_name: K, data: Vec<u8>) -> Self {
        Self {
            type_name,
            data,
            pd: PhantomData,
        }
    }
}

impl<V, K: ToString> Any<V, K> {
    pub fn into_inner(self, class_list: &ClassRegistry) -> Result<ClassPtr, FromStreamError> {
        match class_list.instantiate(&self.type_name.to_string(), &self.data) {
            Ok(ptr) => Ok(ptr),
            Err(Error::ParsingFailed(parse_error)) => Err(parse_error),
            Err(e) => panic!("{e:?}"),
        }
    }
}

impl<V, K: ToStream> ToStream for Any<V, K> {
    fn to_stream<W>(&self, stream: &mut WriteStream<W>) -> io::Result<usize>
    where
        W: WriteBytesExt,
    {
        let data = Vec::to_bytes(&self.data);
        Ok(stream.write(&self.type_name)? + stream.write(&data)?)
    }
}

impl<V: std::fmt::Debug, K: FromStream> FromStream for Any<V, K> {
    fn from_stream<R>(stream: &mut ReadStream<R>) -> Result<Self, FromStreamError>
    where
        R: ReadBytesExt,
    {
        let type_name = stream.read()?;
        let data: Vec<u8> = stream.read()?;
        let data = Vec::from_bytes(data.as_ref())?;
        Ok(Self {
            type_name,
            data,
            pd: PhantomData,
        })
    }
}

#[derive(Clone, Copy, Default, Deserialize)]
pub struct DateTime(pub u64);

impl Debug for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let second = self.0 & 0b1_1111;
        let minute = (self.0 >> 6) & 0b1_1111;
        let hour = (self.0 >> 12) & 0b1111;
        let day = (self.0 >> 17) & 0b1111;
        let month = (self.0 >> 22) & 0b111;
        let year = self.0 >> 26;
        f.debug_tuple("DateTime")
            .field(&self.0)
            .field(&format!(
                "{year:04}-{month:02}-{day:02} {hour:02}:{minute:02}:{second:02}"
            ))
            .finish()
    }
}

impl ToStream for DateTime {
    fn to_stream<W>(&self, stream: &mut WriteStream<W>) -> io::Result<usize>
    where
        W: WriteBytesExt,
    {
        stream.u64(self.0)
    }
}

impl FromStream for DateTime {
    fn from_stream<R>(stream: &mut ReadStream<R>) -> Result<Self, FromStreamError>
    where
        R: ReadBytesExt,
        Self: Sized,
    {
        Ok(Self(stream.read()?))
    }
}

#[derive(Debug, FromStream, ToStream)]
pub struct Data;

#[derive(Debug, Clone, Deserialize)]
pub enum Variant {
    None,
    I64(i64),
    F64(f64),
    Bool(bool),
    String(String),
    DateTime(DateTime),
    U64(u64),
}

impl FromStream for Variant {
    fn from_stream<R>(stream: &mut ReadStream<R>) -> Result<Self, FromStreamError>
    where
        R: ReadBytesExt,
        Self: Sized,
    {
        match stream.u8()? {
            0 => Ok(Variant::None),
            1 => Ok(Variant::I64(stream.read()?)),
            2 => Ok(Variant::F64(stream.read()?)),
            3 => Ok(Variant::Bool(stream.read()?)),
            4 => Ok(Variant::String(stream.read()?)),
            5 => Ok(Variant::DateTime(stream.read()?)),
            6 => Ok(Variant::U64(stream.read()?)),
            t => Err(io::Error::new(io::ErrorKind::InvalidData, format!("invalid variant type {t}")).into()),
        }
    }
}

impl ToStream for Variant {
    fn to_stream<W>(&self, stream: &mut WriteStream<W>) -> io::Result<usize>
    where
        W: WriteBytesExt,
    {
        let n = match self {
            Variant::None => stream.u8(0)?,
            Variant::I64(v) => stream.u8(1)? + stream.write(v)?,
            Variant::F64(v) => stream.u8(2)? + stream.write(v)?,
            Variant::Bool(v) => stream.u8(3)? + stream.write(v)?,
            Variant::String(v) => stream.u8(4)? + stream.write(v)?,
            Variant::DateTime(v) => stream.u8(5)? + stream.write(v)?,
            Variant::U64(v) => stream.u8(6)? + stream.write(v)?,
        };
        Ok(n)
    }
}

#[derive(Debug, FromStream, ToStream)]
pub struct PropertyVariant {
    pub id: u32,
    pub value: Variant,
}

#[derive(Debug, FromStream, ToStream)]
pub struct ResultRange {
    pub offset: u32,
    pub size: u32,
}

#[derive(Debug)]
pub struct QList<T: std::fmt::Debug>(pub Vec<T>);

impl<T: std::fmt::Debug> QList<T> {
    #[must_use]
    pub fn new() -> Self {
        Self(Vec::new())
    }
}

impl<T: std::fmt::Debug> std::default::Default for QList<T> {
    fn default() -> Self {
        Self(Vec::default())
    }
}

impl<T: std::fmt::Debug> From<Vec<T>> for QList<T> {
    fn from(value: Vec<T>) -> Self {
        Self(value)
    }
}

impl<T: std::fmt::Debug> FromIterator<T> for QList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl TryFrom<Vec<&str>> for QList<StationURL> {
    type Error = StationURLParseError;

    fn try_from(vec: Vec<&str>) -> Result<Self, Self::Error> {
        Ok(Self(
            vec.into_iter()
                .map(FromStr::from_str)
                .collect::<Result<_, Self::Error>>()?,
        ))
    }
}

impl TryFrom<Vec<String>> for QList<StationURL> {
    type Error = StationURLParseError;

    fn try_from(vec: Vec<String>) -> Result<Self, Self::Error> {
        Ok(Self(
            vec.iter()
                .map(String::as_str)
                .map(FromStr::from_str)
                .collect::<Result<_, Self::Error>>()?,
        ))
    }
}

impl<T> FromStream for QList<T>
where
    T: FromStream + std::fmt::Debug,
{
    fn from_stream<R>(stream: &mut ReadStream<R>) -> Result<Self, FromStreamError>
    where
        R: ReadBytesExt,
    {
        // Looks like it's using 32bit after all?
        // let len = stream.u16()? as usize;
        let len = stream.u32()? as usize;
        let mut res = Vec::with_capacity(len);
        for _ in 0..len {
            res.push(stream.read()?);
        }
        Ok(Self(res))
    }
}

impl<T> ToStream for QList<T>
where
    T: ToStream + std::fmt::Debug,
{
    fn to_stream<W>(&self, stream: &mut WriteStream<W>) -> io::Result<usize>
    where
        W: WriteBytesExt,
    {
        // Looks like it's using 32bit after all?
        // let mut total = stream.u16(self.0.len() as u16)?;
        #[allow(clippy::cast_possible_truncation)]
        let mut total = stream.u32(self.0.len() as u32)?;
        for item in &self.0 {
            total += stream.write(item)?;
        }
        Ok(total)
    }
}

impl<T: std::fmt::Debug> std::ops::Deref for QList<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, FromStream, ToStream)]
pub struct Property {
    pub id: u32,
    pub value: u32,
}

impl Display for Property {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} => {}", self.id, self.value)
    }
}

impl FromStr for Property {
    type Err = QuazalError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let (id, value) = value.split_once(" => ").ok_or(Self::Err::InvalidValue)?;
        let id = id.parse().map_err(|_| Self::Err::InvalidValue)?;
        let value = value.parse().map_err(|_| Self::Err::InvalidValue)?;
        Ok(Self { id, value })
    }
}

impl FromStr for QList<Property> {
    type Err = QuazalError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        value.split(';').map(FromStr::from_str).collect()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse_stationurl() {
        let parsed: StationURL = "prudp:/address=127.0.0.1;port=3074;sid=15;type=3".parse().unwrap();
        assert_eq!(parsed.address.as_str(), "127.0.0.1");
        assert_eq!(parsed.port, 3074);
        assert_eq!(parsed.params.get("sid").map(String::as_str), Some("15"));
        assert_eq!(parsed.params.get("type").map(String::as_str), Some("3"));
    }

    #[test]
    fn any_type_registration() {
        let mut registry = ClassRegistry::default();
        registry.register_class::<u32>("u32");
        registry.register_class::<u8>("u8");
        registry.register_class::<String>("string");

        let inst = registry.instantiate("u32", b"ABCD").unwrap();
        assert_eq!(*inst.as_any().downcast_ref::<u32>().unwrap(), 0x4443_4241u32);

        let inst = registry.instantiate("u8", b"ABCD").unwrap();
        assert_eq!(*inst.as_any().downcast_ref::<u8>().unwrap(), 0x41u8);
        let inst = registry.instantiate("string", b"\x05\x00ABCD\x00").unwrap();
        assert_eq!(inst.as_any().downcast_ref::<String>().unwrap().as_str(), "ABCD");
    }

    #[test]
    fn unknown_any_type() {
        let mut registry = ClassRegistry::default();
        registry.register_class::<u32>("u32");

        let inst = registry.instantiate("u8", b"ABCD");
        assert!(matches!(inst, Err(Error::UnknownClass(x)) if x.as_str() == "u8"));
    }

    #[test]
    fn invalid_any_type() {
        let mut registry = ClassRegistry::default();
        registry.register_class::<u32>("u32");

        let inst = registry.instantiate("u32", b"A");
        assert!(
            matches!(inst, Err(Error::ParsingFailed(FromStreamError::IO(e))) if e.kind() == std::io::ErrorKind::UnexpectedEof)
        );
    }
}
