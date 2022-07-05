use crate::bindings::*;
use const_cstr::const_cstr;
use lazy_static::lazy_static;
use std::os::raw::*;
use std::ptr::{null, null_mut};

ett!(
    ETT_PRUDP,
    ETT_PRUDP_SOURCE,
    ETT_PRUDP_DEST,
    ETT_PRUDP_FRAGMENT,
    ETT_PRUDP_FRAGMENTS
);

lazy_static! {
    static ref STREAM_TYPE_NAMES: [value_string; 8] = [
        value_string {
            value: 1,
            strptr: const_cstr!("DO").as_ptr(),
        },
        value_string {
            value: 2,
            strptr: const_cstr!("RV").as_ptr(),
        },
        value_string {
            value: 3,
            strptr: const_cstr!("RVSec").as_ptr(),
        },
        value_string {
            value: 4,
            strptr: const_cstr!("SBMGMT").as_ptr(),
        },
        value_string {
            value: 5,
            strptr: const_cstr!("NAT").as_ptr(),
        },
        value_string {
            value: 6,
            strptr: const_cstr!("SessionDiscovery").as_ptr(),
        },
        value_string {
            value: 7,
            strptr: const_cstr!("NATEcho").as_ptr(),
        },
        value_string {
            value: 8,
            strptr: const_cstr!("Routing").as_ptr(),
        },
    ];
    static ref PACKET_TYPE_NAMES: [value_string; 5] = [
        value_string {
            value: 0,
            strptr: const_cstr!("SYN").as_ptr(),
        },
        value_string {
            value: 1,
            strptr: const_cstr!("CONNECT").as_ptr(),
        },
        value_string {
            value: 2,
            strptr: const_cstr!("DATA").as_ptr(),
        },
        value_string {
            value: 3,
            strptr: const_cstr!("DISCONNECT").as_ptr(),
        },
        value_string {
            value: 4,
            strptr: const_cstr!("PING").as_ptr(),
        },
    ];
}

header_fields!(
  {
      hf_prudp_source_port {
          p_id: unsafe { &mut hf_prudp_source_port as *mut _ },
          hfinfo: header_field_info {
              name: const_cstr!("Port").as_ptr(),
              abbrev: const_cstr!("prudp.src.port").as_ptr(),
              type_: ftenum_FT_UINT8,
              display: field_display_e_BASE_DEC as i32,
              strings: null(),
              bitmask: 0x0F,
              blurb: const_cstr!("PRUDP Source Port").as_ptr(),
              id: -1,
              parent: 0,
              ref_type: hf_ref_type_HF_REF_TYPE_NONE,
              same_name_prev_id: -1,
              same_name_next: null_mut(),
          },
      }
  },
  {
      hf_prudp_source_type {
          p_id: unsafe { &mut hf_prudp_source_type as *mut _ },
          hfinfo: header_field_info {
              name: const_cstr!("Type").as_ptr(),
              abbrev: const_cstr!("prudp.src.type").as_ptr(),
              type_: ftenum_FT_UINT8,
              display: field_display_e_BASE_DEC as i32,
              strings: STREAM_TYPE_NAMES.as_ptr() as *const c_void,
              bitmask: 0xF0,
              blurb: const_cstr!("PRUDP Source Type").as_ptr(),
              id: -1,
              parent: 0,
              ref_type: hf_ref_type_HF_REF_TYPE_NONE,
              same_name_prev_id: -1,
              same_name_next: null_mut(),
          },
      }
  },
  {
      hf_prudp_dest_port {
          p_id: unsafe { &mut hf_prudp_dest_port as *mut _ },
          hfinfo: header_field_info {
              name: const_cstr!("Port").as_ptr(),
              abbrev: const_cstr!("prudp.dst.port").as_ptr(),
              type_: ftenum_FT_UINT8,
              display: field_display_e_BASE_DEC as i32,
              strings: null(),
              bitmask: 0x0F,
              blurb: const_cstr!("PRUDP Destination Port").as_ptr(),
              id: -1,
              parent: 0,
              ref_type: hf_ref_type_HF_REF_TYPE_NONE,
              same_name_prev_id: -1,
              same_name_next: null_mut(),
          },
      }
  },
  {
      hf_prudp_dest_type {
          p_id: unsafe { &mut hf_prudp_dest_type as *mut _ },
          hfinfo: header_field_info {
              name: const_cstr!("Type").as_ptr(),
              abbrev: const_cstr!("prudp.dst.type").as_ptr(),
              type_: ftenum_FT_UINT8,
              display: field_display_e_BASE_DEC as i32,
              strings: STREAM_TYPE_NAMES.as_ptr() as *const c_void,
              bitmask: 0xF0,
              blurb: const_cstr!("PRUDP Destination Type").as_ptr(),
              id: -1,
              parent: 0,
              ref_type: hf_ref_type_HF_REF_TYPE_NONE,
              same_name_prev_id: -1,
              same_name_next: null_mut(),
          },
      }
  },
  {
      hf_prudp_type {
          p_id: unsafe { &mut hf_prudp_type as *mut _ },
          hfinfo: header_field_info {
              name: const_cstr!("Type").as_ptr(),
              abbrev: const_cstr!("prudp.type").as_ptr(),
              type_: ftenum_FT_UINT8,
              display: field_display_e_BASE_DEC as i32,
              strings: PACKET_TYPE_NAMES.as_ptr() as *const c_void,
              bitmask: 0b0000_0111,
              blurb: const_cstr!("PRUDP Packet Type").as_ptr(),
              id: -1,
              parent: 0,
              ref_type: hf_ref_type_HF_REF_TYPE_NONE,
              same_name_prev_id: -1,
              same_name_next: null_mut(),
          },
      }
  },
  {
      hf_prudp_flags {
          p_id: unsafe { &mut hf_prudp_flags as *mut _ },
          hfinfo: header_field_info {
              name: const_cstr!("Flags").as_ptr(),
              abbrev: const_cstr!("prudp.flags").as_ptr(),
              type_: ftenum_FT_UINT8,
              display: field_display_e_BASE_DEC as i32,
              strings: null(),
              bitmask: 0b1111_1000,
              blurb: const_cstr!("PRUDP Flags").as_ptr(),
              id: -1,
              parent: 0,
              ref_type: hf_ref_type_HF_REF_TYPE_NONE,
              same_name_prev_id: -1,
              same_name_next: null_mut(),
          },
      }
  },
  {
      hf_prudp_flags_ack {
          p_id: unsafe { &mut hf_prudp_flags_ack as *mut _ },
          hfinfo: header_field_info {
              name: const_cstr!("Ack").as_ptr(),
              abbrev: const_cstr!("prudp.flags.ack").as_ptr(),
              type_: ftenum_FT_BOOLEAN,
              display: 5,
              strings: null(),
              bitmask: 0b0000_1000,
              blurb: const_cstr!("PRUDP Ack Flag").as_ptr(),
              id: -1,
              parent: 0,
              ref_type: hf_ref_type_HF_REF_TYPE_NONE,
              same_name_prev_id: -1,
              same_name_next: null_mut(),
          },
      }
  },
  {
      hf_prudp_flags_reliable {
          p_id: unsafe { &mut hf_prudp_flags_reliable as *mut _ },
          hfinfo: header_field_info {
              name: const_cstr!("Reliable").as_ptr(),
              abbrev: const_cstr!("prudp.flags.reliable").as_ptr(),
              type_: ftenum_FT_BOOLEAN,
              display: 5,
              strings: null(),
              bitmask: 0b0001_0000,
              blurb: const_cstr!("PRUDP Reliable Flag").as_ptr(),
              id: -1,
              parent: 0,
              ref_type: hf_ref_type_HF_REF_TYPE_NONE,
              same_name_prev_id: -1,
              same_name_next: null_mut(),
          },
      }
  },
  {
      hf_prudp_flags_need_ack {
          p_id: unsafe { &mut hf_prudp_flags_need_ack as *mut _ },
          hfinfo: header_field_info {
              name: const_cstr!("NeedAck").as_ptr(),
              abbrev: const_cstr!("prudp.flags.need_ack").as_ptr(),
              type_: ftenum_FT_BOOLEAN,
              display: 5,
              strings: null(),
              bitmask: 0b0010_0000,
              blurb: const_cstr!("PRUDP NeedAck Flag").as_ptr(),
              id: -1,
              parent: 0,
              ref_type: hf_ref_type_HF_REF_TYPE_NONE,
              same_name_prev_id: -1,
              same_name_next: null_mut(),
          },
      }
  },
  {
      hf_prudp_flags_has_size {
          p_id: unsafe { &mut hf_prudp_flags_has_size as *mut _ },
          hfinfo: header_field_info {
              name: const_cstr!("HasSize").as_ptr(),
              abbrev: const_cstr!("prudp.flags.has_size").as_ptr(),
              type_: ftenum_FT_BOOLEAN,
              display: 5,
              strings: null(),
              bitmask: 0b0100_0000,
              blurb: const_cstr!("PRUDP HasSize Flag").as_ptr(),
              id: -1,
              parent: 0,
              ref_type: hf_ref_type_HF_REF_TYPE_NONE,
              same_name_prev_id: -1,
              same_name_next: null_mut(),
          },
      }
  },
  {hf_prudp_session_id, "SessionID", "prudp.session_id", ftenum_FT_UINT8 | field_display_e_BASE_HEX, "PRUDP Session ID"},
  {hf_prudp_signature, "Signature", "prudp.signature", ftenum_FT_UINT32 | field_display_e_BASE_HEX, "PRUDP Signature"},
  {hf_prudp_sequence, "Sequence", "prudp.sequence", ftenum_FT_UINT16 | field_display_e_BASE_DEC, "PRUDP Sequence"},
  {hf_prudp_fragment_id, "FragmentID", "prudp.fragment_id", ftenum_FT_UINT8 | field_display_e_BASE_DEC, "PRUDP Fragment ID"},
  {hf_prudp_conn_signature, "Connection Signature", "prudp.conn_signature", ftenum_FT_UINT32 | field_display_e_BASE_HEX, "PRUDP Connection Signature"},
  {hf_prudp_size, "Size", "prudp.size", ftenum_FT_UINT16 | field_display_e_BASE_DEC, "PRUDP Size"},
  {hf_prudp_checksum, "Checksum", "prudp.checksum", ftenum_FT_UINT8 | field_display_e_BASE_HEX, "PRUDP Checksum"},


  {
    hf_prudp_fragments, "Fragments", "prudp.fragments", ftenum_FT_NONE | field_display_e_BASE_NONE, "PRUDP Fragments"
  },
  {
    hf_prudp_fragment, "Fragment", "prudp.fragment", ftenum_FT_FRAMENUM | field_display_e_BASE_NONE, "PRUDP Fragment"
  },
  {
    hf_prudp_fragment_overlap, "Fragment overlap", "prudp.fragment.overlap", ftenum_FT_BOOLEAN | field_display_e_BASE_NONE, "PRUDP Fragment overlap"
  },
  {
    hf_prudp_fragment_overlap_conflict, "Fragment overlap conflict", "prudp.fragment.overlap.conflict", ftenum_FT_BOOLEAN | field_display_e_BASE_NONE, "PRUDP Fragment overlapping with conflicting data"
  },
  {
    hf_prudp_fragment_multiple_tails, "Multiple tail fragments", "prudp.fragment.multiple_tails", ftenum_FT_BOOLEAN | field_display_e_BASE_NONE, "PRUDP packet has multiple tail fragments"
  },
  {
    hf_prudp_fragment_too_long_fragment, "Fragment too long", "prudp.fragment.too_long_fragment", ftenum_FT_BOOLEAN | field_display_e_BASE_NONE, "PRUDP Fragment too long"
  },
  {
    hf_prudp_fragment_error, "Fragment error", "prudp.fragment.error", ftenum_FT_FRAMENUM | field_display_e_BASE_NONE, "PRUDP Fragment error"
  },
  {
    hf_prudp_fragment_count, "Fragment count", "prudp.fragment.count", ftenum_FT_UINT32 | field_display_e_BASE_DEC, "PRUDP Fragment count"
  },
  {
    hf_prudp_reassembled_in, "Reassembled in", "prudp.reassembled.in", ftenum_FT_FRAMENUM | field_display_e_BASE_NONE, "PRUDP packet reassembled in"
  },
  {
    hf_prudp_reassembled_length, "Reassembled length", "prudp.reassembled.length", ftenum_FT_UINT32 | field_display_e_BASE_DEC, "PRUDP packet reassembled length"
  },
  {
    hf_prudp_reassembled_data, "Reassembled data", "prudp.reassembled.data", ftenum_FT_BYTES | field_display_e_BASE_NONE, "PRUDP packet reassembled data"
  }
);
