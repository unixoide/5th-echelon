use std::mem::transmute;
use std::sync::OnceLock;

use tracing::error;
use tracing::info;
use windows::core::s;

use super::get_proc;
use crate::config::get;

#[no_mangle]
unsafe extern "cdecl" fn UPLAY_OVERLAY_Show() -> isize {
    info!("UPLAY_OVERLAY_Show");
    let Some(cfg) = get() else {
        error!("Config not loaded!");
        return 0;
    };
    if cfg!(feature = "forward_calls")
        || cfg.forward_all_calls
        || cfg.forward_calls.iter().any(|s| s == "UPLAY_OVERLAY_Show")
    {
        static FUNC: OnceLock<unsafe extern "cdecl" fn() -> isize> = OnceLock::new();
        let func = FUNC.get_or_init(|| transmute(get_proc(s!("UPLAY_OVERLAY_Show")).unwrap()));
        (func)()
    } else {
        0
    }
}
