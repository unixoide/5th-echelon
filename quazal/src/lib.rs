#![feature(seek_stream_len, proc_macro_hygiene, cursor_remaining, hash_extract_if)]
#![deny(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]

#[macro_use]
extern crate slog;
#[macro_use]
extern crate quazal_macros;

use std::collections::HashMap;
use std::net::SocketAddr;
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
    #[display(fmt = "Service {_0} not found")]
    ServiceNotFound(#[error(not(source))] String),
}

#[derive(Debug)]
pub struct ClientInfo<T = ()> {
    sequence_id: u16,
    client_signature: Option<u32>,
    server_signature: u32,
    client_session: u8,
    server_session: u8,
    packet_fragments: HashMap<u8, Vec<u8>>,
    address: SocketAddr,
    last_seen: Instant,
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
            sequence_id: 1,
            client_signature: None,
            server_signature: rand::random(),
            client_session: Default::default(),
            server_session: Default::default(),
            user_id: None,
            packet_fragments: HashMap::default(),
            address,
            additional: Default::default(),
            last_seen: std::time::Instant::now(),
        }
    }

    pub fn address(&self) -> &SocketAddr {
        &self.address
    }

    pub fn seen(&mut self) {
        self.last_seen = std::time::Instant::now();
    }
}
