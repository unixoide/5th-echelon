use std::fs::File;
use std::net::SocketAddr;
use std::net::SocketAddrV4;

use clap::{App, Arg};
use etherparse::InternetSlice;
use pcap_parser::traits::PcapNGPacketBlock;
use pcap_parser::traits::PcapReaderIterator;
use pcap_parser::*;
use quazal::prudp::packet::PacketFlag;
use quazal::prudp::packet::PacketType;
use quazal::prudp::packet::QPacket;
use quazal::rmc::Packet;
use quazal::Context;

fn main() {
    let matches = App::new("pcap-parser")
        .arg(Arg::with_name("input").required(true))
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("port")
                .takes_value(true)
                .default_value("3074")
                .validator(|v| {
                    if v.parse::<u16>().is_ok() {
                        return Ok(());
                    }
                    Err(format!("{} isn't a positive number", &*v))
                }),
        )
        .arg(
            Arg::with_name("key")
                .short("k")
                .long("key")
                .takes_value(true)
                .default_value("CD&ML"),
        )
        .get_matches();

    let file = File::open(matches.value_of("input").unwrap()).expect("Can't open file");

    let port: u16 = matches.value_of("port").unwrap().parse().unwrap();
    let key: &str = matches.value_of("key").unwrap();

    let mut reader = PcapNGReader::new(65536, file).expect("PcapNGReader");
    loop {
        match reader.next() {
            Ok((offset, block)) => {
                match block {
                    PcapBlockOwned::Legacy(_) => todo!(),
                    PcapBlockOwned::LegacyHeader(_) => todo!(),
                    PcapBlockOwned::NG(block) => match block {
                        Block::SectionHeader(_) => {}
                        Block::InterfaceDescription(_) => {}
                        Block::EnhancedPacket(epb) => parse(epb.packet_data(), port, key),
                        Block::SimplePacket(_) => todo!(),
                        Block::NameResolution(_) => todo!(),
                        Block::InterfaceStatistics(_) => {}
                        Block::SystemdJournalExport(_) => todo!(),
                        Block::DecryptionSecrets(_) => todo!(),
                        Block::Custom(_) => todo!(),
                        Block::Unknown(_) => todo!(),
                    },
                }
                reader.consume(offset);
            }
            Err(PcapError::Eof) => break,
            Err(PcapError::Incomplete) => {
                reader.refill().unwrap();
            }
            Err(e) => panic!("error while reading: {:?}", e),
        }
    }
}

fn parse(data: &[u8], port: u16, key: &str) {
    let packet = match etherparse::SlicedPacket::from_ethernet(data) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("ether parse error: {:?}", e);
            return;
        }
    };
    if let Some(transport) = packet.transport {
        match transport {
            etherparse::TransportSlice::Udp(udp) => {
                if udp.source_port() == port || udp.destination_port() == port {
                    let (src, dst) = if let Some(InternetSlice::Ipv4(ip, _)) = packet.ip {
                        (
                            SocketAddr::V4(SocketAddrV4::new(ip.source_addr(), udp.source_port())),
                            SocketAddr::V4(SocketAddrV4::new(
                                ip.destination_addr(),
                                udp.destination_port(),
                            )),
                        )
                    } else {
                        panic!()
                    };
                    dump(packet.payload, key, src, dst);
                }
            }
            etherparse::TransportSlice::Tcp(_) => {}
            etherparse::TransportSlice::Unknown(_) => unimplemented!(),
        }
    }
}

fn dump(data: &[u8], key: &str, src: SocketAddr, dst: SocketAddr) {
    let ctx = Context {
        access_key: key.as_bytes().to_owned(),
        ..Default::default()
    };

    let mut offset = 0;

    while offset < data.len() {
        let (prudp, size) = match QPacket::from_bytes(&ctx, &data[offset..]) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("[{} -> {}] PRUDP Parsing error: {:?}", src, dst, e);
                dbg!(&data);
                dbg!(&data[offset..]);
                break;
            }
        };
        offset += size as usize;
        // println!("{:?}", prudp);
        print!(
            "[{} -> {}] {}>{} {:?} ({}) {:?}",
            src,
            dst,
            prudp.source.port,
            prudp.destination.port,
            prudp.destination.stream_type,
            prudp
                .flags
                .iter()
                .map(|f| format!("{:?}", f))
                .collect::<Vec<_>>()
                .join("|"),
            prudp.packet_type
        );

        if matches!(prudp.packet_type, PacketType::Data) && !prudp.flags.contains(PacketFlag::Ack) {
            if let Some(0) = prudp.fragment_id {
                let rmc = match Packet::from_bytes(&prudp.payload) {
                    Ok(p) => p,
                    Err(e) => {
                        eprintln!("[{} -> {}] RMC Parsing error: {:?}", src, dst, e);
                        dbg!(&prudp.payload);
                        break;
                    }
                };
                // println!("{:?}", rmc);
                match rmc {
                    Packet::Request(req) => {
                        print!(" {}.{}", req.protocol_id, req.method_id)
                    }
                    Packet::Response(_) => {}
                }
            }
        }
        println!()
    }
}
