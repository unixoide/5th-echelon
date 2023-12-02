use hooks_proc::forwardable_export;

#[forwardable_export]
unsafe extern "cdecl" fn UPLAY_OVERLAY_Show() -> isize {
    0
}
