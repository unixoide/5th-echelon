use std::io::Cursor;

use hmac::{digest::Update, Hmac, Mac};
use md5::{Digest, Md5};
use sodiumoxide::crypto::secretbox;

use crate::{
    prudp::packet::Rc4,
    rmc::basic::{FromStream, ReadStream, ToStream},
};

pub const SESSION_KEY_SIZE: usize = 16;

lazy_static! {
  // TODO use random key
  static ref KEY: secretbox::Key = secretbox::Key::from_slice(&[0; secretbox::KEYBYTES]).unwrap();
}

#[derive(Debug, ToStream, FromStream)]
pub struct KerberosTicketInternal {
    pub principle_id: u32,
    pub valid_until: u64,
    pub session_key: [u8; SESSION_KEY_SIZE],
}

impl KerberosTicketInternal {
    fn seal(&self) -> Vec<u8> {
        let n = secretbox::gen_nonce();
        let mut c = secretbox::seal(&self.as_bytes(), &n, &KEY);

        let mut res = Vec::with_capacity(c.len() + secretbox::NONCEBYTES);
        res.extend_from_slice(n.as_ref());
        res.append(&mut c);
        res
    }

    pub fn open(data: &[u8]) -> std::io::Result<Self> {
        if data.len() < secretbox::NONCEBYTES {
            return Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, ""));
        }
        let n = secretbox::Nonce::from_slice(&data[..secretbox::NONCEBYTES]).unwrap();
        let data = &data[secretbox::NONCEBYTES..];
        let data = secretbox::open(data, &n, &KEY)
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, ""))?;
        let mut stream = ReadStream::from_bytes(data);
        stream.read()
    }
}

#[derive(Debug)]
pub struct KerberosTicket {
    pub session_key: [u8; SESSION_KEY_SIZE],
    pub pid: u32,
    pub internal: KerberosTicketInternal,
}

impl KerberosTicket {
    fn derive_key(peer_pid: u32, password: Option<&str>) -> Vec<u8> {
        // derive key
        let count = 65000 + (peer_pid % 1024);
        let mut key = password.unwrap_or("UbiDummyPwd").as_bytes().to_vec();

        for _ in 0..count {
            let h = Md5::new().chain(key).finalize();
            key = h.to_vec();
        }
        key
    }

    pub fn from_bytes(buf: &[u8], peer_pid: u32, password: Option<&str>) -> std::io::Result<Self> {
        let off = buf.len() - Md5::output_size();
        let (buf, _mac) = buf.split_at(off);

        let key = Self::derive_key(peer_pid, password);
        let buf: Vec<u8> = Rc4::new(&key).zip(buf).map(|(a, b)| a ^ b).collect();
        let mut rdr = ReadStream::from_reader(Cursor::new(buf));

        let session_key = FromStream::from_stream(&mut rdr)?;
        let pid = FromStream::from_stream(&mut rdr)?;
        let internal: Vec<u8> = rdr.read_all()?;
        let internal = KerberosTicketInternal::open(&internal)?;

        Ok(Self {
            pid,
            session_key,
            internal,
        })
    }

    pub fn as_bytes(&self, peer_pid: u32, password: Option<&str>) -> Vec<u8> {
        let mut buf = self.session_key.to_vec();
        buf.append(&mut self.pid.as_bytes());
        buf.append(&mut self.internal.seal().as_bytes());

        let key = dbg!(Self::derive_key(peer_pid, password));

        let mut buf: Vec<u8> = Rc4::new(&key).zip(&buf).map(|(a, b)| a ^ b).collect();

        let mut mac: Hmac<Md5> = Hmac::new_from_slice(&key).unwrap();
        Mac::update(&mut mac, &buf);
        buf.extend(mac.finalize().into_bytes());

        buf
    }
}
