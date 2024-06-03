use std::ffi::c_char;
use std::sync::mpsc;
use std::sync::Mutex;
use std::sync::OnceLock;
use std::time::Duration;

use hooks_proc::forwardable_export;
use tracing::error;
use tracing::info;
use tracing::warn;
use windows::core::s;
use windows::core::PCSTR;
use windows::Win32::Foundation::HMODULE;
use windows::Win32::System::LibraryLoader::GetProcAddress;
use windows::Win32::System::LibraryLoader::LoadLibraryA;
use windows::Win32::UI::WindowsAndMessaging::MessageBoxA;
use windows::Win32::UI::WindowsAndMessaging::MB_OK;

mod ach;
mod avatar;
mod friends;
mod overlay;
mod party;
mod presence;
mod save;
mod types;
mod user;
mod win;

use types::List;
use types::UplayFriend;
use types::UplayList;
use types::UplayOverlapped;
use types::UplaySave;

use self::types::UplayEvent;
use self::types::UplayEventType;

type Result<T> = std::result::Result<T, anyhow::Error>;

fn get_proc(name: PCSTR) -> Option<unsafe extern "system" fn() -> isize> {
    static DLL_HANDLE: OnceLock<windows::core::Result<HMODULE>> = OnceLock::new();
    let handle = DLL_HANDLE
        .get_or_init(|| unsafe { LoadLibraryA(s!("uplay_r1_loader.orig.dll")) })
        .as_ref()
        .map_err(|e| unsafe {
            error!("Library loading error: {e:?}");
            let mut s = format!("{e:?}");
            let v = s.as_mut_vec();
            v.push(b'\0');
            MessageBoxA(None, PCSTR(v.as_ptr()), s!("Error"), MB_OK);
            e
        })
        .map(|l| {
            info!("Library loaded");
            l
        })
        .unwrap();
    unsafe { GetProcAddress(*handle, name) }
}

#[forwardable_export]
unsafe extern "cdecl" fn UPLAY_GetLastError(out_error_string: *mut *const c_char) -> bool {
    false
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Event {
    UserAccountSharing,
    FriendsGameInviteAccepted(String),
    PartyGameInviteAccepted(String),
}

pub static EVENTS: OnceLock<Mutex<mpsc::Receiver<Event>>> = OnceLock::new();

unsafe fn into_friend_invite_accepted(event: *mut UplayEvent, ubi_name: String) {
    // TODO user better names

    #[repr(C)]
    struct Bar {
        unknown: usize,
        unknown1: usize,
        username: [u8; 128],
    }

    #[repr(C)]
    struct Foo {
        unknown: [usize; 2],
        bar: *const Bar,
        value496: usize,
    }

    #[repr(C)]
    struct FriendAccepted {
        foo: *const Foo,
    }

    static mut BAR: Bar = Bar {
        unknown: 1,
        unknown1: 0,
        username: [0u8; 128],
    };
    static mut FOO: Foo = Foo {
        unknown: [0usize; 2],
        bar: unsafe { std::ptr::addr_of!(BAR) },
        value496: 496,
    };

    static mut FRIEND_ACCEPTED: FriendAccepted = FriendAccepted {
        foo: unsafe { std::ptr::addr_of!(FOO) },
    };

    (*event).event_type = UplayEventType::FriendsGameInviteAccepted;
    let mut ub = ubi_name.into_bytes();
    ub.push(0);
    let l = if ub.len() > BAR.username.len() {
        BAR.username.len()
    } else {
        ub.len()
    };
    BAR.username[..l].copy_from_slice(&ub[..l]);
    (*event).unknown = std::ptr::addr_of!(FRIEND_ACCEPTED) as usize;
}

unsafe fn into_party_invite_accepted(event: *mut UplayEvent, ubi_name: String) {
    // TODO user better names

    #[repr(C)]
    struct PartyAccepted {
        unknown: usize,
        username: *const Foo,
    }

    #[repr(C)]
    struct Foo {
        unknown: [usize; 2],
        username: *const [u8; 128],
        length: usize,
    }

    static mut BAR: [u8; 128] = [0u8; 128];
    static mut FOO: Foo = Foo {
        unknown: [0usize; 2],
        username: unsafe { std::ptr::addr_of!(BAR) },
        length: 0usize,
    };

    static mut PARTY_ACCEPTED: PartyAccepted = PartyAccepted {
        unknown: 0,
        username: unsafe { std::ptr::addr_of!(FOO) },
    };

    (*event).event_type = UplayEventType::PartyGameInviteAccepted;
    let mut ub = ubi_name.into_bytes();
    ub.push(0);
    let l = if ub.len() > BAR.len() {
        BAR.len()
    } else {
        ub.len()
    };
    BAR[..l].copy_from_slice(&ub[..l]);
    BAR[l] = 0;
    FOO.length = l + 1;
    warn!(
        "&PARTY_ACCEPTED = {:?} &FOO = {:?} &BAR = {:?}",
        std::ptr::addr_of!(PARTY_ACCEPTED),
        std::ptr::addr_of!(FOO),
        std::ptr::addr_of!(BAR)
    );
    (*event).unknown = std::ptr::addr_of!(PARTY_ACCEPTED) as usize;
}

#[forwardable_export(log = false)]
unsafe extern "cdecl" fn UPLAY_GetNextEvent(event: *mut UplayEvent) -> bool {
    if event.is_null() {
        return false;
    }

    if let Some(evt) = EVENTS
        .get()
        .map(Mutex::lock)
        .map(std::result::Result::unwrap)
        .as_deref()
        .map(mpsc::Receiver::try_recv)
        .and_then(std::result::Result::ok)
    {
        info!("New event {evt:?}");
        match evt {
            Event::UserAccountSharing => {
                (*event).event_type = UplayEventType::UserAccountSharing;
            }
            Event::FriendsGameInviteAccepted(user) => into_friend_invite_accepted(event, user),
            Event::PartyGameInviteAccepted(user) => into_party_invite_accepted(event, user),
        }
        // (*event).event_type =
        true
    } else {
        false
    }
}

#[forwardable_export]
unsafe extern "cdecl" fn UPLAY_GetOverlappedOperationResult_(
    overlapped: *mut UplayOverlapped,
    result: *mut usize,
) -> bool {
    let overlapped = unsafe { overlapped.as_ref() };
    let result = unsafe { result.as_mut() };
    if let Some((overlapped, result)) = overlapped.filter(|o| o.is_completed != 0).zip(result) {
        *result = overlapped.result;
        true
    } else {
        false
    }
}

#[forwardable_export]
unsafe extern "cdecl" fn UPLAY_GetOverlappedOperationResult(
    overlapped: *mut UplayOverlapped,
    result: *mut usize,
) -> bool {
    let overlapped = unsafe { overlapped.as_ref() };
    let result = unsafe { result.as_mut() };
    if let Some((overlapped, result)) = overlapped.filter(|o| o.is_completed != 0).zip(result) {
        *result = overlapped.result;
        true
    } else {
        false
    }
}

#[forwardable_export]
unsafe extern "cdecl" fn UPLAY_HasOverlappedOperationCompleted(
    overlapped: *mut UplayOverlapped,
) -> bool {
    let overlapped = unsafe { overlapped.as_ref() };
    overlapped.is_some_and(|o| o.is_completed != 0)
}

#[forwardable_export]
unsafe extern "cdecl" fn UPLAY_Quit() -> bool {
    false
}

#[forwardable_export]
unsafe extern "cdecl" fn UPLAY_Release(ptr: *mut List) -> bool {
    if !ptr.is_null() {
        let list = *Box::from_raw(ptr);
        let list: UplayList = list.try_into().unwrap();
        drop(list);
    }
    true
}

#[forwardable_export]
unsafe extern "cdecl" fn UPLAY_Startup(
    uplay_id: usize,
    game_version: usize,
    language_country_code_utf8: *const c_char,
) -> isize {
    if let Some(config) = crate::config::get() {
        if config.enable_overlay {
            let (tx, rx) = crossbeam_channel::unbounded();
            // needs to be done in a separate thread, otherwise it'll not work
            std::thread::Builder::new()
                .name(String::from("overlay-thread"))
                .spawn(|| {
                    // TODO: blocks game if running in fullscreen mode. Waiting 10s seems to do the trick
                    std::thread::sleep(std::time::Duration::from_secs(10));
                    if let Err(err) = crate::overlay::init(crate::overlay::Engine::DX9, rx) {
                        error!("Couldn't initialize overlay: {err}");
                    }
                })
                .unwrap();

            std::thread::Builder::new()
                .name(String::from("updates-thread"))
                .spawn(move || {
                    crate::api::runtime().unwrap().block_on(async {
                        let mut failures = 0;
                        loop {
                            tokio::time::sleep(Duration::from_secs(1)).await;
                            let event: Option<std::result::Result<_, _>> = crate::api::event()
                                .await
                                .map(|resp| resp.invite)
                                .transpose();
                            if let Some(invite) = event {
                                if invite.is_err() {
                                    failures += 1;
                                } else {
                                    failures = 0;
                                }
                                tx.send(invite).unwrap();
                                if failures > 0 && failures % 10 == 0 {
                                    crate::api::relogin().await;
                                }
                            }
                        }
                    });
                })
                .unwrap();
        }
    }
    0 // 0 = all good, 1 = error occured, 2 = ??? (), 3 = ??? (potentially offline mode)
}

#[forwardable_export(log = false)]
unsafe extern "cdecl" fn UPLAY_Update() -> bool {
    true
}
