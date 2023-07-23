use std::ffi::c_char;
use std::ffi::c_void;
use std::mem::transmute;
use std::sync::OnceLock;

use hooks_proc::forwardable_export;
use tracing::error;
use tracing::info;
use windows::core::s;

use super::get_proc;
use super::UplayOverlapped;
use crate::config::get;

#[no_mangle]
unsafe extern "cdecl" fn UPLAY_PARTY_DisablePartyMemberMenuItem() -> isize {
    info!("UPLAY_PARTY_DisablePartyMemberMenuItem");
    let Some(cfg) = get() else {
        error!("Config not loaded!");
        return 0;
    };
    if cfg!(feature = "forward_calls")
        || cfg.forward_all_calls
        || cfg
            .forward_calls
            .iter()
            .any(|s| s == "UPLAY_PARTY_DisablePartyMemberMenuItem")
    {
        static FUNC: OnceLock<unsafe extern "cdecl" fn() -> isize> = OnceLock::new();
        let func = FUNC.get_or_init(|| {
            transmute(get_proc(s!("UPLAY_PARTY_DisablePartyMemberMenuItem")).unwrap())
        });
        (func)()
    } else {
        0
    }
}

#[no_mangle]
unsafe extern "cdecl" fn UPLAY_PARTY_EnablePartyMemberMenuItem() -> isize {
    info!("UPLAY_PARTY_EnablePartyMemberMenuItem");
    let Some(cfg) = get() else {
        error!("Config not loaded!");
        return 0;
    };
    if cfg!(feature = "forward_calls")
        || cfg.forward_all_calls
        || cfg
            .forward_calls
            .iter()
            .any(|s| s == "UPLAY_PARTY_EnablePartyMemberMenuItem")
    {
        static FUNC: OnceLock<unsafe extern "cdecl" fn() -> isize> = OnceLock::new();
        let func = FUNC.get_or_init(|| {
            transmute(get_proc(s!("UPLAY_PARTY_EnablePartyMemberMenuItem")).unwrap())
        });
        (func)()
    } else {
        0
    }
}

#[no_mangle]
unsafe extern "cdecl" fn UPLAY_PARTY_GetFullMemberList() -> isize {
    info!("UPLAY_PARTY_GetFullMemberList");
    let Some(cfg) = get() else {
        error!("Config not loaded!");
        return 0;
    };
    if cfg!(feature = "forward_calls")
        || cfg.forward_all_calls
        || cfg
            .forward_calls
            .iter()
            .any(|s| s == "UPLAY_PARTY_GetFullMemberList")
    {
        static FUNC: OnceLock<unsafe extern "cdecl" fn() -> isize> = OnceLock::new();
        let func =
            FUNC.get_or_init(|| transmute(get_proc(s!("UPLAY_PARTY_GetFullMemberList")).unwrap()));
        (func)()
    } else {
        0
    }
}

#[no_mangle]
unsafe extern "cdecl" fn UPLAY_PARTY_GetInGameMemberList() -> isize {
    info!("UPLAY_PARTY_GetInGameMemberList");
    let Some(cfg) = get() else {
        error!("Config not loaded!");
        return 0;
    };
    if cfg!(feature = "forward_calls")
        || cfg.forward_all_calls
        || cfg
            .forward_calls
            .iter()
            .any(|s| s == "UPLAY_PARTY_GetInGameMemberList")
    {
        static FUNC: OnceLock<unsafe extern "cdecl" fn() -> isize> = OnceLock::new();
        let func = FUNC
            .get_or_init(|| transmute(get_proc(s!("UPLAY_PARTY_GetInGameMemberList")).unwrap()));
        (func)()
    } else {
        0
    }
}

#[forwardable_export]
unsafe extern "cdecl" fn UPLAY_PARTY_Init(flags: usize) -> bool {
    true
}

#[forwardable_export]
unsafe extern "cdecl" fn UPLAY_PARTY_InvitePartyToGame(overlapped: *mut UplayOverlapped) -> bool {
    false
}

#[forwardable_export]
unsafe extern "cdecl" fn UPLAY_PARTY_InviteToParty(
    account_id: *const c_char,
    overlapped: *mut UplayOverlapped,
) -> bool {
    false
}

#[forwardable_export]
unsafe extern "cdecl" fn UPLAY_PARTY_IsInParty(account_id: *const c_char) -> bool {
    false
}

#[forwardable_export]
unsafe extern "cdecl" fn UPLAY_PARTY_IsPartyLeader(account_id: *const c_char) -> bool {
    false
}

#[forwardable_export]
unsafe extern "cdecl" fn UPLAY_PARTY_PromoteToLeader(
    account_id: *const c_char,
    overlapped: *mut UplayOverlapped,
) -> bool {
    false
}

#[forwardable_export]
unsafe extern "cdecl" fn UPLAY_PARTY_RespondToGameInvite(
    invitation_id: *const c_void,
    accept: bool,
) -> bool {
    false
}

#[no_mangle]
unsafe extern "cdecl" fn UPLAY_PARTY_SetGuest() -> isize {
    info!("UPLAY_PARTY_SetGuest");
    let Some(cfg) = get() else {
        error!("Config not loaded!");
        return 0;
    };
    if cfg!(feature = "forward_calls")
        || cfg.forward_all_calls
        || cfg
            .forward_calls
            .iter()
            .any(|s| s == "UPLAY_PARTY_SetGuest")
    {
        static FUNC: OnceLock<unsafe extern "cdecl" fn() -> isize> = OnceLock::new();
        let func = FUNC.get_or_init(|| transmute(get_proc(s!("UPLAY_PARTY_SetGuest")).unwrap()));
        (func)()
    } else {
        0
    }
}

#[no_mangle]
unsafe extern "cdecl" fn UPLAY_PARTY_SetUserData() -> isize {
    info!("UPLAY_PARTY_SetUserData");
    let Some(cfg) = get() else {
        error!("Config not loaded!");
        return 0;
    };
    if cfg!(feature = "forward_calls")
        || cfg.forward_all_calls
        || cfg
            .forward_calls
            .iter()
            .any(|s| s == "UPLAY_PARTY_SetUserData")
    {
        static FUNC: OnceLock<unsafe extern "cdecl" fn() -> isize> = OnceLock::new();
        let func = FUNC.get_or_init(|| transmute(get_proc(s!("UPLAY_PARTY_SetUserData")).unwrap()));
        (func)()
    } else {
        0
    }
}

#[no_mangle]
unsafe extern "cdecl" fn UPLAY_PARTY_ShowGameInviteOverlayUI() -> isize {
    info!("UPLAY_PARTY_ShowGameInviteOverlayUI");
    let Some(cfg) = get() else {
        error!("Config not loaded!");
        return 0;
    };
    if cfg!(feature = "forward_calls")
        || cfg.forward_all_calls
        || cfg
            .forward_calls
            .iter()
            .any(|s| s == "UPLAY_PARTY_ShowGameInviteOverlayUI")
    {
        static FUNC: OnceLock<unsafe extern "cdecl" fn() -> isize> = OnceLock::new();
        let func = FUNC.get_or_init(|| {
            transmute(get_proc(s!("UPLAY_PARTY_ShowGameInviteOverlayUI")).unwrap())
        });
        (func)()
    } else {
        0
    }
}
