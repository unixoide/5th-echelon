use std::io::Cursor;

use hmac::digest::Update;
use hmac::Hmac;
use hmac::Mac;
use md5::Digest;
use md5::Md5;
use sodiumoxide::crypto::secretbox;

use crate::prudp::packet::Rc4;
use crate::rmc::basic::FromStream;
use crate::rmc::basic::FromStreamError;
use crate::rmc::basic::ReadStream;
use crate::rmc::basic::ToStream;

pub const SESSION_KEY_SIZE: usize = 16;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, ToStream, FromStream)]
pub struct KerberosTicketInternal {
    pub principle_id: u32,
    pub valid_until: u64,
    pub session_key: [u8; SESSION_KEY_SIZE],
}

impl KerberosTicketInternal {
    fn seal(&self, key: &secretbox::Key) -> Vec<u8> {
        let n = secretbox::gen_nonce();
        let mut c = secretbox::seal(&self.to_bytes(), &n, key);

        let mut res = Vec::with_capacity(c.len() + secretbox::NONCEBYTES);
        res.extend_from_slice(n.as_ref());
        res.append(&mut c);
        res
    }

    pub fn open(data: &[u8], key: &secretbox::Key) -> Result<Self, FromStreamError> {
        // eprintln!("####\nopen {:?}\n####", key.0);
        if data.len() < secretbox::NONCEBYTES {
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                format!("got {} bytes, required at least {}", data.len(), secretbox::NONCEBYTES),
            )
            .into());
        }
        let n = secretbox::Nonce::from_slice(&data[..secretbox::NONCEBYTES]).unwrap();
        let data = &data[secretbox::NONCEBYTES..];
        let data = secretbox::open(data, &n, key)
            .map_err(|()| std::io::Error::new(std::io::ErrorKind::InvalidData, "open failed"))?;
        let mut stream = ReadStream::from_bytes(data);
        stream.read()
    }
}

#[allow(clippy::module_name_repetitions)]
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

    pub fn from_bytes(
        buf: &[u8],
        peer_pid: u32,
        password: Option<&str>,
        key: &secretbox::Key,
    ) -> Result<Self, FromStreamError> {
        let off = buf.len() - Md5::output_size();
        let (buf, _mac) = buf.split_at(off);

        let obf_key = Self::derive_key(peer_pid, password);
        let buf: Vec<u8> = Rc4::new(&obf_key).zip(buf).map(|(a, b)| a ^ b).collect();
        let mut rdr = ReadStream::from_reader(Cursor::new(buf));

        let session_key = FromStream::from_stream(&mut rdr)?;
        let pid = FromStream::from_stream(&mut rdr)?;
        let internal: Vec<u8> = rdr.read_all()?;
        let internal = KerberosTicketInternal::open(&internal, key)?;

        Ok(Self {
            session_key,
            pid,
            internal,
        })
    }

    #[must_use]
    pub fn as_bytes(&self, peer_pid: u32, password: Option<&str>, key: &secretbox::Key) -> Vec<u8> {
        let mut buf = self.session_key.to_vec();
        buf.append(&mut self.pid.to_bytes());
        buf.append(&mut self.internal.seal(key).to_bytes());

        let key = Self::derive_key(peer_pid, password);

        let mut buf: Vec<u8> = Rc4::new(&key).zip(&buf).map(|(a, b)| a ^ b).collect();

        let mut mac: Hmac<Md5> = Hmac::new_from_slice(&key).unwrap();
        Mac::update(&mut mac, &buf);
        buf.extend(mac.finalize().into_bytes());

        buf
    }
}
