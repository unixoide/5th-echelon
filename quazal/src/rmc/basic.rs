/// This module provides basic functionalities for reading and writing data to/from streams.
use std::collections::HashMap;
use std::ffi::CString;
use std::io::Cursor;
use std::io::{self};

use byteorder::LittleEndian;
use byteorder::ReadBytesExt;
use byteorder::WriteBytesExt;
use derive_more::Display;
use derive_more::Error;
use derive_more::From;

/// Errors that can occur when reading from a stream.
#[derive(Error, Display, Debug, From)]
pub enum FromStreamError {
    /// An I/O error occurred.
    IO(#[error(source)] std::io::Error),
    /// Not enough bytes were available in the stream.
    #[display("{_0} bytes missing")]
    MissingBytes(#[error(ignore)] usize),
    /// An error occurred while parsing an integer.
    ParseInt(#[error(source)] std::num::ParseIntError),
    /// An error occurred while parsing a `StationURL`.
    ParseStationURL(#[error(source)] super::types::StationURLParseError),
    /// An error occurred while parsing a `CString`.
    ParseCString(#[error(source)] std::ffi::FromVecWithNulError),
    /// An error occurred during UTF-8 conversion.
    Utf8(#[error(source)] std::str::Utf8Error),
}

/// Errors that can occur when writing to a stream.
#[derive(Error, Display, Debug, From)]
pub enum ToStreamError {
    /// An I/O error occurred.
    IO(#[error(source)] std::io::Error),
}

/// A reader for streams, providing methods to read various data types.
/// It wraps a `ReadBytesExt` implementor and provides convenient methods
/// for reading primitive types, length-prefixed buffers, and custom `FromStream` types.
pub struct ReadStream<R: ReadBytesExt> {
    rdr: R,
}

/// Macro to generate `ReadStream` methods for reading numeric types.
/// It automatically handles little-endian byte order.
macro_rules! read_stream_num {
    ($i: ident, $f: ident) => {
        pub fn $i(&mut self) -> std::result::Result<$i, FromStreamError> {
            Ok(self.rdr.$f::<LittleEndian>()?)
        }
    };
}

impl<R: ReadBytesExt> ReadStream<R> {
    /// Creates a new `ReadStream` from a reader.
    pub fn from_reader(rdr: R) -> Self {
        Self { rdr }
    }

    /// Reads a `u8` from the stream.
    pub fn u8(&mut self) -> std::result::Result<u8, FromStreamError> {
        Ok(self.rdr.read_u8()?)
    }
    read_stream_num!(u16, read_u16);
    read_stream_num!(u32, read_u32);
    read_stream_num!(u64, read_u64);

    /// Reads an `i8` from the stream.
    pub fn i8(&mut self) -> std::result::Result<i8, FromStreamError> {
        Ok(self.rdr.read_i8()?)
    }

    read_stream_num!(i16, read_i16);
    read_stream_num!(i32, read_i32);
    read_stream_num!(i64, read_i64);

    read_stream_num!(f32, read_f32);
    read_stream_num!(f64, read_f64);

    /// Reads a boolean from the stream.
    pub fn bool(&mut self) -> std::result::Result<bool, FromStreamError> {
        self.u8().map(|b| b != 0)
    }

    /// Reads `l` bytes from the stream.
    pub fn read_n_bytes(&mut self, l: usize) -> std::result::Result<Vec<u8>, FromStreamError> {
        let mut buf = vec![0u8; l];
        match self.rdr.read_exact(&mut buf) {
            Ok(()) => Ok(buf),
            Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => Err(FromStreamError::MissingBytes(l)),
            Err(e) => Err(FromStreamError::IO(e)),
        }
    }

    /// Reads a `u8` length-prefixed byte buffer from the stream.
    pub fn buf_u8(&mut self) -> std::result::Result<Vec<u8>, FromStreamError> {
        let l = self.u8()?;
        self.read_n_bytes(l as usize)
    }

    /// Reads a `u16` length-prefixed byte buffer from the stream.
    pub fn buf_u16(&mut self) -> std::result::Result<Vec<u8>, FromStreamError> {
        let l = self.u16()?;
        self.read_n_bytes(l as usize)
    }

    /// Reads a `u32` length-prefixed byte buffer from the stream.
    pub fn buf_u32(&mut self) -> std::result::Result<Vec<u8>, FromStreamError> {
        let l = self.u32()?;
        self.read_n_bytes(l as usize)
    }

    /// Reads a `FromStream` implementor from the stream.
    pub fn read<F>(&mut self) -> std::result::Result<F, FromStreamError>
    where
        F: FromStream,
    {
        <F as FromStream>::from_stream(self)
    }

    /// Reads all remaining bytes from the stream.
    pub fn read_all(&mut self) -> std::result::Result<Vec<u8>, FromStreamError> {
        let mut buf = vec![];
        self.rdr.read_to_end(&mut buf)?;
        Ok(buf)
    }
}

impl<T: AsRef<[u8]>> ReadStream<Cursor<T>> {
    /// Creates a new `ReadStream` from a byte slice.
    pub fn from_bytes(data: T) -> Self {
        Self::from_reader(Cursor::new(data))
    }
}

/// Trait to read a type from a stream or bytes in the Quazal encoding.
pub trait FromStream {
    /// Reads a single instance from the given stream.
    fn from_stream<R>(_stream: &mut ReadStream<R>) -> std::result::Result<Self, FromStreamError>
    where
        R: ReadBytesExt,
        Self: Sized;

    /// Reads a single instance from the given bytes. Doesn't fail if there are unused trailing bytes.
    fn from_bytes(data: &[u8]) -> std::result::Result<Self, FromStreamError>
    where
        Self: Sized,
    {
        Self::from_stream(&mut ReadStream::from_bytes(data))
    }
}

/// A writer for streams, providing methods to write various data types.
/// It wraps a `WriteBytesExt` implementor and provides convenient methods
/// for writing primitive types, length-prefixed buffers, and custom `ToStream` types.
pub struct WriteStream<W: WriteBytesExt> {
    wtr: W,
}

/// Macro to generate `WriteStream` methods for writing numeric types.
/// It automatically handles little-endian byte order.
macro_rules! write_stream_num {
    ($i: ident, $f: ident) => {
        pub fn $i(&mut self, value: $i) -> io::Result<usize> {
            self.wtr.$f::<LittleEndian>(value).map(|()| ::std::mem::size_of::<$i>())
        }
    };
}

impl<W: WriteBytesExt> WriteStream<W> {
    /// Creates a new `WriteStream` from a writer.
    pub fn from_writer(wtr: W) -> Self {
        Self { wtr }
    }

    /// Writes a `u8` to the stream.
    pub fn u8(&mut self, value: u8) -> io::Result<usize> {
        self.wtr.write_u8(value).map(|()| 1)
    }
    write_stream_num!(u16, write_u16);
    write_stream_num!(u32, write_u32);
    write_stream_num!(u64, write_u64);

    /// Writes an `i8` to the stream.
    pub fn i8(&mut self, value: i8) -> io::Result<usize> {
        self.wtr.write_i8(value).map(|()| 1)
    }

    write_stream_num!(i16, write_i16);
    write_stream_num!(i32, write_i32);
    write_stream_num!(i64, write_i64);

    write_stream_num!(f32, write_f32);
    write_stream_num!(f64, write_f64);

    /// Writes a boolean to the stream.
    pub fn bool(&mut self, value: bool) -> io::Result<usize> {
        self.u8(u8::from(value))
    }

    /// Writes a byte slice to the stream.
    pub fn write_n_bytes<T: AsRef<[u8]>>(&mut self, data: T) -> io::Result<usize> {
        self.wtr.write_all(data.as_ref()).map(|()| data.as_ref().len())
    }

    /// Writes a `u8` length-prefixed byte buffer to the stream.
    pub fn buf_u8<T: AsRef<[u8]>>(&mut self, data: T) -> io::Result<usize> {
        let d = data.as_ref();
        #[allow(clippy::cast_possible_truncation)]
        self.u8(d.len() as u8)?;
        self.write_n_bytes(d)
    }

    /// Writes a `u16` length-prefixed byte buffer to the stream.
    pub fn buf_u16<T: AsRef<[u8]>>(&mut self, data: T) -> io::Result<usize> {
        let d = data.as_ref();
        #[allow(clippy::cast_possible_truncation)]
        self.u16(d.len() as u16)?;
        self.write_n_bytes(d)
    }

    /// Writes a `u32` length-prefixed byte buffer to the stream.
    pub fn buf_u32<T: AsRef<[u8]>>(&mut self, data: T) -> io::Result<usize> {
        let d = data.as_ref();
        #[allow(clippy::cast_possible_truncation)]
        self.u32(d.len() as u32)?;
        self.write_n_bytes(d)
    }

    /// Writes a `ToStream` implementor to the stream.
    pub fn write<T>(&mut self, value: &T) -> io::Result<usize>
    where
        T: ToStream,
    {
        value.to_stream(self)
    }
}

/// Trait to write a type to a stream or bytes.
pub trait ToStream {
    /// Writes the instance to the given stream. Returns the amount of bytes written.
    fn to_stream<W>(&self, _stream: &mut WriteStream<W>) -> io::Result<usize>
    where
        W: WriteBytesExt;

    /// Converts to a bytes vector.
    fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        let mut s = WriteStream::from_writer(Cursor::new(&mut buf));
        self.to_stream(&mut s).unwrap();
        buf
    }
}

impl FromStream for bool {
    fn from_stream<R>(stream: &mut ReadStream<R>) -> std::result::Result<Self, FromStreamError>
    where
        R: ReadBytesExt,
        Self: Sized,
    {
        stream.bool()
    }
}

impl ToStream for bool {
    fn to_stream<W>(&self, stream: &mut WriteStream<W>) -> io::Result<usize>
    where
        W: WriteBytesExt,
    {
        stream.bool(*self)
    }

    fn to_bytes(&self) -> Vec<u8> {
        u8::from(*self).to_le_bytes().to_vec()
    }
}

macro_rules! impl_num {
    ($i: ident) => {
        impl FromStream for $i {
            fn from_stream<R>(stream: &mut ReadStream<R>) -> std::result::Result<Self, FromStreamError>
            where
                R: ReadBytesExt,
                Self: Sized,
            {
                stream.$i()
            }
        }

        impl ToStream for $i {
            fn to_stream<W>(&self, stream: &mut WriteStream<W>) -> io::Result<usize>
            where
                W: WriteBytesExt,
            {
                stream.$i(*self)
            }

            fn to_bytes(&self) -> Vec<u8> {
                self.to_le_bytes().to_vec()
            }
        }
    };
}

impl_num!(u8);
impl_num!(u16);
impl_num!(u32);
impl_num!(u64);
impl_num!(i8);
impl_num!(i16);
impl_num!(i32);
impl_num!(i64);
impl_num!(f32);
impl_num!(f64);

impl<const N: usize> FromStream for [u8; N] {
    #[allow(non_snake_case)]
    fn from_stream<R>(stream: &mut ReadStream<R>) -> std::result::Result<Self, FromStreamError>
    where
        R: ReadBytesExt,
        Self: Sized,
    {
        let buf = stream.read_n_bytes(N)?;
        if buf.len() == N {
            let mut tmp = [0u8; N];
            tmp.copy_from_slice(&buf);
            Ok(tmp)
        } else {
            Err(io::Error::new(io::ErrorKind::UnexpectedEof, "").into())
        }
    }
}

impl<const N: usize> ToStream for [u8; N] {
    fn to_stream<W>(&self, stream: &mut WriteStream<W>) -> io::Result<usize>
    where
        W: WriteBytesExt,
    {
        stream.write_n_bytes(self.as_ref())
    }
}

/// Macro to implement `FromStream` and `ToStream` for tuples of various lengths.
/// This allows automatic serialization/deserialization of tuples where each element
/// implements `FromStream` and `ToStream` respectively.
macro_rules! tuple_impls {
    ($($len:tt => ($($n:tt $name:ident)+))+) => {
        $(
            impl<$($name: FromStream),+> FromStream for ($($name,)+) {
                fn from_stream<R>(stream: &mut ReadStream<R>) ->  std::result::Result<Self, FromStreamError>
                where
                    R: ReadBytesExt,
                    Self: Sized,
                {
                    #![allow(non_snake_case)]

                    $(
                        let $name = stream.read()?;
                    )+
                    Ok(($($name),+))
                }
            }

            impl<$($name: ToStream),+> ToStream for ($($name,)+) {
                fn to_stream<W>(&self, stream: &mut WriteStream<W>) -> io::Result<usize>
                where
                    W: WriteBytesExt,
                {
                    Ok(
                        0
                        $(
                            +stream.write(&self.$n)?
                        )+
                    )
                }
            }
        )+
    }
}

tuple_impls! {
    1  => (0 T0)
    2  => (0 T0 1 T1)
    3  => (0 T0 1 T1 2 T2)
    4  => (0 T0 1 T1 2 T2 3 T3)
    5  => (0 T0 1 T1 2 T2 3 T3 4 T4)
    6  => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5)
    7  => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6)
    8  => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7)
    9  => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8)
    10 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9)
    11 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10)
    12 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11)
    /*
    13 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12)
    14 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13)
    15 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14)
    16 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15)
     */
}

impl FromStream for CString {
    /// Reads a `CString` from the stream. It is prefixed by a `u16` length.
    fn from_stream<R>(stream: &mut ReadStream<R>) -> std::result::Result<Self, FromStreamError>
    where
        R: ReadBytesExt,
    {
        let len = stream.u16()?;
        if len == 0 {
            return Ok(CString::default());
        }
        let data = stream.read_n_bytes(len as usize)?;
        Ok(CString::from_vec_with_nul(data)?)
    }
}

impl ToStream for CString {
    /// Writes a `CString` to the stream, prefixed by its `u16` length.
    fn to_stream<W>(&self, stream: &mut WriteStream<W>) -> io::Result<usize>
    where
        W: WriteBytesExt,
    {
        if self.is_empty() {
            return stream.u16(0);
        }
        let data = self.as_bytes_with_nul();
        #[allow(clippy::cast_possible_truncation)]
        Ok(stream.u16(data.len() as u16)? + stream.write_n_bytes(data)?)
    }
}

impl FromStream for String {
    /// Reads a `String` from the stream. It is prefixed by a `u16` length and null-terminated.
    fn from_stream<R>(stream: &mut ReadStream<R>) -> std::result::Result<Self, FromStreamError>
    where
        R: ReadBytesExt,
    {
        let cstr = CString::from_stream(stream)?;
        Ok(cstr.into_string().map_err(|e| e.utf8_error())?)
    }
}

impl ToStream for String {
    /// Writes a `String` to the stream, prefixed by its `u16` length and null-terminated.
    fn to_stream<W>(&self, stream: &mut WriteStream<W>) -> io::Result<usize>
    where
        W: WriteBytesExt,
    {
        if self.is_empty() {
            return stream.u16(0);
        }
        let data = self.as_bytes();
        #[allow(clippy::cast_possible_truncation)]
        Ok(stream.u16(data.len() as u16 + 1)? + stream.write_n_bytes(data)? + stream.u8(0)?)
    }
}

impl<T> FromStream for Vec<T>
where
    T: FromStream,
{
    /// Reads a `Vec<T>` from the stream. It is prefixed by a `u32` length, followed by `len` elements of type `T`.
    fn from_stream<R>(stream: &mut ReadStream<R>) -> std::result::Result<Self, FromStreamError>
    where
        R: ReadBytesExt,
    {
        let len = stream.u32()? as usize;
        let mut res = Vec::with_capacity(len);
        for _ in 0..len {
            res.push(stream.read()?);
        }
        Ok(res)
    }
}

impl<T> ToStream for Vec<T>
where
    T: ToStream,
{
    /// Writes a `Vec<T>` to the stream, prefixed by its `u32` length, followed by its elements.
    fn to_stream<W>(&self, stream: &mut WriteStream<W>) -> io::Result<usize>
    where
        W: WriteBytesExt,
    {
        #[allow(clippy::cast_possible_truncation)]
        let mut total = stream.u32(self.len() as u32)?;
        for item in self {
            total += stream.write(item)?;
        }
        Ok(total)
    }
}

impl<K, V, S: ::std::hash::BuildHasher> ToStream for HashMap<K, V, S>
where
    K: ToStream,
    V: ToStream,
{
    /// Writes a `HashMap<K, V>` to the stream. It is prefixed by a `u32` count,
    /// followed by key-value pairs.
    fn to_stream<W>(&self, stream: &mut WriteStream<W>) -> io::Result<usize>
    where
        W: WriteBytesExt,
    {
        // Format:
        //   count: u32
        //   entries (repeated):
        //     key: K
        //     value: V
        #[allow(clippy::cast_possible_truncation)]
        let mut total = stream.u32(self.len() as u32)?;
        for (k, v) in self {
            total += stream.write(k)?;
            total += stream.write(v)?;
        }
        Ok(total)
    }
}

impl<K, V, S: ::std::hash::BuildHasher + std::default::Default> FromStream for HashMap<K, V, S>
where
    K: FromStream + std::hash::Hash + std::cmp::Eq,
    V: FromStream,
{
    /// Reads a `HashMap<K, V>` from the stream. It is prefixed by a `u32` count,
    /// followed by `count` key-value pairs.
    fn from_stream<R>(stream: &mut ReadStream<R>) -> std::result::Result<Self, FromStreamError>
    where
        R: ReadBytesExt,
        Self: Sized,
    {
        // Format:
        //   count: u32
        //   entries (repeated):
        //     key: K
        //     value: V
        let n = stream.u32()?;
        let mut m: Self = HashMap::default();
        for _i in 0..n {
            let k = stream.read()?;
            let v = stream.read()?;
            m.insert(k, v);
        }
        Ok(m)
    }
}
