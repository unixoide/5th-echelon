use std::ffi::CString;

use hooks_proc::forwardable_export;
use tracing::error;
use tracing::info;

use super::List;
use super::UplayList;
use super::UplayOverlapped;
use crate::config::get;

#[forwardable_export]
unsafe extern "cdecl" fn UPLAY_USER_ClearGameSession() -> bool {
    // Counterpart to UPLAY_USER_SetGameSession: the player left their session and should no
    // longer show up as joinable in anybody's friend list.
    if let Err(e) = crate::api::set_game_session(None, false, &[]) {
        error!("Withdrawing the announced session failed: {e:?}");
    }
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
unsafe extern "cdecl" fn UPLAY_USER_GetCdKeys(cd_keys_list: *mut *mut List, overlapped: *mut UplayOverlapped) -> bool {
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

#[repr(C)]
struct SessionData {
    unknown1: u32,
    checksum: u32,
    account_id: [u8; 0x80],
    some_data: [u8; 0x164],
    some_data_size: u32,
}

impl std::fmt::Debug for SessionData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SessionData")
            .field("unknown1", &self.unknown1)
            .field("checksum", &self.checksum)
            .field("account_id", &std::ffi::CStr::from_bytes_until_nul(&self.account_id))
            .field("some_data", &&self.some_data[..self.some_data_size as usize])
            .field("some_data_size", &self.some_data_size)
            .finish()
    }
}

#[derive(Debug)]
#[repr(C)]
struct SessionDataWrapper<'a> {
    data: &'a SessionData,
    size: u32,
}

#[forwardable_export(always_call)]
unsafe extern "cdecl" fn UPLAY_USER_SetGameSession(game_session_identifier: *mut (), flags: usize, session_data: &SessionDataWrapper<'_>, invite_only: bool) -> bool {
    if crate::hooks::is_modded() {
        crate::show_msgbox("Fatal error. Game modified", "MOD");
        std::process::exit(1);
    }

    // The game announces here which session it is in - genuine Uplay would pass that on to
    // Ubisoft's presence service, from where friends' clients read it back. Dropping it is
    // what breaks invitations: when the other side accepts, the game looks for the inviter's
    // session in its friend list, finds nothing and gives up with
    // "Failed to find party session for invite."
    //
    // The identifier is not a pointer despite its type - the game passes small numbers that
    // match the session ids of the dedicated server (0x33 = session 51).
    #[allow(clippy::cast_possible_truncation)]
    let session_id = game_session_identifier as usize as u32;

    // The payload has to travel along: the id alone gets an accepted invitation past the
    // lookup, but the other side then opens a session of its own instead of joining. The
    // block is passed through byte for byte - nothing in it needs interpreting, and `size`
    // comes from the game itself (496 bytes).
    let payload = std::slice::from_raw_parts(std::ptr::from_ref(session_data.data).cast::<u8>(), session_data.size as usize);

    if let Err(e) = crate::api::set_game_session(Some(session_id), invite_only, payload) {
        // Not fatal: announcing failed, the session itself is fine. Invitations into it will
        // not work, everything else keeps running.
        error!("Announcing session {session_id} failed: {e:?}");
    } else {
        info!("Session {session_id} announced (invite_only={invite_only})");
    }
    true
}
