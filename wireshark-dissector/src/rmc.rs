#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::bindings::*;
use const_cstr::const_cstr;
use lazy_static::lazy_static;

use std::os::raw::*;
use std::ptr::{null, null_mut};

static mut PROTO_RMC: i32 = -1;

ett!(ETT_RMC);

header_fields!(
    {
        hf_rmc_is_request {
            p_id: unsafe { &mut hf_rmc_is_request as *mut _ },
            hfinfo: header_field_info {
                name: const_cstr!("IsRequest").as_ptr(),
                abbrev: const_cstr!("rmc.is_request").as_ptr(),
                type_: ftenum_FT_BOOLEAN,
                display: 8,
                strings: null(),
                bitmask: 0x80,
                blurb: const_cstr!("RMC IsRequest").as_ptr(),
                id: -1,
                parent: 0,
                ref_type: hf_ref_type_HF_REF_TYPE_NONE,
                same_name_prev_id: -1,
                same_name_next: null_mut(),
            },
        }
    },
    {
        hf_rmc_protocol_id8 {
            p_id: unsafe { &mut hf_rmc_protocol_id8 as *mut _ },
            hfinfo: header_field_info {
                name: const_cstr!("Protocol").as_ptr(),
                abbrev: const_cstr!("rmc.protocol").as_ptr(),
                type_: ftenum_FT_UINT8,
                display: field_display_e_BASE_DEC as i32,
                strings: null(),
                bitmask: 0x7f,
                blurb: const_cstr!("RMC Protocol").as_ptr(),
                id: -1,
                parent: 0,
                ref_type: hf_ref_type_HF_REF_TYPE_NONE,
                same_name_prev_id: -1,
                same_name_next: null_mut(),
            },
        }
    },
    {
        hf_rmc_protocol_id16, "Protocol", "rmc.protocol", ftenum_FT_UINT16, "RMC Protocol"
    },
    {
        hf_rmc_method_id {
            p_id: unsafe { &mut hf_rmc_method_id as *mut _ },
            hfinfo: header_field_info {
                name: const_cstr!("Method").as_ptr(),
                abbrev: const_cstr!("rmc.method").as_ptr(),
                type_: ftenum_FT_UINT32,
                display: field_display_e_BASE_DEC as i32,
                strings: null(),
                bitmask: 0x7fff,
                blurb: const_cstr!("RMC Method").as_ptr(),
                id: -1,
                parent: 0,
                ref_type: hf_ref_type_HF_REF_TYPE_NONE,
                same_name_prev_id: -1,
                same_name_next: null_mut(),
            },
        }
    },
    {
        hf_rmc_call_id, "CallID", "rmc.call", ftenum_FT_UINT32, "RMC Call ID"
    },
    {
        hf_rmc_parameters, "Parameters", "rmc.params", ftenum_FT_BYTES|field_display_e_BASE_NONE, "RMC Method Parameters"
    },
    {
        hf_rmc_error_code, "ErrorCode", "rmc.error", ftenum_FT_UINT32|field_display_e_BASE_HEX, "RMC Response Error"
    },
    {
        hf_rmc_data, "Data", "rmc.data", ftenum_FT_BYTES|field_display_e_BASE_NONE, "RMC Response Data"
    }
);

pub extern "C" fn proto_register() {
    unsafe {
        PROTO_RMC = proto_register_protocol(
            const_cstr!("RMC (Quazal)").as_ptr(),
            const_cstr!("RMC").as_ptr(),
            const_cstr!("rmc").as_ptr(),
        );

        // Register fields
        let hf_unsafe = std::mem::transmute(HF.as_ptr());
        proto_register_field_array(PROTO_RMC, hf_unsafe, HF.len() as i32);

        // Register ett
        let ett = create_ett();
        proto_register_subtree_array(ett.as_ptr(), ett.len() as i32);
    }
}

pub extern "C" fn dissect_rmc(
    tvb: *mut tvbuff_t,
    pinfo: *mut packet_info,
    tree: *mut proto_tree,
    _data: *mut c_void,
) -> c_int {
    #![allow(unused_assignments)]
    unsafe {
        col_set_str(
            (*pinfo).cinfo,
            COL_PROTOCOL as c_int,
            const_cstr!("RMC").as_ptr(),
        );
        col_clear((*pinfo).cinfo, COL_INFO as gint);

        let l = tvb_captured_length(tvb) as usize;
        let buf = tvb_get_ptr(tvb, 0, -1);
        let packet =
            match quazal::rmc::Packet::from_bytes(std::slice::from_raw_parts(buf, l as usize)) {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("{}", e);
                    return tvb_captured_length(tvb) as c_int;
                }
            };

        let root_ti = proto_tree_add_item(tree, PROTO_RMC, tvb, 0, -1, ENC_NA);
        let rmc_tree = proto_item_add_subtree(root_ti, ETT_RMC);
        let mut offset = 4;

        proto_tree_add_item(
            rmc_tree,
            hf_rmc_is_request,
            tvb,
            offset,
            1,
            ENC_LITTLE_ENDIAN,
        );

        if tvb_get_guint8(tvb, offset) & 0x7f == 0x7f {
            offset += 1;
            proto_tree_add_item(
                rmc_tree,
                hf_rmc_protocol_id16,
                tvb,
                offset,
                2,
                ENC_LITTLE_ENDIAN,
            );
            offset += 2;
        } else {
            proto_tree_add_item(
                rmc_tree,
                hf_rmc_protocol_id8,
                tvb,
                offset,
                1,
                ENC_LITTLE_ENDIAN,
            );
            offset += 1;
        }

        match packet {
            quazal::rmc::Packet::Request(r) => {
                col_append_str(
                    (*pinfo).cinfo,
                    COL_PROTOCOL as gint,
                    const_cstr!(" Request").as_ptr(),
                );
                col_append_str_uint(
                    (*pinfo).cinfo,
                    COL_INFO as gint,
                    const_cstr!("Protocol").as_ptr(),
                    r.protocol_id as guint32,
                    const_cstr!("").as_ptr(),
                );
                col_append_str_uint(
                    (*pinfo).cinfo,
                    COL_INFO as gint,
                    const_cstr!("Method").as_ptr(),
                    r.method_id,
                    const_cstr!(" ").as_ptr(),
                );
                col_append_str_uint(
                    (*pinfo).cinfo,
                    COL_INFO as gint,
                    const_cstr!("Call").as_ptr(),
                    r.call_id,
                    const_cstr!(" ").as_ptr(),
                );
                proto_tree_add_item(rmc_tree, hf_rmc_call_id, tvb, offset, 4, ENC_LITTLE_ENDIAN);
                offset += 4;

                proto_tree_add_item(
                    rmc_tree,
                    hf_rmc_method_id,
                    tvb,
                    offset,
                    4,
                    ENC_LITTLE_ENDIAN,
                );
                offset += 4;

                match tvb_reported_length_remaining(tvb, offset) {
                    0 => {}
                    i if i > 0 => {
                        proto_tree_add_item(rmc_tree, hf_rmc_parameters, tvb, offset, -1, ENC_NA);
                    }
                    i => panic!("what? {}", i),
                }
            }
            quazal::rmc::Packet::Response(r) => {
                col_append_str(
                    (*pinfo).cinfo,
                    COL_PROTOCOL as gint,
                    const_cstr!(" Response").as_ptr(),
                );
                col_append_str_uint(
                    (*pinfo).cinfo,
                    COL_INFO as gint,
                    const_cstr!("Protocol").as_ptr(),
                    r.protocol_id as guint32,
                    const_cstr!("").as_ptr(),
                );
                // skip error indicator
                offset += 1;
                match r.result {
                    Ok(r) => {
                        col_append_str_uint(
                            (*pinfo).cinfo,
                            COL_INFO as gint,
                            const_cstr!("Method").as_ptr(),
                            r.method_id,
                            const_cstr!(" ").as_ptr(),
                        );
                        col_append_str_uint(
                            (*pinfo).cinfo,
                            COL_INFO as gint,
                            const_cstr!("Call").as_ptr(),
                            r.call_id,
                            const_cstr!(" ").as_ptr(),
                        );

                        proto_tree_add_item(
                            rmc_tree,
                            hf_rmc_call_id,
                            tvb,
                            offset,
                            4,
                            ENC_LITTLE_ENDIAN,
                        );
                        offset += 4;

                        proto_tree_add_item(
                            rmc_tree,
                            hf_rmc_method_id,
                            tvb,
                            offset,
                            4,
                            ENC_LITTLE_ENDIAN,
                        );
                        offset += 4;

                        match tvb_reported_length_remaining(tvb, offset) {
                            0 => {}
                            i if i > 0 => {
                                proto_tree_add_item(rmc_tree, hf_rmc_data, tvb, offset, -1, ENC_NA);
                            }
                            i => panic!("what? {}", i),
                        }
                    }
                    Err(r) => {
                        col_append_str_uint(
                            (*pinfo).cinfo,
                            COL_INFO as gint,
                            const_cstr!("Call").as_ptr(),
                            r.call_id,
                            const_cstr!(" ").as_ptr(),
                        );
                        col_append_str_uint(
                            (*pinfo).cinfo,
                            COL_INFO as gint,
                            const_cstr!("Error").as_ptr(),
                            r.error_code,
                            const_cstr!(" ").as_ptr(),
                        );

                        proto_tree_add_item(
                            rmc_tree,
                            hf_rmc_error_code,
                            tvb,
                            offset,
                            4,
                            ENC_LITTLE_ENDIAN,
                        );
                        offset += 4;

                        proto_tree_add_item(
                            rmc_tree,
                            hf_rmc_call_id,
                            tvb,
                            offset,
                            4,
                            ENC_LITTLE_ENDIAN,
                        );
                        offset += 4;
                    }
                }
            }
        }

        tvb_captured_length(tvb) as c_int
    }
}
