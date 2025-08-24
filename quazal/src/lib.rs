#![feature(seek_stream_len, proc_macro_hygiene, cursor_split, hash_extract_if, associated_type_defaults)]
#![deny(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]

#[macro_use]
extern crate slog;
#[macro_use]
extern crate quazal_macros;

use std::collections::HashMap;
use std::net::SocketAddr;
use std::str::FromStr;
use std::time::Instant;

use derive_more::Display;
use derive_more::Error as DeriveError;

pub mod config;
pub mod kerberos;
pub mod prudp;
pub mod rmc;

pub use crate::config::*;

#[derive(Debug, Display, DeriveError)]
pub enum Error {
    #[display("Service {_0} not found")]
    ServiceNotFound(#[error(not(source))] String),
    InvalidValue,
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub struct ConnectionID(u32);

impl From<ConnectionID> for u32 {
    fn from(value: ConnectionID) -> Self {
        value.0
    }
}

impl FromStr for ConnectionID {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ConnectionID(s.parse()?))
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct Signature(u32);

#[derive(Debug)]
pub struct ClientInfo<T = ()> {
    server_sequence_id: u16,
    client_sequence_id: u16,
    client_signature: Option<u32>,
    server_signature: u32,
    client_session: u8,
    server_session: u8,
    packet_fragments: HashMap<u8, Vec<u8>>,
    address: SocketAddr,
    last_seen: Instant,
    pub connection_id: Option<ConnectionID>,
    pub user_id: Option<u32>,
    pub additional: T,
}

impl<T> ClientInfo<T> {
    #[must_use]
    pub fn new(address: SocketAddr) -> ClientInfo<T>
    where
        T: Default,
    {
        ClientInfo {
            server_sequence_id: 1,
            client_sequence_id: 1,
            client_signature: None,
            server_signature: rand::random(),
            client_session: Default::default(),
            server_session: Default::default(),
            user_id: None,
            packet_fragments: HashMap::default(),
            address,
            additional: Default::default(),
            last_seen: std::time::Instant::now(),
            connection_id: None,
        }
    }

    pub fn address(&self) -> &SocketAddr {
        &self.address
    }

    pub fn seen(&mut self) {
        self.last_seen = std::time::Instant::now();
    }
}
