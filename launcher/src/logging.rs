use std::ffi::CString;

use tracing_subscriber::layer::SubscriberExt as _;
use tracing_subscriber::util::SubscriberInitExt as _;
use windows::core::PCSTR;
use windows::Win32::UI::WindowsAndMessaging::MessageBoxA;
use windows::Win32::UI::WindowsAndMessaging::MB_OK;

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

pub fn init() {
    enable_console();
    catch_panics();

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();
}
