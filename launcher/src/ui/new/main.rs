use std::cell::RefCell;
use std::collections::HashMap;
use std::net::IpAddr;
use std::path::Path;
use std::rc::Rc;

use imgui::ImColor32;
use imgui::WindowFlags;

use super::client::ClientMenu;
use super::colors::BLUE;
use super::colors::GREEN;
use super::colors::GREY;
use super::colors::RED;
use super::colors::YELLOW;
use super::server::ServerMenu;
use super::settings::SettingsMenu;
use crate::config::ConfigMut;
use crate::dll_utils::get_dll_version;
use crate::games::GameVersion;
use crate::ui::icons::*;
use crate::ui::BackgroundValue;
use crate::ui::Fonts;
use crate::ui::GameHook;
use crate::ui::GameHookState;
use crate::ui::GameHooks;
use crate::ui::Size;
use crate::version::Version;

const ID_MODAL_SEARCHING: &str = "Identifying...";

#[derive(Default)]
enum MainMenuType<'a> {
    #[default]
    None,
    Client(ClientMenu<'a>),
    Server(Box<ServerMenu>),
    Settings(SettingsMenu),
}
pub struct Main<'a> {
    menu_type: MainMenuType<'a>,
    cfg: Rc<RefCell<ConfigMut>>,
    adapters: &'a [(String, IpAddr)],
    exe_loader: Rc<RefCell<BackgroundValue<GameHooks>>>,
    show_outdated_launcher_warning: bool,
    target_dir: &'a Path,
    fonts: Fonts,
    launcher_version: Version,
    bundled_dll_version: Option<Version>,
    installed_dll_version: Option<Version>,
    server_version: Option<Version>,
}

impl<'a> Main<'a> {
    pub fn new(
        cfg: ConfigMut,
        adapters: &'a [(String, IpAddr)],
        exe_loader: BackgroundValue<GameHooks>,
        update_available: bool,
        target_dir: &'a Path,
        fonts: Fonts,
    ) -> Self {
        let bundled_dll_version: Option<Version> = option_env!("HOOKS_VERSION")
            .map(core::str::FromStr::from_str)
            .and_then(Result::ok);
        let installed_dll_version = get_dll_version(target_dir.join("uplay_r1_loader.dll")).ok();
        let server_version = get_dll_version(
            std::env::current_exe()
                .unwrap()
                .parent()
                .unwrap()
                .join("dedicated_server.exe"),
        )
        .ok();
        let launcher_version = env!("CARGO_PKG_VERSION").parse().unwrap_or_default();
        Self {
            menu_type: MainMenuType::None,
            cfg: Rc::new(RefCell::new(cfg)),
            adapters,
            exe_loader: Rc::new(RefCell::new(exe_loader)),
            show_outdated_launcher_warning: update_available,
            target_dir,
            fonts,
            bundled_dll_version,
            installed_dll_version,
            server_version,
            launcher_version,
        }
    }

    pub fn render(&mut self, ui: &imgui::Ui, window_size: Size) {
        ui.window("Launcher")
            .flags(WindowFlags::NO_BRING_TO_FRONT_ON_FOCUS)
            .size(window_size, imgui::Condition::Always)
            .position([0f32, 0f32], imgui::Condition::Always)
            .movable(false)
            .resizable(false)
            .title_bar(false)
            .build(|| {
                let font = ui.push_font(self.fonts.header);
                let header_size = ui.calc_text_size("FIFTH ECHELON");
                font.end();
                ui.set_window_font_scale(0.7);
                let text_size = ui.calc_text_size("Launcher Version");
                ui.set_window_font_scale(1.0);
                ui.child_window("##Header")
                    .size([
                        0f32,
                        header_size[1] + text_size[1] * 2f32 + unsafe { ui.style() }.frame_padding[1] * 2f32 + 10f32,
                    ])
                    .build(|| {
                        let font = ui.push_font(self.fonts.header);
                        let center = ui.content_region_max()[0] / 2.0;
                        ui.same_line_with_pos(center - header_size[0] / 2.0);
                        ui.text("FIFTH ECHELON");
                        font.end();
                        ui.set_window_font_scale(0.7);
                        ui.text_disabled(format!("Launcher Version: {}", self.launcher_version));
                        if let Some(bundled_dll_version) = self.bundled_dll_version {
                            ui.same_line();
                            ui.text_disabled(format!("Bundled DLL Version: {bundled_dll_version}"));
                        }
                        if let Some(installed_dll_version) = self.installed_dll_version {
                            ui.same_line();
                            ui.text_disabled(format!("Installed DLL Version: {installed_dll_version}"));
                        }
                        if let Some(server_version) = self.server_version {
                            ui.same_line();
                            ui.text_disabled(format!("Server Version: {server_version}"));
                        }
                        ui.text_disabled(format!("Game Dir: {}", self.target_dir.display()));
                        ui.set_window_font_scale(1.0);
                    });
                ui.child_window("##Content")
                    .size([0f32, -ui.frame_height_with_spacing()])
                    .build(|| {
                        match self.menu_type {
                            MainMenuType::None => {
                                ui.modal_popup("Outdated Launcher", || {
                                    ui.text("Launcher is outdated.");
                                    if ui.button("Update Now") {
                                        let mut child = std::process::Command::new(std::env::current_exe().unwrap())
                                            .arg("update")
                                            .spawn()
                                            .unwrap();
                                        child.try_wait().unwrap();
                                        std::process::exit(0);
                                    }
                                    if ui.button("Discard") {
                                        ui.close_current_popup();
                                    }
                                });
                                if self.show_outdated_launcher_warning {
                                    self.show_outdated_launcher_warning = false;
                                    ui.open_popup("Outdated Launcher");
                                }

                                let join_server_text = format!("{} Join Server", ICON_GAMEPAD);
                                let host_server_text = format!("{} Server Management", ICON_SERVER);
                                let text_size = [&join_server_text, &host_server_text]
                                    .iter()
                                    .map(|s| ui.calc_text_size(s)[0])
                                    .fold(0f32, |acc, cur| acc.max(cur))
                                    + 32f32;

                                let width = ui.content_region_max()[0];
                                let mut cursor = ui.cursor_pos();
                                cursor[0] =
                                    (width - (text_size * 2f32 + unsafe { ui.style() }.frame_padding[0] * 2f32)) / 2.0
                                        - unsafe { ui.style() }.frame_padding[0];
                                ui.set_cursor_pos(cursor);

                                if ui.button_with_size(&join_server_text, [text_size, 100f32]) {
                                    self.menu_type = MainMenuType::Client(ClientMenu::new(
                                        Rc::clone(&self.cfg),
                                        self.adapters,
                                        self.target_dir,
                                        Rc::clone(&self.exe_loader),
                                    ));
                                }
                                if ui.is_item_hovered() {
                                    ui.tooltip_text("Connect to an existing server and host or join games.");
                                }
                                ui.same_line();
                                if ui.button_with_size(&host_server_text, [text_size, 100f32]) {
                                    self.menu_type = MainMenuType::Server(Box::new(ServerMenu::new(self.adapters)));
                                }
                                if ui.is_item_hovered() {
                                    ui.tooltip_text("Run the server component for other players to use.");
                                }
                                let mut cursor = ui.cursor_pos();
                                cursor[0] =
                                    (width - (text_size * 2f32 + unsafe { ui.style() }.frame_padding[0] * 2f32)) / 2.0
                                        - unsafe { ui.style() }.frame_padding[0];
                                ui.set_cursor_pos(cursor);
                                if ui.button_with_size(
                                    format!("{} Settings", ICON_GEAR),
                                    [text_size * 2f32 + unsafe { ui.style() }.frame_padding[0] * 2f32, 0f32],
                                ) {
                                    self.menu_type = MainMenuType::Settings(SettingsMenu::new(Rc::clone(&self.cfg)));
                                }
                                // ui.text(format!("{:#?}", self.cfg));
                            }
                            MainMenuType::Client(ref mut client_menu) => {
                                if !client_menu.render(ui) {
                                    self.menu_type = MainMenuType::None
                                }
                            }
                            MainMenuType::Server(ref mut server_menu) => {
                                if !server_menu.render(ui) {
                                    self.menu_type = MainMenuType::None
                                }
                            }
                            MainMenuType::Settings(ref mut settings_menu) => {
                                if !settings_menu.render(ui) {
                                    self.menu_type = MainMenuType::None
                                }
                            }
                        };
                    });
                ui.child_window("##StatusBar")
                    .size([0f32, ui.frame_height()])
                    .build(|| {
                        self.search_modal(ui);
                        ui.set_window_font_scale(0.8);
                        if let Some(hooks) = self.exe_loader.borrow_mut().try_mut() {
                            ui.text("Game Versions: ");
                            for gv in [
                                GameVersion::SplinterCellBlacklistDx9,
                                GameVersion::SplinterCellBlacklistDx11,
                            ] {
                                ui.same_line();
                                let (color, tooltip) = if let Some(gh) = hooks.get(gv) {
                                    (game_hook_state_to_color(&gh.state), game_hook_state_to_text(&gh.state))
                                } else {
                                    (RED, "Not found")
                                };
                                ui.text_colored(color.to_rgba_f32s(), gv.label_short());
                                if ui.is_item_hovered() {
                                    ui.tooltip_text(tooltip);
                                }
                            }

                            if hooks
                                .games
                                .values()
                                .any(|gv| matches!(gv.state, GameHookState::UnsupportedBinary))
                            {
                                ui.same_line();
                                if ui.button("Attempt to identify") {
                                    hooks.start_searching();
                                    ui.open_popup(ID_MODAL_SEARCHING);
                                }
                            }
                        } else {
                            ui.text("Searching Game Binaries...");
                        }
                    })
            });
        // ui.show_default_style_editor();
    }

    fn search_modal(&mut self, ui: &imgui::Ui) -> bool {
        let mut el = self.exe_loader.borrow_mut();
        let Some(games) = el.try_mut() else {
            return false;
        };
        ui.modal_popup_config(ID_MODAL_SEARCHING)
            .resizable(false)
            .movable(false)
            .always_auto_resize(true)
            .build(|| {
                ui.text("Identifying...");
                for hook in games.iter() {
                    ui.text(format!("{}: ", hook.version.label()));
                    ui.same_line();
                    match &hook.state {
                        GameHookState::Resolved(_) => ui.text_colored(GREEN.to_rgba_f32s(), "OK"),
                        GameHookState::FileNotFound => ui.text_colored(RED.to_rgba_f32s(), "Not found"),
                        GameHookState::Searching => ui.text_colored(YELLOW.to_rgba_f32s(), "Testing..."),
                        GameHookState::Ignored => ui.text_colored(GREY.to_rgba_f32s(), "IGNORED"),
                        GameHookState::UnsupportedBinary => ui.text_colored(RED.to_rgba_f32s(), "UNSUPPORTED"),
                        GameHookState::Failed(f) => ui.text_colored(RED.to_rgba_f32s(), format!("FAILURE: {f}")),
                    }
                }
                if games.search_status() && ui.button("Save Results") {
                    let gen_hash = |g: &GameHook| {
                        let GameHookState::Resolved(a) = &g.state else {
                            return None;
                        };
                        let Ok(hash) = hooks_addresses::hash_file(g.version.full_path(&g.target_dir)) else {
                            return None;
                        };
                        Some(HashMap::from([(hash, *a.clone())]))
                    };
                    let dx9 = games
                        .get(GameVersion::SplinterCellBlacklistDx9)
                        .and_then(gen_hash)
                        .unwrap_or_default();
                    let dx11 = games
                        .get(GameVersion::SplinterCellBlacklistDx11)
                        .and_then(gen_hash)
                        .unwrap_or_default();
                    hooks_addresses::save_addresses(self.target_dir, dx9, dx11);
                    ui.close_current_popup();
                }
            })
            .is_some()
    }
}

fn game_hook_state_to_color(state: &GameHookState) -> ImColor32 {
    match state {
        GameHookState::Resolved(_) => GREEN,
        GameHookState::Failed(_) | GameHookState::FileNotFound => RED,
        GameHookState::UnsupportedBinary => YELLOW,
        GameHookState::Ignored => GREY,
        GameHookState::Searching => BLUE,
    }
}

fn game_hook_state_to_text(state: &GameHookState) -> &'static str {
    match state {
        GameHookState::Resolved(_) => "Identified",
        GameHookState::Failed(_) | GameHookState::FileNotFound => "Error",
        GameHookState::UnsupportedBinary => "Unknown binary",
        GameHookState::Ignored => "Ignored binary",
        GameHookState::Searching => "Identifying binary...",
    }
}
