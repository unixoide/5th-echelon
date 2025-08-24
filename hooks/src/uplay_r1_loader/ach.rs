use std::ffi::c_char;
use std::ffi::c_void;

use hooks_proc::forwardable_export;

use super::UplayOverlapped;

#[forwardable_export]
unsafe extern "cdecl" fn UPLAY_ACH_EarnAchievement(achievement_id: *const c_void, overlapped: *mut UplayOverlapped) -> bool {
    false
}

#[forwardable_export]
unsafe extern "cdecl" fn UPLAY_ACH_GetAchievementImage(id: *const c_void, out_image: *mut *mut c_void, overlapped: *mut UplayOverlapped) -> bool {
    false
}

#[forwardable_export]
unsafe extern "cdecl" fn UPLAY_ACH_GetAchievements(
    filter: *const c_void,
    account_id_or_null: *const c_char,
    out_achievement_list: *mut *mut c_void,
    overlapped: *mut UplayOverlapped,
) -> bool {
    false
}

#[forwardable_export]
unsafe extern "cdecl" fn UPLAY_ACH_Write(achievement_id: *const c_void) -> bool {
    false
}
