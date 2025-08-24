//! A decoder for RMC (Remote Method Call) packets.
#![deny(clippy::pedantic)]

use std::env::args;
use std::io::stdin;
use std::io::stdout;
use std::io::Read;
use std::io::Write;

use quazal::rmc::Packet;

/// The main entry point for the decoder.
fn main() {
    let mut data = vec![];
    stdin().read_to_end(&mut data).unwrap();
    // Attempt to decode the input data as an RMC Packet.
    if let Ok(pack) = dbg!(Packet::from_bytes(&data)) {
        let a = args().nth(1);
        // If the first command-line argument is "dump", extract and print the raw data.
        if let Some("dump") = a.as_deref() {
            // For a Request packet, dump its raw parameters.
            if let Packet::Request(r) = pack {
                stdout().write_all(&r.parameters).unwrap();
            }
            // For a Response packet, dump its raw result data if decoding was successful.
            else if let Packet::Response(r) = pack {
                if let Ok(rd) = r.result {
                    stdout().write_all(&rd.data).unwrap();
                }
            }
        }
    }
}
