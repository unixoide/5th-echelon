#![windows_subsystem = "windows"]

use std::collections::HashMap;
use std::ffi::CString;
use std::ffi::OsStr;
use std::ffi::OsString;
use std::fs;
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::os::windows::ffi::OsStringExt;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;

use hooks_addresses::Addresses;
use imgui::Context;
use imgui::Style;
use imgui::StyleColor;
use server_api::users::users_client::UsersClient;
use server_api::users::LoginRequest;
use server_api::users::RegisterRequest;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use windows::core::w;
use windows::core::PCSTR;
use windows::Win32::Foundation::ERROR_MORE_DATA;
use windows::Win32::Foundation::ERROR_SUCCESS;
use windows::Win32::System::Registry;
use windows::Win32::UI::WindowsAndMessaging::MessageBoxA;
use windows::Win32::UI::WindowsAndMessaging::MB_OK;

#[cfg(target_os = "windows")]
#[path = "windows.rs"]
mod sys;

#[cfg(not(target_os = "windows"))]
#[path = "unix.rs"]
mod sys;

mod dll_utils;
mod render;

const ID_MODAL_ASK_SEARCH: &str = "Unknown Executables";
const ID_MODAL_SEARCHING: &str = "Identifying...";
const ID_MODAL_LOADING: &str = "Loading...";
const ID_MODAL_REGISTER: &str = "Register";

#[cfg(feature = "embed-dll")]
static COMPRESSED_DLL: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/uplay_r1_loader.dll.brotli"));

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum GameVersion {
    SplinterCellBlacklistDx9,
    SplinterCellBlacklistDx11,
}

impl GameVersion {
    fn executable(self) -> &'static str {
        match self {
            GameVersion::SplinterCellBlacklistDx9 => "Blacklist_game.exe",
            GameVersion::SplinterCellBlacklistDx11 => "Blacklist_DX11_game.exe",
        }
    }

    fn full_path(self, base_dir: &Path) -> PathBuf {
        base_dir.join(self.executable())
    }

    fn label(&self) -> &'static str {
        match self {
            GameVersion::SplinterCellBlacklistDx9 => "DirectX 9",
            GameVersion::SplinterCellBlacklistDx11 => "DirectX 11",
        }
    }
}

fn sc_style(style: &mut Style) {
    style.colors[StyleColor::Text as usize] = [1.00, 1.00, 1.00, 1.00];
    style.colors[StyleColor::TextDisabled as usize] = [0.50, 0.50, 0.50, 1.00];
    style.colors[StyleColor::WindowBg as usize] = [0.03, 0.07, 0.04, 0.94];
    style.colors[StyleColor::ChildBg as usize] = [0.00, 0.00, 0.00, 0.00];
    style.colors[StyleColor::PopupBg as usize] = [0.08, 0.08, 0.08, 0.94];
    style.colors[StyleColor::Border as usize] = [0.38, 1.00, 0.00, 0.50];
    style.colors[StyleColor::BorderShadow as usize] = [0.01, 0.13, 0.00, 0.63];
    style.colors[StyleColor::FrameBg as usize] = [0.17, 0.48, 0.16, 0.54];
    style.colors[StyleColor::FrameBgHovered as usize] = [0.26, 0.98, 0.32, 0.40];
    style.colors[StyleColor::FrameBgActive as usize] = [0.26, 0.98, 0.28, 0.67];
    style.colors[StyleColor::TitleBg as usize] = [0.01, 0.07, 0.01, 1.00];
    style.colors[StyleColor::TitleBgActive as usize] = [0.0, 0.56, 0.29, 1.0];
    style.colors[StyleColor::TitleBgCollapsed as usize] = [0.00, 0.56, 0.09, 0.51];
    style.colors[StyleColor::MenuBarBg as usize] = [0.0, 0.56, 0.29, 1.0];
    // style.colors[StyleColor::TitleBg as usize] = [0.01, 0.07, 0.01, 1.00];
    // style.colors[StyleColor::TitleBgActive as usize] = [0.0, 0.29, 0.68, 1.0];
    // style.colors[StyleColor::TitleBgCollapsed as usize] = [0.00, 0.56, 0.09, 0.51];
    // style.colors[StyleColor::MenuBarBg as usize] = [0.0, 0.29, 0.68, 1.0];
    style.colors[StyleColor::ScrollbarBg as usize] = [0.00, 0.15, 0.00, 0.53];
    style.colors[StyleColor::ScrollbarGrab as usize] = [0.10, 0.41, 0.06, 1.00];
    style.colors[StyleColor::ScrollbarGrabHovered as usize] = [0.00, 0.66, 0.04, 1.00];
    style.colors[StyleColor::ScrollbarGrabActive as usize] = [0.04, 0.87, 0.00, 1.00];
    style.colors[StyleColor::CheckMark as usize] = [0.26, 0.98, 0.40, 1.00];
    style.colors[StyleColor::SliderGrab as usize] = [0.21, 0.61, 0.00, 1.00];
    style.colors[StyleColor::SliderGrabActive as usize] = [0.36, 0.87, 0.22, 1.00];
    style.colors[StyleColor::Button as usize] = [0.00, 0.60, 0.05, 0.40];
    style.colors[StyleColor::ButtonHovered as usize] = [0.20, 0.78, 0.32, 1.00];
    style.colors[StyleColor::ButtonActive as usize] = [0.00, 0.57, 0.07, 1.00];
    style.colors[StyleColor::Header as usize] = [0.12, 0.82, 0.28, 0.31];
    style.colors[StyleColor::HeaderHovered as usize] = [0.00, 0.74, 0.11, 0.80];
    style.colors[StyleColor::HeaderActive as usize] = [0.09, 0.69, 0.04, 1.00];
    style.colors[StyleColor::Separator as usize] = [0.09, 0.67, 0.01, 0.50];
    style.colors[StyleColor::SeparatorHovered as usize] = [0.32, 0.75, 0.10, 0.78];
    style.colors[StyleColor::SeparatorActive as usize] = [0.10, 0.75, 0.11, 1.00];
    style.colors[StyleColor::ResizeGrip as usize] = [0.32, 0.98, 0.26, 0.20];
    style.colors[StyleColor::ResizeGripHovered as usize] = [0.26, 0.98, 0.28, 0.67];
    style.colors[StyleColor::ResizeGripActive as usize] = [0.22, 0.69, 0.06, 0.95];
    style.colors[StyleColor::Tab as usize] = [0.18, 0.58, 0.18, 0.86];
    style.colors[StyleColor::TabHovered as usize] = [0.26, 0.98, 0.28, 0.80];
    style.colors[StyleColor::TabActive as usize] = [0.20, 0.68, 0.24, 1.00];
    style.colors[StyleColor::TabUnfocused as usize] = [0.07, 0.15, 0.08, 0.97];
    style.colors[StyleColor::TabUnfocusedActive as usize] = [0.14, 0.42, 0.19, 1.00];
    style.colors[StyleColor::PlotLines as usize] = [0.61, 0.61, 0.61, 1.00];
    style.colors[StyleColor::PlotLinesHovered as usize] = [1.00, 0.43, 0.35, 1.00];
    style.colors[StyleColor::PlotHistogram as usize] = [0.90, 0.70, 0.00, 1.00];
    style.colors[StyleColor::PlotHistogramHovered as usize] = [1.00, 0.60, 0.00, 1.00];
    style.colors[StyleColor::TableHeaderBg as usize] = [0.19, 0.19, 0.20, 1.00];
    style.colors[StyleColor::TableBorderStrong as usize] = [0.31, 0.31, 0.35, 1.00];
    style.colors[StyleColor::TableBorderLight as usize] = [0.23, 0.23, 0.25, 1.00];
    style.colors[StyleColor::TableRowBg as usize] = [0.00, 0.00, 0.00, 0.00];
    style.colors[StyleColor::TableRowBgAlt as usize] = [1.00, 1.00, 1.00, 0.06];
    style.colors[StyleColor::TextSelectedBg as usize] = [0.00, 0.89, 0.20, 0.35];
    style.colors[StyleColor::DragDropTarget as usize] = [1.00, 1.00, 0.00, 0.90];
    style.colors[StyleColor::NavHighlight as usize] = [0.26, 0.98, 0.35, 1.00];
    style.colors[StyleColor::NavWindowingHighlight as usize] = [1.00, 1.00, 1.00, 0.70];
    style.colors[StyleColor::NavWindowingDimBg as usize] = [0.80, 0.80, 0.80, 0.20];
    style.colors[StyleColor::ModalWindowDimBg as usize] = [0.80, 0.80, 0.80, 0.35];
}

fn show_msgbox(msg: &str, caption: &str) {
    let msg = CString::new(msg).unwrap();
    let caption = CString::new(caption).unwrap();
    unsafe {
        MessageBoxA(
            None,
            PCSTR(msg.as_ptr().cast::<u8>()),
            PCSTR(caption.as_ptr().cast::<u8>()),
            MB_OK,
        );
    }
}

fn get_install_dir() -> Option<PathBuf> {
    let mut buf = vec![0u16; 2048];
    let mut bufsz = buf.len() as u32 * 2;
    let buf = loop {
        let e = unsafe {
            Registry::RegGetValueW(
                Registry::HKEY_LOCAL_MACHINE,
                w!(r"SOFTWARE\Ubisoft\Splinter Cell Blacklist"),
                w!("installdir"),
                Registry::RRF_RT_REG_SZ | Registry::RRF_SUBKEY_WOW6432KEY,
                None,
                Some(buf.as_mut_ptr().cast()),
                Some(&mut bufsz),
            )
        };
        match e {
            ERROR_MORE_DATA => {
                buf.resize(bufsz as usize / 2, 0);
            }
            ERROR_SUCCESS => {
                // RegGetValue returns null terminated data
                buf.resize(bufsz as usize / 2 - 1, 0);
                break Some(buf);
            }
            _ => {
                break None;
            }
        }
    };

    buf.as_deref()
        .map(OsString::from_wide)
        .as_deref()
        .and_then(OsStr::to_str)
        .map(PathBuf::from)
}

enum GameHookState {
    Resolved(Box<Addresses>),
    FileNotFound,
    Searching,
    Ignored,
    UnsupportedBinary,
    Failed(String),
}

struct GameHook {
    version: GameVersion,
    state: GameHookState,
    target_dir: PathBuf,
    background_search: BackgroundValue<GameHookState>,
}

impl GameHook {
    pub fn new(gv: GameVersion, target_dir: PathBuf) -> Self {
        let res = hooks_addresses::get_from_path(&gv.full_path(&target_dir))
            .inspect_err(|e| println!("{e}"));
        let state = match res {
            Ok(addrs) => GameHookState::Resolved(Box::new(addrs)),
            Err(hooks_addresses::Error::UnknownBinary(_)) => GameHookState::FileNotFound,
            Err(hooks_addresses::Error::BinaryMismatch(_, _)) => GameHookState::UnsupportedBinary,
            Err(hooks_addresses::Error::NoFileName(_)) => GameHookState::FileNotFound,
            Err(hooks_addresses::Error::IdFailed) => {
                GameHookState::Failed("couldn't identify".to_string())
            }
            Err(hooks_addresses::Error::IO(e)) => GameHookState::Failed(format!("{e}")),
        };
        Self {
            version: gv,
            state,
            target_dir,
            background_search: BackgroundValue::Unset,
        }
    }

    pub fn is_ready(&self) -> bool {
        matches!(self.state, GameHookState::Resolved(_))
    }

    pub fn search(&mut self) {
        self.state = GameHookState::Searching;
        let path = self.version.full_path(&self.target_dir);
        self.background_search = BackgroundValue::Handle(std::thread::spawn(move || {
            hooks_addresses::search_patterns(&path)
                .inspect_err(|e| println!("{e}"))
                .map_or_else(
                    |e| GameHookState::Failed(e.to_string()),
                    |a| GameHookState::Resolved(Box::new(a)),
                )
        }));
    }
}

struct GameHooks {
    games: HashMap<GameVersion, GameHook>,
}

impl GameHooks {
    fn new(games: HashMap<GameVersion, GameHook>) -> Self {
        Self { games }
    }

    fn has_only_unknown(&self) -> bool {
        !self.games.values().any(|g| {
            matches!(
                g.state,
                GameHookState::Resolved(_) | GameHookState::Ignored | GameHookState::Searching
            )
        })
    }

    fn is_searching(&self) -> bool {
        self.games
            .values()
            .any(|g| matches!(g.state, GameHookState::Searching))
    }

    fn get(&self, gv: GameVersion) -> Option<&GameHook> {
        self.games.get(&gv)
    }

    fn iter_ready(&self) -> impl Iterator<Item = &GameHook> {
        self.games.values().filter(|g| g.is_ready())
    }

    fn start_searching(&mut self) {
        for hook in self.games.values_mut() {
            if matches!(hook.state, GameHookState::UnsupportedBinary) {
                hook.search();
            }
        }
    }

    fn search_status(&mut self) -> bool {
        let mut finished = true;
        for hook in self.games.values_mut() {
            if let GameHookState::Searching = hook.state {
                if let Some(new_state) = hook.background_search.try_take() {
                    hook.state = new_state;
                } else {
                    finished = false;
                }
            }
        }
        finished
    }

    fn ignore_unknown_binaries(&mut self) {
        for hook in self.games.values_mut() {
            if !matches!(hook.state, GameHookState::Resolved(_)) {
                hook.state = GameHookState::Ignored;
            }
        }
    }

    fn iter(&self) -> impl Iterator<Item = &GameHook> {
        self.games.values()
    }
}

fn enable_console() {
    unsafe {
        let _ = windows::Win32::System::Console::AttachConsole(
            windows::Win32::System::Console::ATTACH_PARENT_PROCESS,
        );
    }
}

fn catch_panics() {
    std::panic::set_hook(Box::new(|panic_info| {
        let mut expl = String::new();

        let message = match (
            panic_info.payload().downcast_ref::<&str>(),
            panic_info.payload().downcast_ref::<String>(),
        ) {
            (Some(s), _) => Some((*s).to_string()),
            (_, Some(s)) => Some(s.to_string()),
            (None, None) => None,
        };

        let cause = match message {
            Some(m) => m,
            None => "Unknown".into(),
        };

        match panic_info.location() {
            Some(location) => expl.push_str(&format!(
                "Panic occurred in file '{}' at line {}",
                location.file(),
                location.line()
            )),
            None => expl.push_str("Panic location unknown."),
        }
        let msg = format!("{}\n{}", expl, cause);
        eprintln!("PANIC: {msg}");

        show_msgbox(&msg, "PANIC");

        std::process::exit(1);
    }));
}

fn load_config(target_dir: &Path) -> hooks_config::Config {
    fs::read_to_string(hooks_config::get_config_path(target_dir))
        .ok()
        .and_then(|s| toml::from_str(&s).ok())
        .unwrap_or_else(|| {
            let cfg = hooks_config::default();
            let _ = toml::to_string_pretty(&cfg)
                .ok()
                .and_then(|s| fs::write(hooks_config::get_config_path(target_dir), s).ok());
            cfg
        })
}

fn main() {
    enable_console();
    catch_panics();

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let target_dir = find_target_dir();
    println!("Found target dir {target_dir:?}");

    #[cfg(feature = "embed-dll")]
    drop_dll(&target_dir);

    let mut cfg = load_config(&target_dir);
    let mut saved_cfg = cfg.clone();
    let mut api_server = cfg.api_server.to_string();
    let adapters = sys::find_adapter_names();
    let (adapters, adapter_ips) = adapters.into_iter().unzip::<_, _, Vec<_>, Vec<_>>();

    /* create context */
    let mut imgui = Context::create();
    sc_style(imgui.style_mut());

    /* disable creation of files on disc */
    imgui.set_ini_filename(None);
    imgui.set_log_filename(None);

    /* setup platform and renderer, and fonts to imgui */
    let header_font = setup_fonts(&mut imgui);

    let mut selected_game: Option<(GameVersion, GameState)> = None;

    let mut launch_error = None;

    let mut addresses = None;

    let mut exe_loader = { load_game_binaries(&target_dir) };

    let dll_version = get_dll_version(target_dir.join("uplay_r1_loader.dll"));
    let expected_dll_version: (usize, usize, usize) = {
        let mut iter = env!("HOOKS_VERSION")
            .split('.')
            .flat_map(|s| s.parse().ok());
        (
            iter.next().unwrap_or_default(),
            iter.next().unwrap_or_default(),
            iter.next().unwrap_or_default(),
        )
    };

    let mut show_outdated_dll_warning = dll_version.0 < expected_dll_version.0
        || dll_version.0 == expected_dll_version.0 && dll_version.1 < expected_dll_version.1
        || dll_version.0 == expected_dll_version.0
            && dll_version.1 == expected_dll_version.1
            && dll_version.2 < expected_dll_version.2;

    render::render(
        imgui,
        |ui: &mut imgui::Ui, w: f32, h: f32, logo_texture: imgui::TextureId| {
            /* create imgui UI here */
            ui.window("Settings")
                .size([w, h], imgui::Condition::Always)
                .position([0f32, 0f32], imgui::Condition::Always)
                .movable(false)
                .resizable(false)
                .title_bar(false)
                .build(|| {
                    if addresses.is_none() {
                        addresses = draw_loading_screen(ui, &mut exe_loader);
                        return;
                    }

                    let addresses = addresses.as_mut().unwrap();
                    draw_title(ui, header_font, logo_texture);
                    ui.separator();
                    ui.set_window_font_scale(0.75);
                    ui.text_disabled(format!(
                        "Install dir: {}",
                        target_dir.as_os_str().to_string_lossy()
                    ));
                    ui.text_disabled(format!("Launcher Version: {}", env!("CARGO_PKG_VERSION")));
                    ui.same_line();
                    #[cfg(feature = "embed-dll")]
                    {
                        ui.text_disabled(format!("Bundled DLL version: {}", env!("HOOKS_VERSION")));
                    }
                    #[cfg(not(feature = "embed-dll"))]
                    {
                        ui.text_disabled(format!("Current DLL version: {}", env!("HOOKS_VERSION")));
                    }
                    ui.same_line();
                    ui.text_disabled(format!(
                        "Installed DLL version: {}.{}.{}",
                        dll_version.0, dll_version.1, dll_version.2
                    ));
                    ui.set_window_font_scale(1.0);

                    ui.modal_popup("Outdated DLL", || {
                        ui.text("Installed DLL is outdated.");
                        if ui.button("Ok") {
                            ui.close_current_popup();
                        }
                    });
                    if show_outdated_dll_warning {
                        show_outdated_dll_warning = false;
                        ui.open_popup("Outdated DLL");
                    }

                    draw_main_settings(ui, &mut cfg);
                    draw_networking_settings(
                        ui,
                        &mut cfg,
                        &mut api_server,
                        &adapters,
                        &adapter_ips,
                    );
                    if let Some(GameHook {
                        state: GameHookState::Resolved(ref addr),
                        ..
                    }) = selected_game.and_then(|sg| addresses.get(sg.0))
                    {
                        draw_debug_settings(ui, &mut cfg, addr);
                    }
                    ui.separator();
                    ui.disabled(saved_cfg == cfg, || {
                        if ui.button("Save") {
                            if cfg.networking.adapter.is_some() {
                                cfg.networking.ip_address.take();
                            }
                            fs::write(
                                hooks_config::get_config_path("."),
                                toml::to_string_pretty(&cfg).unwrap(),
                            )
                            .unwrap();
                            saved_cfg = cfg.clone();
                        }
                    });
                    ui.same_line();
                    ui.disabled(saved_cfg == cfg, || {
                        if ui.button("Reset") {
                            cfg = saved_cfg.clone();
                        }
                    });
                    ui.same_line();

                    selected_game = get_selected_executable(ui, addresses, selected_game.map(|(gv, _)| gv));
                    ui.same_line();

                    ui.enabled(saved_cfg == cfg && matches!(selected_game, Some((_, GameState::Ready))), || {
                        if ui.button("Launch") {
                            let executable = selected_game.unwrap().0.full_path(&target_dir);
                            match std::process::Command::new(&executable).spawn() {
                                Err(e) => launch_error = Some(format!("{executable:?}: {e}")),
                                Ok(_) => std::process::exit(0),
                            }
                        }
                        if let Some(error) = &launch_error {
                            ui.text_colored([1.0, 0.0, 0.0, 1.0], error)
                        }
                    });
                    ui.same_line();
                    ui.enabled(saved_cfg == cfg && matches!(selected_game, Some((_, GameState::NotReady))), || {
                        if ui.button("Identify") {                            
                            ui.open_popup(ID_MODAL_SEARCHING);
                            addresses.start_searching();
                        }
                    });

                    let should_open_search_modal = ui.modal_popup_config(ID_MODAL_ASK_SEARCH)
                        .resizable(false)
                        .movable(false)
                        .build(|| {
                            ui.text("None of the executabels seem to be known by this launcher.\n\nAttempt to identify?");
                            if ui.button("Yes") {
                                addresses.start_searching();
                                ui.close_current_popup();
                                return true;
                            }
                            ui.same_line();
                            if ui.button("No") {
                                addresses.ignore_unknown_binaries();
                                ui.close_current_popup();
                            }
                            false
                        });

                    let search_modal_opened = ui.modal_popup_config(ID_MODAL_SEARCHING)
                        .resizable(false )
                        .movable(false)
                        .always_auto_resize(true)
                        .build(|| {
                            {
                                ui.text("Identifying...");
                                for hook in addresses.iter() {
                                    ui.text(format!("{}: ", hook.version.label()));
                                    ui.same_line();
                                    match &hook.state {
                                        GameHookState::Resolved(_) => ui.text_colored([0f32, 1f32, 0f32, 1f32], "OK"),
                                        GameHookState::FileNotFound => ui.text_colored([0f32, 0f32, 0f32, 1f32], "Not found"),
                                        GameHookState::Searching => ui.text_colored([1f32, 1f32, 0f32, 1f32], "Testing..."),
                                        GameHookState::Ignored => ui.text_colored([0.4f32, 0.4f32, 0.4f32, 1f32], "IGNORED"),
                                        GameHookState::UnsupportedBinary => ui.text_colored([1f32, 0f32, 0f32, 1f32], "UNSUPPORTED"),
                                        GameHookState::Failed(f) => ui.text_colored([1f32, 0f32, 0f32, 1f32], format!("FAILURE: {f}")),
                                    }
                                }
                                if addresses.search_status() && ui.button("Save Results") {
                                    let gen_hash = |g: &GameHook| {
                                        let GameHookState::Resolved(a) = &g.state else { return None; };
                                        let Ok(hash) = hooks_addresses::hash_file(g.version.full_path(&g.target_dir)) else {return None;};
                                        Some(HashMap::from([(hash, *a.clone())]))
                                    };
                                    let dx9 = addresses.get(GameVersion::SplinterCellBlacklistDx9).and_then(gen_hash).unwrap_or_default();
                                    let dx11 = addresses.get(GameVersion::SplinterCellBlacklistDx11).and_then(gen_hash).unwrap_or_default();
                                    hooks_addresses::save_addresses(&target_dir, dx9, dx11);
                                    ui.close_current_popup();
                                }
                            }
                        }).is_some();

                    if should_open_search_modal.is_none() && !search_modal_opened && addresses.has_only_unknown() {
                        ui.open_popup(ID_MODAL_ASK_SEARCH);
                    } else if let Some(true) = should_open_search_modal {
                        ui.open_popup(ID_MODAL_SEARCHING);
                    }
                });
        },
    );
}

fn find_target_dir() -> PathBuf {
    let mut target_dir = std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .to_owned();

    if let Some(install_dir) = get_install_dir() {
        target_dir = install_dir;
    }

    let candidates: Vec<_> = [
        "Blacklist_game.exe",
        "SYSTEM\\Blacklist_game.exe",
        "src\\SYSTEM\\Blacklist_game.exe",
    ]
    .into_iter()
    .map(|p| target_dir.join(p))
    .collect();

    for path in candidates {
        if path.exists() {
            if let Some(dir) = path.parent() {
                target_dir = dir.canonicalize().unwrap();
                std::env::set_current_dir(dir).unwrap();
                break;
            }
        }
    }
    target_dir
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GameState {
    Ready,
    NotReady,
}

impl From<bool> for GameState {
    fn from(b: bool) -> Self {
        if b {
            GameState::Ready
        } else {
            GameState::NotReady
        }
    }
}

fn get_selected_executable(
    ui: &imgui::Ui,
    games: &GameHooks,
    selected_gv: Option<GameVersion>,
) -> Option<(GameVersion, GameState)> {
    let mut versions = games
        .iter()
        .map(|g| match g.version {
            GameVersion::SplinterCellBlacklistDx11 => {
                ((g.version, g.is_ready().into()), "DirectX 11")
            }
            GameVersion::SplinterCellBlacklistDx9 => {
                ((g.version, g.is_ready().into()), "DirectX 9")
            }
        })
        .collect::<Vec<_>>();
    versions.sort_by_key(|(gv, _)| gv.0);

    let (versions, options): (Vec<(GameVersion, GameState)>, Vec<&str>) =
        versions.into_iter().unzip();
    let largest_text = options
        .iter()
        .map(|s| ui.calc_text_size(s)[0])
        .max_by(|a, b| a.abs().partial_cmp(&b.abs()).unwrap())
        .unwrap_or_default()
        + ui.frame_height()
        + unsafe { ui.style() }.frame_padding[0];
    let mut selected = if let Some(sgv) = selected_gv {
        versions
            .iter()
            .enumerate()
            .find_map(|(idx, gv)| if gv.0 == sgv { Some(idx) } else { None })
            .unwrap_or_default()
    } else {
        0
    };
    let _token = ui.push_item_width(largest_text);
    ui.combo_simple_string("##game", &mut selected, &options);
    if versions.is_empty() {
        None
    } else {
        Some(versions[selected])
    }
}

fn setup_fonts(imgui: &mut Context) -> imgui::FontId {
    let font_size = 24.0;
    imgui.fonts().add_font(&[imgui::FontSource::TtfData {
        data: include_bytes!("../fonts/static/Orbitron-Regular.ttf"),
        size_pixels: font_size,
        config: Some(imgui::FontConfig {
            name: Some(String::from("Orbitron")),
            ..imgui::FontConfig::default()
        }),
    }]);
    let header_font = imgui.fonts().add_font(&[imgui::FontSource::TtfData {
        data: include_bytes!("../fonts/static/Orbitron-Regular.ttf"),
        size_pixels: font_size * 4.0,
        config: Some(imgui::FontConfig {
            name: Some(String::from("Orbitron Header")),
            ..imgui::FontConfig::default()
        }),
    }]);
    imgui.fonts().add_font(&[imgui::FontSource::TtfData {
        data: include_bytes!("../fonts/static/SpaceGrotesk-Regular.ttf"),
        size_pixels: font_size,
        config: Some(imgui::FontConfig {
            name: Some(String::from("Space Grotesk")),
            ..imgui::FontConfig::default()
        }),
    }]);
    imgui.fonts().add_font(&[imgui::FontSource::TtfData {
        data: include_bytes!("../fonts/Silkscreen-Regular.ttf"),
        size_pixels: font_size,
        config: Some(imgui::FontConfig {
            name: Some(String::from("Silkscreen")),
            ..imgui::FontConfig::default()
        }),
    }]);
    header_font
}

#[derive(Default)]
enum BackgroundValue<T> {
    Handle(std::thread::JoinHandle<T>),
    Value(T),
    #[default]
    Unset,
}

impl<T> BackgroundValue<T> {
    fn is_finished(&self) -> bool {
        match self {
            BackgroundValue::Handle(h) => h.is_finished(),
            BackgroundValue::Value(_) => true,
            BackgroundValue::Unset => unreachable!(),
        }
    }

    fn maybe_value(&mut self) {
        if let BackgroundValue::Handle(h) = self {
            if h.is_finished() {
                let h = std::mem::replace(self, BackgroundValue::Unset);
                let val = if let BackgroundValue::Handle(h) = h {
                    h.join().unwrap()
                } else {
                    unreachable!();
                };
                let _ = std::mem::replace(self, BackgroundValue::Value(val));
            }
        }
    }

    fn try_get(&mut self) -> Option<&T> {
        self.maybe_value();
        if let BackgroundValue::Value(v) = self {
            Some(v)
        } else {
            None
        }
    }

    fn try_take(&mut self) -> Option<T> {
        self.maybe_value();
        if let BackgroundValue::Value(_) = self {
            let v = std::mem::replace(self, BackgroundValue::Unset);
            Some(v.into_inner())
        } else {
            None
        }
    }

    fn into_inner(self) -> T {
        match self {
            BackgroundValue::Handle(h) => h.join().unwrap(),
            BackgroundValue::Value(v) => v,
            BackgroundValue::Unset => unreachable!(),
        }
    }
}

fn load_game_binaries(target_dir: &Path) -> BackgroundValue<GameHooks> {
    let target_dir = target_dir.to_path_buf();
    BackgroundValue::Handle(std::thread::spawn(move || {
        GameHooks::new(
            [
                GameVersion::SplinterCellBlacklistDx9,
                GameVersion::SplinterCellBlacklistDx11,
            ]
            .into_iter()
            .map(|gv| (gv, GameHook::new(gv, target_dir.clone())))
            .collect(),
        )
    }))
}

fn draw_loading_screen(
    ui: &imgui::Ui,
    exe_loader: &mut BackgroundValue<GameHooks>,
) -> Option<GameHooks> {
    ui.modal_popup_config(ID_MODAL_LOADING)
        .movable(false)
        .resizable(false)
        .build(|| ui.text("Searching for executables..."));
    ui.open_popup(ID_MODAL_LOADING);
    exe_loader.try_take()
}

fn draw_login(ui: &imgui::Ui, cfg: &mut hooks_config::Config) -> Option<Option<String>> {
    static TEST_ACCOUNTS: [&str; 3] = ["---", "sam_the_fisher", "AAAABBBB"];
    static TEST_ACCOUNT_PWDS: [&str; 3] = ["", "password1234", "CCCCDDDD"];
    let mut current_item = TEST_ACCOUNTS
        .iter()
        .zip(TEST_ACCOUNT_PWDS.iter())
        .enumerate()
        .find(|(_, (&acc, &pwd))| {
            acc == cfg.user.username.as_str() && pwd == cfg.user.password.as_str()
        })
        .map(|(i, _)| i)
        .unwrap_or_default();
    if ui.combo_simple_string("Test Account", &mut current_item, &TEST_ACCOUNTS) && current_item > 0
    {
        cfg.user.username = TEST_ACCOUNTS[current_item].into();
        cfg.user.password = TEST_ACCOUNT_PWDS[current_item].into();
    }
    ui.input_text("Username", &mut cfg.user.username).build();
    ui.input_text("Password", &mut cfg.user.password)
        .password(true)
        .build();

    static LOGIN_TEST: Mutex<Option<BackgroundValue<Option<String>>>> = Mutex::new(None);

    let logging_in = {
        if let Some(h) = &*LOGIN_TEST.lock().unwrap() {
            !h.is_finished()
        } else {
            false
        }
    };
    ui.disabled(
        cfg.user.username.is_empty() || cfg.user.password.is_empty() || logging_in,
        || {
            if ui.button("Test Login") {
                let api_url = cfg.api_server.to_string();
                let username = cfg.user.username.clone();
                let password = cfg.user.password.clone();
                LOGIN_TEST
                    .lock()
                    .unwrap()
                    .replace(BackgroundValue::Handle(std::thread::spawn(|| {
                        tokio::runtime::Builder::new_current_thread()
                            .enable_all()
                            .build()
                            .unwrap()
                            .block_on(
                                async move { test_login(api_url, &username, &password).await },
                            )
                    })));
            }
        },
    );

    if logging_in {
        ui.open_popup("logging_in")
    }
    ui.modal_popup_config("logging_in")
        .title_bar(false)
        .movable(false)
        .resizable(false)
        .build(|| {
            static mut TOTAL_TIME_MS: u64 = 0;
            // safety: data races are not our concern here
            unsafe { TOTAL_TIME_MS += (ui.io().delta_time * 1000.) as u64 };
            match unsafe { TOTAL_TIME_MS } / 500 % 3 {
                0 => ui.text(format!("Attempting to login as {}...", cfg.user.username)),
                1 => ui.text(format!("Attempting to login as {}.  ", cfg.user.username)),
                2 => ui.text(format!("Attempting to login as {}.. ", cfg.user.username)),
                _ => unreachable!(),
            }
            if !logging_in {
                ui.close_current_popup();
            }
        });

    if !logging_in {
        #[allow(clippy::map_clone)]
        let login_error: Option<Option<String>> = {
            LOGIN_TEST
                .lock()
                .unwrap()
                .as_mut()
                .map(|r| r.try_get().and_then(Option::from).map(String::clone))
        };
        login_error
    } else {
        None
    }
}

fn draw_register(ui: &imgui::Ui, cfg: &mut hooks_config::Config) {
    static REGISTER: Mutex<Option<BackgroundValue<Option<String>>>> = Mutex::new(None);

    ui.same_line();

    ui.enabled(
        cfg.user.username.is_empty() || cfg.user.password.is_empty(),
        || {
            if ui.button("Register") {
                ui.open_popup(ID_MODAL_REGISTER);
            }
        },
    );

    ui.modal_popup_config(ID_MODAL_REGISTER)
        .movable(false)
        .always_auto_resize(true)
        .resizable(false)
        .build(|| {
            static mut UBI_ID: String = String::new();

            ui.input_text("Username", &mut cfg.user.username).build();
            ui.input_text("Password", &mut cfg.user.password)
                .password(true)
                .build();
            #[allow(static_mut_refs)]
            ui.input_text("Ubisoft ID", unsafe { &mut UBI_ID }).build();

            let is_finished = {
                if let Some(h) = &*REGISTER.lock().unwrap() {
                    h.is_finished()
                } else {
                    false
                }
            };

            let register_error: Option<String> = {
                #[allow(clippy::map_clone)]
                REGISTER
                    .lock()
                    .unwrap()
                    .as_mut()
                    .and_then(|r| r.try_get().and_then(Option::from))
                    .map(String::clone)
            };

            if let Some(err) = register_error {
                ui.text_colored([1.0f32, 0f32, 0f32, 1f32], err);
            } else if is_finished {
                ui.text_colored([0f32, 1f32, 0f32, 1f32], "Registration successful!");
                if ui.button("Close") {
                    ui.close_current_popup();
                }
                return;
            } else {
                ui.new_line();
            }

            if ui.button("Register") {
                let api_url = cfg.api_server.to_string();
                let username = cfg.user.username.clone();
                let password = cfg.user.password.clone();
                REGISTER
                    .lock()
                    .unwrap()
                    .replace(BackgroundValue::Handle(std::thread::spawn(|| {
                        tokio::runtime::Builder::new_current_thread()
                            .enable_all()
                            .build()
                            .unwrap()
                            .block_on(async move {
                                #[allow(static_mut_refs)]
                                register(api_url, &username, &password, unsafe { &UBI_ID }).await
                            })
                    })));
            }
            ui.same_line();
            if ui.button("Cancel") {
                cfg.user.username = String::new();
                cfg.user.password = String::new();
                ui.close_current_popup();
            }
        });
}

fn draw_main_settings(ui: &imgui::Ui, cfg: &mut hooks_config::Config) {
    let login_result = draw_login(ui, cfg);
    draw_register(ui, cfg);

    if let Some(maybe_err) = login_result {
        ui.same_line();
        if let Some(err) = maybe_err {
            ui.text_colored([1.0f32, 0f32, 0f32, 1f32], err);
        } else {
            ui.text_colored([0f32, 1f32, 0f32, 1f32], "Successful");
        }
    }

    ui.checkbox("Join invites automatically", &mut cfg.auto_join_invite);
}

fn draw_networking_settings(
    ui: &imgui::Ui,
    cfg: &mut hooks_config::Config,
    api_server: &mut String,
    adapters: &[String],
    adapter_ips: &[IpAddr],
) {
    if ui.collapsing_header(
        "Networking",
        imgui::TreeNodeFlags::FRAME_PADDING | imgui::TreeNodeFlags::DEFAULT_OPEN,
    ) {
        ui.indent();
        let mut custom_server = cfg.config_server.is_some();
        ui.checkbox("Use custom config server", &mut custom_server);
        if custom_server && cfg.config_server.is_none() {
            cfg.config_server = Some(String::new());
        } else if !custom_server && cfg.config_server.is_some() {
            cfg.config_server = None;
        }
        if let Some(cs) = cfg.config_server.as_mut() {
            ui.input_text("Config Server", cs).build();
        }
        ui.input_text("API Server", api_server).build();
        match api_server.parse() {
            Ok(u) => cfg.api_server = u,
            Err(e) => {
                ui.text_colored([1.0f32, 0f32, 0f32, 1f32], format!("Error: {e}"));
            }
        };
        ui.disabled(cfg.networking.ip_address.is_some(), || {
            let mut enforce_adapter = cfg.networking.adapter.is_some();
            ui.checkbox("Enforce network adapter", &mut enforce_adapter);
            if enforce_adapter && cfg.networking.adapter.is_none() {
                cfg.networking.adapter = Some(String::new());
            } else if !enforce_adapter && cfg.networking.adapter.is_some() {
                cfg.networking.adapter = None;
            }
            if let Some(adapter) = cfg.networking.adapter.as_mut() {
                let mut current_item = adapters.binary_search(adapter).unwrap_or_default();
                ui.combo_simple_string("Adapter", &mut current_item, adapters);
                *adapter = adapters[current_item].clone();
                ui.text_colored(
                    imgui::color::ImColor32::from_rgba(0, 255, 255, 255).to_rgba_f32s(),
                    format!("Current IP: {}", adapter_ips[current_item]),
                );
            }
        });
        let mut enforce_ip = cfg.networking.ip_address.is_some();
        ui.checkbox("Enforce IP address", &mut enforce_ip);
        if enforce_ip && cfg.networking.ip_address.is_none() {
            cfg.networking.ip_address = Some(Ipv4Addr::new(127, 0, 0, 1));
        } else if !enforce_ip && cfg.networking.ip_address.is_some() {
            cfg.networking.ip_address = None;
        }
        if let Some(ip) = cfg.networking.ip_address.as_mut() {
            let mut ip_str = ip.to_string();
            ui.input_text("IP", &mut ip_str).build();
            match ip_str.parse() {
                Ok(i) => *ip = i,
                Err(e) => {
                    ui.text_colored([1.0f32, 0f32, 0f32, 1f32], format!("Error: {e}"));
                }
            }
        }
        ui.unindent();
    }

    if (cfg.networking.adapter.is_some() || cfg.networking.ip_address.is_some())
        && !cfg.enable_all_hooks
    {
        cfg.enable_hooks.insert(hooks_config::Hook::GetAdaptersInfo);
        cfg.enable_hooks.insert(hooks_config::Hook::Gethostbyname);
    }
}

fn draw_debug_settings(ui: &imgui::Ui, cfg: &mut hooks_config::Config, addr: &Addresses) {
    if ui.collapsing_header("Debugging", imgui::TreeNodeFlags::FRAME_PADDING) {
        ui.indent();
        static LOG_LEVELS: [hooks_config::LogLevel; 5] = [
            hooks_config::LogLevel::Trace,
            hooks_config::LogLevel::Debug,
            hooks_config::LogLevel::Info,
            hooks_config::LogLevel::Warning,
            hooks_config::LogLevel::Error,
        ];
        let mut current_item = LOG_LEVELS.binary_search(&cfg.logging.level).unwrap();
        ui.combo_simple_string("Log Level", &mut current_item, &LOG_LEVELS);
        cfg.logging.level = LOG_LEVELS[current_item];
        ui.checkbox("Enable Overlay", &mut cfg.enable_overlay);
        ui.checkbox("Forward all calls to UPlay", &mut cfg.forward_all_calls);
        ui.input_text("Unreal Engine command line", &mut cfg.internal_command_line)
            .build();
        ui.checkbox("Enable All Hooks", &mut cfg.enable_all_hooks);
        if !cfg.enable_all_hooks
            && ui.collapsing_header("Individual Hooks", imgui::TreeNodeFlags::FRAME_PADDING)
        {
            ui.indent();
            for (variant, label) in hooks_config::Hook::VARIANTS
                .iter()
                .zip(hooks_config::Hook::LABELS.iter())
            {
                if addr.hook_addr(*variant).is_none() {
                    continue;
                }
                let found = cfg.enable_hooks.contains(variant);
                let mut enabled = found;
                ui.checkbox(*label, &mut enabled);
                match (enabled, found) {
                    (true, true) => {}
                    (true, false) => {
                        cfg.enable_hooks.insert(*variant);
                    }
                    (false, true) => {
                        cfg.enable_hooks.remove(variant);
                    }
                    (false, false) => {}
                }
            }
            ui.unindent();
        }
        ui.unindent();
    }
}

fn draw_title(ui: &imgui::Ui, header_font: imgui::FontId, logo_texture: imgui::TextureId) {
    if false {
        let _font = ui.push_font(header_font);
        let header = "5th Echelon";
        let text_size = ui.calc_text_size(header);
        let win_size = ui.window_size();
        let mut cur_pos: [f32; 2] = ui.cursor_pos();
        cur_pos[0] = (win_size[0] - text_size[0]) / 2.0;
        ui.set_cursor_pos(cur_pos);
        ui.text(header);
    } else {
        let logo_width = 200.0;
        let ratio = logo_width / render::LOGO_WIDTH as f32;
        let win_size = ui.window_size();
        let mut cur_pos: [f32; 2] = ui.cursor_pos();
        cur_pos[0] = (win_size[0] - render::LOGO_WIDTH as f32) / 2.0;
        ui.set_cursor_pos(cur_pos);
        imgui::Image::new(
            logo_texture,
            [logo_width, render::LOGO_HEIGTH as f32 * ratio],
        )
        .build(ui);
    }
}

#[cfg(feature = "embed-dll")]
fn drop_dll(dir: &std::path::Path) {
    use std::fs::File;
    use std::io::Cursor;

    let dll_path = dir.join("uplay_r1_loader.dll");
    if !dll_path.exists() {
        panic!("uplay_r1_loader.dll not found. Make sure the launcher is placed in the right directory");
    }
    let backup_path = dir.join("uplay_r1_loader.orig.dll");
    if !backup_path.exists() && fs::copy(&dll_path, backup_path).is_err() {
        panic!("Backup failed");
    }

    let decompress = || {
        let mut dll_file = File::create(dll_path).ok()?;
        brotli::BrotliDecompress(&mut Cursor::new(COMPRESSED_DLL), &mut dll_file).ok()
    };

    if (decompress)().is_none() {
        panic!("Update failed");
    }
}

async fn test_login(api_url: String, username: &str, password: &str) -> Option<String> {
    let Ok(mut client) = UsersClient::connect(api_url).await else {
        return Some(String::from("Connection failed"));
    };

    let resp = match client
        .login(LoginRequest {
            username: username.to_string(),
            password: password.to_string(),
        })
        .await
    {
        Ok(resp) => resp,
        Err(status) => {
            if matches!(status.code(), tonic::Code::Unauthenticated) {
                return Some(String::from("Login failed"));
            } else {
                return Some(String::from("Error when sending request"));
            }
        }
    };

    let resp = resp.into_inner();
    if resp.error.is_empty() {
        None
    } else {
        Some(resp.error)
    }
}

async fn register(api_url: String, username: &str, password: &str, ubi_id: &str) -> Option<String> {
    let Ok(mut client) = UsersClient::connect(api_url).await else {
        return Some(String::from("Connection failed"));
    };

    let resp = match client
        .register(RegisterRequest {
            username: username.to_string(),
            password: password.to_string(),
            ubi_id: ubi_id.to_string(),
        })
        .await
    {
        Ok(resp) => resp,
        Err(status) => {
            if matches!(status.code(), tonic::Code::AlreadyExists) {
                return Some(String::from(
                    "Username already taken or Ubisoft ID already registered",
                ));
            } else {
                return Some(String::from("Error when sending request"));
            }
        }
    };

    let resp = resp.into_inner();
    if resp.error.is_empty() {
        None
    } else {
        Some(resp.error)
    }
}

fn get_dll_version(dll_path: impl AsRef<Path>) -> (usize, usize, usize) {
    let Ok(data) = fs::read(dll_path) else {
        return (0, 0, 0);
    };
    let Ok(dll) = dll_utils::parse(&data) else {
        return (0, 0, 0);
    };
    dll.version
}
