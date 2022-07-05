use std::{
    collections::HashMap,
    io::{self, Cursor},
};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

pub struct ReadStream<R: ReadBytesExt> {
    rdr: R,
}

macro_rules! read_stream_num {
    ($i: ident, $f: ident) => {
        pub fn $i(&mut self) -> io::Result<$i> {
            self.rdr.$f::<LittleEndian>()
        }
    };
}

impl<R: ReadBytesExt> ReadStream<R> {
    pub fn from_reader(rdr: R) -> Self {
        Self { rdr }
    }

    pub fn u8(&mut self) -> io::Result<u8> {
        self.rdr.read_u8()
    }
    read_stream_num!(u16, read_u16);
    read_stream_num!(u32, read_u32);
    read_stream_num!(u64, read_u64);

    pub fn i8(&mut self) -> io::Result<i8> {
        self.rdr.read_i8()
    }

    read_stream_num!(i16, read_i16);
    read_stream_num!(i32, read_i32);
    read_stream_num!(i64, read_i64);

    read_stream_num!(f32, read_f32);
    read_stream_num!(f64, read_f64);

    pub fn bool(&mut self) -> io::Result<bool> {
        self.u8().map(|b| b != 0)
    }

    pub fn read_n_bytes(&mut self, l: usize) -> io::Result<Vec<u8>> {
        let mut buf = vec![0u8; l];
        self.rdr.read_exact(&mut buf)?;
        Ok(buf)
    }

    pub fn buf_u8(&mut self) -> io::Result<Vec<u8>> {
        let l = self.u8()?;
        self.read_n_bytes(l as usize)
    }

    pub fn buf_u16(&mut self) -> io::Result<Vec<u8>> {
        let l = self.u16()?;
        self.read_n_bytes(l as usize)
    }

    pub fn buf_u32(&mut self) -> io::Result<Vec<u8>> {
        let l = self.u32()?;
        self.read_n_bytes(l as usize)
    }

    pub fn read<F>(&mut self) -> io::Result<F>
    where
        F: FromStream,
    {
        <F as FromStream>::from_stream(self)
    }

    pub fn read_all(&mut self) -> io::Result<Vec<u8>> {
        let mut buf = vec![];
        self.rdr.read_to_end(&mut buf)?;
        Ok(buf)
    }
}

impl<T: AsRef<[u8]>> ReadStream<Cursor<T>> {
    pub fn from_bytes(data: T) -> Self {
        Self::from_reader(Cursor::new(data))
    }
}

pub trait FromStream: std::fmt::Debug {
    fn from_stream<R>(_stream: &mut ReadStream<R>) -> io::Result<Self>
    where
        R: ReadBytesExt,
        Self: Sized;

    fn from_bytes(data: &[u8]) -> io::Result<Self>
    where
        Self: Sized,
    {
        Self::from_stream(&mut ReadStream::from_bytes(data))
    }
}

pub struct WriteStream<W: WriteBytesExt> {
    wtr: W,
}

macro_rules! write_stream_num {
    ($i: ident, $f: ident) => {
        pub fn $i(&mut self, value: $i) -> io::Result<usize> {
            self.wtr
                .$f::<LittleEndian>(value)
                .map(|_| ::std::mem::size_of::<$i>())
        }
    };
}

impl<W: WriteBytesExt> WriteStream<W> {
    pub fn from_writer(wtr: W) -> Self {
        Self { wtr }
    }

    pub fn u8(&mut self, value: u8) -> io::Result<usize> {
        self.wtr.write_u8(value).map(|_| 1)
    }
    write_stream_num!(u16, write_u16);
    write_stream_num!(u32, write_u32);
    write_stream_num!(u64, write_u64);

    pub fn i8(&mut self, value: i8) -> io::Result<usize> {
        self.wtr.write_i8(value).map(|_| 1)
    }

    write_stream_num!(i16, write_i16);
    write_stream_num!(i32, write_i32);
    write_stream_num!(i64, write_i64);

    write_stream_num!(f32, write_f32);
    write_stream_num!(f64, write_f64);

    pub fn bool(&mut self, value: bool) -> io::Result<usize> {
        self.u8(value as u8)
    }

    pub fn write_n_bytes<T: AsRef<[u8]>>(&mut self, data: T) -> io::Result<usize> {
        self.wtr
            .write_all(data.as_ref())
            .map(|_| data.as_ref().len())
    }

    pub fn buf_u8<T: AsRef<[u8]>>(&mut self, data: T) -> io::Result<usize> {
        let d = data.as_ref();
        self.u8(d.len() as u8)?;
        self.write_n_bytes(d)
    }

    pub fn buf_u16<T: AsRef<[u8]>>(&mut self, data: T) -> io::Result<usize> {
        let d = data.as_ref();
        self.u16(d.len() as u16)?;
        self.write_n_bytes(d)
    }

    pub fn buf_u32<T: AsRef<[u8]>>(&mut self, data: T) -> io::Result<usize> {
        let d = data.as_ref();
        self.u32(d.len() as u32)?;
        self.write_n_bytes(d)
    }

    pub fn write<T>(&mut self, value: &T) -> io::Result<usize>
    where
        T: ToStream,
    {
        value.to_stream(self)
    }
}

pub trait ToStream {
    fn to_stream<W>(&self, _stream: &mut WriteStream<W>) -> io::Result<usize>
    where
        W: WriteBytesExt;

    fn as_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        let mut s = WriteStream::from_writer(Cursor::new(&mut buf));
        self.to_stream(&mut s).unwrap();
        buf
    }
}

impl FromStream for bool {
    fn from_stream<R>(stream: &mut ReadStream<R>) -> io::Result<Self>
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

    fn as_bytes(&self) -> Vec<u8> {
        (*self as u8).to_le_bytes().to_vec()
    }
}

macro_rules! impl_num {
    ($i: ident) => {
        impl FromStream for $i {
            fn from_stream<R>(stream: &mut ReadStream<R>) -> io::Result<Self>
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

            fn as_bytes(&self) -> Vec<u8> {
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
    fn from_stream<R>(stream: &mut ReadStream<R>) -> io::Result<Self>
    where
        R: ReadBytesExt,
        Self: Sized,
    {
        let buf = stream.read_n_bytes(N)?;
        if buf.len() != N {
            Err(io::Error::new(io::ErrorKind::UnexpectedEof, ""))
        } else {
            let mut tmp = [0u8; N];
            tmp.copy_from_slice(&buf);
            Ok(tmp)
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

macro_rules! tuple_impls {
    ($($len:tt => ($($n:tt $name:ident)+))+) => {
        $(
            impl<$($name: FromStream),+> FromStream for ($($name,)+) {
                #[allow(non_snake_case)]
                fn from_stream<R>(stream: &mut ReadStream<R>) -> io::Result<Self>
                where
                    R: ReadBytesExt,
                    Self: Sized,
                {
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

impl FromStream for String {
    fn from_stream<R>(stream: &mut ReadStream<R>) -> io::Result<Self>
    where
        R: ReadBytesExt,
    {
        let len = stream.u16()?;
        if len == 0 {
            return Ok(String::new());
        }
        let data = stream.read_n_bytes(len as usize)?;
        let data = &data[..data.len() - 1]; // remove NULL byte
        Ok(String::from_utf8_lossy(data).to_string())
    }
}

impl ToStream for String {
    fn to_stream<W>(&self, stream: &mut WriteStream<W>) -> io::Result<usize>
    where
        W: WriteBytesExt,
    {
        if self.is_empty() {
            return stream.u16(0);
        }
        let data = self.as_bytes();
        Ok(stream.u16(data.len() as u16 + 1)? + stream.write_n_bytes(data)? + stream.u8(0)?)
    }
}

impl<T> FromStream for Vec<T>
where
    T: FromStream,
{
    fn from_stream<R>(stream: &mut ReadStream<R>) -> io::Result<Self>
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
    fn to_stream<W>(&self, stream: &mut WriteStream<W>) -> io::Result<usize>
    where
        W: WriteBytesExt,
    {
        let mut total = stream.u32(self.len() as u32)?;
        for item in self.iter() {
            total += stream.write(item)?;
        }
        Ok(total)
    }
}

impl<K, V> ToStream for HashMap<K, V>
where
    K: ToStream,
    V: ToStream,
{
    fn to_stream<W>(&self, stream: &mut WriteStream<W>) -> io::Result<usize>
    where
        W: WriteBytesExt,
    {
        let mut total = stream.u32(self.len() as u32)?;
        for (k, v) in self.iter() {
            total += stream.write(k)?;
            total += stream.write(v)?;
        }
        Ok(total)
    }
}

impl<K, V> FromStream for HashMap<K, V>
where
    K: FromStream + std::hash::Hash + std::cmp::Eq,
    V: FromStream,
{
    fn from_stream<R>(stream: &mut ReadStream<R>) -> io::Result<Self>
    where
        R: ReadBytesExt,
        Self: Sized,
    {
        let n = stream.u32()?;
        let mut m: Self = Default::default();
        for _i in 0..n {
            let k = stream.read()?;
            let v = stream.read()?;
            m.insert(k, v);
        }
        Ok(m)
    }
}
