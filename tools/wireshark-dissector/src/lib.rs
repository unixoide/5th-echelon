#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[macro_use]
mod macros;
mod bindings;
mod prudp;
mod rmc;

use bindings::*;

#[no_mangle]
#[used]
pub static plugin_version: [u8; 6] = *b"0.0.1\0";

#[no_mangle]
#[used]
pub static plugin_want_major: i32 = 3;

#[no_mangle]
#[used]
pub static plugin_want_minor: i32 = 6;

extern "C" fn proto_reg_handoff() {
    prudp::proto_reg_handoff();
}

extern "C" fn proto_register() {
    prudp::proto_register();
    rmc::proto_register();
}

static PLUGIN: proto_plugin = proto_plugin {
    register_handoff: Some(proto_reg_handoff),
    register_protoinfo: Some(proto_register),
};

#[no_mangle]
extern "C" fn plugin_register() {
    unsafe {
        proto_register_plugin(&PLUGIN);
    }
}
