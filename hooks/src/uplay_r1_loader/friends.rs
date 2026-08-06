use std::ffi::c_char;
use std::ffi::c_void;

use hooks_proc::forwardable_export;
use tracing::debug;
use tracing::error;
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
unsafe extern "cdecl" fn UPLAY_FRIENDS_GetFriendList(friend_list_filter: *mut c_void, out_friend_list: *mut uplay_r1_loader::List) -> bool {
    let friends = crate::api::list_friends().unwrap_or_default();
    let list = uplay_r1_loader::UplayList::Friends(
        friends
            .into_iter()
            .into_iter()
            .map(|f| uplay_r1_loader::UplayFriend {
                id: f.id,
                username: f.username,
                is_online: f.is_online,
                session_id: f.session_id,
                session_data: f.session_data,
                pid: f.pid,
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
unsafe extern "cdecl" fn UPLAY_FRIENDS_InviteToGame(account_id_utf8: *const c_char, overlapped: *mut UplayOverlapped) -> bool {
    // The game passes the account id of the player to invite and expects the usual Uplay
    // contract in return: `true` means "request accepted". This used to return `false`
    // unconditionally, so the game reported every invitation as failed - even though the
    // server had already stored it and the recipient's client picked it up.
    let Some(account_id) = (!account_id_utf8.is_null())
        .then(|| std::ffi::CStr::from_ptr(account_id_utf8).to_str().ok())
        .flatten()
    else {
        error!("UPLAY_FRIENDS_InviteToGame called without a usable account id");
        return false;
    };

    // Deliberately no unwrap: this runs on the game's own thread, so a failing request would
    // tear down the whole game instead of surfacing as an invitation that did not go through.
    if let Err(e) = crate::api::invite_friend(account_id) {
        error!("Invitation for {account_id} failed: {e:?}");
        return false;
    }
    info!("Invitation for {account_id} handed to the server");

    if !overlapped.is_null() && overlapped.is_aligned() {
        // set_success() also marks it completed and, unlike set_completed(), leaves a defined
        // result behind rather than whatever happened to be in the field.
        (*overlapped).set_success();
    }
    true
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
