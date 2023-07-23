use std::mem::transmute;
use std::sync::OnceLock;

use tracing::error;
use tracing::info;
use windows::core::s;

use super::get_proc;
use crate::config::get;

#[no_mangle]
unsafe extern "cdecl" fn UPLAY_PRESENCE_SetRichPresenceLine() -> isize {
    info!("UPLAY_PRESENCE_SetRichPresenceLine");
    let Some(cfg) = get() else {
        error!("Config not loaded!");
        return 0;
    };
    if cfg!(feature = "forward_calls")
        || cfg.forward_all_calls
        || cfg
            .forward_calls
            .iter()
            .any(|s| s == "UPLAY_PRESENCE_SetRichPresenceLine")
    {
        static FUNC: OnceLock<unsafe extern "cdecl" fn() -> isize> = OnceLock::new();
        let func = FUNC
            .get_or_init(|| transmute(get_proc(s!("UPLAY_PRESENCE_SetRichPresenceLine")).unwrap()));
        (func)()
    } else {
        0
    }
}

#[no_mangle]
unsafe extern "cdecl" fn UPLAY_PRESENCE_SetRichPresencePropertyInt32() -> isize {
    info!("UPLAY_PRESENCE_SetRichPresencePropertyInt32");
    let Some(cfg) = get() else {
        error!("Config not loaded!");
        return 0;
    };
    if cfg!(feature = "forward_calls")
        || cfg.forward_all_calls
        || cfg
            .forward_calls
            .iter()
            .any(|s| s == "UPLAY_PRESENCE_SetRichPresencePropertyInt32")
    {
        static FUNC: OnceLock<unsafe extern "cdecl" fn() -> isize> = OnceLock::new();
        let func = FUNC.get_or_init(|| {
            transmute(get_proc(s!("UPLAY_PRESENCE_SetRichPresencePropertyInt32")).unwrap())
        });
        (func)()
    } else {
        0
    }
}

#[no_mangle]
unsafe extern "cdecl" fn UPLAY_PRESENCE_SetRichPresencePropertyString() -> isize {
    info!("UPLAY_PRESENCE_SetRichPresencePropertyString");
    let Some(cfg) = get() else {
        error!("Config not loaded!");
        return 0;
    };
    if cfg!(feature = "forward_calls")
        || cfg.forward_all_calls
        || cfg
            .forward_calls
            .iter()
            .any(|s| s == "UPLAY_PRESENCE_SetRichPresencePropertyString")
    {
        static FUNC: OnceLock<unsafe extern "cdecl" fn() -> isize> = OnceLock::new();
        let func = FUNC.get_or_init(|| {
            transmute(get_proc(s!("UPLAY_PRESENCE_SetRichPresencePropertyString")).unwrap())
        });
        (func)()
    } else {
        0
    }
}
