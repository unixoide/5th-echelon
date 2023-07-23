#![deny(clippy::pedantic)]

use std::env;
use std::fs;

use quazal::rmc::basic::FromStream;
use quazal::rmc::types::Variant;

fn main() {
    let fname = env::args().nth(1).unwrap();
    let data = fs::read(fname).unwrap();
    let map: Vec<(String, Variant)> = FromStream::from_bytes(&data).unwrap();

    println!("{map:#?}");
}
