use std::ffi::CString;

use hooks_proc::forwardable_export;
use tracing::error;

use super::List;
use super::UplayList;
use super::UplayOverlapped;
use crate::config::get;

#[forwardable_export]
unsafe extern "cdecl" fn UPLAY_USER_ClearGameSession() -> bool {
    true
}

#[forwardable_export]
unsafe extern "cdecl" fn UPLAY_USER_GetAccountId(buffer: *mut u8) -> bool {
    let account_id = match CString::new(cfg.user.account_id.clone()) {
        Ok(account_id) => account_id,
        Err(e) => {
            error!("Couldn't convert account_id: {}!", e);
            return false;
        }
    };
    let account_id = account_id.as_bytes_with_nul();
    buffer.copy_from_nonoverlapping(account_id.as_ptr(), account_id.len());
    true
}

#[forwardable_export]
unsafe extern "cdecl" fn UPLAY_USER_GetCdKeys(
    cd_keys_list: *mut *mut List,
    overlapped: *mut UplayOverlapped,
) -> bool {
    let list = UplayList::CdKeys(cfg.user.cd_keys.clone());
    *cd_keys_list = Box::into_raw(Box::new(list.into()));

    if !overlapped.is_null() {
        (*overlapped).unk = 0;
        (*overlapped).is_completed = 1;
        (*overlapped).result = 0;
    }
    true
}

#[forwardable_export]
unsafe extern "cdecl" fn UPLAY_USER_GetEmail(out_email: *mut i8) -> bool {
    false
}

#[forwardable_export]
unsafe extern "cdecl" fn UPLAY_USER_GetPassword(buffer: *mut u8) -> bool {
    let Some(cfg) = get() else {
        error!("Config not loaded!");
        return false;
    };
    let password = match CString::new(cfg.user.password.clone()) {
        Ok(password) => password,
        Err(e) => {
            error!("Couldn't convert password: {}!", e);
            return false;
        }
    };
    let password = password.as_bytes_with_nul();
    buffer.copy_from_nonoverlapping(password.as_ptr(), password.len());
    true
}

#[forwardable_export]
unsafe extern "cdecl" fn UPLAY_USER_GetUsername(buffer: *mut u8) -> bool {
    let username = match CString::new(cfg.user.username.clone()) {
        Ok(username) => username,
        Err(e) => {
            error!("Couldn't convert username: {}!", e);
            return false;
        }
    };
    let username = username.as_bytes_with_nul();
    buffer.copy_from_nonoverlapping(username.as_ptr(), username.len());
    true
}

#[forwardable_export]
unsafe extern "cdecl" fn UPLAY_USER_IsConnected() -> bool {
    true
}

#[forwardable_export]
unsafe extern "cdecl" fn UPLAY_USER_SetGameSession(
    game_session_identifier: *mut (),
    session_data: *mut (),
    flags: usize,
    invite_only: bool,
) -> bool {
    true
}
