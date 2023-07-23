use std::ffi::c_char;
use std::ffi::c_void;
use std::mem::transmute;
use std::sync::OnceLock;

use hooks_proc::forwardable_export;
use tracing::debug;
use tracing::error;
use tracing::info;
use windows::core::s;

use super::get_proc;
use super::UplayOverlapped;
use crate::config::get;
use crate::uplay_r1_loader;

#[no_mangle]
unsafe extern "cdecl" fn UPLAY_FRIENDS_AddToBlackList() -> isize {
    info!("UPLAY_FRIENDS_AddToBlackList");
    let Some(cfg) = get() else {
        error!("Config not loaded!");
        return 0;
    };
    if cfg!(feature = "forward_calls")
        || cfg.forward_all_calls
        || cfg
            .forward_calls
            .iter()
            .any(|s| s == "UPLAY_FRIENDS_AddToBlackList")
    {
        static FUNC: OnceLock<unsafe extern "cdecl" fn() -> isize> = OnceLock::new();
        let func =
            FUNC.get_or_init(|| transmute(get_proc(s!("UPLAY_FRIENDS_AddToBlackList")).unwrap()));
        (func)()
    } else {
        0
    }
}

#[no_mangle]
unsafe extern "cdecl" fn UPLAY_FRIENDS_DisableFriendMenuItem() -> isize {
    info!("UPLAY_FRIENDS_DisableFriendMenuItem");
    let Some(cfg) = get() else {
        error!("Config not loaded!");
        return 0;
    };
    if cfg!(feature = "forward_calls")
        || cfg.forward_all_calls
        || cfg
            .forward_calls
            .iter()
            .any(|s| s == "UPLAY_FRIENDS_DisableFriendMenuItem")
    {
        static FUNC: OnceLock<unsafe extern "cdecl" fn() -> isize> = OnceLock::new();
        let func = FUNC.get_or_init(|| {
            transmute(get_proc(s!("UPLAY_FRIENDS_DisableFriendMenuItem")).unwrap())
        });
        (func)()
    } else {
        0
    }
}

#[no_mangle]
unsafe extern "cdecl" fn UPLAY_FRIENDS_EnableFriendMenuItem() -> isize {
    info!("UPLAY_FRIENDS_EnableFriendMenuItem");
    let Some(cfg) = get() else {
        error!("Config not loaded!");
        return 0;
    };
    if cfg!(feature = "forward_calls")
        || cfg.forward_all_calls
        || cfg
            .forward_calls
            .iter()
            .any(|s| s == "UPLAY_FRIENDS_EnableFriendMenuItem")
    {
        static FUNC: OnceLock<unsafe extern "cdecl" fn() -> isize> = OnceLock::new();
        let func = FUNC
            .get_or_init(|| transmute(get_proc(s!("UPLAY_FRIENDS_EnableFriendMenuItem")).unwrap()));
        (func)()
    } else {
        0
    }
}

#[forwardable_export]
unsafe extern "cdecl" fn UPLAY_FRIENDS_GetFriendList(
    friend_list_filter: *mut c_void,
    out_friend_list: *mut uplay_r1_loader::List,
) -> bool {
    let list = uplay_r1_loader::UplayList::Friends(vec![uplay_r1_loader::UplayFriend {
        id: "MYID".into(),
        username: "thesam".into(),
    }]);
    let list: uplay_r1_loader::List = list.into();
    debug!("list = {list:?}");
    (*out_friend_list).count = list.count;
    (*out_friend_list).list = list.list;
    true
}

#[forwardable_export]
unsafe extern "cdecl" fn UPLAY_FRIENDS_Init(flags: usize) -> bool {
    true
}

#[forwardable_export]
unsafe extern "cdecl" fn UPLAY_FRIENDS_InviteToGame(
    account_id_utf8: *const c_char,
    overlapped: *mut UplayOverlapped,
) -> bool {
    if !overlapped.is_null() && overlapped.is_aligned() {
        (*overlapped).set_completed(true);
    }
    false
}

#[forwardable_export]
unsafe extern "cdecl" fn UPLAY_FRIENDS_IsBlackListed(account_id_utf8: *const c_char) -> bool {
    false
}

#[forwardable_export]
unsafe extern "cdecl" fn UPLAY_FRIENDS_IsFriend(account_id_utf8: *const c_char) -> bool {
    false
}

#[no_mangle]
unsafe extern "cdecl" fn UPLAY_FRIENDS_RequestFriendship() -> isize {
    info!("UPLAY_FRIENDS_RequestFriendship");
    let Some(cfg) = get() else {
        error!("Config not loaded!");
        return 0;
    };
    if cfg!(feature = "forward_calls")
        || cfg.forward_all_calls
        || cfg
            .forward_calls
            .iter()
            .any(|s| s == "UPLAY_FRIENDS_RequestFriendship")
    {
        static FUNC: OnceLock<unsafe extern "cdecl" fn() -> isize> = OnceLock::new();
        let func = FUNC
            .get_or_init(|| transmute(get_proc(s!("UPLAY_FRIENDS_RequestFriendship")).unwrap()));
        (func)()
    } else {
        0
    }
}

#[no_mangle]
unsafe extern "cdecl" fn UPLAY_FRIENDS_ShowFriendSelectionUI() -> isize {
    info!("UPLAY_FRIENDS_ShowFriendSelectionUI");
    let Some(cfg) = get() else {
        error!("Config not loaded!");
        return 0;
    };
    if cfg!(feature = "forward_calls")
        || cfg.forward_all_calls
        || cfg
            .forward_calls
            .iter()
            .any(|s| s == "UPLAY_FRIENDS_ShowFriendSelectionUI")
    {
        static FUNC: OnceLock<unsafe extern "cdecl" fn() -> isize> = OnceLock::new();
        let func = FUNC.get_or_init(|| {
            transmute(get_proc(s!("UPLAY_FRIENDS_ShowFriendSelectionUI")).unwrap())
        });
        (func)()
    } else {
        0
    }
}
