use std::cell::RefCell;
use std::ffi::OsStr;
use std::fs;
use std::io::Write as _;
use std::net::IpAddr;
use std::net::ToSocketAddrs;
use std::path::Path;
use std::path::PathBuf;
use std::rc::Rc;
use std::time::Duration;

use hooks_config::SaveGameExt;
use imgui::ImColor32;
use imgui::TableColumnFlags;
use imgui::TableColumnSetup;
use imgui::TableFlags;
use tracing::debug;
use tracing::error;
use tracing::info;

use super::colors::GREEN;
use super::colors::RED;
use super::colors::YELLOW;
use super::server::SERVER_PROCESS;
use crate::config::ConfigMut;
use crate::config::Profile;
use crate::config::P2P_DEFAULT_PORT;
use crate::config::QUAZAL_DEFAULT_LOCAL_PORT;
use crate::config::QUAZAL_DEFAULT_PORT;
use crate::config::RPC_DEFAULT_PORT;
use crate::games::GameVersion;
use crate::network;
use crate::network::try_locate_server;
use crate::registry;
use crate::ui::icons::*;
use crate::ui::AnimatedText;
use crate::ui::BackgroundValue;
use crate::ui::GameHooks;

type BackgroundNetworkTest = BackgroundValue<Option<network::Error>>;
type BackgroundNetwork<T> = BackgroundValue<Result<T, network::Error>>;

static mut CLIENT_PROCESS: Option<std::process::Child> = None;

const SC_BL_UBI_GAME_ID: &str = "449";

fn find_ubisoft_launcher_dir() -> Option<PathBuf> {
    registry::read_string(registry::Key::LocaLMachine, "SOFTWARE/Ubisoft/Launcher", "InstallDir")
        .as_deref()
        .and_then(OsStr::to_str)
        .map(PathBuf::from)
}

fn preexisting_savegame() -> Option<PathBuf> {
    let launcher_dir = find_ubisoft_launcher_dir()?;

    let savegames_dir = launcher_dir.join("savegames");
    if !savegames_dir.exists() {
        return None;
    }

    for entry in savegames_dir.read_dir().unwrap() {
        let Ok(entry) = entry else {
            continue;
        };

        let game_save_dir = entry.path().join(SC_BL_UBI_GAME_ID);
        if !game_save_dir.exists() {
            continue;
        }
        let save_file = game_save_dir.join("1.save");
        if save_file.exists() {
            return Some(save_file);
        }
    }

    None
}

pub struct ClientMenu<'a> {
    cfg: Rc<RefCell<ConfigMut>>,
    selected_profile: usize,
    launch_login_test: Option<BackgroundNetwork<String>>,
    test_config_server: Option<BackgroundNetworkTest>,
    test_rpc_login: Option<BackgroundNetworkTest>,
    test_quazal_login: Option<BackgroundNetworkTest>,
    test_p2p: Option<BackgroundNetworkTest>,
    hovered_row: usize,
    profile_to_delete: Option<usize>,
    profile_editor: ProfileEditor<'a>,
    system_dir: &'a Path,
    selected_game_version: Option<usize>,
    game_versions: Rc<RefCell<BackgroundValue<GameHooks>>>,
    has_savegame: bool,
    preexisting_savegame: Option<PathBuf>,
}

impl<'a> ClientMenu<'a> {
    pub fn new(
        cfg: Rc<RefCell<ConfigMut>>,
        adapters: &'a [(String, IpAddr)],
        system_dir: &'a Path,
        game_versions: Rc<RefCell<BackgroundValue<GameHooks>>>,
    ) -> Self {
        let selected_profile = cfg
            .borrow()
            .profiles
            .iter()
            .position(|p| p.name == cfg.borrow().default_profile)
            .unwrap_or_default();
        let profile_editor = ProfileEditor::new(Rc::clone(&cfg), adapters);
        let has_savegame = { cfg.borrow().hook_config.save.get_savegame_path(1).exists() };
        let preexisting_savegame = preexisting_savegame();
        Self {
            cfg,
            selected_profile,
            launch_login_test: Default::default(),
            hovered_row: Default::default(),
            test_config_server: Default::default(),
            test_rpc_login: Default::default(),
            test_quazal_login: Default::default(),
            test_p2p: Default::default(),
            profile_to_delete: Default::default(),
            profile_editor,
            system_dir,
            selected_game_version: None,
            game_versions,
            has_savegame,
            preexisting_savegame,
        }
    }

    pub fn render(&mut self, ui: &imgui::Ui) -> bool {
        if let Some(profile_name) = self.launch_login_successful() {
            let gv = self.cfg.borrow().default_game;
            launch_game(&profile_name, &mut self.cfg.borrow_mut(), gv, self.system_dir);
        }

        let no_profiles = self.cfg.borrow().profiles.is_empty();
        if !self.profile_editor.render(ui).unwrap_or(true) && no_profiles {
            return false;
        }
        if !no_profiles {
            self.diagnose_modal(ui);
        }

        self.import_savegame_modal(ui);
        if !self.has_savegame {
            ui.open_popup("Import Savegame");
            self.has_savegame = true;
        }

        if ui.arrow_button("Back", imgui::Direction::Left) {
            return false;
        }
        let is_testing = self.launch_login_is_testing();
        // SAFETY: static is only accessed in a single thread
        #[allow(static_mut_refs)]
        let is_launched = if let Some(child) = unsafe { CLIENT_PROCESS.as_mut() } {
            if child.try_wait().unwrap().is_some() {
                unsafe {
                    CLIENT_PROCESS = None;
                }
                true
            } else {
                false
            }
        } else {
            false
        };
        ui.disabled(is_testing || is_launched, || {
            if no_profiles && self.profile_editor.profile.is_none() {
                self.profile_editor.profile.get_or_insert_default();
                self.profile_editor.open(ui, true);
            }

            self.profiles_table(ui);

            {
                let font_height = ui.current_font_size();
                let cursor = ui.cursor_screen_pos();
                ui.invisible_button("arrow", [font_height, font_height]);
                let draw_list = ui.get_window_draw_list();
                let color = unsafe { ui.style() }.colors[imgui::StyleColor::Text as usize];
                draw_list
                    .add_line(
                        [cursor[0] + font_height / 4.0, cursor[1] + font_height / 3.0],
                        [cursor[0] + font_height / 2.0, cursor[1]],
                        color,
                    )
                    .thickness(2.0)
                    .build();
                draw_list
                    .add_line(
                        [cursor[0] + font_height / 2.0, cursor[1]],
                        [cursor[0] + font_height / 4.0 * 3.0, cursor[1] + font_height / 3.0],
                        color,
                    )
                    .thickness(2.0)
                    .build();
                draw_list
                    .add_line(
                        [cursor[0] + font_height / 2.0, cursor[1]],
                        [cursor[0] + font_height / 2.0, cursor[1] + font_height / 4.0 * 3.0],
                        color,
                    )
                    .thickness(2.0)
                    .build();
                draw_list
                    .add_line(
                        [cursor[0] + font_height / 2.0, cursor[1] + font_height / 4.0 * 3.0],
                        [cursor[0] + font_height / 4.0 * 3.0, cursor[1] + font_height / 4.0 * 3.0],
                        color,
                    )
                    .thickness(2.0)
                    .build();
            }
            ui.same_line();

            if ui.button(format!("{} Add New", ICON_CIRCLE_PLUS)) {
                self.profile_editor.profile.get_or_insert_default();
                self.profile_editor.open(ui, true);
            }
            ui.same_line();
            ui.disabled(no_profiles, || {
                if is_testing {
                    ui.button("Testing...");
                    return;
                }

                let gv = &mut *self.game_versions.borrow_mut();
                if let Some(hooks) = gv.try_get() {
                    let game_versions: Vec<GameVersion> = hooks.iter_ready().map(|hook| hook.version).collect();
                    if self.selected_game_version.is_none() {
                        let dg = self.cfg.borrow().default_game;
                        let selected_game_version = hooks
                            .iter_ready()
                            .position(|hook| hook.version == dg)
                            .unwrap_or_default();
                        self.selected_game_version = Some(selected_game_version);
                    }
                    if let Some(selected_game_version) = self.selected_game_version.as_mut() {
                        if *selected_game_version >= game_versions.len() {
                            *selected_game_version = game_versions.len() - 1;
                        }
                        if !game_versions.is_empty() {
                            ui.set_next_item_width(
                                ui.calc_text_size(game_versions.last().unwrap().label())[0]
                                    + unsafe { ui.style().frame_padding[0] * 2.0 }
                                    + ui.frame_height(),
                            );
                            if ui.combo("###Game Version", selected_game_version, &game_versions, |gv| {
                                std::borrow::Cow::Borrowed(gv.label())
                            }) {
                                self.cfg.borrow_mut().update(|cfg| {
                                    cfg.default_game = game_versions[*selected_game_version];
                                });
                            }
                        }
                    }
                }
                ui.same_line();
                if ui.button(format!("{} Launch", ICON_ROCKET)) {
                    let profile = &self.cfg.borrow().profiles[self.selected_profile];
                    launch_game_test_first(profile, &mut self.launch_login_test, self.cfg.borrow().default_game);
                }
                ui.same_line();
                if ui.button(format!("{} Diagnose", ICON_MAGNIFYING_GLASS)) {
                    let profile = &self.cfg.borrow().profiles[self.selected_profile];
                    let server = profile.server.clone();
                    self.test_config_server = Some(BackgroundValue::new_async(async move {
                        network::test_cfg_server(&server).await.err()
                    }));
                    
                    ui.open_popup("###Diagnose");
                }
            });

            if let Some(err) = self.launch_login_failed() {
                ui.text_colored(RED.to_rgba_f32s(), format!("{err}"));
            }
        });
        true
    }
}

impl ClientMenu<'_> {
    fn launch_login_is_testing(&self) -> bool {
        !self
            .launch_login_test
            .as_ref()
            .map(BackgroundValue::is_finished)
            .unwrap_or(true)
    }

    fn launch_login_successful(&mut self) -> Option<String> {
        if self
            .launch_login_test
            .as_mut()
            .and_then(BackgroundValue::try_get)
            .map(Result::is_ok)
            .unwrap_or(false)
        {
            self.launch_login_test
                .take()
                .and_then(|mut bv| bv.try_take())
                .and_then(Result::ok)
        } else {
            None
        }
    }

    fn launch_login_failed(&mut self) -> Option<&network::Error> {
        self.launch_login_test
            .as_mut()
            .and_then(BackgroundValue::try_get)
            .and_then(|r| r.as_ref().err())
    }

    fn delete_profile_modal(&mut self, ui: &imgui::Ui) {
        ui.modal_popup_config("Delete Profile##popup")
            .always_auto_resize(true)
            .build(|| {
                ui.text(format!(
                    "Delete profile {}?",
                    self.cfg.borrow().profiles[self.profile_to_delete.unwrap()].name,
                ));
                if ui.button("Yes") {
                    self.cfg.borrow_mut().update(|cfg| {
                        cfg.profiles.remove(self.profile_to_delete.take().unwrap());
                    });
                    if self.cfg.borrow().profiles.is_empty() {
                        self.selected_profile = 0;
                    } else if self.selected_profile >= self.cfg.borrow().profiles.len() {
                        self.selected_profile = self.cfg.borrow().profiles.len() - 1;
                    }

                    ui.close_current_popup();
                }
                ui.same_line();
                if ui.button("No") {
                    ui.close_current_popup();
                }
            });
    }

    fn profiles_table(&mut self, ui: &imgui::Ui) {
        if let Some(table) = ui.begin_table_header(
            "Profiles",
            [
                imgui::TableColumnSetup::new("Selected"),
                imgui::TableColumnSetup::new("Profile"),
                imgui::TableColumnSetup::new("Username"),
                imgui::TableColumnSetup::new("Server"),
                imgui::TableColumnSetup::new("Actions"),
            ],
        ) {
            self.delete_profile_modal(ui);
            // self.add_profile_modal(ui);
            self.profile_editor.render(ui);
            let mut new_default_profile = None;
            for (i, profile) in self.cfg.borrow().profiles.iter().enumerate() {
                ui.table_next_row();
                if self.hovered_row == i {
                    // ui.table_set_bg_color(
                    //     imgui::TableBgTarget::ROW_BG0,
                    //     unsafe { ui.style() }.colors[StyleColor::HeaderActive as usize],
                    // );
                }
                ui.table_next_column();
                let mut checked = self.selected_profile == i;
                ui.checkbox(format!("##selected_profile-{}", i), &mut checked);
                if checked {
                    if self.selected_profile != i {
                        new_default_profile = Some(profile.name.clone());
                    }
                    self.selected_profile = i;
                }
                if ui.is_item_hovered() {
                    self.hovered_row = i;
                }
                ui.table_next_column();
                ui.text(&profile.name);
                if ui.is_item_hovered() {
                    self.hovered_row = i;
                }
                ui.table_next_column();
                ui.text(&profile.user.username);
                if ui.is_item_hovered() {
                    self.hovered_row = i;
                }
                ui.table_next_column();
                if let Some(api_server) = profile.api_server_url.as_ref() {
                    ui.text(format!("{} | {}", &profile.server, api_server));
                } else {
                    ui.text(&profile.server);
                }
                if ui.is_item_hovered() {
                    self.hovered_row = i;
                }
                // buttons
                ui.table_next_column();
                if ui.button(format!("{}##launch-{}", ICON_ROCKET, i)) {
                    launch_game_test_first(profile, &mut self.launch_login_test, self.cfg.borrow().default_game);
                }
                if ui.is_item_hovered() {
                    ui.tooltip_text("Launch");
                    self.hovered_row = i;
                }
                ui.same_line();
                if ui.button(format!("{}##edit_profile-{}", ICON_PEN, i)) {
                    self.profile_editor.profile = Some(self.cfg.borrow().profiles[i].clone());
                    self.profile_editor.open(ui, false);
                }
                if ui.is_item_hovered() {
                    ui.tooltip_text("Edit Profile");
                    self.hovered_row = i;
                }
                ui.same_line();
                if ui.button(format!("{}##clone_profile-{}", ICON_CLONE, i)) {
                    self.profile_editor.profile = Some(self.cfg.borrow().profiles[i].clone());
                    self.profile_editor.open(ui, true);
                }
                if ui.is_item_hovered() {
                    ui.tooltip_text("Clone Profile");
                    self.hovered_row = i;
                }
                ui.same_line();
                if ui.button(format!("{}##delete_profile-{}", ICON_CIRCLE_XMARK, i)) {
                    self.profile_to_delete = Some(i);
                    ui.open_popup("Delete Profile##popup");
                }
                if ui.is_item_hovered() {
                    ui.tooltip_text("Delete Profile");
                    self.hovered_row = i;
                }
            }
            table.end();
            if let Some(new_default_profile) = new_default_profile {
                self.cfg
                    .borrow_mut()
                    .update(|cfg| cfg.default_profile = new_default_profile);
            }
        }
    }

    fn diagnose_modal(&mut self, ui: &imgui::Ui) {
        let show_result = |task: Option<&mut BackgroundNetworkTest>, help_options: &[&str]| {
            if task.is_none() {
                ui.text_disabled("Waiting...");
                return false;
            }
            if let Some(res) = task.and_then(BackgroundValue::try_get) {
                if let Some(err) = res {
                    ui.text_colored(RED.to_rgba_f32s(), format!("{err}"));
                    ui.table_next_row();
                    ui.table_next_column();
                    if !help_options.is_empty() {
                        ui.text_colored(ImColor32::from_rgb(66, 250, 250).to_rgba_f32s(), "Things to try:");
                        for help_option in help_options {
                            let var_markers = help_option.chars().filter(|c| *c == '`').count();
                            if var_markers > 0 && var_markers % 2 == 0 {
                                ui.bullet();
                                help_option.split_terminator('`').enumerate().for_each(|(i, s)| {
                                    ui.same_line();
                                    if i % 2 == 0 {
                                        ui.text_disabled(s);
                                    } else {
                                        ui.text_colored(ImColor32::from_rgb(66, 250, 250).to_rgba_f32s(), s);
                                    }
                                });
                            } else {
                                ui.bullet();
                                ui.text_disabled(help_option);
                            }
                        }
                    }
                } else {
                    ui.text_colored(GREEN.to_rgba_f32s(), "OK");
                }
                true
            } else {
                ui.text("Testing...");
                false
            }
        };

        unsafe {
            imgui::sys::igSetNextWindowSizeConstraints(
                imgui::sys::ImVec2 { x: 400.0, y: 150.0 },
                imgui::sys::ImVec2 { x: -1.0, y: 200.0 },
                None,
                std::ptr::null_mut(),
            );
        }
        ui.modal_popup_config(format!(
            "Diagnosing {}###Diagnose",
            self.cfg.borrow().profiles[self.selected_profile].name
        ))
        .always_auto_resize(true)
        .build(|| {
            if unsafe { SERVER_PROCESS.is_some() } {
                ui.text_colored(YELLOW.to_rgba_f32s(),"WARNING:");
                ui.same_line();
                ui.text("If you're also running the server on this machine,\nthe following tests are not very reliable. Especially P2P");
            }
            if let Some(table) = ui.begin_table_with_flags("DiagnoseTable", 2, TableFlags::NO_CLIP)
            {
                ui.table_setup_column_with(TableColumnSetup {
                    name: "",
                    flags: TableColumnFlags::NO_CLIP
                        | TableColumnFlags::NO_RESIZE
                        | TableColumnFlags::WIDTH_FIXED,
                    init_width_or_weight: ui.calc_text_size("Quazal Connection")[0],
                    ..Default::default()
                });

                let profile = &self.cfg.borrow().profiles[self.selected_profile];
                let api_server_url = profile.api_server_url().into_owned();
                let api_url = profile.api_server_url().into_owned();
                let api_port = api_url.port().unwrap_or(RPC_DEFAULT_PORT);
                let quazal_port= QUAZAL_DEFAULT_PORT;
                let server = profile.server.clone();
                let username = profile.user.username.clone();
                let password = profile.user.password.clone();

                ui.table_next_row();
                ui.table_next_column();
                ui.text("Config Server");
                ui.table_next_column();

                if show_result(
                    self.test_config_server.as_mut(),
                    &[
                        &format!("Make sure that TCP port `80` for the server at `{server}` is reachable"),
                        // "In the server config, `listen` in `service.onlineconfig` should end in `:80`",
                    ],) && self.test_rpc_login.is_none()
                {
                    let username = username.clone();
                    let password = password.clone();
                    self.test_rpc_login = Some(BackgroundValue::new_async(async move {
                        crate::network::test_login(api_server_url.to_string(), &username, &password)
                            .await
                            .err()
                    }));
                }
            
                ui.table_next_row();
                ui.table_next_column();
                ui.text("RPC Connection");
                ui.table_next_column();
                if show_result(
                    self.test_rpc_login.as_mut(),
                    &[
                        &format!("Make sure that the TCP port `{api_port}` for the server at `{server}` is reachable"),
                        // &format!("In the server config, `api_server` should end in `:{api_port}`"),
                        &format!("Verify that the password for the user `{username}` is correct"),
                    ],
                ) && self.test_quazal_login.is_none()
                {
                    let username = username.clone();
                    let password = password.clone();
                    let server = server.clone();
                    self.test_quazal_login = Some(BackgroundValue::new_async(async move {
                        network::test_quazal_login(&server, &username, &password)
                            .await
                            .err()
                    }));
                }
                
                ui.table_next_row();
                ui.table_next_column();
                ui.text("Quazal Connection");
                ui.table_next_column();
                if show_result(self.test_quazal_login.as_mut(), &[
                    &format!("Make sure that the UDP port `{quazal_port}` for the server at `{server}` is reachable"),
                    &format!("Make sure that you allow connections to UDP port `{QUAZAL_DEFAULT_LOCAL_PORT}` in your firewall"),
                    // &format!("In the server config, `listen` in `service.sc_bl_auth` should end in `:{QUAZAL_DEFAULT_LOCAL_PORT}`"),
                    &format!("Verify that the password for the user `{username}` is correct"),
                ]) && self.test_p2p.is_none()
                {
                    let password = password.clone();
                    self.test_p2p = Some(BackgroundValue::new_async(async move {
                        network::test_p2p(api_url.to_string(), &username, &password).await.err()
                    }));
                }
                ui.table_next_row();
                ui.table_next_column();
                ui.text("P2P Connection");
                ui.table_next_column();
                show_result(self.test_p2p.as_mut(), &[
                    &format!("Make sure that you allow connections to UDP port `{P2P_DEFAULT_PORT}` in your firewall"),
                    "`NOTE`: If you're currently in a game, the client can't perform this check.\nIn this case you can ignore the \"client did not respond in time\" error!",
                    ]);
                table.end();
            }

            if ui.button("Close") {
                self.test_config_server.take();
                self.test_rpc_login.take();
                self.test_quazal_login.take();
                self.test_p2p.take();
                ui.close_current_popup();
            }
        });
    }

    fn import_savegame_modal(&mut self, ui: &imgui::Ui) {
        ui.modal_popup_config("Import Savegame")
            .always_auto_resize(true)
            .build(|| {
                ui.text("Looks like you don't have a 5th Echelon savegame yet.");
                ui.text("Please choose one of the following options:");
                if self.preexisting_savegame.is_some() {
                    if ui.button("Import existing savegame") {
                        import_save_game(
                            &self.cfg.borrow().hook_config,
                            self.preexisting_savegame.as_deref().unwrap(),
                        );
                        ui.close_current_popup();
                    }
                    if ui.is_item_hovered() {
                        ui.tooltip_text("Imports your save game from the Ubisoft Launcher");
                    }
                    ui.same_line();
                }
                if ui.button("Generate new savegame") {
                    generate_save_game(&self.cfg.borrow().hook_config);
                    ui.close_current_popup();
                }
                if ui.is_item_hovered() {
                    ui.tooltip_text("Rank 5 + a little bit of cash");
                }
                ui.same_line();
                if ui.button("Don't do anything") {
                    ui.close_current_popup();
                }
                if ui.is_item_hovered() {
                    ui.tooltip_text("Do a fresh start");
                }
            });
    }
}

fn launch_game_test_first(
    profile: &Profile,
    launch_login_test: &mut Option<BackgroundNetwork<String>>,
    _game_version: GameVersion,
) {
    let server = profile.server.clone();
    let api_server_url = profile.api_server_url().into_owned();
    let username = profile.user.username.clone();
    let password = profile.user.password.clone();
    let profile_name = profile.name.clone();
    *launch_login_test = Some(BackgroundValue::new_async(async move {
        crate::network::test_cfg_server(&server).await?;
        crate::network::test_login(api_server_url.to_string(), &username, &password).await?;
        crate::network::test_quazal_login(&server, &username, &password).await?;

        Ok(profile_name)
    }));
}

fn launch_game(profile_name: &str, cfg: &mut ConfigMut, game_version: GameVersion, system_dir: &Path) {
    info!("Launching {} with {}", profile_name, game_version.label());
    let saved = cfg.update(|cfg| {
        let Some(profile) = cfg.profiles.iter().find(|p| p.name == profile_name) else {
            return;
        };
        cfg.hook_config.api_server = profile.api_server_url().into_owned();
        cfg.hook_config.config_server = Some(profile.server.clone());
        cfg.hook_config.user.username = profile.user.username.clone();
        cfg.hook_config.user.password = profile.user.password.clone();
        cfg.hook_config.user.account_id = profile.user.account_id.clone();
        cfg.hook_config.networking.adapter = profile.adapter.clone();
    });
    if saved.is_err() {
        return;
    }

    let child = std::process::Command::new(game_version.full_path(system_dir))
        .spawn()
        .unwrap();
    unsafe {
        CLIENT_PROCESS = Some(child);
    }
}

struct ProfileEditor<'a> {
    cfg: Rc<RefCell<ConfigMut>>,
    profile: Option<Profile>,
    adapters: &'a [(String, IpAddr)],
    try_to_register_shown: bool,
    add_profile_login_test: Option<BackgroundNetworkTest>,
    try_locate_server: Option<BackgroundNetwork<IpAddr>>,
    try_register: Option<BackgroundNetworkTest>,
    hostname_cache: std::collections::HashMap<String, bool>,
    search_text: AnimatedText<'a>,
    edit_existing: bool,
    show_password: bool,
    host_error: Option<String>,
}

impl<'a> ProfileEditor<'a> {
    fn new(cfg: Rc<RefCell<ConfigMut>>, adapters: &'a [(String, IpAddr)]) -> Self {
        Self {
            cfg,
            profile: None,
            try_to_register_shown: false,
            add_profile_login_test: Default::default(),
            adapters,
            try_locate_server: Default::default(),
            hostname_cache: Default::default(),
            try_register: Default::default(),
            search_text: AnimatedText::new(
                &["Searching...", "Searching.", "Searching.."],
                Duration::from_millis(200),
            ),
            edit_existing: false,
            show_password: false,
            host_error: None,
        }
    }

    fn update_config(&mut self, mut profile: Profile) {
        profile.user.account_id = profile.user.username.clone();
        self.cfg.borrow_mut().update(|cfg| {
            let old_profile = if self.edit_existing {
                cfg.profiles.iter_mut().find(|p| p.name == profile.name)
            } else {
                None
            };
            if let Some(old_profile) = old_profile {
                *old_profile = profile
            } else {
                cfg.profiles.push(profile);
            }
        });
    }

    fn render(&mut self, ui: &imgui::Ui) -> Option<bool> {
        // keep hostname cache small
        if self.hostname_cache.len() > 100 {
            let l = self.hostname_cache.len();
            self.hostname_cache.drain().take(l - 100).for_each(|_| {});
        }

        if self.edit_existing {
            ui.modal_popup_config("Edit Profile###Profile Editor")
        } else {
            ui.modal_popup_config("Add Profile###Profile Editor")
        }
        .always_auto_resize(true)
        .build(|| {
            self.locate_modal(ui);
            self.ask_register_modal(ui);
            let mut close_popup = false;
            if let Some(None) = self.add_profile_login_test.as_mut().and_then(BackgroundValue::try_get) {
                self.add_profile_login_test.take();

                if let Some(profile) = self.profile.take() {
                    self.update_config(profile);
                    close_popup = true;
                }
            }
            let is_testing = !self
                .add_profile_login_test
                .as_ref()
                .map(BackgroundValue::is_finished)
                .unwrap_or(true);
            if close_popup {
                ui.close_current_popup();
                return true;
            }
            let mut cancelled = false;
            ui.disabled(is_testing, || {
                let mut valid = true;
                if let Some(profile) = self.profile.as_mut() {
                    ui.disabled(self.edit_existing, || {
                        ui.input_text(format!("{} Profile Name", ICON_ADDRESS_CARD), &mut profile.name)
                            .build();
                        if ui.is_item_hovered() {
                            ui.tooltip_text("Provide a name for this profile");
                        }
                        if !self.edit_existing && self.cfg.borrow().profiles.iter().any(|p| p.name == profile.name) {
                            ui.text_colored(RED.to_rgba_f32s(), "Profile with this name already exists");
                            valid = false;
                        }
                    });
                    {
                        let mut adapters = self
                            .adapters
                            .iter()
                            .map(|(name, ip)| format!("{name} - {ip}"))
                            .collect::<Vec<String>>();
                        adapters.sort();
                        adapters.insert(0, String::from("Any adapter"));
                        let mut selected_adapter = profile
                            .adapter
                            .as_ref()
                            .and_then(|adapter| adapters.iter().position(|a| a == adapter))
                            .unwrap_or(0);
                        ui.combo_simple_string(format!("{} Adapter", ICON_PLUG), &mut selected_adapter, &adapters);
                        if ui.is_item_hovered() {
                            ui.tooltip_text(
                                "Pin to a specific adapter.\n\
                                Improves stability when attempting to join others",
                            );
                        }
                        if selected_adapter == 0 {
                            profile.adapter = None;
                        } else {
                            profile.adapter = Some(adapters[selected_adapter].to_string());
                        }
                    }
                    ui.disabled(!profile.server.is_empty(), || {
                        if ui.button(format!(
                            "{} Try to locate server automatically",
                            ICON_MAGNIFYING_GLASS_LOCATION
                        )) {
                            info!("Trying to locate server automatically");
                            let adapter = profile
                                .adapter
                                .as_ref()
                                .and_then(|adapter| self.adapters.iter().find(|(name, _ip)| name == adapter))
                                .map(|(name, ip)| (name.clone(), *ip));
                            let adapters = self.adapters.to_vec();

                            self.try_locate_server = Some(BackgroundValue::new_async(async move {
                                let adapter: Option<(&str, IpAddr)> =
                                    adapter.as_ref().map(|(name, ip)| (name.as_str(), *ip));
                                let adapters = adapters
                                    .iter()
                                    .map(|(name, ip)| (name.as_str(), *ip))
                                    .collect::<Vec<_>>();
                                try_locate_server(adapter, &adapters).await
                            }));
                            ui.open_popup("Locate Server");
                        }
                        if ui.is_item_hovered() {
                            ui.tooltip_text("Attempts to automatically find the server");
                        }
                    });
                    {
                        ui.input_text(format!("{} Server", ICON_SERVER), &mut profile.server)
                            .build();
                        if ui.is_item_hovered() {
                            ui.tooltip_text("IP/Hostname of the server to join");
                        }
                        /*
                        // prevent lookups while editing
                        let is_focused = ui.is_item_focused();
                        let valid_ip = profile.server.is_empty() || profile.server.parse::<IpAddr>().is_ok();
                        let valid_hostname = profile.server.is_empty()
                            || valid_ip
                            || is_focused
                            || *self.hostname_cache.entry(profile.server.clone()).or_insert_with(|| {
                                format!("{}:0", profile.server)
                                    .to_socket_addrs()
                                    .ok()
                                    .as_mut()
                                    .and_then(Iterator::next)
                                    .is_some()
                            });
                        if !valid_ip && !valid_hostname {
                            ui.text_colored(RED.to_rgba_f32s(), "Invalid server address");
                            valid = false;
                        }
                        */
                        if let Some(ref err) = self.host_error {
                            ui.text_colored(RED.to_rgba_f32s(), err);
                        }
                    }
                    {
                        ui.input_text(format!("{} Username", ICON_PERSON), &mut profile.user.username)
                            .build();
                        if ui.is_item_hovered() {
                            ui.tooltip_text("Your username");
                        }
                    }
                    {
                        ui.input_text(format!("{} Password", ICON_KEY), &mut profile.user.password)
                            .password(!self.show_password)
                            .build();
                        if ui.is_item_hovered() {
                            ui.tooltip_text("Your password");
                        }
                        ui.same_line();
                        if self.show_password {
                            ui.text(ICON_EYE.to_string());
                        } else {
                            ui.text(ICON_EYE_SLASH.to_string());
                        }
                        self.show_password = ui.is_item_hovered();
                    }
                    {
                        let mut different_api_server = profile.api_server_url.is_some();

                        ui.checkbox("Use Different API Server", &mut different_api_server);
                        if different_api_server {
                            let url = profile
                                .api_server_url
                                .get_or_insert(profile.api_server_url().into_owned());
                            let mut url_text = url.to_string();
                            ui.input_text("API Server URL", &mut url_text).build();
                            if let Ok(url) = url::Url::parse(&url_text) {
                                profile.api_server_url = Some(url);
                            } else {
                                ui.text_colored(RED.to_rgba_f32s(), "Invalid URL");
                                valid = false;
                            }
                        } else {
                            profile.api_server_url.take();
                        }
                    }
                    valid = valid
                        && !(profile.name.is_empty()
                            || profile.server.is_empty()
                            || profile.user.username.is_empty()
                            || profile.user.password.is_empty());
                } else {
                    valid = false
                };
                ui.enabled(valid, || {
                    if is_testing {
                        ui.button("Testing...");
                        return;
                    }
                    if let Some(is_login_failure) = self.is_add_profile_unknown_user() {
                        if is_login_failure && !self.try_to_register_shown {
                            self.add_profile_login_test.take();
                            ui.open_popup("Try to register?");
                        }
                        if let Some(Some(error)) =
                            self.add_profile_login_test.as_mut().and_then(BackgroundValue::try_get)
                        {
                            ui.text_colored(RED.to_rgba_f32s(), error.to_string());
                        }
                    }
                    if ui.button("Save") {
                        let profile = self.profile.as_ref().unwrap();
                        let valid_ip = profile.server.is_empty() || profile.server.parse::<IpAddr>().is_ok();
                        let valid_hostname = profile.server.is_empty()
                            || valid_ip
                            || *self.hostname_cache.entry(profile.server.clone()).or_insert_with(|| {
                                format!("{}:0", profile.server)
                                    .to_socket_addrs()
                                    .ok()
                                    .as_mut()
                                    .and_then(Iterator::next)
                                    .is_some()
                            });
                        if !valid_ip && !valid_hostname {
                            self.host_error = Some("Invalid server address".to_string());
                            return;
                        } else {
                            self.host_error = None;
                        }
                        let api_server_url = profile.api_server_url().into_owned();
                        let username = profile.user.username.clone();
                        let password = profile.user.password.clone();

                        self.add_profile_login_test = Some(BackgroundValue::new_async(async move {
                            crate::network::test_login(api_server_url.to_string(), &username, &password)
                                .await
                                .err()
                        }));
                    }
                });
                ui.same_line();
                if ui.button("Cancel") {
                    ui.close_current_popup();
                    self.profile.take();
                    self.add_profile_login_test.take();
                    cancelled = true;
                }
            });
            !cancelled
        })
    }

    fn is_add_profile_unknown_user(&mut self) -> Option<bool> {
        if let Some(Some(error)) = self.add_profile_login_test.as_mut().and_then(BackgroundValue::try_get) {
            if matches!(error, network::Error::UserNotFound) && !self.try_to_register_shown {
                return Some(true);
            }
            return Some(false);
        }
        None
    }

    fn open(&mut self, ui: &imgui::Ui, is_new: bool) {
        self.try_to_register_shown = false;
        self.edit_existing = !is_new;
        ui.open_popup("###Profile Editor");
    }

    fn ask_register_modal(&mut self, ui: &imgui::Ui) {
        ui.modal_popup_config("Try to register?")
            .always_auto_resize(true)
            .build(|| {
                self.try_to_register_shown = true;
                ui.text("User does not exist. Do you want to try to register?");
                ui.disabled(self.try_register.is_some(), || {
                    if ui.button("Yes") {
                        let api_server_url = self.profile.as_ref().unwrap().api_server_url().into_owned();
                        let username = self.profile.as_ref().unwrap().user.username.clone();
                        let password = self.profile.as_ref().unwrap().user.password.clone();
                        self.try_register = Some(BackgroundValue::new_async(async move {
                            network::register(api_server_url.to_string(), &username, &password, &username)
                                .await
                                .err()
                        }));
                    }
                    ui.same_line();
                    if ui.button("No") {
                        ui.close_current_popup();
                    }
                });
                if let Some(res) = self.try_register.as_mut().and_then(BackgroundValue::try_get) {
                    if let Some(err) = res {
                        ui.text_colored(RED.to_rgba_f32s(), format!("{err}"));
                    } else {
                        ui.text_colored(GREEN.to_rgba_f32s(), "Successfully registered");
                    }
                    if ui.button("Close") {
                        self.try_to_register_shown = false;
                        self.try_register.take();
                        ui.close_current_popup();
                    }
                }
            });
    }

    fn locate_modal(&mut self, ui: &imgui::Ui) {
        ui.modal_popup_config("Locate Server")
            .resizable(false)
            .always_auto_resize(
                self.try_locate_server
                    .as_mut()
                    .and_then(BackgroundValue::try_get)
                    .is_some(),
            )
            .build(|| {
                debug!("Popup opened");
                if let Some(res) = self.try_locate_server.as_mut().and_then(BackgroundValue::try_get) {
                    match res {
                        Err(err) => {
                            ui.text_colored(RED.to_rgba_f32s(), format!("{err}"));
                        }
                        Ok(ip) => {
                            ui.text("Found server at ");
                            ui.same_line();
                            ui.text_colored(GREEN.to_rgba_f32s(), format!("{ip}"));
                            ui.same_line();
                            ui.text(".");
                        }
                    }
                    if ui.button("Close") {
                        if let Some(mut res) = self.try_locate_server.take() {
                            if let Some(Ok(ip)) = res.try_take() {
                                if let Some(profile) = self.profile.as_mut() {
                                    profile.server = ip.to_string();
                                }
                            }
                        }
                        ui.close_current_popup();
                    }
                } else {
                    self.search_text.update();
                    ui.text(self.search_text.text());
                }
            });
    }
}

fn import_save_game(hooks_config: &hooks_config::Config, preexisting_save_game: &Path) {
    let Ok(content) = fs::read(preexisting_save_game) else {
        error!("Failed to read preexisting savegame");
        return;
    };
    let mut metadata_size = [0u8; 4];
    if content.len() < 4 {
        error!("Invalid preexisting savegame. Empty");
        return;
    }
    metadata_size.copy_from_slice(&content[..4]);
    let metadata_size = u32::from_le_bytes(metadata_size);
    if content.len() - 4 < metadata_size as usize {
        error!("Invalid preexisting savegame. Not enough data");
        return;
    }
    let savegame_data = &content[metadata_size as usize + 4..];

    if savegame_data.is_empty() {
        error!("Invalid preexisting savegame. Empty payload");
        return;
    }
    if savegame_data[0] != 1 {
        error!("Invalid preexisting savegame. Invalid data");
        return;
    }
    let sg_path = hooks_config.save.get_savegame_path(1);
    let sg_dir = sg_path.parent().unwrap();
    if !sg_dir.exists() {
        if let Err(e) = fs::create_dir_all(sg_dir) {
            error!("Failed to create savegame directory: {e}");
            return;
        }
    }
    if let Err(e) = fs::write(&sg_path, savegame_data) {
        error!("Failed to write savegame: {e}");
    }

    let _ = fs::write(sg_path.with_extension("meta"), b"sc6_save.sav");
}

fn generate_save_game(hooks_config: &hooks_config::Config) {
    let sg_path = hooks_config.save.get_savegame_path(1);
    let sg_dir = sg_path.parent().unwrap();
    if !sg_dir.exists() {
        if let Err(e) = fs::create_dir_all(sg_dir) {
            error!("Failed to create savegame directory: {e}");
            return;
        }
    }
    let Ok(mut f) = fs::File::create(&sg_path) else {
        error!("Failed to create savegame");
        return;
    };
    if let Err(e) = f.write_all(&[1, 0, 0, 0, 0, 0]) {
        error!("Failed to write savegame: {e}");
        return;
    }
    let save_game_data = include_bytes!("base_savegame.xml");
    let payload_length = save_game_data.len() as u32 + 4;
    if let Err(e) = f.write_all(&payload_length.to_le_bytes()) {
        error!("Failed to write savegame: {e}");
        return;
    }
    if let Err(e) = f.write_all(b"masW") {
        error!("Failed to write savegame: {e}");
    }
    if let Err(e) = f.write_all(save_game_data) {
        error!("Failed to write savegame: {e}");
    }
    if let Err(e) = f.flush() {
        error!("Failed to write savegame: {e}");
    }

    let _ = fs::write(sg_path.with_extension("meta"), b"sc6_save.sav");
}
