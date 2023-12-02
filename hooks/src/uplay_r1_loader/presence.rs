use hooks_proc::forwardable_export;

#[forwardable_export]
unsafe extern "cdecl" fn UPLAY_PRESENCE_SetRichPresenceLine() -> isize {
    0
}

#[forwardable_export]
unsafe extern "cdecl" fn UPLAY_PRESENCE_SetRichPresencePropertyInt32() -> isize {
    0
}

#[forwardable_export]
unsafe extern "cdecl" fn UPLAY_PRESENCE_SetRichPresencePropertyString() -> isize {
    0
}
