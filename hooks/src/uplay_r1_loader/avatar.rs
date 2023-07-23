use std::ffi::c_char;
use std::ffi::c_void;

use hooks_proc::forwardable_export;

use super::UplayOverlapped;

#[forwardable_export]
unsafe extern "cdecl" fn UPLAY_AVATAR_Get(
    account_id: *const c_char,
    avatar_size: *mut u32,
    out_rgba: *mut *mut c_void,
    overlapped: *mut UplayOverlapped,
) -> bool {
    false
}
