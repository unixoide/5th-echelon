use std::ffi::c_void;
use std::ffi::CStr;
use std::ffi::OsString;
use std::os::windows::ffi::OsStrExt;

use retour::static_detour;
use tracing::debug;
use tracing::error;
use tracing::info;
use tracing::instrument;
use windows::core::PCWSTR;
use windows::Win32::Foundation::HANDLE;
use windows::Win32::System::Threading::GetThreadId;
use windows::Win32::System::Threading::SetThreadDescription;

use crate::addresses::Addresses;
use crate::config::Config;
use crate::config::Hook;

mod datatypes;
mod utils;

use self::datatypes::GearBasicString;
use self::datatypes::MaybeGoal;
use self::datatypes::NetFiniteState;
use self::datatypes::NetFiniteStateID;
use self::datatypes::NetFiniteStateMachine;
use self::datatypes::QuazalStep;

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
    static NetResultSessionHook: unsafe extern "thiscall" fn(*mut c_void, usize, *mut GearBasicString) -> *mut c_void;
    static NetResultLobbyHook: unsafe extern "thiscall" fn(*mut c_void, usize, *mut GearBasicString) -> *mut c_void;
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

fn net_result_session(this: *mut c_void, code: usize, text: *mut GearBasicString) -> *mut c_void {
    unsafe {
        if !text.is_null() && text.is_aligned() && !(*text).internal.is_null() {
            let string2 = &mut *text;
            let internal = &mut *string2.internal;
            let cstr = internal.as_str();
            info!("NetResultSession: code={code:x} text={cstr}");
        }
    }
    unsafe { NetResultSessionHook.call(this, code, text) }
}

fn net_result_lobby(this: *mut c_void, code: usize, text: *mut GearBasicString) -> *mut c_void {
    unsafe {
        if !text.is_null() && text.is_aligned() && !(*text).internal.is_null() {
            let string2 = &mut *text;
            let internal = &mut *string2.internal;
            let cstr = internal.as_str();
            info!("NetResultLobby: code={code:x} text={cstr}");
        }
    }
    unsafe { NetResultSessionHook.call(this, code, text) }
}

unsafe fn hook<T, F>(hook: &retour::StaticDetour<T>, target: Option<T>, f: F, name: &str)
where
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
        info!("Hook {} enabled", name);
    }
}

macro_rules! hook {
    ($hook:expr, $addr:expr, $func:ident) => {
        hook(
            &$hook,
            $addr.map(|a| unsafe { ::std::mem::transmute(a) }),
            $func,
            stringify!($hook),
        );
    };
}

pub unsafe fn init(config: &Config, addr: &Addresses) {
    if config.enable_all_hooks || config.enable_hooks.contains(&Hook::Printer) {
        hook!(PrinterHook, addr.func_printer, printer);
    }
    if config.enable_all_hooks || config.enable_hooks.contains(&Hook::LeaveState) {
        hook!(
            LeaveStateHook,
            addr.func_net_finite_state_leave_state,
            leave_state
        );
    }
    if config.enable_all_hooks || config.enable_hooks.contains(&Hook::NextState) {
        hook!(
            NextStateHook,
            addr.func_net_finite_state_machine_next_state,
            next_state
        );
    }
    if config.enable_all_hooks || config.enable_hooks.contains(&Hook::NetResultBase) {
        hook!(
            NetResultBaseHook,
            addr.func_net_result_base,
            net_result_base
        );
    }
    if config.enable_all_hooks || config.enable_hooks.contains(&Hook::Goal) {
        hook!(
            SomethingWithGoalHook,
            addr.func_something_with_goal,
            something_with_goal
        );
    }
    if config.enable_all_hooks || config.enable_hooks.contains(&Hook::SetStep) {
        hook!(
            QuazalStepSequenceJobSetStateHook,
            addr.func_quazal_stepsequencejob_setstep,
            quazal_stepsequencejob_setstep
        );
    }
    if config.enable_all_hooks || config.enable_hooks.contains(&Hook::Thread) {
        hook!(ThreadStarterHook, addr.func_thread_starter, set_thread_name);
    }
    if config.enable_all_hooks || config.enable_hooks.contains(&Hook::ChangeState) {
        hook!(ChangeStateHook, addr.func_goal_change_state, change_state);
    }
    if config.enable_all_hooks || config.enable_hooks.contains(&Hook::NetCore) {
        hook!(NetCoreHook, addr.func_net_core, net_core);
    }
    if config.enable_all_hooks || config.enable_hooks.contains(&Hook::NetResultSession) {
        hook!(
            NetResultSessionHook,
            addr.func_net_result_session,
            net_result_session
        );
    }
    if config.enable_all_hooks || config.enable_hooks.contains(&Hook::NetResultLobby) {
        hook!(
            NetResultLobbyHook,
            addr.func_net_result_lobby,
            net_result_lobby
        );
    }
}

pub unsafe fn deinit(config: &Config) {
    if config.enable_all_hooks || config.enable_hooks.contains(&Hook::Printer) {
        let _ = PrinterHook.disable();
    }
    if config.enable_all_hooks || config.enable_hooks.contains(&Hook::NextState) {
        let _ = NextStateHook.disable();
    }
    if config.enable_all_hooks || config.enable_hooks.contains(&Hook::LeaveState) {
        let _ = LeaveStateHook.disable();
    }
    if config.enable_all_hooks || config.enable_hooks.contains(&Hook::NetResultBase) {
        let _ = NetResultBaseHook.disable();
    }
    if config.enable_all_hooks || config.enable_hooks.contains(&Hook::Goal) {
        let _ = SomethingWithGoalHook.disable();
    }
    if config.enable_all_hooks || config.enable_hooks.contains(&Hook::SetStep) {
        let _ = QuazalStepSequenceJobSetStateHook.disable();
    }
    if config.enable_all_hooks || config.enable_hooks.contains(&Hook::Thread) {
        let _ = ThreadStarterHook.disable();
    }
    if config.enable_all_hooks || config.enable_hooks.contains(&Hook::ChangeState) {
        let _ = QuazalStepSequenceJobSetStateHook.disable();
    }
    if config.enable_all_hooks || config.enable_hooks.contains(&Hook::NetCore) {
        let _ = NetCoreHook.disable();
    }
    if config.enable_all_hooks || config.enable_hooks.contains(&Hook::NetResultSession) {
        let _ = NetResultSessionHook.disable();
    }
    if config.enable_all_hooks || config.enable_hooks.contains(&Hook::NetResultLobby) {
        let _ = NetResultLobbyHook.disable();
    }
}
