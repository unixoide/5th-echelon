#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
// u128 FFI warning
#![allow(improper_ctypes)]
#![allow(clippy::all)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// https://github.com/vchekan/wireshark-kafka/blob/master/wireshark-ffi/src/bindings.rs#L9-L16
// These struct are not thread-safe but we are using it in single-threaded Wireshark
// and this is the only way to tell Rust that you are in charge and you are the one
// to deal with dragons.
unsafe impl Sync for hf_register_info {}
unsafe impl Send for hf_register_info {}

unsafe impl Sync for value_string {}
unsafe impl Send for value_string {}

unsafe impl Sync for fragment_items {}
unsafe impl Send for fragment_items {}

unsafe impl Sync for ei_register_info {}
unsafe impl Send for ei_register_info {}
