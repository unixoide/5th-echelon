#[macro_use]
extern crate quazal_macros;
#[macro_use]
extern crate slog;

use std::env;
use std::fs;
use std::io;
use std::io::Write;
use std::net::SocketAddr;
use std::net::UdpSocket;
use std::path::Path;
use std::path::PathBuf;

use quazal::prudp::packet::QPacket;
use quazal::Config;
use quazal::Context;
use quazal::Service;
use slog::Drain;
use slog::Logger;
use sloggers::Build;

mod challenge;
mod clan;
mod game_session;
mod ladder;
mod locale;
mod overlord_challenge;
mod overlord_core;
mod overlord_news;
mod player_stats;
mod privileges;
mod protocols;
mod secure;
mod simple_http;
mod ticket;
mod tracking;
mod tracking_ext;
mod ubi_acc_mgmt;
mod uplay_win;
mod user_storage;

mod game_session_ex;

fn start_server(logger: slog::Logger, ctx: Context, is_secure: bool) {
    use quazal::prudp::packet::*;
    use quazal::prudp::Server;
    use quazal::rmc::*;

    let mut handler = RVSecHandler::<()>::new(logger.clone());

    if is_secure {
        handler.register_protocol(secure::new_protocol());
        handler.register_protocol(overlord_core::new_protocol());
        handler.register_protocol(overlord_news::new_protocol());
        handler.register_protocol(overlord_challenge::new_protocol());
        handler.register_protocol(ladder::new_protocol());
        handler.register_protocol(locale::new_protocol());
        handler.register_protocol(ubi_acc_mgmt::new_protocol());
        handler.register_protocol(challenge::new_protocol());
        handler.register_protocol(clan::new_protocol());
        handler.register_protocol(player_stats::new_protocol());
        handler.register_protocol(privileges::new_protocol());
        handler.register_protocol(user_storage::new_protocol());
        handler.register_protocol(uplay_win::new_protocol());
        handler.register_protocol(game_session::new_protocol());
        handler.register_protocol(game_session_ex::new_protocol());
        handler.register_protocol(tracking_ext::new_protocol());
        handler.register_protocol(tracking::new_protocol());
    } else {
        handler.register_protocol(ticket::new_protocol());
    }

    let mut registry = StreamHandlerRegistry::new(logger.clone());
    registry.register(
        VPort {
            stream_type: StreamType::RVSec,
            port: ctx.vport,
        },
        Box::new(handler),
    );

    /*
    let mut handler = RVSecHandler::<()>::new(logger.clone());
    handler.register_protocol(overlord_core::new_protocol());
    handler.register_protocol(secure::new_protocol());
    handler.register_protocol(ladder::new_protocol());
    registry.register(
        VPort {
            stream_type: StreamType::RVSec,
            port: 2, // don't know why it connects to port 2 yet
        },
        Box::new(handler),
    );
    */
    let mut server = Server::new(logger, &ctx, registry);
    if is_secure {
        server.user_handler = Some(handle_user_packet);
    }
    server.bind(&ctx.listen).unwrap();
    server.serve().unwrap()
}

fn handle_user_packet(packet: QPacket, client: SocketAddr, socket: &UdpSocket) {
    assert_eq!(packet.source.port, 1);
    assert_eq!(packet.destination.port, 1);

    let payload = packet.payload;

    let mut response = payload;
    write!(
        &mut response,
        "udp:/address={};port={}\0",
        client.ip(),
        client.port()
    )
    .unwrap();
    socket.send_to(dbg!(&response), client).unwrap();
}

fn build_term_logger() -> Logger {
    sloggers::terminal::TerminalLoggerBuilder::new()
        .level(sloggers::types::Severity::Trace)
        .format(sloggers::types::Format::Compact)
        .build()
        .unwrap()
}

fn rotate_log_files<S: AsRef<Path>>(fname: S, i: i32) -> io::Result<PathBuf> {
    let fname = fname.as_ref();
    if fname.exists() {
        let maybe_iteration = fname
            .extension()
            .and_then(|s| s.to_str())
            .and_then(|s| s.parse::<i32>().ok());
        let new_fname = match maybe_iteration {
            Some(j) if j == i - 1 => format!(
                "{}.{}",
                fname.file_stem().and_then(|f| f.to_str()).unwrap(),
                i
            ),
            _ => format!("{}.{}", fname.display(), i),
        };
        if i < 10 {
            rotate_log_files(&new_fname, i + 1)?;
        }
        fs::rename(fname, new_fname)?;
    }
    Ok(fname.to_path_buf())
}

fn build_file_logger() -> Logger {
    let fname = rotate_log_files("server.log.json", 1).unwrap();
    sloggers::file::FileLoggerBuilder::new(fname)
        .truncate()
        .level(sloggers::types::Severity::Trace)
        .format(sloggers::types::Format::Json)
        .build()
        .unwrap()
}

fn main() {
    let logger = build_term_logger();
    let logger = Logger::root(slog::Duplicate(logger, build_file_logger()).fuse(), o!());

    let services = Config::load_from_file(
        env::args()
            .nth(1)
            .unwrap_or_else(|| "service.toml".to_string()),
    )
    .unwrap_or_else(|e| {
        error!(logger, "Couldn't load service file, generating default"; "error" => %e);
        vec![(
            "sc_bl".to_string(),
            Service::Authentication(Context::splinter_cell_blacklist()),
        )]
    });

    let mut threads = vec![];
    for (name, svc) in services {
        let logger = logger.new(o!("service" => name.clone()));
        info!(logger, "Loaded service {:#?}", svc);
        let handle = match svc {
            quazal::Service::Authentication(ctx) => std::thread::Builder::new()
                .name(name)
                .spawn(move || start_server(logger, ctx, false)),
            quazal::Service::Secure(ctx) => std::thread::Builder::new()
                .name(name)
                .spawn(move || start_server(logger, ctx, true)),
            quazal::Service::Config(cfg) => std::thread::Builder::new()
                .name(name)
                .spawn(|| simple_http::serve(cfg.listen, cfg.content).unwrap()),
            quazal::Service::Content(srv) => std::thread::Builder::new()
                .name(name)
                .spawn(|| simple_http::serve_many(srv.listen, srv.files).unwrap()),
        };
        threads.push(handle.unwrap());
    }

    threads
        .into_iter()
        .map(std::thread::JoinHandle::join)
        .for_each(std::result::Result::unwrap);
}
