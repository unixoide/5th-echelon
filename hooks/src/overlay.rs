use std::sync::mpsc;
use std::sync::Mutex;
use std::time::Duration;
use std::time::Instant;

use hudhook::ImguiRenderLoop;
use imgui::Style;
use imgui::StyleColor;
use server_api::misc::InviteEvent;
use server_api::users::User;
use tracing::info;
use windows::core::PCSTR;
use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState;
use windows::Win32::UI::Input::KeyboardAndMouse::VK_F5;
use windows::Win32::UI::WindowsAndMessaging::DefWindowProcA;

use crate::uplay_r1_loader::Event;
use crate::uplay_r1_loader::EVENTS;

static NOTIFICATION_TIMEOUT: Duration = Duration::from_secs(30);
static INITIAL_POPUP_DURATION: Duration = Duration::from_secs(10);

// TODO: move to separate crate
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

fn setup_fonts(imgui: &mut imgui::Context) {
    let font_size = 13.0;
    imgui.fonts().add_font(&[imgui::FontSource::TtfData {
        data: include_bytes!("../../launcher/fonts/static/Orbitron-Regular.ttf"),
        size_pixels: font_size,
        config: Some(imgui::FontConfig {
            name: Some(String::from("Orbitron")),
            ..imgui::FontConfig::default()
        }),
    }]);
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum Engine {
    DX9,
    DX11,
}

impl Engine {
    pub fn detect() -> Option<Self> {
        let exe = std::env::current_exe().ok()?;
        let fname = exe.file_name()?.to_str()?.to_lowercase();
        match fname.as_str() {
            "blacklist_game.exe" => Some(Self::DX9),
            "blacklist_dx11_game.exe" => Some(Self::DX11),
            _ => None,
        }
    }
}

#[derive(Debug, Default)]
enum UiState {
    Show,
    #[default]
    Hide,
}

struct Invite {
    event: InviteEvent,
    clicked: bool,
}

struct MyRenderLoop {
    tx: mpsc::Sender<Event>,
    username: String,
    ui_state: UiState,
    debounce: bool,
    invite_notification: Option<(Instant, String)>,
    new_invites: crossbeam_channel::Receiver<Result<Option<InviteEvent>, crate::api::Error>>,
    active_invites: Vec<Invite>,
    connection_error: Option<crate::api::Error>,
    initial_popup: Instant,
}

impl MyRenderLoop {
    fn render_show(&mut self, ui: &imgui::Ui) {
        self.show_debug(ui);
        self.show_advanced(ui);
    }

    #[allow(
        clippy::unused_self,
        unused_variables,
        clippy::needless_pass_by_ref_mut
    )]
    fn render_hide(&mut self, ui: &mut imgui::Ui) {}

    fn join_session(&self, sender: &User) {
        self.tx
            .send(Event::FriendsGameInviteAccepted(sender.id.clone()))
            .unwrap();
    }

    fn show_invites(&mut self, ui: &imgui::Ui) {
        if let Some((expires, user)) = self.invite_notification.as_ref() {
            if expires.elapsed() > NOTIFICATION_TIMEOUT {
                self.invite_notification.take();
            } else {
                let win_size = ui.io().display_size;
                ui.window("Invite")
                    .bg_alpha(0.45)
                    .no_decoration()
                    .no_inputs()
                    .no_nav()
                    .movable(false)
                    .menu_bar(false)
                    .always_auto_resize(true)
                    .position([win_size[0] - 10.0, 10.0], imgui::Condition::Always)
                    .position_pivot([1.0, 0.0])
                    .build(|| {
                        ui.text("Invitation from ");
                        ui.same_line();
                        ui.text_colored([1.0, 0.0, 0.0, 1.0], user);
                        let diff = NOTIFICATION_TIMEOUT - expires.elapsed();
                        ui.text(format!("{}s", diff.as_secs()));
                    });
            }
        }
    }

    fn show_initial_info(&self, ui: &imgui::Ui) {
        if let Some(dur) = self.initial_popup.checked_duration_since(Instant::now()) {
            let win_size = ui.io().display_size;
            ui.window("Overlay loaded.")
                .bg_alpha(0.45)
                .no_decoration()
                .no_inputs()
                .no_nav()
                .title_bar(true)
                .movable(false)
                .menu_bar(false)
                .always_auto_resize(true)
                .position([win_size[0] - 10.0, 10.0], imgui::Condition::Always)
                .position_pivot([1.0, 0.0])
                .build(|| {
                    ui.text("Press");
                    ui.same_line();
                    ui.text_colored([1.0, 0.0, 0.0, 1.0], "F5");
                    ui.same_line();
                    ui.text("to open it.");

                    // TODO: draw decreasing bar (doesn't work right now)
                    let ws = ui.window_size();
                    let draw_list = ui.get_window_draw_list();
                    draw_list
                        .add_line(
                            [0., ws[1]],
                            [
                                ws[0] * (dur.as_secs_f32() / INITIAL_POPUP_DURATION.as_secs_f32()),
                                ws[1],
                            ],
                            [1.0, 0.0, 0.0, 1.0],
                        )
                        .build();
                });
        }
    }

    fn show_errors(&mut self, ui: &imgui::Ui) {
        if let Some(err) = &self.connection_error {
            let win_size = ui.io().display_size;
            ui.window("Error")
                .bg_alpha(0.45)
                .no_decoration()
                .no_inputs()
                .no_nav()
                .movable(false)
                .menu_bar(false)
                .always_auto_resize(true)
                .position([win_size[0] - 10.0, 10.0], imgui::Condition::Always)
                .position_pivot([1.0, 0.0])
                .build(|| {
                    ui.text_colored([1.0, 0.0, 0.0, 1.0], "SERVER ERROR");
                    let msg = match err {
                        crate::api::Error::IO(e) => format!("{e}"),
                        crate::api::Error::MissingUrl => "API server not configured".into(),
                        crate::api::Error::Transport(e) => format!("{e}"),
                        crate::api::Error::GRPCStatus(e) => match e.code() {
                            tonic::Code::Ok => unreachable!(),
                            tonic::Code::DeadlineExceeded => "Connection lost".into(),
                            tonic::Code::Unauthenticated => {
                                "Unauthenticated. Relogin required".into()
                            }
                            _ => format!("{}", e.code()),
                        },
                        crate::api::Error::LoginFailure => "Login failed".into(),
                        crate::api::Error::InvalidToken(_) => "Relogin required".into(),
                        crate::api::Error::NotConnected => "Not connected".into(),
                    };
                    ui.text_colored([1.0, 0.0, 0.0, 1.0], msg);
                });
        }
    }

    #[allow(clippy::unused_self)]
    fn show_advanced(&mut self, ui: &imgui::Ui) {
        ui.window("Advanced")
            .always_auto_resize(true)
            .resizable(false)
            .build(|| {
                let mut has_fields = false;
                if let Some(mpv) = unsafe { get_min_players_var().as_mut() } {
                    ui.input_int("Min Number of Players", mpv).build();
                    has_fields = true;
                }
                if let Some(mpv) = unsafe { get_max_players_var().as_mut() } {
                    ui.input_int("Max Number of Players", mpv).build();
                    has_fields = true;
                }

                if !has_fields {
                    ui.text_colored([1.0, 1.0, 0.0, 0.0], "No advanced settings available!");
                }
            });
    }

    fn show_debug(&mut self, ui: &imgui::Ui) {
        let win_size = ui.io().display_size;
        ui.window("Debug")
            .position([win_size[0] - 10.0, 10.0], imgui::Condition::FirstUseEver)
            .position_pivot([1.0, 0.0])
            .always_auto_resize(true)
            .build(|| {
                ui.input_text("username", &mut self.username).build();
                if ui.button("Friend Accepted Invite") {
                    info!("Send friend invite accept for {}", self.username);
                    self.tx
                        .send(Event::FriendsGameInviteAccepted(self.username.clone()))
                        .unwrap();
                }
                ui.same_line();
                if ui.button("Party Accepted Invite") {
                    info!("Send party invite accept for {}", self.username);
                    self.tx
                        .send(Event::PartyGameInviteAccepted(self.username.clone()))
                        .unwrap();
                }
            });

        if !self.active_invites.is_empty() {
            ui.window("Invites").position_pivot([0.5, 0.5]).build(|| {
                for invite in &mut self.active_invites {
                    if let Some(ref sender) = invite.event.sender {
                        ui.text(sender.username.as_str());
                        ui.disabled(invite.clicked, || {
                            if ui.button("Accept") {
                                self.tx
                                    .send(Event::FriendsGameInviteAccepted(sender.id.clone()))
                                    .unwrap();
                                self.ui_state = UiState::Hide;
                                invite.clicked = true;
                            }
                        });
                    }
                }
            });
        }

        let color = [0.0, 0.0, 0.0, 0.5];
        ui.get_background_draw_list()
            .add_rect([0.0, 0.0], win_size, color)
            .filled(true)
            .build();
    }
}

fn get_game_settings() -> *mut i32 {
    if let Some(ncaddr) = unsafe { crate::hooks::NET_CORE_ADDR } {
        unsafe {
            // let g_netcore = std::ptr::read(0x32b_5dc4 as *mut *mut *mut i32);
            let g_netcore = ncaddr as *mut *mut i32;
            if g_netcore.is_null() {
                return std::ptr::null_mut();
            }
            let game_session = std::ptr::read(g_netcore.byte_add(0x5d0));
            if game_session.is_null() {
                return std::ptr::null_mut();
            }
            game_session.byte_add(0x594)
        }
    } else {
        std::ptr::null_mut()
    }
}

fn get_max_players_var() -> *mut i32 {
    unsafe {
        let game_settings = get_game_settings();
        if game_settings.is_null() {
            return std::ptr::null_mut();
        }

        game_settings.byte_add(0x20)
    }
}

fn get_min_players_var() -> *mut i32 {
    unsafe {
        let game_settings = get_game_settings();
        if game_settings.is_null() {
            return std::ptr::null_mut();
        }

        game_settings.byte_add(0x1c)
    }
}

impl ImguiRenderLoop for MyRenderLoop {
    fn initialize(
        &mut self,
        ctx: &mut imgui::Context,
        _render_context: &mut dyn hudhook::RenderContext,
    ) {
        sc_style(ctx.style_mut());
        setup_fonts(ctx);
        ctx.io_mut().font_global_scale = 2.0;
    }
    fn render(&mut self, ui: &mut imgui::Ui) {
        #[allow(clippy::cast_possible_wrap)]
        let f5 = unsafe { GetAsyncKeyState(VK_F5.0.into()) & 0x8000u16 as i16 != 0 };
        if f5 {
            if !self.debounce {
                self.debounce = true;
                self.ui_state = match self.ui_state {
                    UiState::Show => UiState::Hide,
                    UiState::Hide => UiState::Show,
                };
            }
        } else {
            self.debounce = false;
        }
        match self.ui_state {
            UiState::Show => self.render_show(ui),
            UiState::Hide => self.render_hide(ui),
        }

        if let Ok(evt) = self.new_invites.try_recv() {
            match evt {
                Err(e) => self.connection_error = Some(e),
                Ok(evt) => {
                    self.connection_error = None;
                    if let Some(evt) = evt {
                        if let Some(ref sender) = evt.sender {
                            self.invite_notification
                                .replace((Instant::now(), sender.username.clone()));
                        }
                        let force_join = evt.force_join;
                        if evt.sender.is_some()
                            && (force_join || hooks_config::get().unwrap().auto_join_invite)
                        {
                            self.join_session(&evt.sender.unwrap());
                        } else {
                            self.active_invites.push(Invite {
                                event: evt,
                                clicked: force_join,
                            });
                        }
                    }
                }
            }
        }

        self.show_errors(ui);
        self.show_invites(ui);
        self.show_initial_info(ui);
    }

    fn message_filter(&self, _io: &imgui::Io) -> hudhook::MessageFilter {
        if matches!(self.ui_state, UiState::Show) {
            hudhook::MessageFilter::InputAll
        } else {
            hudhook::MessageFilter::empty()
        }
    }

    fn on_wnd_proc(
        &self,
        hwnd: windows::Win32::Foundation::HWND,
        umsg: u32,
        wparam: windows::Win32::Foundation::WPARAM,
        lparam: windows::Win32::Foundation::LPARAM,
    ) {
        if matches!(self.ui_state, UiState::Show) {
            // Forward to default handler so that the window doesn't break
            unsafe {
                DefWindowProcA(hwnd, umsg, wparam, lparam);
            }
        }
    }
}

fn init_hudhook<T: hudhook::Hooks + 'static>(
    invites: crossbeam_channel::Receiver<Result<Option<InviteEvent>, crate::api::Error>>,
) -> anyhow::Result<()> {
    let (tx, rx) = mpsc::channel();
    EVENTS.get_or_init(|| Mutex::new(rx));
    hudhook::Hudhook::builder()
        .with::<T>(MyRenderLoop {
            tx,
            username: String::from("ABCD"),
            ui_state: UiState::default(),
            debounce: false,
            invite_notification: None,
            new_invites: invites,
            active_invites: Vec::new(),
            connection_error: None,
            initial_popup: Instant::now() + INITIAL_POPUP_DURATION,
        })
        .with_hmodule(unsafe { GetModuleHandleA(PCSTR::null())?.into() })
        .build()
        .apply()
        .map_err(|e| anyhow::anyhow!("Error adding gui hook: {e:?}"))
}

fn init_dx9(
    invites: crossbeam_channel::Receiver<Result<Option<InviteEvent>, crate::api::Error>>,
) -> anyhow::Result<()> {
    init_hudhook::<hudhook::hooks::dx9::ImguiDx9Hooks>(invites)
}

fn init_dx11(
    invites: crossbeam_channel::Receiver<Result<Option<InviteEvent>, crate::api::Error>>,
) -> anyhow::Result<()> {
    init_hudhook::<hudhook::hooks::dx11::ImguiDx11Hooks>(invites)
}

pub fn init(
    engine: Engine,
    invites: crossbeam_channel::Receiver<Result<Option<InviteEvent>, crate::api::Error>>,
) -> anyhow::Result<()> {
    match engine {
        Engine::DX9 => init_dx9(invites),
        Engine::DX11 => init_dx11(invites),
    }
}
