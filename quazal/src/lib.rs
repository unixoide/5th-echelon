#![feature(seek_stream_len, proc_macro_hygiene, cursor_remaining)]

#[macro_use]
extern crate slog;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate quazal_macros;

use derive_more::{Display, Error as DeriveError};

use std::{collections::HashMap, net::SocketAddr};

pub mod config;
pub mod kerberos;
pub mod prudp;
pub mod rmc;

pub use crate::config::*;

#[derive(Debug, Display, DeriveError)]
pub enum Error {
    #[display(fmt = "Service {} not found", _0)]
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
    pub user_id: Option<u32>,
    pub additional: T,
}

impl<T> ClientInfo<T> {
    pub fn new(address: SocketAddr) -> ClientInfo<T>
    where
        T: Default,
    {
        ClientInfo {
            sequence_id: 1,
            client_signature: Default::default(),
            server_signature: rand::random(),
            client_session: Default::default(),
            server_session: Default::default(),
            user_id: Default::default(),
            packet_fragments: Default::default(),
            address,
            additional: Default::default(),
        }
    }

    pub fn address(&self) -> &SocketAddr {
        &self.address
    }
}
