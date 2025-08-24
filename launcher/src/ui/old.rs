//! Implements the old, single-window launcher UI.
//!
//! This module contains all the logic for rendering the old user interface,
//! including settings for login, networking, and debugging, as well as game
//! launching functionality.

use std::collections::HashMap;
use std::fs;
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::path::PathBuf;
use std::sync::Mutex;

use hooks_addresses::Addresses;
use imgui_winit_support::winit::dpi::LogicalSize;

use super::load_game_binaries;
use super::new::themes::old as sc_style;
use super::Fonts;
use super::GameHook;
use super::GameHookState;
use super::GameHooks;
use crate::config::Config;
use crate::config::UIVersion;
use crate::dll_utils::get_dll_version;
use crate::games::GameState;
use crate::games::GameVersion;
use crate::network::register;
use crate::network::test_login;
use crate::render;
use crate::ui::BackgroundValue;
use crate::version::Version;

// Constants for modal popup IDs.
const ID_MODAL_ASK_SEARCH: &str = "Unknown Executables";
const ID_MODAL_SEARCHING: &str = "Identifying...";
const ID_MODAL_LOADING: &str = "Loading...";
const ID_MODAL_REGISTER: &str = "Register";

/// Draws a loading screen while the game executables are being located.
fn draw_loading_screen(ui: &imgui::Ui, exe_loader: &mut BackgroundValue<GameHooks>) -> Option<GameHooks> {
    ui.modal_popup_config(ID_MODAL_LOADING)
        .movable(false)
        .resizable(false)
        .build(|| ui.text("Searching for executables..."));
    ui.open_popup(ID_MODAL_LOADING);
    exe_loader.try_take()
}

/// Draws the login UI and handles the login process.
fn draw_login(ui: &imgui::Ui, cfg: &mut hooks_config::Config) -> Option<Option<String>> {
    static TEST_ACCOUNTS: [&str; 3] = ["---", "sam_the_fisher", "AAAABBBB"];
    static TEST_ACCOUNT_PWDS: [&str; 3] = ["", "password1234", "CCCCDDDD"];
    let mut current_item = TEST_ACCOUNTS
        .iter()
        .zip(TEST_ACCOUNT_PWDS.iter())
        .enumerate()
        .find(|(_, (&acc, &pwd))| acc == cfg.user.username.as_str() && pwd == cfg.user.password.as_str())
        .map(|(i, _)| i)
        .unwrap_or_default();
    if ui.combo_simple_string("Test Account", &mut current_item, &TEST_ACCOUNTS) && current_item > 0 {
        cfg.user.username = TEST_ACCOUNTS[current_item].into();
        cfg.user.password = TEST_ACCOUNT_PWDS[current_item].into();
    }
    ui.input_text("Username", &mut cfg.user.username).build();
    ui.input_text("Password", &mut cfg.user.password).password(true).build();

    static LOGIN_TEST: Mutex<Option<BackgroundValue<Option<String>>>> = Mutex::new(None);

    let logging_in = {
        if let Some(h) = &*LOGIN_TEST.lock().unwrap() {
            !h.is_finished()
        } else {
            false
        }
    };
    ui.disabled(cfg.user.username.is_empty() || cfg.user.password.is_empty() || logging_in, || {
        if ui.button("Test Login") {
            let api_url = cfg.api_server.to_string();
            let username = cfg.user.username.clone();
            let password = cfg.user.password.clone();
            LOGIN_TEST.lock().unwrap().replace(BackgroundValue::Handle(std::thread::spawn(|| {
                tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap()
                    .block_on(async move { test_login(api_url, &username, &password).await.as_ref().map_err(ToString::to_string).err() })
            })));
        }
    });

    // Show a loading popup while logging in.
    if logging_in {
        ui.open_popup("logging_in")
    }
    ui.modal_popup_config("logging_in").title_bar(false).movable(false).resizable(false).build(|| {
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
        let login_error: Option<Option<String>> = { LOGIN_TEST.lock().unwrap().as_mut().map(|r| r.try_get().and_then(Option::from).map(String::clone)) };
        login_error
    } else {
        None
    }
}

/// Draws the registration UI and handles the registration process.
fn draw_register(ui: &imgui::Ui, cfg: &mut hooks_config::Config) {
    static REGISTER: Mutex<Option<BackgroundValue<Option<String>>>> = Mutex::new(None);

    ui.same_line();

    ui.enabled(cfg.user.username.is_empty() || cfg.user.password.is_empty(), || {
        if ui.button("Register") {
            ui.open_popup(ID_MODAL_REGISTER);
        }
    });

    ui.modal_popup_config(ID_MODAL_REGISTER).movable(false).always_auto_resize(true).resizable(false).build(|| {
        static mut UBI_ID: String = String::new();

        ui.input_text("Username", &mut cfg.user.username).build();
        ui.input_text("Password", &mut cfg.user.password).password(true).build();
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
            REGISTER.lock().unwrap().as_mut().and_then(|r| r.try_get().and_then(Option::from)).map(String::clone)
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
            REGISTER.lock().unwrap().replace(BackgroundValue::Handle(std::thread::spawn(|| {
                tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async move {
                    #[allow(static_mut_refs)]
                    register(api_url, &username, &password, unsafe { &UBI_ID })
                        .await
                        .as_ref()
                        .map_err(ToString::to_string)
                        .err()
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

/// Draws the main settings section of the UI.
fn draw_main_settings(ui: &imgui::Ui, cfg: &mut Config) {
    let login_result = draw_login(ui, &mut cfg.hook_config);
    draw_register(ui, &mut cfg.hook_config);

    if let Some(maybe_err) = login_result {
        ui.same_line();
        if let Some(err) = maybe_err {
            ui.text_colored([1.0f32, 0f32, 0f32, 1f32], err);
        } else {
            ui.text_colored([0f32, 1f32, 0f32, 1f32], "Successful");
        }
    }

    ui.checkbox("Join invites automatically", &mut cfg.hook_config.auto_join_invite);
    let ui_versions = [None, Some(UIVersion::Old), Some(UIVersion::New)];
    let mut selected_ui_version = ui_versions.iter().position(|v| v == &cfg.ui_version).unwrap_or(0);

    if ui.combo("UI Version to use\n(requires launcher restart)", &mut selected_ui_version, &ui_versions, |uv| {
        std::borrow::Cow::Borrowed(match uv {
            Some(UIVersion::Old) => "Old",
            Some(UIVersion::New) => "New",
            None => "Choose on next start",
        })
    }) {
        cfg.ui_version = ui_versions[selected_ui_version];
    }
}

/// Draws the networking settings section of the UI.
fn draw_networking_settings(ui: &imgui::Ui, cfg: &mut hooks_config::Config, api_server: &mut String, adapters: &[String], adapter_ips: &[IpAddr]) {
    if ui.collapsing_header("Networking", imgui::TreeNodeFlags::FRAME_PADDING | imgui::TreeNodeFlags::DEFAULT_OPEN) {
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

    if (cfg.networking.adapter.is_some() || cfg.networking.ip_address.is_some()) && !cfg.enable_all_hooks {
        cfg.enable_hooks.insert(hooks_config::Hook::GetAdaptersInfo);
        cfg.enable_hooks.insert(hooks_config::Hook::Gethostbyname);
    }
}

/// Draws the debugging settings section of the UI.
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
        ui.input_text("Unreal Engine command line", &mut cfg.internal_command_line).build();
        ui.checkbox("Enable All Hooks", &mut cfg.enable_all_hooks);
        if !cfg.enable_all_hooks && ui.collapsing_header("Individual Hooks", imgui::TreeNodeFlags::FRAME_PADDING) {
            ui.indent();
            for (variant, label) in hooks_config::Hook::VARIANTS.iter().zip(hooks_config::Hook::LABELS.iter()) {
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

/// Draws the title and logo.
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
        imgui::Image::new(logo_texture, [logo_width, render::LOGO_HEIGTH as f32 * ratio]).build(ui);
    }
}

/// Draws the game executable selection dropdown.
fn get_selected_executable(ui: &imgui::Ui, games: &GameHooks, selected_gv: Option<GameVersion>) -> Option<(GameVersion, GameState)> {
    let mut versions = games
        .iter()
        .map(|g| match g.version {
            GameVersion::SplinterCellBlacklistDx11 => ((g.version, g.is_ready().into()), "DirectX 11"),
            GameVersion::SplinterCellBlacklistDx9 => ((g.version, g.is_ready().into()), "DirectX 9"),
        })
        .collect::<Vec<_>>();
    versions.sort_by_key(|(gv, _)| gv.0);

    let (versions, options): (Vec<(GameVersion, GameState)>, Vec<&str>) = versions.into_iter().unzip();
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

/// Runs the old launcher UI.
pub fn run(target_dir: PathBuf, cfg: Config, adapters: &[String], adapter_ips: &[IpAddr], update_available: bool) {
    let mut cfg = cfg;
    let mut saved_cfg = cfg.clone();
    let mut api_server = cfg.hook_config.api_server.to_string();

    // Create the imgui context.
    let mut imgui = imgui::Context::create();
    sc_style(imgui.style_mut());

    // Disable creation of imgui.ini and imgui_log.txt files.
    imgui.set_ini_filename(None);
    imgui.set_log_filename(None);

    // Set up the fonts for the UI.
    let header_font = Fonts::setup(&mut imgui).header;

    let mut selected_game: Option<(GameVersion, GameState)> = None;

    let mut launch_error = None;

    let mut addresses = None;

    // Load the game binaries in the background.
    let mut exe_loader = load_game_binaries(&target_dir);

    let dll_version = get_dll_version(target_dir.join("uplay_r1_loader.dll")).unwrap_or_default();
    let expected_dll_version: Version = option_env!("HOOKS_VERSION").and_then(|hv| hv.parse::<Version>().ok()).unwrap_or_default();

    let mut show_outdated_dll_warning = dll_version < expected_dll_version;
    let mut show_outdated_launcher_warning = update_available;

    // Run the main render loop.
    render::render(LogicalSize::new(1024, 768), imgui, |ui: &mut imgui::Ui, w: f32, h: f32, logo_texture: imgui::TextureId| {
        ui.window("Settings")
            .size([w, h], imgui::Condition::Always)
            .position([0f32, 0f32], imgui::Condition::Always)
            .movable(false)
            .resizable(false)
            .title_bar(false)
            .build(|| {
                // Show a loading screen until the game executables are found.
                if addresses.is_none() {
                    addresses = draw_loading_screen(ui, &mut exe_loader);
                    return;
                }

                let addresses = addresses.as_mut().unwrap();
                draw_title(ui, header_font, logo_texture);
                ui.separator();
                ui.set_window_font_scale(0.75);
                ui.text_disabled(format!("Install dir: {}", target_dir.as_os_str().to_string_lossy()));
                ui.text_disabled(format!("Launcher Version: {}", *crate::VERSION));
                ui.same_line();
                #[cfg(feature = "embed-dll")]
                {
                    ui.text_disabled(format!("Bundled DLL version: {}", env!("HOOKS_VERSION")));
                }
                ui.same_line();
                ui.text_disabled(format!("Installed DLL version: {}", dll_version));
                ui.set_window_font_scale(1.0);

                // Show modals for outdated DLL or launcher.
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

                ui.modal_popup("Outdated Launcher", || {
                    ui.text("Launcher is outdated.");
                    if ui.button("Update Now") {
                        let mut child = std::process::Command::new(std::env::current_exe().unwrap()).arg("update").spawn().unwrap();
                        child.try_wait().unwrap();
                        std::process::exit(0);
                    }
                    if ui.button("Discard") {
                        ui.close_current_popup();
                    }
                });
                if show_outdated_launcher_warning {
                    show_outdated_launcher_warning = false;
                    ui.open_popup("Outdated Launcher");
                }

                // Draw the main UI sections.
                draw_main_settings(ui, &mut cfg);
                draw_networking_settings(ui, &mut cfg.hook_config, &mut api_server, adapters, adapter_ips);
                if let Some(GameHook {
                    state: GameHookState::Resolved(ref addr),
                    ..
                }) = selected_game.and_then(|sg| addresses.get(sg.0))
                {
                    draw_debug_settings(ui, &mut cfg.hook_config, addr);
                }
                ui.separator();

                // Save/Reset buttons.
                ui.disabled(saved_cfg == cfg, || {
                    if ui.button("Save") {
                        if cfg.hook_config.networking.adapter.is_some() {
                            cfg.hook_config.networking.ip_address.take();
                        }
                        fs::write(hooks_config::get_config_path("."), toml::to_string_pretty(&cfg).unwrap()).unwrap();
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

                // Game selection and launch controls.
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

                // Modals for identifying unknown executables.
                let should_open_search_modal = ui.modal_popup_config(ID_MODAL_ASK_SEARCH).resizable(false).movable(false).build(|| {
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

                let search_modal_opened = ui
                    .modal_popup_config(ID_MODAL_SEARCHING)
                    .resizable(false)
                    .movable(false)
                    .always_auto_resize(true)
                    .build(|| {
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
                                let GameHookState::Resolved(a) = &g.state else {
                                    return None;
                                };
                                let Ok(hash) = hooks_addresses::hash_file(g.version.full_path(&g.target_dir)) else {
                                    return None;
                                };
                                Some(HashMap::from([(hash, *a.clone())]))
                            };
                            let dx9 = addresses.get(GameVersion::SplinterCellBlacklistDx9).and_then(gen_hash).unwrap_or_default();
                            let dx11 = addresses.get(GameVersion::SplinterCellBlacklistDx11).and_then(gen_hash).unwrap_or_default();
                            hooks_addresses::save_addresses(&target_dir, dx9, dx11);
                            ui.close_current_popup();
                        }
                    })
                    .is_some();

                if should_open_search_modal.is_none() && !search_modal_opened && addresses.has_only_unknown() {
                    ui.open_popup(ID_MODAL_ASK_SEARCH);
                } else if let Some(true) = should_open_search_modal {
                    ui.open_popup(ID_MODAL_SEARCHING);
                }
            });
    });
}
