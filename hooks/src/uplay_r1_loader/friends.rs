use std::ffi::c_char;
use std::ffi::c_void;

use hooks_proc::forwardable_export;
use tracing::debug;
use tracing::info;

use super::UplayOverlapped;
use crate::uplay_r1_loader;

#[forwardable_export]
unsafe extern "cdecl" fn UPLAY_FRIENDS_AddToBlackList() -> isize {
    0
}

#[forwardable_export]
unsafe extern "cdecl" fn UPLAY_FRIENDS_DisableFriendMenuItem() -> isize {
    0
}

#[forwardable_export]
unsafe extern "cdecl" fn UPLAY_FRIENDS_EnableFriendMenuItem() -> isize {
    0
}

#[forwardable_export]
unsafe extern "cdecl" fn UPLAY_FRIENDS_GetFriendList(
    friend_list_filter: *mut c_void,
    out_friend_list: *mut uplay_r1_loader::List,
) -> bool {
    let list = uplay_r1_loader::UplayList::Friends(
        crate::api::list_friends()
            .unwrap_or_default()
            .into_iter()
            .map(|f| uplay_r1_loader::UplayFriend {
                id: f.id,
                username: f.username,
                is_online: f.is_online,
            })
            .collect(),
    );
    info!("Returning friends: {list:?}");
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
    crate::api::invite_friend(std::ffi::CStr::from_ptr(account_id_utf8).to_str().unwrap()).unwrap();
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

#[forwardable_export]
unsafe extern "cdecl" fn UPLAY_FRIENDS_RequestFriendship() -> isize {
    0
}

#[forwardable_export]
unsafe extern "cdecl" fn UPLAY_FRIENDS_ShowFriendSelectionUI() -> isize {
    0
}
