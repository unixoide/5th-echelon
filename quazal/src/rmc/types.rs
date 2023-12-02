use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt::Display;
use std::io;
use std::marker::PhantomData;

use byteorder::ReadBytesExt;
use byteorder::WriteBytesExt;

use super::basic::FromStream;
use super::basic::ReadStream;
use super::basic::ToStream;
use super::basic::WriteStream;
use crate::Error;

#[derive(Debug, Default)]
pub struct StationURL(pub String);

impl<S: ToString> From<S> for StationURL {
    fn from(s: S) -> Self {
        Self(s.to_string())
    }
}

impl From<Vec<String>> for QList<StationURL> {
    fn from(value: Vec<String>) -> Self {
        Self(value.into_iter().map(StationURL).collect())
    }
}

impl ToStream for StationURL {
    fn to_stream<W>(&self, stream: &mut WriteStream<W>) -> io::Result<usize>
    where
        W: WriteBytesExt,
    {
        stream.write(&self.0)
    }
}

impl FromStream for StationURL {
    fn from_stream<R>(stream: &mut ReadStream<R>) -> io::Result<Self>
    where
        R: ReadBytesExt,
    {
        Ok(Self(stream.read()?))
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
    fn from_stream<R>(stream: &mut ReadStream<R>) -> io::Result<Self>
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
pub type ClassFactory = fn(&[u8]) -> io::Result<ClassPtr>;

#[derive(Default)]
pub struct ClassRegistry(HashMap<String, ClassFactory>);

impl ClassRegistry {
    pub fn register_class<T: AnyClass + 'static>(&mut self, name: impl Into<String>) {
        self.0.insert(name.into(), |data: &[u8]| {
            T::from_bytes(data).map(|v| Box::new(v) as ClassPtr)
        });
    }

    pub fn instantiate(&self, name: &str, data: &[u8]) -> io::Result<ClassPtr> {
        let f = self.0.get(name).unwrap();
        f(data)
    }
}

#[derive(Debug)]
pub struct Any<V, K> {
    type_name: K,
    data: Vec<u8>,
    pd: PhantomData<V>,
}

impl<V, K: ToString> Any<V, K> {
    pub fn into_inner(self, class_list: &ClassRegistry) -> io::Result<ClassPtr> {
        class_list.instantiate(&self.type_name.to_string(), &self.data)
    }
}

impl<V, K> ToStream for Any<V, K> {
    fn to_stream<W>(&self, _stream: &mut WriteStream<W>) -> io::Result<usize>
    where
        W: WriteBytesExt,
    {
        unimplemented!()
    }
}

impl<V: std::fmt::Debug, K: FromStream> FromStream for Any<V, K> {
    fn from_stream<R>(stream: &mut ReadStream<R>) -> io::Result<Self>
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

#[derive(Debug, Clone, Copy)]
pub struct DateTime(pub u64);

impl ToStream for DateTime {
    fn to_stream<W>(&self, stream: &mut WriteStream<W>) -> io::Result<usize>
    where
        W: WriteBytesExt,
    {
        stream.u64(self.0)
    }
}

impl FromStream for DateTime {
    fn from_stream<R>(stream: &mut ReadStream<R>) -> io::Result<Self>
    where
        R: ReadBytesExt,
        Self: Sized,
    {
        Ok(Self(stream.read()?))
    }
}

#[derive(Debug)]
pub struct Data;

#[derive(Debug, Clone)]
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
    fn from_stream<R>(stream: &mut ReadStream<R>) -> io::Result<Self>
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
            t => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("invalid variant type {t}"),
            )),
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

impl<T> FromStream for QList<T>
where
    T: FromStream,
{
    fn from_stream<R>(stream: &mut ReadStream<R>) -> io::Result<Self>
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

impl TryFrom<&str> for Property {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (id, value) = value.split_once(" => ").ok_or(Error::InvalidValue)?;
        let id = id.parse().map_err(|_| Error::InvalidValue)?;
        let value = value.parse().map_err(|_| Error::InvalidValue)?;
        Ok(Self { id, value })
    }
}

impl TryFrom<&str> for QList<Property> {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.split(';').map(TryFrom::try_from).collect()
    }
}
