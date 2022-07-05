use quazal::rmc::Packet;
use std::env::args;
use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut data = vec![];
    stdin().read_to_end(&mut data).unwrap();
    if let Ok(pack) = dbg!(Packet::from_bytes(&data)) {
        let a = args().nth(1);
        if let Some("dump") = a.as_deref() {
            if let Packet::Request(r) = pack {
                stdout().write_all(&r.parameters).unwrap();
            } else if let Packet::Response(r) = pack {
                if let Ok(rd) = r.result {
                    stdout().write_all(&rd.data).unwrap();
                }
            }
        }
    }
}
