use std::os::raw::*;
use std::ptr::null;
use std::ptr::null_mut;

use const_cstr::const_cstr;
use lazy_static::lazy_static;
use quazal::prudp::packet::PacketFlag;
use quazal::prudp::packet::PacketType;
use quazal::prudp::packet::QPacket;
use quazal::Context;

use crate::bindings::*;

mod fields;
// mod reassemble_fields;

use fields::*;
// use reassemble_fields::*;

static mut PROTO_PRUDP: i32 = -1;
const PROTO_PORT: u32 = 21170;
static mut MSG_REASSEMBLE_TABLE: reassembly_table = reassembly_table {
    fragment_table: null_mut(),
    reassembled_table: null_mut(),
    temporary_key_func: None,
    persistent_key_func: None,
    free_temporary_key_func: None,
};

static mut EI_SIGNATURE_INVALID: expert_field = expert_field {
    ei: EI_INIT_EI,
    hf: EI_INIT_HF,
};

lazy_static! {
    static ref EI: [ei_register_info; 1] = [ei_register_info {
        ids: unsafe { &mut EI_SIGNATURE_INVALID },
        eiinfo: expert_field_info {
            name: const_cstr!("prudp.signature_invalid").as_ptr(),
            group: PI_CHECKSUM as i32,
            severity: PI_ERROR as i32,
            summary: const_cstr!("Checksum is invalid").as_ptr(),
            id: 0,
            protocol: null(),
            orig_severity: 0,
            hf_info: hf_register_info {
                p_id: null_mut(),
                hfinfo: header_field_info {
                    name: null(),
                    abbrev: null(),
                    type_: ftenum_FT_NONE,
                    display: field_display_e_BASE_NONE as i32,
                    strings: null(),
                    bitmask: 0,
                    blurb: null(),
                    id: -1,
                    parent: 0,
                    ref_type: hf_ref_type_HF_REF_TYPE_NONE,
                    same_name_prev_id: -1,
                    same_name_next: null_mut(),
                }
            },
        }
    },];
}

pub extern "C" fn proto_register() {
    unsafe {
        PROTO_PRUDP = proto_register_protocol(
            const_cstr!("Pretty Reliable UDP (Quazal)").as_ptr(),
            const_cstr!("PRUDP").as_ptr(),
            const_cstr!("prudp").as_ptr(),
        );

        // Register fields
        let hf_unsafe = std::mem::transmute(fields::HF.as_ptr());
        proto_register_field_array(PROTO_PRUDP, hf_unsafe, fields::HF.len() as i32);

        // Register ett
        let ett = create_ett();
        proto_register_subtree_array(ett.as_ptr(), ett.len() as i32);

        // register reassembly table
        reassembly_table_register(
            &mut MSG_REASSEMBLE_TABLE,
            &addresses_ports_reassembly_table_functions,
        );

        // register expert items
        let expert_prudp = expert_register_protocol(PROTO_PRUDP);
        let ei_unsafe = std::mem::transmute(EI.as_ptr());
        expert_register_field_array(expert_prudp, ei_unsafe, EI.len() as i32);
    }
}

extern "C" fn dissect_prudp(
    tvb: *mut tvbuff_t,
    pinfo: *mut packet_info,
    tree: *mut proto_tree,
    data: *mut c_void,
) -> c_int {
    unsafe {
        let l = tvb_captured_length(tvb) as usize;
        let buf = tvb_get_ptr(tvb, 0, -1);
        let tmp = std::slice::from_raw_parts(buf, l as usize);
        let ctx = Context::splinter_cell_blacklist();
        let (qpacket, size) = match QPacket::from_bytes(&ctx, tmp) {
            Err(e) => {
                eprintln!("{:?}", e);
                return l as i32;
            }
            Ok(p) => p,
        };
        let invalid_signature = qpacket.validate(&ctx, &tmp[..size as usize]).is_err();

        col_set_str(
            (*pinfo).cinfo,
            COL_PROTOCOL as c_int,
            const_cstr!("PRUDP ").as_ptr(),
        );
        col_clear((*pinfo).cinfo, COL_INFO as gint);

        if qpacket.flags.contains(PacketFlag::Ack) {
            col_append_str(
                (*pinfo).cinfo,
                COL_PROTOCOL as i32,
                const_cstr!("ACK").as_ptr(),
            );
        } else {
            match qpacket.destination.stream_type {
                quazal::prudp::packet::StreamType::DO => col_append_str(
                    (*pinfo).cinfo,
                    COL_PROTOCOL as i32,
                    const_cstr!("DO").as_ptr(),
                ),
                quazal::prudp::packet::StreamType::RV => col_append_str(
                    (*pinfo).cinfo,
                    COL_PROTOCOL as i32,
                    const_cstr!("RV").as_ptr(),
                ),
                quazal::prudp::packet::StreamType::RVSec => col_append_str(
                    (*pinfo).cinfo,
                    COL_PROTOCOL as i32,
                    const_cstr!("RVSec").as_ptr(),
                ),
                quazal::prudp::packet::StreamType::SBMGMT => col_append_str(
                    (*pinfo).cinfo,
                    COL_PROTOCOL as i32,
                    const_cstr!("SBMGMT").as_ptr(),
                ),
                quazal::prudp::packet::StreamType::NAT => col_append_str(
                    (*pinfo).cinfo,
                    COL_PROTOCOL as i32,
                    const_cstr!("NAT").as_ptr(),
                ),
                quazal::prudp::packet::StreamType::SessionDiscovery => col_append_str(
                    (*pinfo).cinfo,
                    COL_PROTOCOL as i32,
                    const_cstr!("SessionDiscovery").as_ptr(),
                ),
                quazal::prudp::packet::StreamType::NATEcho => col_append_str(
                    (*pinfo).cinfo,
                    COL_PROTOCOL as i32,
                    const_cstr!("NATEcho").as_ptr(),
                ),
                quazal::prudp::packet::StreamType::Routing => col_append_str(
                    (*pinfo).cinfo,
                    COL_PROTOCOL as i32,
                    const_cstr!("Routing").as_ptr(),
                ),
                // quazal::prudp::packet::StreamType::Game => col_append_str(
                //     (*pinfo).cinfo,
                //     COL_PROTOCOL as i32,
                //     const_cstr!("Game").as_ptr(),
                // ),
            }
        }

        match qpacket.packet_type {
            PacketType::Syn => {
                col_set_str((*pinfo).cinfo, COL_INFO as i32, const_cstr!("Syn").as_ptr())
            }
            PacketType::Connect => col_set_str(
                (*pinfo).cinfo,
                COL_INFO as i32,
                const_cstr!("Connect").as_ptr(),
            ),
            PacketType::Data => col_set_str(
                (*pinfo).cinfo,
                COL_INFO as i32,
                const_cstr!("Data").as_ptr(),
            ),
            PacketType::Disconnect => col_set_str(
                (*pinfo).cinfo,
                COL_INFO as i32,
                const_cstr!("Disconnect").as_ptr(),
            ),
            PacketType::Ping => col_set_str(
                (*pinfo).cinfo,
                COL_INFO as i32,
                const_cstr!("Ping").as_ptr(),
            ),
            PacketType::User => col_set_str(
                (*pinfo).cinfo,
                COL_INFO as i32,
                const_cstr!("User").as_ptr(),
            ),
            PacketType::Route => col_set_str(
                (*pinfo).cinfo,
                COL_INFO as i32,
                const_cstr!("Route").as_ptr(),
            ),
            PacketType::Raw => {
                col_set_str((*pinfo).cinfo, COL_INFO as i32, const_cstr!("Raw").as_ptr())
            }
        }

        col_append_str_uint(
            (*pinfo).cinfo,
            COL_INFO as gint,
            const_cstr!("Src").as_ptr(),
            qpacket.source.port as guint32,
            const_cstr!(" ").as_ptr(),
        );

        col_append_str_uint(
            (*pinfo).cinfo,
            COL_INFO as gint,
            const_cstr!("Dst").as_ptr(),
            qpacket.destination.port as guint32,
            const_cstr!(" ").as_ptr(),
        );

        let root_ti = proto_tree_add_item(tree, PROTO_PRUDP, tvb, 0, -1, ENC_NA);
        let prudp_tree = proto_item_add_subtree(root_ti, ETT_PRUDP);
        let mut offset = 0;
        let src = tvb_get_guint8(tvb, offset);
        let src_vport = proto_tree_add_subtree_format(
            prudp_tree,
            tvb,
            offset,
            1,
            ETT_PRUDP_SOURCE,
            std::ptr::null_mut(),
            const_cstr!("Source: %02x").as_ptr(),
            src as c_uint,
        );
        proto_tree_add_item(
            src_vport,
            hf_prudp_source_port,
            tvb,
            offset,
            1,
            ENC_LITTLE_ENDIAN,
        );
        proto_tree_add_item(
            src_vport,
            hf_prudp_source_type,
            tvb,
            offset,
            1,
            ENC_LITTLE_ENDIAN,
        );
        offset += 1;
        let dst = tvb_get_guint8(tvb, offset);
        let dst_vport = proto_tree_add_subtree_format(
            prudp_tree,
            tvb,
            offset,
            1,
            ETT_PRUDP_DEST,
            std::ptr::null_mut(),
            const_cstr!("Destination: %02x").as_ptr(),
            dst as c_uint,
        );
        proto_tree_add_item(
            dst_vport,
            hf_prudp_dest_port,
            tvb,
            offset,
            1,
            ENC_LITTLE_ENDIAN,
        );
        proto_tree_add_item(
            dst_vport,
            hf_prudp_dest_type,
            tvb,
            offset,
            1,
            ENC_LITTLE_ENDIAN,
        );
        offset += 1;
        proto_tree_add_item(prudp_tree, hf_prudp_type, tvb, offset, 1, ENC_LITTLE_ENDIAN);

        let bits = [
            &mut hf_prudp_flags_ack,
            &mut hf_prudp_flags_reliable,
            &mut hf_prudp_flags_need_ack,
            &mut hf_prudp_flags_has_size,
            null_mut(),
        ];
        proto_tree_add_bitmask(
            prudp_tree,
            tvb,
            offset as u32,
            hf_prudp_flags,
            ETT_PRUDP,
            bits.as_ptr(),
            ENC_LITTLE_ENDIAN,
        );
        offset += 1;
        proto_tree_add_item(
            prudp_tree,
            hf_prudp_session_id,
            tvb,
            offset,
            1,
            ENC_LITTLE_ENDIAN,
        );
        offset += 1;
        proto_tree_add_item(
            prudp_tree,
            hf_prudp_signature,
            tvb,
            offset,
            4,
            ENC_LITTLE_ENDIAN,
        );
        offset += 4;
        proto_tree_add_item(
            prudp_tree,
            hf_prudp_sequence,
            tvb,
            offset,
            2,
            ENC_LITTLE_ENDIAN,
        );
        offset += 2;

        let _fragment_id = match qpacket.packet_type {
            PacketType::Syn | PacketType::Connect => {
                proto_tree_add_item(
                    prudp_tree,
                    hf_prudp_conn_signature,
                    tvb,
                    offset,
                    4,
                    ENC_LITTLE_ENDIAN,
                );
                offset += 4;
                None
            }
            PacketType::Data => {
                let fragment_id = tvb_get_guint8(tvb, offset);
                proto_tree_add_item(
                    prudp_tree,
                    hf_prudp_fragment_id,
                    tvb,
                    offset,
                    1,
                    ENC_LITTLE_ENDIAN,
                );
                offset += 1;
                Some(fragment_id)
            }
            _ => None,
        };

        let mut size = if qpacket.flags.contains(PacketFlag::HasSize) {
            proto_tree_add_item(prudp_tree, hf_prudp_size, tvb, offset, 2, ENC_LITTLE_ENDIAN);
            let sz = tvb_get_guint16(tvb, offset, ENC_LITTLE_ENDIAN);
            offset += 2;
            sz as i32
        } else {
            (tvb_captured_length(tvb) as i32 - offset - 1) as i32
        };

        let next_tvb = if !qpacket.payload.is_empty() {
            let orig_size = qpacket.payload.len();
            let buf = wmem_alloc((*pinfo).pool, orig_size as u64) as *mut u8;
            std::ptr::copy(qpacket.payload.as_ptr(), buf, orig_size);
            let next_tvb =
                tvb_new_child_real_data(tvb, buf as *const u8, orig_size as u32, orig_size as i32);
            add_new_data_source(pinfo, next_tvb, const_cstr!("Decoded Data").as_ptr());
            offset += size;
            size = orig_size as i32;
            next_tvb
        } else {
            let next_tvb = tvb_new_subset_length(tvb, offset, size);
            offset += size;
            next_tvb
        };

        let pi = proto_tree_add_item(
            prudp_tree,
            hf_prudp_checksum,
            tvb,
            offset,
            1,
            ENC_LITTLE_ENDIAN,
        );
        offset += 1;

        if invalid_signature {
            expert_add_info(pinfo, pi, &mut EI_SIGNATURE_INVALID);
        }

        // let save_fragmented = (*pinfo).fragmented;
        // let next_tvb = if let Some(fragment_id) = fragment_id {
        //     process_fragment(pinfo, prudp_tree, next_tvb, fragment_id, &qpacket, size)
        // } else {
        //     next_tvb
        // };
        // (*pinfo).fragmented = save_fragmented;

        if !next_tvb.is_null()
            && matches!(qpacket.packet_type, PacketType::Data)
            && !qpacket.flags.contains(PacketFlag::Ack)
        {
            crate::rmc::dissect_rmc(next_tvb, pinfo, tree, data);
        }

        let remaining = tvb_captured_length_remaining(tvb, offset);
        if remaining > 0 {
            let tvb = tvb_new_subset_remaining(tvb, offset);
            //dbg!(tvb_get_guint8(tvb, 0));
            dissect_prudp(tvb, pinfo, tree, data)
            // offset
        } else {
            offset
        }
    }
}

unsafe fn process_fragment(
    pinfo: *mut packet_info,
    tree: *mut proto_tree,
    next_tvb: *mut tvbuff_t,
    fragment_id: u8,
    qpacket: &QPacket,
    size: i32,
) -> *mut tvbuff_t {
    if qpacket.flags.contains(PacketFlag::Ack) {
        return next_tvb;
    }

    let id = qpacket.session_id as u32;

    if fragment_id == 0 {
        let head = fragment_get(&mut MSG_REASSEMBLE_TABLE, pinfo, id, null());
        if head.is_null() {
            return next_tvb;
        }
    }
    (*pinfo).fragmented = true as i32;

    let more_frags = (fragment_id > 0) as i32;
    let frag_msg = fragment_add_seq_next(
        &mut MSG_REASSEMBLE_TABLE,
        next_tvb,
        0,
        pinfo,
        id,
        null(),
        size as u32,
        more_frags,
    );

    let prudp_frag_items = fragment_items {
        ett_fragment: &mut ETT_PRUDP_FRAGMENT,
        ett_fragments: &mut ETT_PRUDP_FRAGMENTS,

        hf_fragments: &mut hf_prudp_fragments,
        hf_fragment: &mut hf_prudp_fragment,
        hf_fragment_overlap: &mut hf_prudp_fragment_overlap,
        hf_fragment_overlap_conflict: &mut hf_prudp_fragment_overlap_conflict,
        hf_fragment_multiple_tails: &mut hf_prudp_fragment_multiple_tails,
        hf_fragment_too_long_fragment: &mut hf_prudp_fragment_too_long_fragment,
        hf_fragment_error: &mut hf_prudp_fragment_error,
        hf_fragment_count: &mut hf_prudp_fragment_count,

        hf_reassembled_in: &mut hf_prudp_reassembled_in,
        hf_reassembled_length: &mut hf_prudp_reassembled_length,
        hf_reassembled_data: &mut hf_prudp_reassembled_data,

        tag: const_cstr!("PRUDP Fragments").as_ptr(),
    };

    let new_tvb = process_reassembled_data(
        next_tvb,
        0,
        pinfo,
        const_cstr!("Reassembled Message").as_ptr(),
        frag_msg,
        &prudp_frag_items,
        null_mut(),
        tree,
    );

    if !frag_msg.is_null() {
        col_append_str(
            (*pinfo).cinfo,
            COL_INFO as i32,
            const_cstr!(" (Message Reassembled)").as_ptr(),
        );
    } else {
        col_append_fstr(
            (*pinfo).cinfo,
            COL_INFO as i32,
            const_cstr!(" (Message fragment)").as_ptr(),
        );
    }

    new_tvb

    // if !new_tvb.is_null() {
    //     new_tvb
    // } else {
    //     next_tvb
    // }
}

pub extern "C" fn proto_reg_handoff() {
    unsafe {
        let handle = create_dissector_handle(Some(dissect_prudp), PROTO_PRUDP);
        dissector_add_uint(const_cstr!("udp.port").as_ptr(), PROTO_PORT, handle);
    }
}
