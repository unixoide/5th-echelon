#![deny(clippy::pedantic)]

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
use std::sync::Arc;

use quazal::prudp::packet::QPacket;
use quazal::ClientInfo;
use quazal::Config;
use quazal::Context;
use quazal::Service;
use slog::Drain;
use slog::Logger;
use sloggers::Build;
use storage::Storage;

const SERVER_PID: u32 = 0x1000;

macro_rules! rmc_err {
    ($e:expr, $log:expr, $msg:literal) => (
        $e.map_err(|e| {
            slog::error!($log, $msg; "error" => ?e);
            quazal::rmc::Error::InternalError
        })
    )
}

fn login_required<T>(ci: &ClientInfo<T>) -> quazal::rmc::Result<u32> {
    ci.user_id.ok_or(quazal::rmc::Error::AccessDenied)
}

mod challenge;
mod clan;
mod game_session;
mod game_session_ex;
mod ladder;
mod locale;
mod nat_traversal;
mod overlord_challenge;
mod overlord_core;
mod overlord_news;
mod player_stats;
mod privileges;
mod protocols;
mod secure;
mod simple_http;
mod storage;
mod ticket;
mod tracking;
mod tracking_ext;
mod ubi_acc_mgmt;
mod uplay_win;
mod user_storage;

fn start_server(logger: &slog::Logger, ctx: &Context, storage: &Arc<Storage>, is_secure: bool) {
    use quazal::prudp::packet::StreamHandlerRegistry;
    use quazal::prudp::packet::StreamType;
    use quazal::prudp::packet::VPort;
    use quazal::prudp::Server;
    use quazal::rmc::RVSecHandler;

    let mut handler = RVSecHandler::<()>::new(logger.clone());

    if is_secure {
        handler.register_protocol(challenge::new_protocol());
        handler.register_protocol(clan::new_protocol());
        handler.register_protocol(game_session_ex::new_protocol(Arc::clone(storage)));
        handler.register_protocol(game_session::new_protocol(Arc::clone(storage)));
        handler.register_protocol(ladder::new_protocol());
        handler.register_protocol(locale::new_protocol());
        handler.register_protocol(nat_traversal::new_protocol());
        handler.register_protocol(overlord_challenge::new_protocol());
        handler.register_protocol(overlord_core::new_protocol());
        handler.register_protocol(overlord_news::new_protocol());
        handler.register_protocol(player_stats::new_protocol());
        handler.register_protocol(privileges::new_protocol());
        handler.register_protocol(secure::new_protocol());
        handler.register_protocol(tracking_ext::new_protocol());
        handler.register_protocol(tracking::new_protocol());
        handler.register_protocol(ubi_acc_mgmt::new_protocol(Arc::clone(storage)));
        handler.register_protocol(uplay_win::new_protocol());
        handler.register_protocol(user_storage::new_protocol());
    } else {
        handler.register_protocol(ticket::new_protocol(Arc::clone(storage)));
    }

    let mut registry = StreamHandlerRegistry::new(logger.clone());
    registry.register(
        VPort {
            stream_type: StreamType::RVSec,
            port: ctx.vport,
        },
        Box::new(handler),
    );

    let mut server = Server::new(logger.clone(), ctx, registry);
    server.expired_client_handler = Some(|ci: ClientInfo| {
        if let Some(user_id) = ci.user_id {
            info!(logger, "Cleaning old session of user {user_id}");
            if let Err(e) = storage.delete_user_session(user_id) {
                error!(logger, "session clean error: {e}");
            }
        }
    });
    server.disconnect_handler = Some(|ci: ClientInfo| {
        if let Some(user_id) = ci.user_id {
            info!(logger, "Cleaning closed session of user {user_id}");
            if let Err(e) = storage.delete_user_session(user_id) {
                error!(logger, "session clean error: {e}");
            }
        }
    });
    if is_secure {
        server.user_handler = Some(handle_user_packet);
    }
    server.bind(ctx.listen).unwrap();
    server.serve();
}

fn handle_user_packet(logger: &Logger, packet: QPacket, client: SocketAddr, socket: &UdpSocket) {
    info!(logger, "user rmc incoming");
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
        .level(
            #[allow(clippy::match_same_arms)]
            match std::env::var("RUST_LOG")
                .unwrap_or_else(|_| String::from("info"))
                .as_str()
            {
                "debug" => sloggers::types::Severity::Debug,
                "trace" => sloggers::types::Severity::Trace,
                "info" => sloggers::types::Severity::Info,
                "error" => sloggers::types::Severity::Error,
                "critical" => sloggers::types::Severity::Critical,
                "warning" => sloggers::types::Severity::Warning,
                _ => sloggers::types::Severity::Trace,
            },
        )
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

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let logger = build_term_logger();
    let logger = Logger::root(slog::Duplicate(logger, build_file_logger()).fuse(), o!());
    let storage = Arc::new(Storage::init(logger.clone())?);

    let config_filename = env::args()
        .nth(1)
        .unwrap_or_else(|| "service.toml".to_string());

    let config = Config::load_from_file(&config_filename).unwrap_or_else(|e| {
        error!(logger, "Couldn't load service file, generating default"; "error" => %e);
        Config {
            services: ["sc_bl".to_string()].into(),
            service: [(
                "sc_bl".to_string(),
                Service::Authentication(Context::splinter_cell_blacklist()),
            )]
            .into_iter()
            .collect(),
        }
    });

    // if let Err(e) = config.save_to_file(config_filename) {
    //     error!(logger, "Couldn't save service file"; "error" => %e);
    // }

    let mut threads = vec![];
    for (name, svc) in config.into_services()? {
        let logger = logger.new(o!("service" => name.clone()));
        info!(logger, "Loaded service {:#?}", svc);
        let storage = Arc::clone(&storage);
        let handle = match svc {
            quazal::Service::Authentication(ctx) => std::thread::Builder::new()
                .name(name)
                .spawn(move || start_server(&logger, &ctx, &storage, false)),
            quazal::Service::Secure(ctx) => std::thread::Builder::new()
                .name(name)
                .spawn(move || start_server(&logger, &ctx, &storage, true)),
            quazal::Service::Config(cfg) => std::thread::Builder::new()
                .name(name)
                .spawn(move || simple_http::serve(&logger, cfg.listen, &cfg.content).unwrap()),
            quazal::Service::Content(srv) => std::thread::Builder::new()
                .name(name)
                .spawn(move || simple_http::serve_many(&logger, srv.listen, &srv.files).unwrap()),
        };
        threads.push(handle.unwrap());
    }

    threads
        .into_iter()
        .map(std::thread::JoinHandle::join)
        .for_each(std::result::Result::unwrap);

    Ok(())
}
