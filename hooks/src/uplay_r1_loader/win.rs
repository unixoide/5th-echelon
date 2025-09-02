use std::ffi::c_void;

use hooks_proc::forwardable_export;

use super::UplayOverlapped;

#[forwardable_export]
unsafe extern "cdecl" fn UPLAY_WIN_GetRewards(out_reward_list: *mut *const c_void, overlapped: *mut UplayOverlapped) -> bool {
    false
}
