use std::net::IpAddr;
use std::path::PathBuf;

use imgui_winit_support::winit::dpi::LogicalSize;
use main::Main;
use server::SERVER_PROCESS;

use super::Fonts;
use super::Size;
use crate::config::ConfigMut;
use crate::version::Version;

mod client;
mod colors;
mod main;
mod server;
mod settings;
pub mod themes;

pub fn run(target_dir: PathBuf, cfg: ConfigMut, adapters: &[(String, IpAddr)], update_available: Option<Version>) {
    let mut imgui = imgui::Context::create();
    themes::new(imgui.style_mut());

    // Disable creation if imgui files
    imgui.set_ini_filename(None);
    imgui.set_log_filename(None);

    let fonts = Fonts::setup(&mut imgui);

    let exe_loader = super::load_game_binaries(&target_dir);

    let mut main = Main::new(cfg, adapters, exe_loader, update_available, &target_dir, fonts);

    crate::render::render(LogicalSize::new(1024, 768), imgui, |ui: &mut imgui::Ui, w: f32, h: f32, _logo_texture: imgui::TextureId| {
        main.render(ui, Size { w, h });
    });

    #[allow(static_mut_refs)]
    // stop server process if needed
    if let Some(mut child) = unsafe { SERVER_PROCESS.take() } {
        child.kill().unwrap();
        child.wait().unwrap();
    }
}
