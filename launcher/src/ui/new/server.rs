#![allow(static_mut_refs)]
use std::borrow::Cow;
use std::fmt::Display;
use std::io::BufRead as _;
use std::io::BufReader;
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::process::Command;
use std::process::Stdio;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::Duration;

use dedicated_server_config::Config as ServerConfig;
use imgui::ProgressBar;
use imgui::TableColumnFlags;
use imgui::TableColumnSetup;
use itertools::Itertools;
use serde::Deserialize;
use server_api::games;
use server_api::games::Game;
use server_api::users;
use server_api::users::User;
use tonic::transport::Channel;
use tonic::Request;
use tracing::error;

use super::super::icons::ICON_REPEAT;
use super::super::icons::ICON_TRASH;
use super::colors::ORANGE;
use super::colors::RED;
use super::colors::YELLOW;
use crate::ui::AnimatedText;
use crate::ui::BackgroundValue;
use crate::version::Version;

pub(crate) static mut SERVER_PROCESS: Option<std::process::Child> = None;
static mut ADMIN_TOKEN: String = String::new();

#[derive(Debug)]
pub struct ServerMenu {
    adapters: Vec<(String, IpAddr)>,
    selected_ip: usize,
    dedicated_server_path: PathBuf,
    dedicated_server_cfg_path: PathBuf,
    dedicated_server_log_path: PathBuf,
    server_config: ServerConfig,
    logs: Option<Vec<LogItem>>,
    users: Option<Vec<User>>,
    selected_log_level: usize,
    games: Option<Vec<Game>>,
    download_text: AnimatedText<'static>,
    public_ip: Option<IpAddr>,
    download_progress: Arc<AtomicUsize>,
    downloader: Option<BackgroundValue<()>>,
    server_version: Option<Version>,
    latest_version: BackgroundValue<Option<Version>>,
}

impl ServerMenu {
    pub fn new(adapters: &[(String, IpAddr)]) -> Self {
        let dedicated_server_path = std::env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .join("dedicated_server.exe");
        let dedicated_server_cfg_path = dedicated_server_path.parent().unwrap().join("service.toml");
        let dedicated_server_log_path = dedicated_server_path.parent().unwrap().join("server.log.json");

        let server_config = if dedicated_server_path.exists() && dedicated_server_cfg_path.exists() {
            ServerConfig::load_from_file(&dedicated_server_cfg_path).unwrap()
        } else {
            ServerConfig::default()
        };
        let server_version = if dedicated_server_path.exists() {
            crate::dll_utils::get_dll_version(&dedicated_server_path).ok()
        } else {
            None
        };
        let latest_version = BackgroundValue::new_async(async {
            <crate::updater::Updater>::check_for_updates()
                .await
                .into_iter()
                .find(|a| a.name == "dedicated_server.exe")
                .map(|a| a.version)
        });

        let mut adapters = adapters.to_vec();
        adapters.sort_by(|a, b| a.1.cmp(&b.1));

        let selected_ip = adapters
            .iter()
            .enumerate()
            .find(|(_, (_, ip))| server_config.api_server.ip() == *ip)
            .map_or(0, |(i, _)| i + 1);

        let public_ip = server_config
            .quazal
            .service
            .values()
            .find_map(|svc| match svc {
                quazal::Service::Secure(context) | quazal::Service::Authentication(context) => {
                    context.secure_server_addr
                }
                quazal::Service::Config(_) => None,
                quazal::Service::Content(_) => None,
            })
            .map(|sa| sa.ip())
            .filter(|p_ip| p_ip != &server_config.api_server.ip());

        Self {
            adapters,
            selected_ip,
            dedicated_server_path,
            dedicated_server_cfg_path,
            dedicated_server_log_path,
            server_config,
            logs: None,
            users: None,
            selected_log_level: LogLevel::Info as _,
            games: None,
            download_text: AnimatedText::new(
                &["Downloading...", "Downloading..", "Downloading"],
                Duration::from_millis(200),
            ),
            public_ip,
            download_progress: Arc::new(AtomicUsize::new(0)),
            downloader: None,
            server_version,
            latest_version,
        }
    }

    pub fn render(&mut self, ui: &imgui::Ui) -> bool {
        if ui.arrow_button("Back", imgui::Direction::Left) {
            return false;
        }

        self.download_server_modal(ui);

        if !self.dedicated_server_path.exists() {
            ui.text_colored(RED.to_rgba_f32s(), "Dedicated server binary not found");
            ui.text("You only need this if you want to host the server component for others.");
            ui.text("As long as a server is available, any player can host a game on the server.");
            if ui.button("Download") {
                let progress = self.download_progress.clone();
                let dedicated_server_path = self.dedicated_server_path.clone();
                self.downloader = Some(BackgroundValue::new_async(async move {
                    let assets = <crate::updater::Updater>::check_for_updates().await;
                    let Some(asset) = assets.into_iter().find(|a| a.name == "dedicated_server.exe") else {
                        error!("Dedicated server release not found");
                        return;
                    };
                    <crate::updater::Updater>::download_with_progress(asset, &dedicated_server_path, progress.as_ref())
                        .await;
                }));
                ui.open_popup("Download Server");
            }
            return true;
        } else if let Some((Some(latest_version), ref server_version)) =
            self.latest_version.try_get().zip(self.server_version)
        {
            if latest_version > server_version {
                ui.text_colored(
                    YELLOW.to_rgba_f32s(),
                    format!(
                        "Dedicated server outdated (current: {}, latest: {})",
                        server_version, latest_version
                    ),
                );
                if ui.button("Download") {
                    let progress = self.download_progress.clone();
                    let dedicated_server_path = self.dedicated_server_path.clone();
                    self.downloader = Some(BackgroundValue::new_async(async move {
                        let assets = <crate::updater::Updater>::check_for_updates().await;
                        let Some(asset) = assets.into_iter().find(|a| a.name == "dedicated_server.exe") else {
                            error!("Dedicated server release not found");
                            return;
                        };
                        <crate::updater::Updater>::download_with_progress(
                            asset,
                            &dedicated_server_path,
                            progress.as_ref(),
                        )
                        .await;
                    }));
                    ui.open_popup("Download Server");
                }
            }
        }

        if let Some(child) = unsafe { SERVER_PROCESS.as_mut() } {
            if child.try_wait().unwrap().is_some() {
                unsafe { SERVER_PROCESS = None };
            }
        }

        let mut ips = self
            .adapters
            .iter()
            .map(|(name, ip)| format!("{ip} ({name})"))
            .collect::<Vec<_>>();
        ips.insert(0, String::from("All IPs"));
        ui.combo_simple_string("IP to listen on", &mut self.selected_ip, &ips);

        let mut use_public_ip = self.public_ip.is_some();
        if ui.checkbox("Use different public IP", &mut use_public_ip) {
            if use_public_ip {
                let ip = if self.selected_ip > 0 {
                    self.adapters[self.selected_ip - 1].1
                } else {
                    IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))
                };
                self.public_ip = Some(ip);
            } else {
                self.public_ip = None;
            }
        }
        if ui.is_item_hovered() {
            ui.tooltip_text("Use this if the server is behind a NAT or firewall.");
        }
        if let Some(ip) = self.public_ip.as_mut() {
            let mut ip_txt = ip.to_string();
            if ui.input_text("Public IP", &mut ip_txt).build() {
                self.public_ip = ip_txt.parse().ok();
            }
        }

        // SAFETY: this code is single threaded
        ui.disabled(unsafe { SERVER_PROCESS.is_some() }, || {
            if ui.button("Start") {
                let ip = if self.selected_ip == 0 {
                    "0.0.0.0".parse().unwrap()
                } else {
                    self.adapters[self.selected_ip - 1].1
                };
                self.replace_ips_and_save(ip, self.public_ip.unwrap_or(ip));
                self.start_server();
            }
        });
        ui.same_line();
        // SAFETY: this code is single threaded
        ui.disabled(unsafe { SERVER_PROCESS.is_none() }, || {
            if ui.button("Stop") {
                // SAFETY: this code is single threaded
                let mut child = unsafe { SERVER_PROCESS.take() }.unwrap();
                child.kill().unwrap();
                child.wait().unwrap();
            }
        });

        if let Some(tabbar) = ui.tab_bar("Server") {
            if let Some(item) = ui.tab_item("Logs") {
                ui.child_window("LogsWindow").build(|| self.log_table(ui));

                item.end();
            }
            if unsafe { SERVER_PROCESS.is_some() } {
                if let Some(item) = ui.tab_item("Users") {
                    ui.child_window("UsersWindow").build(|| self.users_table(ui));
                    item.end();
                }
                if let Some(item) = ui.tab_item("Games") {
                    ui.child_window("GamesWindow").build(|| self.games_table(ui));
                    item.end();
                }
            }
            tabbar.end();
        }

        true
    }

    fn replace_ips_and_save(&mut self, ip: IpAddr, public_ip: IpAddr) {
        self.server_config.api_server.set_ip(ip);
        for svc in self.server_config.quazal.service.values_mut() {
            match svc {
                quazal::Service::Authentication(context) | quazal::Service::Secure(context) => {
                    context.listen.set_ip(ip);
                    if let Some(addr) = context.secure_server_addr.as_mut() {
                        addr.set_ip(public_ip)
                    }
                    if let Some(sh) = context.settings.get_mut("storage_host") {
                        if let Ok(mut addr) = sh.parse::<SocketAddr>() {
                            addr.set_ip(public_ip);
                            *sh = addr.to_string();
                        }
                    }
                }
                quazal::Service::Config(online_config) => {
                    online_config.set_ips(ip, public_ip);
                }
                quazal::Service::Content(content_server) => content_server.listen.set_ip(ip),
            }
        }
        self.server_config
            .save_to_file(&self.dedicated_server_cfg_path)
            .unwrap();
    }

    fn start_server(&mut self) {
        let mut child = Command::new(&self.dedicated_server_path)
            .arg("--launcher")
            .current_dir(self.dedicated_server_path.parent().unwrap())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
        let stdout = BufReader::new(child.stdout.take().unwrap());
        for line in stdout.lines() {
            let Ok(line) = line else {
                break;
            };
            if let Some(token) = line.strip_prefix("Admin Key: ") {
                unsafe {
                    ADMIN_TOKEN = token.to_string();
                }
                break;
            }
        }
        unsafe { SERVER_PROCESS = Some(child) };
        self.logs = None;
    }

    fn log_table(&mut self, ui: &imgui::Ui) {
        let log_levels = [
            LogLevel::Trace,
            LogLevel::Debug,
            LogLevel::Info,
            LogLevel::Warn,
            LogLevel::Error,
            LogLevel::Critical,
        ];
        ui.combo("Min Log Level", &mut self.selected_log_level, &log_levels, |lvl| {
            Cow::Owned(lvl.to_string())
        });
        if ui.button(format!("{ICON_REPEAT}")) || self.logs.is_none() {
            self.logs = load_logs(&self.dedicated_server_log_path, log_levels[self.selected_log_level])
                .inspect_err(|e| {
                    error!("Error loading logs: {e}");
                })
                .or_else(|e| {
                    if let Some(io_err) = e.downcast_ref::<std::io::Error>() {
                        // don't try again if the file was not found
                        if io_err.kind() == std::io::ErrorKind::NotFound {
                            return Ok(Vec::new());
                        }
                    }
                    Err(e)
                })
                .ok();
        }
        if ui.is_item_hovered() {
            ui.tooltip_text("Reload logs");
        }

        if let Some(table) = ui.begin_table_header(
            "LogTable",
            [
                TableColumnSetup {
                    name: "TS",
                    flags: TableColumnFlags::WIDTH_FIXED,
                    init_width_or_weight: ui.calc_text_size("2025-05-01T20:28:58.4773547Z")[0],
                    ..Default::default()
                },
                TableColumnSetup {
                    name: "Level",
                    flags: TableColumnFlags::WIDTH_FIXED,
                    init_width_or_weight: ui.calc_text_size("DEBUG")[0],
                    ..Default::default()
                },
                TableColumnSetup {
                    name: "Message",
                    flags: TableColumnFlags::WIDTH_STRETCH,
                    init_width_or_weight: 1.0,
                    ..Default::default()
                },
            ],
        ) {
            if let Some(logs) = self.logs.as_ref() {
                for log in logs {
                    ui.table_next_row();
                    let printer: Box<dyn Fn(&str) -> _> = match log.level {
                        LogLevel::Trace | LogLevel::Debug => Box::new(|s| ui.text_disabled(s)),
                        LogLevel::Info => Box::new(|s| ui.text(s)),
                        LogLevel::Warn => Box::new(|s| ui.text_colored(ORANGE.to_rgba_f32s(), s)),
                        LogLevel::Error | LogLevel::Critical => Box::new(|s| ui.text_colored(RED.to_rgba_f32s(), s)),
                    };
                    ui.table_next_column();
                    (printer)(log.ts.as_str());
                    ui.table_next_column();
                    (printer)(&log.level.to_string());
                    ui.table_next_column();
                    (printer)(log.msg.as_str());
                }
            }
            table.end();
        }
    }

    fn users_table(&mut self, ui: &imgui::Ui) {
        if ui.button(format!("{ICON_REPEAT}")) || self.users.is_none() {
            tokio::runtime::Runtime::new().unwrap().block_on(async {
                self.users = load_users(format!("http://{}", self.server_config.api_server))
                    .await
                    .inspect_err(|e| {
                        error!("Error loading users: {e}");
                    })
                    .ok();
            });
        }
        if ui.is_item_hovered() {
            ui.tooltip_text("Reload users");
        }

        if let Some(table) = ui.begin_table_header(
            "UsersTable",
            [
                TableColumnSetup {
                    name: "User ID",
                    flags: TableColumnFlags::WIDTH_FIXED,
                    init_width_or_weight: ui.calc_text_size("999999")[0],
                    ..Default::default()
                },
                TableColumnSetup {
                    name: "Username",
                    flags: TableColumnFlags::WIDTH_STRETCH,
                    ..Default::default()
                },
                TableColumnSetup {
                    name: "IPs",
                    flags: TableColumnFlags::WIDTH_STRETCH,
                    ..Default::default()
                },
                TableColumnSetup {
                    name: "Actions",
                    flags: TableColumnFlags::WIDTH_FIXED,
                    init_width_or_weight: 100.0,
                    ..Default::default()
                },
            ],
        ) {
            if let Some(users) = self.users.as_ref() {
                let mut deleted = false;
                for user in users {
                    ui.table_next_row();
                    ui.table_next_column();
                    ui.text(&user.id);
                    ui.table_next_column();
                    ui.text(&user.username);
                    ui.table_next_column();
                    ui.text(user.ips.join(", "));
                    ui.table_next_column();
                    if ui.button(format!("{ICON_TRASH}")) {
                        let api_url = format!("http://{}", self.server_config.api_server);
                        let user_id = user.id.clone();
                        deleted = tokio::runtime::Runtime::new().unwrap().block_on(async {
                            delete_user(api_url.clone(), user_id.clone())
                                .await
                                .inspect_err(|e| {
                                    error!("Error deleting user {}: {}", user_id, e);
                                })
                                .is_ok()
                        });
                    }
                    if ui.is_item_hovered() {
                        ui.tooltip_text("Delete user");
                    }
                }
                if deleted {
                    self.users = None;
                }
            }
            table.end();
        }
    }

    fn games_table(&mut self, ui: &imgui::Ui) {
        if ui.button(format!("{ICON_REPEAT}")) || self.games.is_none() {
            tokio::runtime::Runtime::new().unwrap().block_on(async {
                self.games = load_games(format!("http://{}", self.server_config.api_server))
                    .await
                    .inspect_err(|e| {
                        error!("Error loading games: {e}");
                    })
                    .ok();
            });
        }
        if ui.is_item_hovered() {
            ui.tooltip_text("Reload games");
        }

        if let Some(table) = ui.begin_table_header(
            "GamesTable",
            [
                TableColumnSetup {
                    name: "Game ID",
                    flags: TableColumnFlags::WIDTH_FIXED,
                    init_width_or_weight: ui.calc_text_size("999999")[0],
                    ..Default::default()
                },
                TableColumnSetup {
                    name: "Game Type",
                    flags: TableColumnFlags::WIDTH_FIXED,
                    init_width_or_weight: ui.calc_text_size("Unknown(99)")[0],
                    ..Default::default()
                },
                TableColumnSetup {
                    name: "Host",
                    flags: TableColumnFlags::WIDTH_STRETCH,
                    ..Default::default()
                },
                TableColumnSetup {
                    name: "Members",
                    flags: TableColumnFlags::WIDTH_STRETCH,
                    ..Default::default()
                },
                TableColumnSetup {
                    name: "Actions",
                    flags: TableColumnFlags::WIDTH_FIXED,
                    init_width_or_weight: 100.0,
                    ..Default::default()
                },
            ],
        ) {
            if let Some(games) = self.games.as_ref() {
                let mut deleted = false;
                for game in games {
                    ui.table_next_row();
                    ui.table_next_column();
                    ui.text(format!("{}", game.id));
                    ui.table_next_column();
                    ui.text(&game.game_type);
                    ui.table_next_column();
                    ui.text(&game.creator);
                    ui.table_next_column();
                    ui.text(game.participants.join(", "));
                    ui.table_next_column();
                    if ui.button(format!("{ICON_TRASH}")) {
                        let api_url = format!("http://{}", self.server_config.api_server);
                        let game_id = game.id;
                        deleted = tokio::runtime::Runtime::new().unwrap().block_on(async {
                            delete_game(api_url.clone(), game_id)
                                .await
                                .inspect_err(|e| {
                                    error!("Error deleting game {}: {}", game_id, e);
                                })
                                .is_ok()
                        });
                    }
                    if ui.is_item_hovered() {
                        ui.tooltip_text("Delete game");
                    }
                }
                if deleted {
                    self.games = None;
                }
            }
            table.end();
        }
    }

    fn download_server_modal(&mut self, ui: &imgui::Ui) {
        ui.modal_popup_config("Download Server")
            .resizable(false)
            .movable(false)
            .build(|| {
                if let Some(()) = self.downloader.as_mut().and_then(BackgroundValue::try_take) {
                    ui.close_current_popup();
                    self.downloader.take();
                    self.server_version = if self.dedicated_server_path.exists() {
                        crate::dll_utils::get_dll_version(&self.dedicated_server_path).ok()
                    } else {
                        None
                    };
                }
                self.download_text.update();
                ui.text(self.download_text.text());
                ProgressBar::new(self.download_progress.load(Ordering::Relaxed) as f32 / 100.0).build(ui);
            });
    }

    pub fn server_version(&self) -> Option<Version> {
        self.server_version
    }
}

#[derive(Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum LogLevel {
    #[serde(rename = "TRCE")]
    Trace = 0,
    #[serde(rename = "DEBG")]
    Debug = 1,
    #[serde(rename = "INFO")]
    Info = 2,
    #[serde(rename = "WARN")]
    Warn = 3,
    #[serde(rename = "ERRO")]
    Error = 4,
    #[serde(rename = "CRIT")]
    Critical = 5,
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Trace => write!(f, "TRACE"),
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Warn => write!(f, "WARN"),
            LogLevel::Error => write!(f, "ERROR"),
            LogLevel::Critical => write!(f, "CRIT"),
        }
    }
}

#[derive(Debug, Deserialize)]
struct LogItem {
    msg: String,
    level: LogLevel,
    ts: String,
}

fn load_logs(path: &PathBuf, min_level: LogLevel) -> anyhow::Result<Vec<LogItem>> {
    let file = std::fs::File::open(path)?;
    let reader = BufReader::new(file);
    Ok(reader
        .lines()
        .take_while(Result::is_ok)
        .filter_map(Result::ok)
        .filter_map(|line| serde_json::from_str::<LogItem>(&line).ok())
        .filter(|item| item.level >= min_level)
        .tail(100)
        .collect())
}

async fn users_admin_client(
    api_server_url: String,
) -> anyhow::Result<
    users::users_admin_client::UsersAdminClient<
        tonic::service::interceptor::InterceptedService<Channel, impl tonic::service::Interceptor>,
    >,
> {
    let channel = tonic::transport::Channel::from_shared(api_server_url)?
        .connect()
        .await?;

    Ok(users::users_admin_client::UsersAdminClient::with_interceptor(
        channel,
        |mut req: Request<()>| {
            req.metadata_mut()
                .insert("authorization", unsafe { ADMIN_TOKEN.parse().unwrap() });
            Ok(req)
        },
    ))
}

async fn games_admin_client(
    api_server_url: String,
) -> anyhow::Result<
    games::games_admin_client::GamesAdminClient<
        tonic::service::interceptor::InterceptedService<Channel, impl tonic::service::Interceptor>,
    >,
> {
    let channel = tonic::transport::Channel::from_shared(api_server_url)?
        .connect()
        .await?;

    Ok(games::games_admin_client::GamesAdminClient::with_interceptor(
        channel,
        |mut req: Request<()>| {
            req.metadata_mut()
                .insert("authorization", unsafe { ADMIN_TOKEN.parse().unwrap() });
            Ok(req)
        },
    ))
}

async fn load_users(api_server_url: String) -> anyhow::Result<Vec<User>> {
    let mut client = users_admin_client(api_server_url).await?;
    let resp = client.list(users::ListRequest::default()).await?;
    let users = resp.into_inner().users;
    Ok(users)
}

async fn delete_user(api_server_url: String, user_id: String) -> anyhow::Result<()> {
    let mut client = users_admin_client(api_server_url).await?;
    let _resp = client.delete(users::DeleteRequest { id: user_id }).await?;
    Ok(())
}

async fn load_games(api_server_url: String) -> anyhow::Result<Vec<Game>> {
    let mut client = games_admin_client(api_server_url).await?;
    let resp = client.list(games::ListRequest::default()).await?;
    let games = resp.into_inner().games;
    Ok(games)
}

async fn delete_game(api_server_url: String, game_id: u32) -> anyhow::Result<()> {
    let mut client = games_admin_client(api_server_url).await?;
    let _resp = client.delete(games::DeleteRequest { id: game_id }).await?;
    Ok(())
}
