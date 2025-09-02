#![feature(unboxed_closures, tuple_trait, c_variadic, once_cell_try, mapped_lock_guards)]
#![deny(clippy::pedantic)]

use std::ffi::c_char;
use std::ffi::CStr;
use std::ffi::CString;
use std::ffi::OsString;
use std::fs::File;
use std::os::raw::c_void;
#[cfg(target_os = "windows")]
use std::os::windows::ffi::OsStringExt;
use std::path::Path;
use std::path::PathBuf;

use addresses::Addresses;
use hooks_config as config;
use tracing::error;
use tracing::info;
use tracing::instrument;
use tracing::level_filters::LevelFilter;
use windows::core::PCSTR;
use windows::Win32::Foundation::BOOL;
use windows::Win32::Foundation::HMODULE;
use windows::Win32::System::LibraryLoader::GetModuleFileNameW;
use windows::Win32::System::SystemServices::DLL_PROCESS_ATTACH;
use windows::Win32::System::SystemServices::DLL_PROCESS_DETACH;
use windows::Win32::System::SystemServices::DLL_THREAD_ATTACH;
use windows::Win32::System::SystemServices::DLL_THREAD_DETACH;
use windows::Win32::UI::WindowsAndMessaging::MessageBoxA;
use windows::Win32::UI::WindowsAndMessaging::MB_OK;

mod addresses;
mod api;
mod hooks;
mod macros;
mod overlay;
mod uplay_r1_loader;

use macros::fatal_error;

unsafe fn writemem(ptr: *mut u8, data: &[u8]) {
    if let Ok(_handle) = region::protect_with_handle(ptr, data.len(), region::Protection::READ_WRITE) {
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
        writemem(addrs.onlineconfig_url as *mut u8, new_server_cstr.as_bytes_with_nul());
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

extern "C" {
    #[cfg(all(target_family = "windows", target_env = "msvc"))]
    fn snprintf(buffer: *mut c_char, count: usize, format: *const c_char, ...) -> i32;
}

unsafe extern "C" fn debug_print(fmt: *const c_char, args: ...) {
    #![allow(clippy::cast_sign_loss)]
    let mut buf: Vec<u8> = vec![0; 4096];

    let mut required = snprintf(buf.as_mut_ptr().cast(), buf.len() - 1, fmt, args.clone()) as usize;
    if required > buf.len() {
        buf.reserve(required);
        required = snprintf(buf.as_mut_ptr().cast(), buf.len() - 1, fmt, args) as usize;
    }

    let formatted = CStr::from_bytes_with_nul(&buf[..=required]);

    panic!("{formatted:?}");
}

unsafe fn enable_debug_print(addrs: &Addresses) {
    let Some((func_ptr, ref to_nop)) = addrs.debug_print else {
        return;
    };

    let target = func_ptr as *mut unsafe extern "C" fn(*const c_char, ...);

    *target = debug_print;

    for addr in to_nop {
        writemem(addr.start as *mut u8, &vec![0xccu8; addr.end - addr.start]);
        // writemem(addr.start as *mut u8, &vec![0x90u8; addr.end - addr.start]);
    }
}

#[instrument(skip_all)]
fn init(hmodule: Option<HMODULE>) {
    let dir = get_target_dir(hmodule);
    let reload_handle = init_log(&dir);
    if let Some(cmdline) = get_arguments() {
        info!("Cmdline: {}", cmdline);
    }
    let path = config::get_config_path(dir);
    info!("Config path={:?}", path);
    let config = match config::get_or_load(path) {
        Err(e) => {
            fatal_error!("Config couldn't be loaded: {e}");
        }
        Ok(cfg) => cfg,
    };

    let level = match config.logging.level {
        config::LogLevel::Trace => tracing_subscriber::filter::LevelFilter::TRACE,
        config::LogLevel::Debug => tracing_subscriber::filter::LevelFilter::DEBUG,
        config::LogLevel::Info => tracing_subscriber::filter::LevelFilter::INFO,
        config::LogLevel::Warning => tracing_subscriber::filter::LevelFilter::WARN,
        config::LogLevel::Error => tracing_subscriber::filter::LevelFilter::ERROR,
    };
    reload_handle.reload(tracing_subscriber::EnvFilter::default().add_directive(level.into())).unwrap();

    let addr = addresses::get();

    unsafe {
        hooks::init(config, &addr);
        if let Some(config_server) = config.config_server.as_ref() {
            #[cfg(not(feature = "patch-free"))]
            patch_url(config_server, &addr);
        } else {
            info!("Keeping original config server");
        }

        if !config.internal_command_line.is_empty() {
            if let Some(ptr) = addr.unreal_commandline {
                let ptr = ptr as *mut u8;
                let count = if config.internal_command_line.len() > 11 {
                    11
                } else {
                    config.internal_command_line.len()
                };
                ptr.copy_from_nonoverlapping(config.internal_command_line.as_ptr(), count);
            } else {
                error!("Can't set command line. Address is missing");
            }
        }

        enable_debug_print(&addr);
    }

    // needs to be done in a separate thread, otherwise it'll block indefinitely
    std::thread::Builder::new()
        .name(String::from("login-thread"))
        .spawn(move || {
            // try to login once. relogins are attempted by the update thread later on
            let _ = api::login(&config.user.username, &config.user.password);
        })
        .unwrap();
}

fn deinit(hmodule: Option<HMODULE>) {
    // disable tracing as TLS is already destroyed at this point.
    // No tracing calls/instruments are allowed before this point!!
    let _guard = tracing::dispatcher::set_default(&tracing::dispatcher::Dispatch::none());
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
        File::options().create(true).append(true).open(&self.0).unwrap()
    }
}

fn show_msgbox(msg: &str, caption: &str) {
    let msg = CString::new(msg).unwrap();
    let caption = CString::new(caption).unwrap();
    unsafe {
        MessageBoxA(None, PCSTR(msg.as_ptr().cast::<u8>()), PCSTR(caption.as_ptr().cast::<u8>()), MB_OK);
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
    // cannot fail
    Some(PathBuf::from(path))
}

fn get_arguments() -> Option<String> {
    let cmdline = unsafe { windows::Win32::System::Environment::GetCommandLineW().to_string() };
    cmdline.ok()
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
        // cannot fail
        let mut path = PathBuf::from(path);
        path.pop();
        Ok(path)
    });
    if path.is_err() {
        path = std::env::current_dir().map_err(anyhow::Error::from);
    }
    path.unwrap_or_default()
}

type ReconfigurableLogger = tracing_subscriber::reload::Handle<
    tracing_subscriber::EnvFilter,
    tracing_subscriber::layer::Layered<
        tracing_subscriber::fmt::Layer<tracing_subscriber::Registry, tracing_subscriber::fmt::format::DefaultFields, tracing_subscriber::fmt::format::Format, FileWriter>,
        tracing_subscriber::Registry,
    >,
>;

fn init_log(target_dir: &Path) -> ReconfigurableLogger {
    let path = target_dir.join("bl-tracing.log");
    let _ = std::fs::remove_file(&path);
    let subscriber_builder = tracing_subscriber::FmtSubscriber::builder()
        .with_writer(FileWriter(path))
        .with_ansi(false)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_line_number(true)
        .with_env_filter(tracing_subscriber::EnvFilter::builder().with_default_directive(LevelFilter::INFO.into()).from_env_lossy())
        .with_filter_reloading();
    let reload_handle = subscriber_builder.reload_handle();
    subscriber_builder.init();
    tracing::event!(tracing::Level::INFO, "attaching");

    std::panic::set_hook(Box::new(|panic_info| {
        let mut expl = String::new();

        let message = match (panic_info.payload().downcast_ref::<&str>(), panic_info.payload().downcast_ref::<String>()) {
            (Some(s), _) => Some((*s).to_string()),
            (_, Some(s)) => Some(s.to_string()),
            (None, None) => None,
        };

        let cause = match message {
            Some(m) => m,
            None => "Unknown".into(),
        };

        match panic_info.location() {
            Some(location) => {
                expl.push_str(&format!("Panic occurred in file '{}' at line {}", location.file(), location.line()));
            }
            None => expl.push_str("Panic location unknown."),
        }
        let msg = format!("{expl}\n{cause}");

        tracing::error!("{}", msg);

        show_msgbox(&msg, "PANIC");

        unsafe {
            std::arch::asm!("int3");
        }

        std::process::exit(1);
    }));
    reload_handle
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
