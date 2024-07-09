use std::ffi::c_char;
use std::ffi::c_void;
use std::ffi::CStr;
use std::ffi::CString;
use std::ffi::OsString;
use std::io::Write;
use std::os::windows::ffi::OsStrExt;

use retour::static_detour;
use tracing::debug;
use tracing::error;
use tracing::info;
use tracing::instrument;
use tracing::warn;
use windows::core::s;
use windows::core::PCWSTR;
use windows::Win32::Foundation::ERROR_BUFFER_OVERFLOW;
use windows::Win32::Foundation::HANDLE;
use windows::Win32::NetworkManagement::IpHelper::IP_ADAPTER_INFO;
use windows::Win32::Networking::WinSock::gethostname;
use windows::Win32::Networking::WinSock::HOSTENT;
use windows::Win32::System::LibraryLoader::GetProcAddress;
use windows::Win32::System::LibraryLoader::LoadLibraryA;
use windows::Win32::System::Threading::GetThreadId;
use windows::Win32::System::Threading::SetThreadDescription;

use crate::addresses::Addresses;
use crate::config;
use crate::config::Config;
use crate::config::Hook;
use crate::hooks::utils::SomeOrQuestionmark;

mod datatypes;
mod quazal;
mod storm;
mod utils;

use self::datatypes::GearBasicString;
use self::datatypes::MaybeGoal;
use self::datatypes::NetFiniteState;
use self::datatypes::NetFiniteStateID;
use self::datatypes::NetFiniteStateMachine;
use self::datatypes::QuazalStep;

#[repr(C)]
struct SomeStormAddrType {
    vtable: *const c_void,
    addr: u32,
    port: u16,
}

static_detour! {
    static PrinterHook: unsafe extern "thiscall" fn(*mut c_void, *const i8) -> *mut c_void;
    static LeaveStateHook: unsafe extern "thiscall" fn(*mut NetFiniteState, *mut c_void);
    static NextStateHook: unsafe extern "thiscall" fn(*mut NetFiniteStateMachine, *mut c_void, usize);
    static NetResultBaseHook: unsafe extern "thiscall" fn(*mut c_void, *mut GearBasicString);
    static SomethingWithGoalHook: unsafe extern "thiscall" fn(*mut *mut c_void, usize, *mut MaybeGoal, usize, usize);
    static QuazalStepSequenceJobSetStateHook: unsafe extern "thiscall" fn(*mut c_void, *mut QuazalStep);
    static ThreadStarterHook: unsafe extern "thiscall" fn(*mut c_void);
    static ChangeStateHook: unsafe extern "thiscall" fn(*mut MaybeGoal, *mut NetFiniteState, *mut NetFiniteStateID);
    static NetCoreHook: unsafe extern "thiscall" fn(*mut c_void) -> *mut c_void;
    static NetResultCoreHook: unsafe extern "thiscall" fn(*mut c_void, usize, *mut GearBasicString) -> *mut c_void;
    static NetResultSessionHook: unsafe extern "thiscall" fn(*mut c_void, usize, *mut GearBasicString) -> *mut c_void;
    static NetResultRdvSessionHook: unsafe extern "thiscall" fn(*mut c_void, usize, *mut GearBasicString) -> *mut c_void;
    static NetResultLobbyHook: unsafe extern "thiscall" fn(*mut c_void, usize, *mut GearBasicString) -> *mut c_void;
}

#[repr(C)]
struct StormStateMachine {
    vtable: *const c_void,
    // and more
}

#[repr(C)]
struct StormStateMachineAction {
    vtable: *const c_void,
    callback: *const c_void,
    offset: *const c_void,
    state_machine: *const StormStateMachine,
}

#[repr(C)]
struct StormObject {
    name: *const c_char,
    // and more
}

#[repr(C)]
struct StormEventVtable {
    maybe_destructor: *const c_void,
    global_event: extern "thiscall" fn(*const StormEvent) -> &'static StormObject,
    // and more
}

#[repr(C)]
struct StormEvent {
    vtable: &'static StormEventVtable,
    // and more
}

static_detour! {
    static GetAdaptersInfoHook: unsafe extern "stdcall" fn(*mut IP_ADAPTER_INFO, *mut u32) -> u32;
    static GethostbynameHook: unsafe extern "stdcall" fn(*const c_char) -> *mut HOSTENT;
    static GenerateIDHook: unsafe extern "thiscall" fn(*mut NetFiniteStateID, *const i8, bool, *mut c_void);
    static StormSetStateHook: unsafe extern "thiscall" fn(*mut StormStateMachine, *mut NetFiniteStateID);
    static StormStateMachineActionExecuteHook: unsafe extern "thiscall" fn(*mut StormStateMachineAction, *mut  *mut StormEvent, *mut StormEvent);
    static StormErrorFormatter: unsafe extern "thiscall" fn(*mut c_void, *mut GearBasicString) -> *mut GearBasicString;
    static GearStrDestructor: unsafe extern "thiscall" fn(*mut GearBasicString, *mut c_void);
    static AnotherGearStrDestructorHook: unsafe extern "thiscall" fn(*mut GearBasicString, *mut c_void);
    static SomeGearStrConstructor: unsafe extern "thiscall" fn(*mut GearBasicString, *mut c_char) -> *mut GearBasicString;
    static StormHostPortToStringHook: unsafe extern "thiscall" fn(*mut SomeStormAddrType, *mut c_void, *mut c_void) -> *mut c_void;
}

#[instrument(skip_all)]
fn printer(x: *mut c_void, msg: *const i8) -> *mut c_void {
    // log!("printer called: {:08x} {:08x}", x as usize, msg as usize);
    if !msg.is_null() {
        let msg = unsafe { CStr::from_ptr(msg) };
        if !msg.is_empty() {
            info!("{}", msg.to_string_lossy());
        }
    }
    unsafe { PrinterHook.call(x, msg) }
}

#[instrument(skip_all)]
unsafe fn get_state_name(x: *mut c_void) -> Option<&'static CStr> {
    let this: *const *const *const u8 = x.cast::<*const *const u8>().cast_const();
    if this.is_null() {
        return None;
    }
    let vtable = *this;
    let get_state_name = *vtable.add(13);
    if get_state_name.is_null() || !get_state_name.is_aligned() {
        return None;
    }
    let opcode = std::slice::from_raw_parts(get_state_name, 5);
    if opcode[0] != 0xb8 {
        return None;
    }
    let mut tmp = [0u8; 4];
    tmp.copy_from_slice(&opcode[1..]);
    let addr = usize::from_le_bytes(tmp);
    if 0 == addr {
        None
    } else {
        Some(CStr::from_ptr(addr as *const i8))
    }
}

#[instrument(skip_all)]
fn leave_state(x: *mut NetFiniteState, y: *mut c_void) {
    info!("Leaving state {}", utils::state_ptr_to_name(x));
    unsafe { LeaveStateHook.call(x, y) }
}

#[instrument(skip_all)]
fn next_state(sm: *mut NetFiniteStateMachine, y: *mut c_void, z: usize) {
    let id = {
        let state_info = y as *const usize;
        if state_info.is_null() {
            None
        } else {
            let id = unsafe { *state_info };
            Some(id)
        }
    };
    let map = utils::hashes();
    if !sm.is_null() {
        let sm_name = unsafe { (*sm).get_statemachine_name() };
        let vtable = unsafe { (*sm).vtable };
        let current_state = unsafe { (*sm).current_state };
        let last_state = unsafe { (*sm).last_state };
        if let Some((_id, name)) = id.and_then(|id| Some((id, map.get(&id)?))) {
            info!(
                "Next state: {name} StateMachine(inst={sm:?}, vtable={vtable:?}, name={sm_name}) current={} last={}",
                utils::state_ptr_to_name(current_state),
                utils::state_ptr_to_name(last_state)
            );
        } else {
            info!(
                "Next state: StateMachine(inst={sm:?}, vtable={vtable:?}, name={sm_name}) current={} last={}",
                utils::state_ptr_to_name(current_state),
                utils::state_ptr_to_name(last_state)
            );
        }
    }
    unsafe { NextStateHook.call(sm, y, z) }
}

#[instrument(skip_all)]
fn net_result_base(this: *mut c_void, string: *mut GearBasicString) {
    unsafe {
        if !string.is_null() && string.is_aligned() && !(*string).internal.is_null() {
            let string2 = &mut *string;
            let internal = &mut *string2.internal;
            let cstr = internal.as_str();
            info!("NetResultBase: text={cstr}");
        }

        NetResultBaseHook.call(this, string);
    }
}

#[instrument(skip_all)]
fn something_with_goal(
    this: *mut *mut c_void,
    a1: usize,
    mg: *mut MaybeGoal,
    a3: usize,
    a4: usize,
) {
    unsafe {
        if !mg.is_null() {
            let goal = (*mg).name();
            info!(
                "goal: {:?} (this: {:?} -> {:?})",
                goal, //.to_string_lossy(),
                this,
                if this.is_null() {
                    std::ptr::null()
                } else {
                    *this
                }
            );
        }
        SomethingWithGoalHook.call(this, a1, mg, a3, a4);
    }
}

#[instrument(skip_all)]
fn quazal_stepsequencejob_setstep(step_sequence_job: *mut c_void, step: *mut QuazalStep) {
    unsafe {
        if !step.is_null() && step.is_aligned() && !(*step).description.is_null() {
            let desc = CStr::from_ptr((*step).description);
            info!(
                "Next job step: {} (callback: {:?})",
                desc.to_string_lossy(),
                (*step).callback
            );
        }

        QuazalStepSequenceJobSetStateHook.call(step_sequence_job, step);
    }
}

#[instrument]
fn set_thread_name(worker: *mut c_void) {
    unsafe {
        let name = worker.cast::<i8>().offset(0x18).cast_const();
        let cstr = CStr::from_ptr(name);
        info!("new worker: {:?}", cstr);
        ThreadStarterHook.call(worker);
        let thread_handle = HANDLE(*worker.cast::<isize>().offset(1));
        let ostr = OsString::from(cstr.to_string_lossy().into_owned());
        let mut widename: Vec<_> = ostr.encode_wide().collect();
        widename.push(0);

        debug!(
            "Thread handle: {:?} Thread id: {}",
            thread_handle,
            GetThreadId(thread_handle)
        );

        let _ = SetThreadDescription(thread_handle, PCWSTR::from_raw(widename.as_ptr()));
    }
}

#[instrument(skip_all)]
fn change_state(
    goal_ptr: *mut MaybeGoal,
    state_ptr: *mut NetFiniteState,
    next_state_ptr: *mut NetFiniteStateID,
) {
    unsafe {
        if let Some(((goal, state), next_state)) = goal_ptr
            .as_ref()
            .zip(state_ptr.as_ref())
            .zip(next_state_ptr.as_ref())
        {
            info!(
                "Goal {:?} with {}, state={}(id={:x}), next_state_id={}",
                goal.name(),
                goal.unknown,
                state.get_state_name(),
                state.get_state_id(),
                utils::id_to_name(next_state.id as usize),
            );
        }
        ChangeStateHook.call(goal_ptr, state_ptr, next_state_ptr);
    }
}

#[instrument(skip_all)]
fn net_core(inst: *mut c_void) -> *mut c_void {
    let inst = unsafe { NetCoreHook.call(inst) };
    unsafe {
        let lanmode = inst.cast::<u8>().offset(0x5c8);
        assert_eq!(*lanmode, 0);
        *lanmode = 1;
    };
    inst
}

fn net_result_core(this: *mut c_void, code: usize, text: *mut GearBasicString) -> *mut c_void {
    unsafe {
        if !text.is_null() && text.is_aligned() && !(*text).internal.is_null() {
            let string2 = &mut *text;
            let internal = &mut *string2.internal;
            let cstr = internal.as_str();
            info!("NetResultCore: code={code:x} text={cstr}");
        } else {
            info!("NetResultCore: code={code:x} text=NULL");
        }
    }
    unsafe { NetResultCoreHook.call(this, code, text) }
}

fn net_result_session(this: *mut c_void, code: usize, text: *mut GearBasicString) -> *mut c_void {
    unsafe {
        if !text.is_null() && text.is_aligned() && !(*text).internal.is_null() {
            let string2 = &mut *text;
            let internal = &mut *string2.internal;
            let cstr = internal.as_str();
            info!("NetResultSession: code={code:x} text={cstr}");
        } else {
            info!("NetResultSession: code={code:x} text=NULL");
        }
    }
    unsafe { NetResultSessionHook.call(this, code, text) }
}

fn net_result_rdv_session(
    this: *mut c_void,
    code: usize,
    text: *mut GearBasicString,
) -> *mut c_void {
    unsafe {
        if !text.is_null() && text.is_aligned() && !(*text).internal.is_null() {
            let string2 = &mut *text;
            let internal = &mut *string2.internal;
            let cstr = internal.as_str();
            info!("NetResultRdvSession: code={code:x} text={cstr}");
        } else {
            info!("NetResultRdvSession: code={code:x} text=NULL");
        }
    }
    unsafe { NetResultRdvSessionHook.call(this, code, text) }
}

fn net_result_lobby(this: *mut c_void, code: usize, text: *mut GearBasicString) -> *mut c_void {
    unsafe {
        if !text.is_null() && text.is_aligned() && !(*text).internal.is_null() {
            let string2 = &mut *text;
            let internal = &mut *string2.internal;
            let cstr = internal.as_str();
            info!("NetResultLobby: code={code:x} text={cstr}");
        } else {
            info!("NetResultLobby: code={code:x} text=NULL");
        }
    }
    unsafe { NetResultLobbyHook.call(this, code, text) }
}

#[instrument(skip_all)]
fn storm_host_port_to_str(
    this: *mut SomeStormAddrType,
    x: *mut c_void,
    y: *mut c_void,
) -> *mut c_void {
    unsafe {
        let host = (*this).addr;
        let port = (*this).port;
        info!(
            "Storm uses {}.{}.{}.{}:{}",
            host & 0xff,
            (host >> 8) & 0xff,
            (host >> 16) & 0xff,
            (host >> 24) & 0xff,
            (port >> 8) | (port << 8),
        );
        StormHostPortToStringHook.call(this, x, y)
    }
}

#[instrument(skip_all)]
fn get_adapters_info(adapter_info: *mut IP_ADAPTER_INFO, sizepointer: *mut u32) -> u32 {
    let res = unsafe { GetAdaptersInfoHook.call(adapter_info, sizepointer) };

    if res == ERROR_BUFFER_OVERFLOW.0 {
        return res;
    }

    let cfg = config::get().unwrap();

    if cfg.networking.ip_address.is_none() {
        return res;
    }

    let adapter_ip = cfg.networking.ip_address.unwrap().to_string();
    let target = CString::new(adapter_ip.as_bytes()).unwrap();

    unsafe {
        let mut adapter = adapter_info;

        while !adapter.is_null() {
            let data = &*(std::ptr::from_ref::<[i8]>(&(*adapter).IpAddressList.IpAddress.String)
                as *const [u8]);
            let addr = CStr::from_bytes_until_nul(data).unwrap();
            debug!("{addr:?} == {target:?} ?");
            if addr == target.as_ref() {
                break;
            }
            adapter = (*adapter).Next;
        }

        if adapter.is_null() {
            error!("Adapter with IP {adapter_ip} not found");
            return res;
        }

        (*adapter).Next = std::ptr::null_mut();

        if adapter != adapter_info {
            if adapter.is_aligned() {
                std::ptr::copy(adapter, adapter_info, 1);
            } else {
                warn!(
                "adapter structs are unaligned. {:?} should align to {}. Trying to copy from {:?} as u8",
                adapter,
                std::mem::align_of::<IP_ADAPTER_INFO>(),
                adapter_info,
            );
                let dst = std::slice::from_raw_parts_mut(
                    adapter_info.cast::<u8>(),
                    std::mem::size_of::<IP_ADAPTER_INFO>(),
                );
                let src = std::slice::from_raw_parts(
                    adapter.cast::<u8>(),
                    std::mem::size_of::<IP_ADAPTER_INFO>(),
                );
                dst.copy_from_slice(src);
            }
        }

        let data = &*(std::ptr::from_ref::<[i8]>(&(*adapter_info).IpAddressList.IpAddress.String)
            as *const [u8]);
        debug!("{:?}", CStr::from_bytes_until_nul(data).unwrap());
    }

    res
}

#[instrument(skip_all)]
fn gethostbyname(name: *const c_char) -> *mut HOSTENT {
    let given_host = unsafe { CStr::from_ptr(name) };
    info!("called with {:?}", given_host);
    let ent = unsafe { GethostbynameHook.call(name) };

    let mut hostname = vec![0u8; 1024];
    if unsafe { gethostname(&mut hostname) } != 0 {
        error!("error calling gethostname");
        return ent;
    }
    let hostname = CStr::from_bytes_until_nul(&hostname).unwrap();
    if hostname != given_host {
        warn!("given host doesn't match {:?}", hostname);
        return ent;
    }
    let cfg = config::get().unwrap();

    if cfg.networking.ip_address.is_none() {
        return ent;
    }

    let target = cfg.networking.ip_address.unwrap();

    unsafe {
        let mut addr_list = (*ent).h_addr_list;
        let found = loop {
            let addr = *addr_list;
            if addr.is_null() {
                break None;
            }
            let mut tmp = [0u8; 4];
            addr.copy_to(tmp.as_mut_ptr().cast(), tmp.len());
            let ip_addr = std::net::Ipv4Addr::new(tmp[0], tmp[1], tmp[2], tmp[3]);
            let found = ip_addr == target;
            debug!("{ip_addr:?} == {target:?} ? {}", found);
            if found {
                break Some(addr);
            }
            addr_list = addr_list.add(1);
        };

        if let Some(addr) = found {
            *(*ent).h_addr_list = addr;
            *(*ent).h_addr_list.add(1) = std::ptr::null_mut();
        }
    }
    ent
}

fn generate_id(
    this: *mut NetFiniteStateID,
    name_ptr: *const i8,
    insensitive: bool,
    b: *mut c_void,
) {
    let name = unsafe {
        if name_ptr.is_null() {
            None
        } else {
            Some(CStr::from_ptr(name_ptr))
        }
    };
    unsafe { GenerateIDHook.call(this, name_ptr, insensitive, b) };

    if let Some(this) = unsafe { this.as_ref() } {
        if let Some(name) = name.map(CStr::to_str).and_then(Result::ok) {
            static CREATED: std::sync::atomic::AtomicBool =
                std::sync::atomic::AtomicBool::new(false);
            let mut path = std::env::current_exe().unwrap();
            path.set_file_name("maphashes.txt");
            let file = if CREATED.swap(true, std::sync::atomic::Ordering::AcqRel) {
                std::fs::File::options().append(true).open(path)
            } else {
                std::fs::File::create(path)
            };
            let name = if insensitive {
                name.to_lowercase()
            } else {
                name.into()
            };
            if let Ok(mut f) = file {
                let _ = writeln!(f, "{name}\t{:x}", this.id);
                let _ = f.flush();
            }
        }
    }
}

#[instrument(skip_all)]
fn storm_set_state(this: *mut StormStateMachine, state_id: *mut NetFiniteStateID) {
    let vtable = unsafe { this.as_ref() }.map(|this| this.vtable);
    let state_name = unsafe { state_id.as_ref() }.map(datatypes::NetFiniteStateID::name);
    info!(
        "Setting next storm state: {} (StateMachine(vtable={:?}))",
        SomeOrQuestionmark(state_name),
        SomeOrQuestionmark(vtable),
    );
    unsafe { StormSetStateHook.call(this, state_id) }
}

#[instrument]
fn storm_statemachineaction_execute(
    this: *mut StormStateMachineAction,
    unknown1: *mut *mut StormEvent,
    unknown2: *mut StormEvent,
) {
    let (transition, vtable) = unsafe { this.as_ref() }
        .map(|this| {
            (
                this.callback,
                unsafe { this.state_machine.as_ref() }.map(|sm| sm.vtable),
            )
        })
        .unzip();
    let event = unsafe {
        unknown2
            .as_ref()
            .map(|evt| (evt.vtable.global_event)(evt).name)
            .filter(|n| !n.is_null())
            .map(|n| CStr::from_ptr(n))
    };

    info!(
        "Executing transition: {:?} (StateMachine(vtable={:?})) event={:?}",
        SomeOrQuestionmark(transition),
        SomeOrQuestionmark(vtable.flatten()),
        SomeOrQuestionmark(event),
    );

    unsafe { StormStateMachineActionExecuteHook.call(this, unknown1, unknown2) }
}

#[instrument]
fn storm_some_error_formatter(
    this: *mut c_void,
    out: *mut GearBasicString,
) -> *mut GearBasicString {
    let out = unsafe { StormErrorFormatter.call(this, out) };
    if !out.is_null() {
        info!("storm error: {}", unsafe { &*out }.as_str());
    }
    out
}

#[instrument(skip_all)]
fn gear_str_destructor(this: *mut GearBasicString, x: *mut c_void) {
    if let Some(c) = unsafe { this.as_ref() } {
        let s = c.as_str();
        if !s.is_empty() {
            info!("~GearStr({s:?})");
        }
    }
    unsafe { GearStrDestructor.call(this, x) }
}

#[instrument(skip_all)]
fn gear_str_constructor(this: *mut GearBasicString, x: *mut c_char) -> *mut GearBasicString {
    if !x.is_null() {
        let s = unsafe { CStr::from_ptr(x.cast_const()) };
        info!("GearStr({s:?})");
    }
    unsafe { SomeGearStrConstructor.call(this, x) }
}

pub(self) unsafe fn hook_with_name<T, F>(
    hook: &retour::StaticDetour<T>,
    target: Option<T>,
    f: F,
    name: &str,
) where
    T: retour::Function,
    F: Fn<T::Arguments, Output = T::Output> + Send + 'static,
    <T as retour::Function>::Arguments: std::marker::Tuple,
{
    let Some(target) = target else {
        error!("Address for hook {name} missing");
        return;
    };
    let res = hook.initialize(target, f).and_then(|h| h.enable());
    if let Err(err) = res {
        error!("Hook {} failed: {:?}", name, err);
    } else {
        info!("Hook {} enabled with address {:?}", name, target.to_ptr());
    }
}

macro_rules! hook {
    ($hook:expr, $addr:expr, $func:ident) => {
        $crate::hooks::hook_with_name(
            &$hook,
            $addr.map(|a| unsafe { ::std::mem::transmute(a) }),
            $func,
            stringify!($hook),
        );
    };
}

macro_rules! configurable_hook {
    ($config: expr, $cfg: expr, $hook: expr ; $addr: expr => $cb: ident) => {
        if $config.enable_all_hooks || $config.enable_hooks.contains(&$cfg) {
            $crate::hooks::hook!($hook, $addr, $cb);
        }
    };
}

pub unsafe fn init(config: &Config, addr: &Addresses) {
    configurable_hook!(config, Hook::ChangeState, ChangeStateHook ; addr.func_goal_change_state => change_state);
    configurable_hook!(config, Hook::GearStrDestructor, GearStrDestructor ; addr.func_gear_str_destructor => gear_str_destructor);
    configurable_hook!(config, Hook::GearStrDestructor, SomeGearStrConstructor ; addr.func_some_gear_str_constructor => gear_str_constructor);
    configurable_hook!(config, Hook::GearStrDestructor, AnotherGearStrDestructorHook ; addr.func_another_gear_str_destructor => gear_str_destructor);
    configurable_hook!(config, Hook::GenerateID, GenerateIDHook ; addr.func_generate_id => generate_id);
    configurable_hook!(config, Hook::Goal, SomethingWithGoalHook ; addr.func_something_with_goal => something_with_goal);
    configurable_hook!(config, Hook::LeaveState, LeaveStateHook ; addr.func_net_finite_state_leave_state => leave_state);
    configurable_hook!(config, Hook::NetResultBase, NetResultBaseHook ; addr.func_net_result_base => net_result_base);
    configurable_hook!(config, Hook::NetResultCore, NetResultCoreHook ; addr.func_net_result_core => net_result_core);
    configurable_hook!(config, Hook::NetResultLobby, NetResultLobbyHook ; addr.func_net_result_lobby => net_result_lobby);
    configurable_hook!(config, Hook::NetResultRdvSession, NetResultRdvSessionHook ; addr.func_net_result_rdv_session => net_result_rdv_session);
    configurable_hook!(config, Hook::NetResultSession, NetResultSessionHook ; addr.func_net_result_session => net_result_session);
    configurable_hook!(config, Hook::NextState, NextStateHook ; addr.func_net_finite_state_machine_next_state => next_state);
    configurable_hook!(config, Hook::Printer, PrinterHook ; addr.func_printer => printer);
    configurable_hook!(config, Hook::SetStep, QuazalStepSequenceJobSetStateHook ; addr.func_quazal_stepsequencejob_setstep => quazal_stepsequencejob_setstep);
    configurable_hook!(config, Hook::StormErrorFormatter, StormErrorFormatter ; addr.func_storm_some_error_formatter => storm_some_error_formatter);
    configurable_hook!(config, Hook::StormHostPortToString, StormHostPortToStringHook ; addr.func_storm_host_port_to_str => storm_host_port_to_str);
    configurable_hook!(config, Hook::StormSetState, StormSetStateHook ; addr.func_storm_maybe_set_state => storm_set_state);
    configurable_hook!(config, Hook::StormStateMachineActionExecute, StormStateMachineActionExecuteHook ; addr.func_storm_statemachineaction_execute => storm_statemachineaction_execute);
    configurable_hook!(config, Hook::Thread, ThreadStarterHook ; addr.func_thread_starter => set_thread_name);
    if false {
        // always disable for now
        configurable_hook!(config, Hook::NetCore, NetCoreHook ; addr.func_net_core => net_core);
    }

    // always enable these hooks
    // if config.enable_all_hooks || config.enable_hooks.contains(&Hook::GetAdaptersInfo)
    {
        let lib = LoadLibraryA(s!("iphlpapi.dll")).unwrap();
        let addr = GetProcAddress(lib, s!("GetAdaptersInfo"));
        hook!(GetAdaptersInfoHook, addr, get_adapters_info);
    }
    // if config.enable_all_hooks || config.enable_hooks.contains(&Hook::Gethostbyname)
    {
        let lib = LoadLibraryA(s!("ws2_32.dll")).unwrap();
        let addr = GetProcAddress(lib, s!("gethostbyname"));
        hook!(GethostbynameHook, addr, gethostbyname);
    }

    storm::init_hooks(config, addr);
    quazal::init_hooks(config, addr);
}

macro_rules! disable_configurable_hook {
    ($config:expr, $cfg: expr, $hook: expr) => {
        if $config.enable_all_hooks || $config.enable_hooks.contains(&$cfg) {
            let _ = $hook.disable();
        }
    };
}

pub(crate) use configurable_hook;
pub(crate) use disable_configurable_hook;
pub(crate) use hook;

pub unsafe fn deinit(config: &Config) {
    disable_configurable_hook!(config, Hook::ChangeState, ChangeStateHook);
    disable_configurable_hook!(config, Hook::GearStrDestructor, GearStrDestructor);
    disable_configurable_hook!(
        config,
        Hook::GearStrDestructor,
        AnotherGearStrDestructorHook
    );
    disable_configurable_hook!(config, Hook::GearStrDestructor, SomeGearStrConstructor);
    disable_configurable_hook!(config, Hook::GenerateID, GenerateIDHook);
    disable_configurable_hook!(config, Hook::GetAdaptersInfo, GetAdaptersInfoHook);
    disable_configurable_hook!(config, Hook::Gethostbyname, GethostbynameHook);
    disable_configurable_hook!(config, Hook::Goal, SomethingWithGoalHook);
    disable_configurable_hook!(config, Hook::LeaveState, LeaveStateHook);
    disable_configurable_hook!(config, Hook::NetCore, NetCoreHook);
    disable_configurable_hook!(config, Hook::NetResultBase, NetResultBaseHook);
    disable_configurable_hook!(config, Hook::NetResultCore, NetResultCoreHook);
    disable_configurable_hook!(config, Hook::NetResultLobby, NetResultLobbyHook);
    disable_configurable_hook!(config, Hook::NetResultRdvSession, NetResultRdvSessionHook);
    disable_configurable_hook!(config, Hook::NetResultSession, NetResultSessionHook);
    disable_configurable_hook!(config, Hook::NextState, NextStateHook);
    disable_configurable_hook!(config, Hook::Printer, PrinterHook);
    disable_configurable_hook!(config, Hook::SetStep, QuazalStepSequenceJobSetStateHook);
    disable_configurable_hook!(config, Hook::StormErrorFormatter, StormErrorFormatter);
    disable_configurable_hook!(
        config,
        Hook::StormHostPortToString,
        StormHostPortToStringHook
    );
    disable_configurable_hook!(config, Hook::StormSetState, StormSetStateHook);
    disable_configurable_hook!(
        config,
        Hook::StormStateMachineActionExecute,
        StormStateMachineActionExecuteHook
    );
    disable_configurable_hook!(config, Hook::Thread, ThreadStarterHook);
    storm::deinit_hooks(config);
    quazal::deinit_hooks(config);
}
