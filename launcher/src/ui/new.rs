//! The entry point for the new launcher UI.
//!
//! This module is responsible for initializing the `imgui` context, setting up
//! the theme and fonts, creating the main UI component, and running the main
//! render loop.

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

/// Runs the new launcher UI.
///
/// # Arguments
///
/// * `target_dir` - The directory of the game.
/// * `cfg` - The mutable configuration for the launcher.
/// * `adapters` - A list of available network adapters.
/// * `update_available` - An `Option` containing the version of an available update, if any.
pub fn run(target_dir: PathBuf, cfg: ConfigMut, adapters: &[(String, IpAddr)], update_available: Option<Version>) {
    let mut imgui = imgui::Context::create();
    themes::new(imgui.style_mut());

    // Disable creation of imgui.ini and imgui_log.txt files.
    imgui.set_ini_filename(None);
    imgui.set_log_filename(None);

    // Set up the fonts for the UI.
    let fonts = Fonts::setup(&mut imgui);

    // Load the game binaries in the background.
    let exe_loader = super::load_game_binaries(&target_dir);

    // Create the main UI component.
    let mut main = Main::new(cfg, adapters, exe_loader, update_available, &target_dir, fonts);

    // Run the main render loop.
    crate::render::render(LogicalSize::new(1024, 768), imgui, |ui: &mut imgui::Ui, w: f32, h: f32, _logo_texture: imgui::TextureId| {
        main.render(ui, Size { w, h });
    });

    #[allow(static_mut_refs)]
    // Ensure the server process is terminated when the UI exits.
    if let Some(mut child) = unsafe { SERVER_PROCESS.take() } {
        child.kill().unwrap();
        child.wait().unwrap();
    }
}
