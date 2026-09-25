//! Network Location Awareness (NLA) emulation for Wine/Proton.
//!
//! The game runs an "NLAThread" that enumerates networks via
//! `WSALookupServiceBeginA`/`WSALookupServiceNextA` with the `NS_NLA` namespace.
//! Right after logging in it checks whether that thread found at least one
//! network and logs out ("service not available") if it didn't.
//! Wine only stubs `WSALookupServiceBeginA`, so under Wine the game always
//! thinks it's offline. When running under Wine, this answers NLA queries
//! with a single connected network that has internet access. Everything
//! else is forwarded to the real implementation.

use std::ffi::c_void;

use retour::static_detour;
use tracing::info;
use windows::core::s;
use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use windows::Win32::System::LibraryLoader::GetProcAddress;
use windows::Win32::System::LibraryLoader::LoadLibraryA;

const NS_NLA: u32 = 15;
const NLA_CONNECTIVITY: u32 = 3;
const NLA_NETWORK_UNMANAGED: u32 = 1;
const NLA_INTERNET_YES: u32 = 2;
const WSA_E_NO_MORE: i32 = 10110;
const WSAEFAULT: i32 = 10014;
const SOCKET_ERROR: i32 = -1;
const FAKE_HANDLE: usize = 0x4e4c_4100; // "NLA"

/// `WSAQUERYSETA` on 32-bit Windows.
#[repr(C)]
struct WsaQuerySetA {
    dw_size: u32,
    lpsz_service_instance_name: *const u8,
    lp_service_class_id: *const c_void,
    lp_version: *const c_void,
    lpsz_comment: *const u8,
    dw_name_space: u32,
    lp_ns_provider_id: *const c_void,
    lpsz_context: *const u8,
    dw_number_of_protocols: u32,
    lpafp_protocols: *const c_void,
    lpsz_query_string: *const u8,
    dw_number_of_cs_addrs: u32,
    lpcsa_buffer: *const c_void,
    dw_output_flags: u32,
    lp_blob: *const Blob,
}

#[repr(C)]
struct Blob {
    cb_size: u32,
    p_blob_data: *const u8,
}

/// `NLA_BLOB` header followed by the `connectivity` member of its data union.
#[repr(C)]
struct NlaConnectivityBlob {
    ty: u32,
    dw_size: u32,
    next_offset: u32,
    connectivity_type: u32,
    internet: u32,
}

#[repr(C)]
struct FakeResult {
    set: WsaQuerySetA,
    blob: Blob,
    nla: NlaConnectivityBlob,
    name: [u8; 24],
}

static_detour! {
    static BeginHook: unsafe extern "stdcall" fn(*const WsaQuerySetA, u32, *mut usize) -> i32;
    static NextHook: unsafe extern "stdcall" fn(usize, u32, *mut u32, *mut WsaQuerySetA) -> i32;
    static EndHook: unsafe extern "stdcall" fn(usize) -> i32;
}

// Whether the next WSALookupServiceNextA on the fake handle still has a network to return.
static PENDING: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

fn set_last_error(code: i32) {
    unsafe { windows::Win32::Networking::WinSock::WSASetLastError(code) };
}

fn begin(restrictions: *const WsaQuerySetA, flags: u32, handle: *mut usize) -> i32 {
    let is_nla = unsafe { restrictions.as_ref() }.is_some_and(|r| r.dw_name_space == NS_NLA);
    if !is_nla || handle.is_null() {
        return unsafe { BeginHook.call(restrictions, flags, handle) };
    }
    unsafe { *handle = FAKE_HANDLE };
    PENDING.store(true, std::sync::atomic::Ordering::SeqCst);
    0
}

fn next(handle: usize, flags: u32, buffer_len: *mut u32, results: *mut WsaQuerySetA) -> i32 {
    if handle != FAKE_HANDLE {
        return unsafe { NextHook.call(handle, flags, buffer_len, results) };
    }
    if !PENDING.swap(false, std::sync::atomic::Ordering::SeqCst) {
        set_last_error(WSA_E_NO_MORE);
        return SOCKET_ERROR;
    }
    let needed = std::mem::size_of::<FakeResult>() as u32;
    let Some(len) = (unsafe { buffer_len.as_mut() }) else {
        set_last_error(WSAEFAULT);
        return SOCKET_ERROR;
    };
    if results.is_null() || *len < needed {
        *len = needed;
        PENDING.store(true, std::sync::atomic::Ordering::SeqCst);
        set_last_error(WSAEFAULT);
        return SOCKET_ERROR;
    }
    unsafe {
        let out = results.cast::<FakeResult>();
        std::ptr::write_bytes(out.cast::<u8>(), 0, needed as usize);
        let out = &mut *out;
        out.name[..19].copy_from_slice(b"5th Echelon Network");
        out.nla = NlaConnectivityBlob {
            ty: NLA_CONNECTIVITY,
            dw_size: std::mem::size_of::<NlaConnectivityBlob>() as u32,
            next_offset: 0,
            connectivity_type: NLA_NETWORK_UNMANAGED,
            internet: NLA_INTERNET_YES,
        };
        out.blob = Blob {
            cb_size: std::mem::size_of::<NlaConnectivityBlob>() as u32,
            p_blob_data: std::ptr::from_ref(&out.nla).cast(),
        };
        out.set.dw_size = std::mem::size_of::<WsaQuerySetA>() as u32;
        out.set.lpsz_service_instance_name = out.name.as_ptr();
        out.set.dw_name_space = NS_NLA;
        out.set.lp_blob = std::ptr::from_ref(&out.blob);
    }
    info!("NLA emulation: reported one connected network with internet access");
    0
}

fn end(handle: usize) -> i32 {
    if handle == FAKE_HANDLE {
        PENDING.store(false, std::sync::atomic::Ordering::SeqCst);
        return 0;
    }
    unsafe { EndHook.call(handle) }
}

fn running_under_wine() -> bool {
    unsafe { GetModuleHandleA(s!("ntdll.dll")).is_ok_and(|ntdll| GetProcAddress(ntdll, s!("wine_get_version")).is_some()) }
}

pub unsafe fn init_hooks() {
    if !running_under_wine() {
        return;
    }
    info!("Wine detected, enabling NLA emulation");
    let lib = LoadLibraryA(s!("ws2_32.dll")).unwrap();
    super::hook!(BeginHook, GetProcAddress(lib, s!("WSALookupServiceBeginA")), begin);
    super::hook!(NextHook, GetProcAddress(lib, s!("WSALookupServiceNextA")), next);
    super::hook!(EndHook, GetProcAddress(lib, s!("WSALookupServiceEnd")), end);
}
