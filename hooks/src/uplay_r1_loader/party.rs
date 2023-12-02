use std::ffi::c_char;
use std::ffi::c_void;

use hooks_proc::forwardable_export;

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
