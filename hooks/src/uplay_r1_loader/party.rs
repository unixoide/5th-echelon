use std::ffi::c_char;
use std::ffi::c_void;

use hooks_proc::forwardable_export;
use tracing::info;

use super::UplayOverlapped;

#[forwardable_export]
unsafe extern "cdecl" fn UPLAY_PARTY_DisablePartyMemberMenuItem() -> isize {
    0
}

#[forwardable_export]
unsafe extern "cdecl" fn UPLAY_PARTY_EnablePartyMemberMenuItem() -> isize {
    0
}

#[forwardable_export]
unsafe extern "cdecl" fn UPLAY_PARTY_GetFullMemberList() -> isize {
    0
}

#[forwardable_export]
unsafe extern "cdecl" fn UPLAY_PARTY_GetInGameMemberList() -> isize {
    0
}

#[forwardable_export]
unsafe extern "cdecl" fn UPLAY_PARTY_Init(flags: usize) -> bool {
    true
}

/// Pulls the party along into the game the local player just entered.
///
/// Together with `UPLAY_PARTY_Init` this is the **only** function of this module Blacklist
/// imports at all - the member lists, `IsInParty`, `PromoteToLeader` and the rest are never
/// called, so their stubs below are harmless.
///
/// It used to return `false` unconditionally, which the game reads as "failed" - the same
/// defect `UPLAY_FRIENDS_InviteToGame` had, where fixing it made invitations work. There is
/// nothing to send here: with genuine Uplay the party members' clients learn of the change
/// through the presence service, and that is exactly what the session announcement in
/// `UPLAY_USER_SetGameSession` now does. So the honest answer to the game is "accepted".
#[forwardable_export]
unsafe extern "cdecl" fn UPLAY_PARTY_InvitePartyToGame(overlapped: *mut UplayOverlapped) -> bool {
    info!("Party is being pulled into the current game");
    if !overlapped.is_null() && overlapped.is_aligned() {
        (*overlapped).set_success();
    }
    true
}

#[forwardable_export]
unsafe extern "cdecl" fn UPLAY_PARTY_InviteToParty(account_id: *const c_char, overlapped: *mut UplayOverlapped) -> bool {
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
unsafe extern "cdecl" fn UPLAY_PARTY_PromoteToLeader(account_id: *const c_char, overlapped: *mut UplayOverlapped) -> bool {
    false
}

#[forwardable_export]
unsafe extern "cdecl" fn UPLAY_PARTY_RespondToGameInvite(invitation_id: *const c_void, accept: bool) -> bool {
    false
}

#[forwardable_export]
unsafe extern "cdecl" fn UPLAY_PARTY_SetGuest() -> isize {
    0
}

#[forwardable_export]
unsafe extern "cdecl" fn UPLAY_PARTY_SetUserData() -> isize {
    0
}

#[forwardable_export]
unsafe extern "cdecl" fn UPLAY_PARTY_ShowGameInviteOverlayUI() -> isize {
    0
}
