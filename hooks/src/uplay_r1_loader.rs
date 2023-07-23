use std::ffi::c_char;
use std::mem::transmute;
use std::sync::OnceLock;

use hooks_proc::forwardable_export;
use tracing::error;
use tracing::info;
use tracing::warn;
use windows::core::s;
use windows::core::PCSTR;
use windows::Win32::Foundation::HMODULE;
use windows::Win32::System::LibraryLoader::GetProcAddress;
use windows::Win32::System::LibraryLoader::LoadLibraryA;
use windows::Win32::UI::WindowsAndMessaging::MessageBoxA;
use windows::Win32::UI::WindowsAndMessaging::MB_OK;

use crate::config::get;

mod ach;
mod avatar;
mod friends;
mod overlay;
mod party;
mod presence;
mod save;
mod types;
mod user;
mod win;

use types::List;
use types::UplayFriend;
use types::UplayList;
use types::UplayOverlapped;
use types::UplaySave;

type Result<T> = std::result::Result<T, anyhow::Error>;

fn get_proc(name: PCSTR) -> Option<unsafe extern "system" fn() -> isize> {
    static DLL_HANDLE: OnceLock<windows::core::Result<HMODULE>> = OnceLock::new();
    let handle = DLL_HANDLE
        .get_or_init(|| unsafe { LoadLibraryA(s!("uplay_r1_loader.orig.dll")) })
        .as_ref()
        .map_err(|e| unsafe {
            error!("Library loading error: {:?}", e);
            let mut s = format!("{:?}", e);
            let v = s.as_mut_vec();
            v.push(b'\0');
            MessageBoxA(None, PCSTR(v.as_ptr()), s!("Error"), MB_OK);
            e
        })
        .map(|l| {
            info!("Library loaded");
            l
        })
        .unwrap();
    unsafe { GetProcAddress(*handle, name) }
}

#[forwardable_export]
unsafe extern "cdecl" fn UPLAY_GetLastError(out_error_string: *mut *const c_char) -> bool {
    false
}

#[no_mangle]
unsafe extern "cdecl" fn UPLAY_GetNextEvent(event: *mut ()) -> bool {
    // info!("UPLAY_GetNextEvent");
    let Some(cfg) = get() else {
        error!("Config not loaded!");
        return false;
    };
    if cfg!(feature = "forward_calls")
        || cfg.forward_all_calls
        || cfg.forward_calls.iter().any(|s| s == "UPLAY_GetNextEvent")
    {
        static FUNC: OnceLock<unsafe extern "cdecl" fn(*mut ()) -> bool> = OnceLock::new();
        let func = FUNC.get_or_init(|| transmute(get_proc(s!("UPLAY_GetNextEvent")).unwrap()));
        (func)(event)
    } else {
        false
    }
}

#[forwardable_export]
unsafe extern "cdecl" fn UPLAY_GetOverlappedOperationResult_(
    overlapped: *mut UplayOverlapped,
    result: *mut usize,
) -> bool {
    let overlapped = unsafe { overlapped.as_ref() };
    let result = unsafe { result.as_mut() };
    if let Some((overlapped, result)) = overlapped.filter(|o| o.is_completed != 0).zip(result) {
        *result = overlapped.result;
        true
    } else {
        false
    }
}

#[forwardable_export]
unsafe extern "cdecl" fn UPLAY_GetOverlappedOperationResult(
    overlapped: *mut UplayOverlapped,
    result: *mut usize,
) -> bool {
    let overlapped = unsafe { overlapped.as_ref() };
    let result = unsafe { result.as_mut() };
    if let Some((overlapped, result)) = overlapped.filter(|o| o.is_completed != 0).zip(result) {
        *result = overlapped.result;
        true
    } else {
        false
    }
}

#[forwardable_export]
unsafe extern "cdecl" fn UPLAY_HasOverlappedOperationCompleted(
    overlapped: *mut UplayOverlapped,
) -> bool {
    let overlapped = unsafe { overlapped.as_ref() };
    overlapped.map(|o| o.is_completed != 0).unwrap_or_default()
}

#[forwardable_export]
unsafe extern "cdecl" fn UPLAY_Quit() -> bool {
    false
}

#[forwardable_export]
unsafe extern "cdecl" fn UPLAY_Release(ptr: *mut List) -> bool {
    if !ptr.is_null() {
        let list = *Box::from_raw(ptr);
        let list: UplayList = list.try_into().unwrap();
        drop(list);
    }
    true
}

#[forwardable_export]
unsafe extern "cdecl" fn UPLAY_Startup(
    uplay_id: usize,
    game_version: usize,
    language_country_code_utf8: *const c_char,
) -> isize {
    0 // 0 = all good, 1 = error occured, 2 = ??? (), 3 = ???
}

#[no_mangle]
unsafe extern "cdecl" fn UPLAY_Update() -> bool {
    // info!("UPLAY_Update");
    let Some(cfg) = get() else {
        error!("Config not loaded!");
        return false;
    };
    if cfg!(feature = "forward_calls")
        || cfg.forward_all_calls
        || cfg.forward_calls.iter().any(|s| s == "UPLAY_Update")
    {
        static FUNC: OnceLock<unsafe extern "cdecl" fn() -> bool> = OnceLock::new();
        let func = FUNC.get_or_init(|| transmute(get_proc(s!("UPLAY_Update")).unwrap()));
        (func)()
    } else {
        true
    }
}
