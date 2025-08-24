//! A decoder for Quazal packets.
#![deny(clippy::pedantic)]
#![allow(clippy::cast_possible_truncation)]

use std::env::args;
use std::io::stdin;
use std::io::stdout;
use std::io::Read;
use std::io::Write;

use quazal::prudp::packet::QPacket;
use quazal::Context;
use sloggers::Build;

/// The main entry point for the decoder.
fn main() {
    let mut builder = sloggers::terminal::TerminalLoggerBuilder::new();
    builder.level(sloggers::types::Severity::Debug);
    let _logger = builder.build().unwrap();
    let mut data = vec![];
    stdin().read_to_end(&mut data).unwrap();
    let ctx = Context::splinter_cell_blacklist();
    while !data.is_empty() {
        match dbg!(QPacket::from_bytes(&ctx, &data)) {
            Ok(pack) => {
                eprintln!("Signature is {}", if pack.0.validate(&ctx, &data[..pack.1 as usize]).is_ok() { "valid" } else { "invalid" });
                data.drain(..pack.1 as usize);
                let a = args().nth(1);
                if let Some("dump") = a.as_deref() {
                    stdout().write_all(&pack.0.payload).unwrap();
                }
            }
            Err(e) => panic!("{}", e),
        }
    }
}
