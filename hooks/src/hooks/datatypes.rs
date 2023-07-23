use std::ffi::c_void;
use std::ffi::CStr;

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
    get_state_id: *const c_void,
    get_state_name: unsafe extern "thiscall" fn(*mut NetFiniteState) -> *const i8,
    unknown8: *const c_void,
    unknown9: *const c_void,
}

#[repr(C)]
pub struct NetFiniteState {
    pub vtable: *const NetFiniteStateVMT,
}

impl NetFiniteState {
    pub fn get_state_name(&mut self) -> &'static str {
        unsafe {
            let name = ((*self.vtable).get_state_name)(self as *mut NetFiniteState);
            let name = CStr::from_ptr::<'static>(name);
            name.to_str().unwrap()
        }
    }
}

#[repr(C)]
pub struct NetFiniteStateMachine {
    pub vtable: *const c_void,
    unknown1: u32,
    unknown2: u32,
    unknown3: u32,
    pub current_state: *mut NetFiniteState,
    pub last_state: *mut NetFiniteState,
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
            let p = (self as *const Self).cast::<u8>().offset(12);
            std::slice::from_raw_parts(p, self.size as usize)
        }
    }

    pub fn as_str(&self) -> std::borrow::Cow<str> {
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
    pub fn as_str(&self) -> std::borrow::Cow<str> {
        unsafe { (*self.internal).as_str() }
    }
}

#[repr(C)]
pub struct MaybeGoal {
    unknown: usize,
    pub name: *const i8,
}

#[repr(C)]
pub struct QuazalStep {
    pub callback: *const c_void,
    unknown: u32,
    pub description: *const i8,
}
