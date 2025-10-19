use std::ffi::c_void;
use std::ffi::CStr;

use super::utils;

#[repr(C)]
pub struct NetFiniteStateVMT {
    unknown1: *const c_void,
    unknown2: *const c_void,
    change_state: *const c_void,
    unknown3: *const c_void,
    get_result: *const c_void,
    unknown4: *const c_void,
    unknown5: *const c_void,
    initialize: *const c_void,
    on_enter: *const c_void,
    leave_state: *const c_void,
    unknown6: *const c_void,
    unknown7: *const c_void,
    get_state_id: unsafe extern "thiscall" fn(*const NetFiniteState) -> u32,
    get_state_name: unsafe extern "thiscall" fn(*const NetFiniteState) -> *const i8,
    unknown8: *const c_void,
    unknown9: *const c_void,
}

#[repr(C)]
pub struct NetFiniteState {
    pub vtable: *const NetFiniteStateVMT,
}

impl NetFiniteState {
    pub fn get_state_name(&self) -> &'static str {
        unsafe {
            let name = ((*self.vtable).get_state_name)(std::ptr::from_ref(self));
            let name = CStr::from_ptr::<'static>(name);
            name.to_str().unwrap()
        }
    }
    pub fn get_state_id(&self) -> u32 {
        unsafe { ((*self.vtable).get_state_id)(std::ptr::from_ref(self)) }
    }
}

#[repr(C)]
pub struct NetFiniteStateMachineVMT {
    unknown1: *const c_void,
    unknown2: *const c_void,
    unknown3: *const c_void,
    unknown4: *const c_void,
    get_statemachine_id: extern "thiscall" fn(*const NetFiniteStateMachine, *mut NetFiniteStateID) -> *mut NetFiniteStateID,
}

#[repr(C)]
pub struct NetFiniteStateMachine {
    pub vtable: *const NetFiniteStateMachineVMT,
    unknown1: u32,
    unknown2: u32,
    unknown3: u32,
    pub current_state: *mut NetFiniteState,
    pub last_state: *mut NetFiniteState,
}

impl NetFiniteStateMachine {
    pub fn get_statemachine_id(&self, id: *mut NetFiniteStateID) -> *mut NetFiniteStateID {
        unsafe { ((*self.vtable).get_statemachine_id)(self, id) }
    }

    pub fn get_statemachine_name(&self) -> String {
        let mut id = NetFiniteStateID { id: 0, unknown: 0 };
        self.get_statemachine_id(&mut id);
        utils::id_to_name(id.id as usize)
    }
}

#[repr(C)]
pub struct NetFiniteStateID {
    pub id: u32,
    pub unknown: u32,
}

impl NetFiniteStateID {
    pub fn name(&self) -> String {
        utils::id_to_name(self.id as usize)
    }
}

#[repr(C)]
pub struct GearBasicStringInternal {
    ref_counter: u32,
    pub size: u32,
    capacity: u32,
}

impl GearBasicStringInternal {
    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            let p = std::ptr::from_ref(self).cast::<u8>().offset(12);
            if p.is_null() {
                b"<NULL>"
            } else {
                std::slice::from_raw_parts(p, self.size as usize)
            }
        }
    }

    pub fn as_str(&self) -> std::borrow::Cow<'_, str> {
        String::from_utf8_lossy(self.as_bytes())
    }
}

#[repr(C)]
pub struct GearBasicString {
    allocator: *mut c_void,
    unknown: u32,
    pub internal: *mut GearBasicStringInternal,
    tag: *mut i8,
}

impl GearBasicString {
    #[allow(dead_code)]
    pub fn as_str(&self) -> std::borrow::Cow<'_, str> {
        unsafe {
            if let Some(int) = self.internal.as_ref() {
                int.as_str()
            } else {
                "<NULL>".into()
            }
        }
    }
}

#[repr(C)]
pub struct MaybeGoal {
    pub unknown: usize,
    name: *const i8,
}

impl MaybeGoal {
    pub fn name(&self) -> Option<&CStr> {
        if self.name.is_null() {
            return None;
        }
        Some(unsafe { CStr::from_ptr(self.name) })
    }
}

#[repr(C)]
pub struct QuazalStep {
    pub callback: *const c_void,
    unknown: u32,
    pub description: *const i8,
}
