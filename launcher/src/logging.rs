//! Initializes the logging framework and sets up a panic handler.
//!
//! This module configures `tracing-subscriber` for structured logging and
//! sets a custom panic hook to display panic information in a message box,
//! ensuring that users are notified of critical errors.

use std::ffi::CString;

use tracing_subscriber::layer::SubscriberExt as _;
use tracing_subscriber::util::SubscriberInitExt as _;
use windows::core::PCSTR;
use windows::Win32::UI::WindowsAndMessaging::MessageBoxA;
use windows::Win32::UI::WindowsAndMessaging::MB_OK;

/// Shows a Windows message box with the specified message and caption.
fn show_msgbox(msg: &str, caption: &str) {
    let msg = CString::new(msg).unwrap();
    let caption = CString::new(caption).unwrap();
    unsafe {
        MessageBoxA(None, PCSTR(msg.as_ptr().cast::<u8>()), PCSTR(caption.as_ptr().cast::<u8>()), MB_OK);
    }
}

/// Attaches the process to the parent console, enabling console output.
fn enable_console() {
    unsafe {
        // This allows the launcher to print to the console when run from a terminal.
        let _ = windows::Win32::System::Console::AttachConsole(windows::Win32::System::Console::ATTACH_PARENT_PROCESS);
    }
}

/// Sets a custom panic hook to display panic information in a message box.
fn catch_panics() {
    std::panic::set_hook(Box::new(|panic_info| {
        let mut expl = String::new();

        // Extract the panic message.
        let message = match (panic_info.payload().downcast_ref::<&str>(), panic_info.payload().downcast_ref::<String>()) {
            (Some(s), _) => Some((*s).to_string()),
            (_, Some(s)) => Some(s.to_string()),
            (None, None) => None,
        };

        let cause = match message {
            Some(m) => m,
            None => "Unknown".into(),
        };

        // Get the location of the panic.
        match panic_info.location() {
            Some(location) => expl.push_str(&format!("Panic occurred in file '{}' at line {}", location.file(), location.line())),
            None => expl.push_str("Panic location unknown."),
        }
        let msg = format!("{}\n{}", expl, cause);
        eprintln!("PANIC: {msg}");

        // Show the panic information in a message box.
        show_msgbox(&msg, "PANIC");

        std::process::exit(1);
    }));
}

/// Initializes the logging and panic handling for the application.
pub fn init() {
    enable_console();
    catch_panics();

    // Initialize the tracing subscriber with a default log level of WARN.
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(
            tracing_subscriber::EnvFilter::builder()
                .with_default_directive(tracing_subscriber::filter::LevelFilter::WARN.into())
                .from_env_lossy(),
        )
        .init();
}
