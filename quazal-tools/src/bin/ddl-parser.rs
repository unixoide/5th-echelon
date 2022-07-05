#![feature(arbitrary_enum_discriminant)]

use clap::{App, Arg};
use nom::AsBytes;
use quazal_tools::generate::*;
use quazal_tools::parse::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::{fs, path::Path};

const MAGIC: u32 = 0xCD65_2312;

fn main() {
    let matches = App::new("DDL Binary parser")
        .arg(
            Arg::with_name("verbose")
                .long("verbose")
                .short("v")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("debug")
                .long("debug")
                .short("d")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("output")
                .long("output")
                .short("o")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("generate")
                .long("generate")
                .short("g")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("id_mapping")
                .long("ids")
                .short("i")
                .takes_value(true),
        )
        .arg(Arg::with_name("binary").required(true))
        .get_matches();

    DEBUG.store(
        matches.is_present("debug"),
        std::sync::atomic::Ordering::Relaxed,
    );
    let binary_path = Path::new(matches.value_of("binary").expect("binary is required"));

    if !binary_path.exists() {
        panic!("{} does not exist", binary_path.to_string_lossy());
    }

    let generate_dir: Option<&Path> = matches.value_of("generate").map(Path::new).map(|p| {
        if !p.exists() {
            panic!("{} does not exist", p.to_string_lossy());
        } else {
            p
        }
    });

    let id_mapping: Option<HashMap<String, u16>> = matches
        .value_of("id_mapping")
        .map(fs::File::open)
        .transpose()
        .expect("Couldn't open id mapping file")
        .map(serde_json::from_reader)
        .transpose()
        .expect("Couldn't parse id mapping");

    let file_data = fs::read(binary_path)
        .unwrap_or_else(|e| panic!("Couldn\'t read {}: {}", binary_path.to_string_lossy(), e));

    let magic_bytes = &MAGIC.to_be_bytes()[..];

    let mut data = file_data.as_bytes();

    let mut namespaces = Vec::default();

    while !data.is_empty() {
        let pos = file_data.len() - data.len();
        if pos % 1000 == 0 {
            eprint!("\rPosition: {:#x}", pos);
        }
        data = if data.starts_with(magic_bytes) {
            eprint!("\rPosition: {:#x}", pos);
            eprintln!("\nParsing");
            let (d, mut namespace) = parse_ddl(data, pos).unwrap();
            if matches.is_present("verbose") {
                println!("{:#?}", namespace);
            }

            if id_mapping.is_some() {
                for el in namespace.elements.iter_mut() {
                    if let Element::ProtocolDeclaration(p) = el {
                        let id = id_mapping.as_ref().and_then(|m| {
                            m.get(&format!("{}::{}", p.namespace, p.name1))
                                .or_else(|| m.get(&p.name1))
                        });
                        p.id = id.copied();
                    }
                }
            }

            namespaces.push(namespace);
            d
        } else {
            &data[1..]
        };
    }

    if let Some(output) = matches.value_of("output") {
        serde_json::to_writer_pretty(
            fs::File::create(output).expect("could not open output file"),
            &namespaces,
        )
        .expect("error writing output");
    }

    if let Some(generate) = generate_dir {
        write_modules(
            generate.to_path_buf(),
            namespaces
                .iter()
                .map(|n| generate_source(generate, n))
                .collect::<std::io::Result<Vec<_>>>()
                .unwrap()
                .into_iter()
                .fold(HashSet::new(), |mut s, v| {
                    s.extend(v);
                    s
                })
                .into_iter(),
        )
        .unwrap();
    }
}
