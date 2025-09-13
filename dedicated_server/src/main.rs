#![deny(clippy::pedantic)]
#![feature(iter_intersperse)]

#[macro_use]
extern crate quazal_macros;
#[macro_use]
extern crate slog;

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
use quazal::Context;
use sc_bl_protocols as protocols;
use slog::Drain;
use slog::Logger;
use sloggers::Build;
use storage::Storage;

const DEFAULT_MP_DATA: &str = include_str!("../../data/mp_balancing.ini");

const SERVER_PID: u32 = 0x1000;

macro_rules! rmc_err {
    ($e:expr, $log:expr, $msg:literal) => (
        $e.map_err(|e| {
            slog::error!($log, $msg; "error" => ?e);
            quazal::rmc::Error::InternalError
        })
    )
}

/// Checks if a client is logged in and returns their user ID.
///
/// Returns an `AccessDenied` error if the client is not logged in.
fn login_required<T>(ci: &ClientInfo<T>) -> quazal::rmc::Result<u32> {
    ci.user_id.ok_or(quazal::rmc::Error::AccessDenied)
}

mod api;
mod challenge;
mod clan;
mod config;
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
mod secure;
mod simple_http;
mod storage;
mod ticket;
mod tracking;
mod tracking_ext;
mod ubi_acc_mgmt;
mod uplay_win;
mod user_storage;

use crate::config::Config;

/// Starts a Quazal server (either secure or authentication).
///
/// This function sets up the necessary protocols and handlers for the server
/// and then enters the server loop.
fn start_server(logger: &slog::Logger, ctx: &Context, storage: &Arc<Storage>, is_secure: bool) -> io::Result<()> {
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
    server.bind(ctx.listen)?;
    server.serve();
    Ok(())
}

/// Handles user-specific RMC packets.
///
/// This function is a placeholder for handling user-specific RMC packets.
fn handle_user_packet(_logger: &Logger, packet: QPacket, client: SocketAddr, socket: &UdpSocket) {
    // info!(logger, "user rmc incoming");
    assert_eq!(packet.source.port, 1);
    assert_eq!(packet.destination.port, 1);

    let payload = packet.payload;

    let mut response = payload;
    write!(&mut response, "udp:/address={};port={}\0", client.ip(), client.port()).unwrap();
    // TODO: reenable
    socket.send_to(&response, client).unwrap();
}

/// Builds a terminal logger with a configurable log level.
fn build_term_logger() -> Logger {
    sloggers::terminal::TerminalLoggerBuilder::new()
        .level(
            #[allow(clippy::match_same_arms)]
            match std::env::var("RUST_LOG").unwrap_or_else(|_| String::from("info")).as_str() {
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

/// Rotates log files, keeping a specified number of backups.
fn rotate_log_files<S: AsRef<Path>>(fname: S, i: i32) -> io::Result<PathBuf> {
    let fname = fname.as_ref();
    if fname.exists() {
        let maybe_iteration = fname.extension().and_then(|s| s.to_str()).and_then(|s| s.parse::<i32>().ok());
        let new_fname = match maybe_iteration {
            Some(j) if j == i - 1 => format!("{}.{}", fname.file_stem().and_then(|f| f.to_str()).unwrap(), i),
            _ => format!("{}.{}", fname.display(), i),
        };
        if i < 10 {
            rotate_log_files(&new_fname, i + 1)?;
        }
        fs::rename(fname, new_fname)?;
    }
    Ok(fname.to_path_buf())
}

/// Builds a file logger that writes to a JSON file.
fn build_file_logger() -> Logger {
    let fname = rotate_log_files("server.log.json", 1).unwrap();
    sloggers::file::FileLoggerBuilder::new(fname)
        .truncate()
        .level(sloggers::types::Severity::Trace)
        .format(sloggers::types::Format::Json)
        .build()
        .unwrap()
}

/// Ensures that the necessary data directory and files exist.
fn ensure_data_dir() -> io::Result<()> {
    let mp_ini = std::env::current_exe()?.parent().unwrap().join("data").join("mp_balancing.ini");
    if mp_ini.exists() {
        return Ok(());
    }
    fs::create_dir_all(mp_ini.parent().unwrap())?;
    fs::write(mp_ini, DEFAULT_MP_DATA)?;
    Ok(())
}

#[derive(argh::FromArgs)]
/// dedicated server
struct Args {
    /// path to config file (default: service.toml)
    #[argh(option, short = 'c', long = "config")]
    config_path: Option<PathBuf>,

    /// started through launcher
    #[argh(switch)]
    launcher: bool,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let args = argh::from_env::<Args>();

    let logger = if args.launcher {
        Logger::root(build_file_logger(), o!())
    } else {
        let logger = build_term_logger();
        Logger::root(slog::Duplicate(logger, build_file_logger()).fuse(), o!())
    };

    {
        let logger = logger.clone();
        let old_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |panic_info| {
            error!(logger, "Panic occurred: {panic_info}");
            old_hook(panic_info);
        }));
    }

    let storage = Arc::new(Storage::init(logger.clone())?);

    let config_filename = args.config_path.unwrap_or_else(|| PathBuf::from("service.toml"));

    let config = Config::load_from_file_or_default(&logger, config_filename)?;

    ensure_data_dir()?;

    warn!(logger, "Clearing stale sessions");
    storage.invalidate_sessions()?;

    let mut threads = vec![];
    for (name, svc) in config.quazal.into_services()? {
        let logger = logger.new(o!("service" => name.clone()));
        info!(logger, "Loaded service {:#?}", svc);
        let storage = Arc::clone(&storage);
        let handle = match svc {
            quazal::Service::Authentication(ctx) => std::thread::Builder::new().name(name).spawn(move || {
                if let Err(e) = start_server(&logger, &ctx, &storage, false) {
                    crit!(logger, "Error running authentication server: {e:?}");
                }
            }),
            quazal::Service::Secure(ctx) => std::thread::Builder::new().name(name).spawn(move || {
                if let Err(e) = start_server(&logger, &ctx, &storage, true) {
                    crit!(logger, "Error running secure server: {e:?}");
                }
            }),
            quazal::Service::Config(cfg) => std::thread::Builder::new().name(name).spawn(move || {
                if cfg.listen.port() != 80 {
                    warn!(
                        logger,
                        "Unexpected port {} used for the config server. Clients are expecting port 80. Adjust in the service config or make sure to redirect traffic accordingly",
                        cfg.listen.port()
                    );
                }
                if let Err(e) = simple_http::serve(&logger, cfg.listen, &cfg.content()) {
                    crit!(logger, "Error running config server: {e:?}");
                }
            }),
            quazal::Service::Content(srv) => std::thread::Builder::new().name(name).spawn(move || {
                if let Err(e) = simple_http::serve_many(&logger, srv.listen, &srv.files) {
                    crit!(logger, "Error running content server: {e:?}");
                }
            }),
        };
        threads.push(handle.unwrap());
    }

    threads.push(
        std::thread::Builder::new()
            .name(String::from("api"))
            .spawn(move || {
                let logger = logger.new(o!("service" => "api"));
                if let Err(e) =
                    tokio::runtime::Runtime::new()
                        .unwrap()
                        .block_on(api::start_server(logger.clone(), storage, config.api_server, Arc::new(config.debug), args.launcher))
                {
                    crit!(logger, "Error running api server: {e:?}");
                }
            })
            .unwrap(),
    );

    threads.into_iter().map(std::thread::JoinHandle::join).for_each(std::result::Result::unwrap);

    Ok(())
}
