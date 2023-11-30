use std::ffi::c_void;
use std::ffi::CStr;

use retour::static_detour;
use tracing::error;
use tracing::info;
use tracing::instrument;

use crate::addresses::Addresses;
use crate::config::Config;
use crate::config::Hook;

mod datatypes;
mod utils;

use self::datatypes::GearBasicString;
use self::datatypes::MaybeGoal;
use self::datatypes::NetFiniteStateMachine;
use self::datatypes::QuazalStep;

static_detour! {
    static PrinterHook: unsafe extern "thiscall" fn(*mut c_void, *const i8) -> *mut c_void;
    static LeaveStateHook: unsafe extern "thiscall" fn(*mut c_void, *mut c_void);
    static NextStateHook: unsafe extern "thiscall" fn(*mut *mut c_void, *mut c_void, usize);
    static NetResultBaseHook: unsafe extern "thiscall" fn(*mut c_void, *mut GearBasicString);
    static SomethingWithGoalHook: unsafe extern "thiscall" fn(*mut *mut c_void, usize, *mut MaybeGoal, usize, usize);
    static QuazalStepSequenceJobSetStateHook: unsafe extern "thiscall" fn(*mut c_void, *mut QuazalStep);
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
fn leave_state(x: *mut c_void, y: *mut c_void) {
    let name = unsafe { get_state_name(x) };

    if let Some(name) = name {
        info!("Leaving state {}(inst={:?})", name.to_string_lossy(), x);
    } else {
        info!("Leaving state (inst={:?})", x);
    }
    unsafe { LeaveStateHook.call(x, y) }
}

#[instrument(skip_all)]
fn next_state(x: *mut *mut c_void, y: *mut c_void, z: usize) {
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
    if !x.is_null() {
        let sm = x.cast::<NetFiniteStateMachine>();
        let vtable = unsafe { (*sm).vtable };
        let current_state = unsafe { (*sm).current_state };
        let last_state = unsafe { (*sm).last_state };
        if let Some((_id, name)) = id.and_then(|id| Some((id, map.get(&id)?))) {
            info!(
                "Next state: {name} StateMachine(inst={x:?}, vtable={vtable:?}) current={} last={}",
                utils::state_ptr_to_name(current_state),
                utils::state_ptr_to_name(last_state)
            );
        } else {
            info!(
                "Next state: StateMachine(inst={x:?}, vtable={vtable:?} current={} last={}",
                utils::state_ptr_to_name(current_state),
                utils::state_ptr_to_name(last_state)
            );
        }
    }
    unsafe { NextStateHook.call(x, y, z) }
}

#[instrument(skip_all)]
fn net_result_base(this: *mut c_void, string: *mut GearBasicString) {
    unsafe {
        if !string.is_null() && string.is_aligned() && !(*string).internal.is_null() {
            let string2 = &mut *string;
            let internal = &mut *string2.internal;
            let cstr = internal.as_str();
            info!(
                "string: {:?} -> internal: {:?} -> {}",
                string, string2.internal, cstr
            );
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
            let goal = CStr::from_ptr((*mg).name);
            info!(
                "goal: {} (this: {:?} -> {:?})",
                goal.to_string_lossy(),
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
            // ::std::mem::transmute($addr),
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
}
