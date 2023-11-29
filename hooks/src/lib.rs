#![feature(pointer_is_aligned)]
#![feature(unboxed_closures, tuple_trait)]
#![deny(clippy::pedantic)]

use std::ffi::CString;
use std::ffi::OsString;
use std::fs::File;
use std::os::raw::c_void;
use std::os::windows::ffi::OsStringExt;
use std::path::Path;
use std::path::PathBuf;

use addresses::Addresses;
use tracing::info;
use tracing::instrument;
use windows::core::PCSTR;
use windows::Win32::Foundation::BOOL;
use windows::Win32::Foundation::HMODULE;
use windows::Win32::System::LibraryLoader::GetModuleFileNameW;
use windows::Win32::System::SystemServices::DLL_PROCESS_ATTACH;
use windows::Win32::System::SystemServices::DLL_PROCESS_DETACH;
use windows::Win32::System::SystemServices::DLL_THREAD_ATTACH;
use windows::Win32::System::SystemServices::DLL_THREAD_DETACH;
use windows::Win32::UI::WindowsAndMessaging::MessageBoxA;
use windows::Win32::UI::WindowsAndMessaging::IDOK;
use windows::Win32::UI::WindowsAndMessaging::MB_ICONQUESTION;
use windows::Win32::UI::WindowsAndMessaging::MB_OK;
use windows::Win32::UI::WindowsAndMessaging::MB_OKCANCEL;

mod addresses;
mod config;
mod hooks;
mod macros;
mod uplay_r1_loader;

use macros::fatal_error;

unsafe fn writemem(ptr: *mut u8, data: &[u8]) {
    if let Ok(_handle) =
        region::protect_with_handle(ptr, data.len(), region::Protection::READ_WRITE)
    {
        std::ptr::copy(data.as_ptr(), ptr, data.len());
    }
}

unsafe fn patch_url(new_server: &str, addrs: &Addresses) {
    let Ok(new_server_cstr) = CString::new(new_server) else {
        fatal_error!("Invalid config_server");
    };
    let g_cfg_client = addrs.global_onlineconfig_client as *mut *mut *mut u8;
    if g_cfg_client.is_null() {
        fatal_error!(
            "Couldn't adjust config server";
            "Couldn't adjust config server: g_cfg_client is null"
        );
    }
    let g_cfg_client = *g_cfg_client;
    info!("Global onlineconfig client: {:?}", g_cfg_client);
    if g_cfg_client.is_null() {
        info!("Patching rodata with {}", new_server);
        writemem(
            addrs.onlineconfig_url as *mut u8,
            new_server_cstr.as_bytes_with_nul(),
        );
        return;
    }
    let hostname_ptr = g_cfg_client.add(0x24);
    info!("->hostname: {:?}", hostname_ptr);
    if hostname_ptr.is_null() {
        fatal_error!(
            "Couldn't adjust config server";
            "Couldn't adjust config server: g_cfg_client->hostname is null"
        );
    }
    let hostname_ptr = *hostname_ptr;
    info!("hostname: {:?}", hostname_ptr);
    if hostname_ptr.is_null() {
        fatal_error!(
            "Couldn't adjust config server";
            "Couldn't adjust config server: *g_cfg_client->hostname is null"
        );
    }
    info!("Patching heap with {}", new_server);
    writemem(hostname_ptr, new_server_cstr.as_bytes_with_nul());
}

#[instrument(skip_all)]
fn init(hmodule: Option<HMODULE>) {
    let dir = get_target_dir(hmodule);
    init_log(&dir);
    let path = config::get_config_path(dir);
    info!("Config path={:?}", path);
    let config = match config::get_or_load(path) {
        Err(e) => {
            fatal_error!(
                "Config couldn't be loaded!";
                "Config couldn't be loaded: {}",
                e
            );
        }
        Ok(cfg) => cfg,
    };

    let addr = addresses::get();

    unsafe {
        hooks::init(config, &addr);
        if let Some(config_server) = config.config_server.as_ref() {
            patch_url(config_server, &addr);
        } else {
            info!("Keeping original config server");
        }

        if !config.internal_command_line.is_empty() {
            let ptr = addr.unreal_commandline as *mut u8;
            let count = if config.internal_command_line.len() > 11 {
                11
            } else {
                config.internal_command_line.len()
            };
            ptr.copy_from_nonoverlapping(config.internal_command_line.as_ptr(), count);
        }
    }
}

#[instrument(skip_all)]
fn deinit(hmodule: Option<HMODULE>) {
    let dir = get_target_dir(hmodule);
    let path = config::get_config_path(dir);
    info!("Config path={:?}", path);
    let config = match config::get_or_load(path) {
        Err(e) => {
            fatal_error!("Config couldn't be loaded: {e}");
        }
        Ok(cfg) => cfg,
    };
    unsafe { hooks::deinit(config) };
}

struct FileWriter(PathBuf);

impl<'a> tracing_subscriber::fmt::MakeWriter<'a> for FileWriter {
    type Writer = File;

    fn make_writer(&'a self) -> Self::Writer {
        File::options()
            .create(true)
            .append(true)
            .open(&self.0)
            .unwrap()
    }
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

fn show_msgbox_ok_cancel(msg: &str, caption: &str) -> bool {
    let msg = CString::new(msg).unwrap();
    let caption = CString::new(caption).unwrap();
    unsafe {
        MessageBoxA(
            None,
            PCSTR(msg.as_ptr().cast::<u8>()),
            PCSTR(caption.as_ptr().cast::<u8>()),
            MB_OKCANCEL | MB_ICONQUESTION,
        ) == IDOK
    }
}

fn get_executable(hinst: HMODULE) -> Option<PathBuf> {
    let mut path = vec![0u16; 4096];
    let sz = unsafe { GetModuleFileNameW(hinst, &mut path) } as usize;
    let err = std::io::Error::last_os_error();
    if err.raw_os_error().unwrap_or_default() != 0 {
        return None;
    };
    let path = OsString::from_wide(&path[..sz]);
    PathBuf::try_from(path).ok()
}

fn get_target_dir(hinst: Option<HMODULE>) -> PathBuf {
    let mut path = hinst.ok_or(anyhow::anyhow!("no hinst")).and_then(|hinst| {
        let mut path = vec![0u16; 4096];
        let sz = unsafe { GetModuleFileNameW(hinst, &mut path) } as usize;
        let err = std::io::Error::last_os_error();
        if err.raw_os_error().unwrap_or_default() != 0 {
            return Err(err.into());
        };
        let path = OsString::from_wide(&path[..sz]);
        let mut path = PathBuf::try_from(path)?;
        path.pop();
        Ok(path)
    });
    if path.is_err() {
        path = std::env::current_dir().map_err(anyhow::Error::from);
    }
    path.unwrap_or_default()
}

fn init_log(target_dir: &Path) {
    let path = target_dir.join("bl-tracing.log");
    let _ = std::fs::remove_file(&path);
    tracing_subscriber::FmtSubscriber::builder()
        .with_writer(FileWriter(path))
        .with_ansi(false)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_line_number(true)
        .init();
    tracing::event!(tracing::Level::INFO, "attaching");

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
        tracing::error!("{}", msg);

        show_msgbox(&msg, "PANIC");
    }));
}

#[no_mangle]
unsafe extern "system" fn DllMain(hinst: HMODULE, reason: u32, _reserved: *mut c_void) -> BOOL {
    match reason {
        DLL_PROCESS_ATTACH => init(Some(hinst)),
        DLL_PROCESS_DETACH => deinit(Some(hinst)),
        DLL_THREAD_ATTACH | DLL_THREAD_DETACH => {}
        _ => {
            fatal_error!("Unexpected reason: {reason}");
        }
    };
    BOOL::from(true)
}
