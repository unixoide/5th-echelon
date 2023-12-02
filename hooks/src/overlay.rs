use std::sync::mpsc;
use std::sync::Mutex;
use std::time::Duration;
use std::time::Instant;

use hudhook::ImguiRenderLoop;
use server_api::misc::InviteEvent;
use tracing::info;
use windows::core::PCSTR;
use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState;
use windows::Win32::UI::Input::KeyboardAndMouse::VK_BACK;
use windows::Win32::UI::Input::KeyboardAndMouse::VK_DELETE;
use windows::Win32::UI::Input::KeyboardAndMouse::VK_DOWN;
use windows::Win32::UI::Input::KeyboardAndMouse::VK_END;
use windows::Win32::UI::Input::KeyboardAndMouse::VK_ESCAPE;
use windows::Win32::UI::Input::KeyboardAndMouse::VK_F5;
use windows::Win32::UI::Input::KeyboardAndMouse::VK_HOME;
use windows::Win32::UI::Input::KeyboardAndMouse::VK_INSERT;
use windows::Win32::UI::Input::KeyboardAndMouse::VK_LEFT;
use windows::Win32::UI::Input::KeyboardAndMouse::VK_NEXT;
use windows::Win32::UI::Input::KeyboardAndMouse::VK_PRIOR;
use windows::Win32::UI::Input::KeyboardAndMouse::VK_RETURN;
use windows::Win32::UI::Input::KeyboardAndMouse::VK_RIGHT;
use windows::Win32::UI::Input::KeyboardAndMouse::VK_SPACE;
use windows::Win32::UI::Input::KeyboardAndMouse::VK_TAB;
use windows::Win32::UI::Input::KeyboardAndMouse::VK_UP;
use windows::Win32::UI::WindowsAndMessaging::DefWindowProcA;

use crate::uplay_r1_loader::Event;
use crate::uplay_r1_loader::EVENTS;

static NOTIFICATION_TIMEOUT: Duration = Duration::from_secs(30);

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum Engine {
    DX9,
    DX11,
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
    new_invites: crossbeam_channel::Receiver<InviteEvent>,
    active_invites: Vec<Invite>,
}

impl MyRenderLoop {
    fn render_show(&mut self, ui: &imgui::Ui) {
        let win_size = ui.io().display_size;
        ui.window("Debug")
            .position([win_size[0] - 10.0, 10.0], imgui::Condition::FirstUseEver)
            .position_pivot([1.0, 0.0])
            .size([300.0, 110.0], imgui::Condition::FirstUseEver)
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
                    ui.group(|| {
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
                    });
                }
            });
        }

        let color = [0.0, 0.0, 0.0, 0.5];
        ui.get_background_draw_list()
            .add_rect([0.0, 0.0], win_size, color)
            .filled(true)
            .build();
    }

    #[allow(
        clippy::unused_self,
        unused_variables,
        clippy::needless_pass_by_ref_mut
    )]
    fn render_hide(&mut self, ui: &mut imgui::Ui) {}
}

impl hudhook::hooks::ImguiRenderLoop for MyRenderLoop {
    fn initialize(&mut self, ctx: &mut imgui::Context) {
        #![allow(clippy::cast_lossless)]

        ctx.io_mut().key_map[imgui::Key::Backspace as usize] = VK_BACK.0 as u32;
        ctx.io_mut().key_map[imgui::Key::Delete as usize] = VK_DELETE.0 as u32;
        ctx.io_mut().key_map[imgui::Key::LeftArrow as usize] = VK_LEFT.0 as u32;
        ctx.io_mut().key_map[imgui::Key::RightArrow as usize] = VK_RIGHT.0 as u32;
        ctx.io_mut().key_map[imgui::Key::UpArrow as usize] = VK_UP.0 as u32;
        ctx.io_mut().key_map[imgui::Key::DownArrow as usize] = VK_DOWN.0 as u32;
        ctx.io_mut().key_map[imgui::Key::Home as usize] = VK_HOME.0 as u32;
        ctx.io_mut().key_map[imgui::Key::End as usize] = VK_END.0 as u32;
        ctx.io_mut().key_map[imgui::Key::Tab as usize] = VK_TAB.0 as u32;
        ctx.io_mut().key_map[imgui::Key::PageUp as usize] = VK_PRIOR.0 as u32;
        ctx.io_mut().key_map[imgui::Key::PageDown as usize] = VK_NEXT.0 as u32;
        ctx.io_mut().key_map[imgui::Key::Insert as usize] = VK_INSERT.0 as u32;
        ctx.io_mut().key_map[imgui::Key::Space as usize] = VK_SPACE.0 as u32;
        ctx.io_mut().key_map[imgui::Key::Enter as usize] = VK_RETURN.0 as u32;
        ctx.io_mut().key_map[imgui::Key::Escape as usize] = VK_ESCAPE.0 as u32;
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
            if let Some(ref sender) = evt.sender {
                self.invite_notification
                    .replace((Instant::now(), sender.username.clone()));
            }
            self.active_invites.push(Invite {
                event: evt,
                clicked: false,
            });
        }

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

        let mut opened = true;
        ui.show_metrics_window(&mut opened);
    }

    fn should_block_messages(&self, _io: &imgui::Io) -> bool {
        matches!(self.ui_state, UiState::Show)
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

fn init_hudhook<T: hudhook::hooks::Hooks + 'static>(
    invites: crossbeam_channel::Receiver<InviteEvent>,
) -> anyhow::Result<()> {
    let (tx, rx) = mpsc::channel();
    EVENTS.get_or_init(|| Mutex::new(rx));
    hudhook::Hudhook::builder()
        .with(
            MyRenderLoop {
                tx,
                username: String::from("ABCD"),
                ui_state: UiState::default(),
                debounce: false,
                invite_notification: None,
                new_invites: invites,
                active_invites: Vec::new(),
            }
            .into_hook::<T>(),
        )
        .with_hmodule(unsafe { GetModuleHandleA(PCSTR::null())?.into() })
        .build()
        .apply()
        .map_err(|e| anyhow::anyhow!("Error adding gui hook: {e:?}"))
}

fn init_dx9(invites: crossbeam_channel::Receiver<InviteEvent>) -> anyhow::Result<()> {
    init_hudhook::<hudhook::hooks::dx9::ImguiDx9Hooks>(invites)
}

fn init_dx11(invites: crossbeam_channel::Receiver<InviteEvent>) -> anyhow::Result<()> {
    init_hudhook::<hudhook::hooks::dx11::ImguiDx11Hooks>(invites)
}

pub fn init(
    engine: Engine,
    invites: crossbeam_channel::Receiver<InviteEvent>,
) -> anyhow::Result<()> {
    match engine {
        Engine::DX9 => init_dx9(invites),
        Engine::DX11 => init_dx11(invites),
    }
}
