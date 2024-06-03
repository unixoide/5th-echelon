use std::ffi::c_char;
use std::ffi::c_int;
use std::ffi::c_void;
use std::ffi::CStr;

use dll_syringe::function::FunctionPtr;
use retour::static_detour;
use tracing::info;
use tracing::instrument;
use windows::core::s;
use windows::Win32::Foundation::FreeLibrary;
use windows::Win32::Networking::WinSock::AF_INET;
use windows::Win32::Networking::WinSock::SOCKADDR;
use windows::Win32::System::LibraryLoader::GetProcAddress;
use windows::Win32::System::LibraryLoader::LoadLibraryA;

use crate::addresses::Addresses;
use crate::config::Config;
use crate::config::Hook;

static_detour! {
    static SomeEventHook: unsafe extern "thiscall" fn(*mut c_void, *mut c_void, *mut c_void) -> *mut c_void;
    static SomeEvent2Hook: unsafe extern "thiscall" fn(*mut c_void, *mut c_void, *mut c_void, *mut c_void, *mut c_void, *mut c_void) -> *mut c_void;
    static SendToHook: unsafe extern "stdcall" fn(usize, *const c_char, c_int, c_int, *const SOCKADDR, c_int) -> c_int;
    static RecvFromHook: unsafe extern "stdcall" fn(usize, *const c_char, c_int, c_int, *const SOCKADDR, *mut c_int) -> c_int;
    static EventMaybeQueuePopHook: unsafe extern "thiscall" fn(usize) -> *const  *const *const c_void;
    static EventHandlerHook: unsafe extern "thiscall" fn(*mut c_void,*mut c_void,*mut c_void,*mut c_void,*mut c_void,*mut c_void) -> usize;
}

fn to_hex_stream(data: &[u8]) -> String {
    data.iter().fold(String::new(), |mut output, b| {
        use std::fmt::Write;
        let _ = write!(output, "{b:02x}");
        output
    })
}

fn deref_addr<'a, T>(addr: *const T) -> Option<&'a T> {
    if !addr.is_aligned() {
        return None;
    }
    unsafe { addr.as_ref() }
}

fn get_storm_event_name<'a>(instance_addr: *const c_void) -> Option<&'a CStr> {
    let vtable_addr = *deref_addr(instance_addr.cast::<*const *const usize>())?;
    let event_type_method_addr = *deref_addr(unsafe { vtable_addr.add(1) })?;
    if event_type_method_addr.is_null() {
        return None;
    }
    let op = unsafe { *event_type_method_addr.cast::<u16>() };
    // 83 D3 cmp r/m32, imm8
    if op != 0x3d83 {
        // info!("no cmp");
        return None;
    }
    let imm = unsafe {
        *event_type_method_addr
            .cast::<u8>()
            .add(2 + std::mem::size_of::<*const u8>())
    };
    if imm != 0 {
        // info!("non zero");
        return None;
    }
    let mut id_addr = *deref_addr(unsafe {
        std::ptr::read_unaligned(
            event_type_method_addr
                .cast::<u16>()
                .add(1)
                .cast::<*const *const c_char>(),
        )
    })?;
    if id_addr.is_null() {
        // info!("id_addr is null");
        // return None;
        let func_ptr: extern "thiscall" fn(*const c_void) -> *const *const c_char =
            unsafe { std::mem::transmute(event_type_method_addr) };
        id_addr = *deref_addr((func_ptr)(instance_addr))?;
    }
    unsafe { Some(CStr::from_ptr(id_addr)) }
}

#[instrument]
fn some_event_hook(this: *mut c_void, a: *mut c_void, b: *mut c_void) -> *mut c_void {
    if let Some(evt_name) = get_storm_event_name(b.cast()) {
        info!("event: {evt_name:?}");
    }
    unsafe { SomeEventHook.call(this, a, b) }
}

#[instrument(skip(this, a, b, c))]
fn some_event2(
    this: *mut c_void,
    a: *mut c_void,
    b: *mut c_void,
    c: *mut c_void,
    d: *mut c_void,
    e: *mut c_void,
) -> *mut c_void {
    if let Some(evt_name) = get_storm_event_name(c.cast()) {
        info!("event2: {evt_name:?}");
    }
    unsafe { SomeEvent2Hook.call(this, a, b, c, d, e) }
}

#[instrument(skip_all)]
fn sendto(
    s: usize,
    buf: *const c_char,
    len: c_int,
    flag: c_int,
    to: *const SOCKADDR,
    tolen: c_int,
) -> c_int {
    if let Some(to_ref) = unsafe { to.as_ref() } {
        let port = 13000u16;
        #[allow(clippy::cast_possible_truncation)]
        if !buf.is_null()
            && to_ref.sa_family == AF_INET
            && to_ref.sa_data[0] == (port >> 8) as i8
            && to_ref.sa_data[1] == port as i8
        {
            #[allow(clippy::cast_sign_loss)]
            let data = unsafe { std::slice::from_raw_parts(buf.cast::<u8>(), len as usize) };
            info!("sendto: {}", to_hex_stream(data));
        }
    }
    unsafe { SendToHook.call(s, buf, len, flag, to, tolen) }
}

#[instrument(skip_all)]
fn recvfrom(
    s: usize,
    buf: *const c_char,
    len: c_int,
    flag: c_int,
    from: *const SOCKADDR,
    fromlen: *mut c_int,
) -> c_int {
    let outlen = unsafe { RecvFromHook.call(s, buf, len, flag, from, fromlen) };
    if let Some(from_ref) = unsafe { from.as_ref() } {
        let port = 13000u16;
        #[allow(clippy::cast_possible_truncation)]
        if !buf.is_null()
            && outlen > 0
            && from_ref.sa_family == AF_INET
            && from_ref.sa_data[0] == (port >> 8) as i8
            && from_ref.sa_data[1] == port as i8
        {
            #[allow(clippy::cast_sign_loss)]
            let data = unsafe { std::slice::from_raw_parts(buf.cast::<u8>(), outlen as usize) };
            info!("recvfrom: {}", to_hex_stream(data));
        }
    }
    outlen
}

#[instrument]
fn event_queue_pop(this: usize) -> *const *const *const c_void {
    let res = unsafe { EventMaybeQueuePopHook.call(this) };
    if !res.is_null() {
        let instance_addr = unsafe { **res.add(1) };
        if let Some(evt_name) = get_storm_event_name(instance_addr) {
            info!("event: {evt_name:?}");
        }
    }
    res
}

#[instrument]
fn event_handler(
    this: *mut c_void,
    param_1: *mut c_void,
    param_2: *mut c_void,
    param_3: *mut c_void,
    param_4: *mut c_void,
    param_5: *mut c_void,
) -> usize {
    if !param_3.is_null() {
        if let Some(evt_name) = get_storm_event_name(param_3.cast()) {
            info!("event: {evt_name:?}");
        }
    }
    unsafe { EventHandlerHook.call(this, param_1, param_2, param_3, param_4, param_5) }
}

pub unsafe fn init_hooks(config: &Config, addr: &Addresses) {
    super::configurable_hook!(config, Hook::StormEventDispatcher, SomeEventHook; addr.func_storm_event_dispatch => some_event_hook);
    super::configurable_hook!(config, Hook::StormEventDispatcher, SomeEvent2Hook; addr.func_storm_event_dispatch2 => some_event2);
    super::configurable_hook!(config, Hook::StormEventDispatcher, EventMaybeQueuePopHook; addr.func_storm_event_maybe_queue_pop => event_queue_pop);
    super::configurable_hook!(config, Hook::StormEventDispatcher, EventHandlerHook; addr.func_storm_event_handler => event_handler);
    if let Ok(lib) = LoadLibraryA(s!("ws2_32.dll")) {
        if let Some(sendto_addr) = GetProcAddress(lib, s!("sendto")) {
            super::configurable_hook!(config, Hook::StormPackets, SendToHook; Some(sendto_addr.as_ptr())  => sendto);
        }
        if let Some(recvfrom_addr) = GetProcAddress(lib, s!("recvfrom")) {
            super::configurable_hook!(config, Hook::StormPackets, RecvFromHook; Some(recvfrom_addr.as_ptr())  => recvfrom);
        }
        let _ = FreeLibrary(lib);
    }
}

pub unsafe fn deinit_hooks(config: &Config) {
    super::disable_configurable_hook!(config, Hook::StormEventDispatcher, SomeEventHook);
    super::disable_configurable_hook!(config, Hook::StormEventDispatcher, SomeEvent2Hook);
    super::disable_configurable_hook!(config, Hook::StormEventDispatcher, EventHandlerHook);
    super::disable_configurable_hook!(config, Hook::StormEventDispatcher, EventMaybeQueuePopHook);
    super::disable_configurable_hook!(config, Hook::StormPackets, SendToHook);
    super::disable_configurable_hook!(config, Hook::StormPackets, RecvFromHook);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_storm_event_name() {
        let name = b"hello world\0";
        let name_addr = name.as_ptr();
        let id_obj = [name_addr];
        let id_addr: [u8; std::mem::size_of::<*const u8>()] =
            unsafe { std::mem::transmute(id_obj.as_ptr()) };
        let mut func = vec![0x83, 0x3d];
        for c in id_addr {
            func.push(c);
        }
        func.push(0);
        let vtable = [std::ptr::null(), func.as_ptr()];
        let instance = [vtable.as_ptr()];
        let evt_name = get_storm_event_name(instance.as_ptr().cast());
        assert!(evt_name.is_some());
        assert!(evt_name.unwrap().to_str().unwrap() == "hello world");
    }
}
