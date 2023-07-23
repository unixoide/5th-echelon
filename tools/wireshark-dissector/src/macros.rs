#![allow(unused_macros)]
// https://github.com/vchekan/wireshark-kafka/blob/master/src/macros.rs

macro_rules! header_fields {
  ( $($attrs:tt),* ) => {
      // Declare
      $(header_field_declare!($attrs);)*
      // Register
      lazy_static! {
          pub(crate) static ref HF: Vec<hf_register_info> = vec![
              $(header_field_register!($attrs),)*
          ];
      }
  };
}

macro_rules! header_field_declare {
    // String
    ( { $hf:ident, $name:tt, $abbrev:tt, $blurb:tt } ) => {
        pub(crate) static mut $hf: i32 = -1;
    };

    // Ints
    ( { $hf:ident, $name:expr, $abbrev:expr, $type_:ident $(|$display:ident)?, $blurb:expr $(, $enum:ident)? } ) => {
        pub(crate) static mut $hf: i32 = -1;
    };

    // Raw field declaration
    ( { $hf:ident $decl:tt} ) => {
        pub(crate) static mut $hf: i32 = -1;
    };
}

macro_rules! _resolve_strings {
    () => {
        std::ptr::null()
    };
    ($strings:ident) => {
        $strings.as_ptr()
    };
}

macro_rules! _resolve_display {
    () => {
        field_display_e_BASE_DEC as i32
    };
    ($display:ident) => {
        $display as i32
    };
}

macro_rules! header_field_register {
  // String
  ( { $hf:ident, $name:tt, $abbrev:tt, $blurb:expr } ) => {
      hf_register_info {
          p_id: unsafe { &mut $hf as *mut _ },
          hfinfo: header_field_info {
              name: const_cstr!($name).as_ptr(),
              abbrev: const_cstr!($abbrev).as_ptr(),
              type_: ftenum_FT_STRING,
              display: field_display_e_BASE_NONE as i32,
              strings: std::ptr::null(),
              bitmask: 0,
              blurb: _resolve_blurp!($blurb),
              id: -1,
              parent: 0,
              ref_type: hf_ref_type_HF_REF_TYPE_NONE,
              same_name_prev_id: -1,
              same_name_next: std::ptr::null_mut(),
          }
      }
  };

  // Ints
  ( { $hf:ident, $name:expr, $abbrev:expr, $type_:ident $(| $display:ident)?, $blurb:expr $(, $enum:ident)? } ) => {
      hf_register_info {
          p_id: unsafe { & $hf as *const i32 as *mut i32 },
          hfinfo: header_field_info {
              name: const_cstr!($name).as_ptr(),
              abbrev: const_cstr!($abbrev).as_ptr(),
              type_: $type_,
              display: _resolve_display!($($display)?),
              strings: _resolve_strings!($($enum)?),
              bitmask: 0,
              blurb: _resolve_blurp!($blurb),
              id: -1,
              parent: 0,
              ref_type: hf_ref_type_HF_REF_TYPE_NONE,
              same_name_prev_id: -1,
              same_name_next: std::ptr::null_mut(),
          }
      }
  };

  // Raw field declaration
  ( { $hf:ident $decl:tt} ) => {
      hf_register_info $decl
  }
}

// TODO: fix this, it always i8_str
macro_rules! _resolve_blurp {
    ( - ) => {
        std::ptr::null::<i8>()
    };
    ($s:tt) => {
        const_cstr!($s).as_ptr()
    };
}

macro_rules! ett {
  ($($ett:ident),*) => {
      // Declare
      $(pub(crate) static mut $ett : i32 = -1;)*
      // Registration helper
      pub(crate) fn create_ett() -> Vec<*mut i32> {
          vec![
              $( unsafe {(&mut $ett) as *mut _},)*
          ]
      }
  };
}

macro_rules! dissect_field {
    // i32
    ($tree:ident, $tvb:ident, $pinfo:ident, $offset:ident, $api_version:ident, $f:ident, { $hf:ident : i32 }) => {
        unsafe {
            proto_tree_add_item($tree, $hf, $tvb, $offset, 4, ENC_BIG_ENDIAN);
        }
        $offset += 4;
    };

    // i64
    ($tree:ident, $tvb:ident, $pinfo:ident, $offset:ident, $api_version:ident, $f:ident, { $hf:ident : i64 }) => {
        unsafe {
            proto_tree_add_item($tree, $hf, $tvb, $offset, 8, ENC_BIG_ENDIAN);
        }
        $offset += 8;
    };

    // i16
    ($tree:ident, $tvb:ident, $pinfo:ident, $offset:ident, $api_version:ident, $f:ident, { $hf:ident : i16 }) => {
        unsafe {
            proto_tree_add_item($tree, $hf, $tvb, $offset, 2, ENC_BIG_ENDIAN);
        }
        $offset += 2;
    };

    // u8
    ($tree:ident, $tvb:ident, $pinfo:ident, $offset:ident, $api_version:ident, $f:ident, { $hf:ident : u8 }) => {
        unsafe {
            proto_tree_add_item($tree, $hf, $tvb, $offset, 1, ENC_NA);
        }
        $offset += 1;
    };

    // Bool
    ($tree:ident, $tvb:ident, $pinfo:ident, $offset:ident, $api_version:ident, $f:ident, { $hf:ident : bool }) => {
        unsafe {
            proto_tree_add_item($tree, $hf, $tvb, $offset, 1, ENC_NA);
        }
        $offset += 1;
    };

    // String
    ($tree:ident, $tvb:ident, $pinfo:ident, $offset:ident, $api_version:ident, $f:ident, { $hf:ident : String, $ett:ident }) => {
        unsafe {
            $offset = dissect_kafka_string($tree, $hf, $ett, $tvb, $pinfo, $offset);
        }
    };

    // Array
    ($tree:ident, $tvb:ident, $pinfo:ident, $offset:ident, $api_version:ident, $f:ident, [$t:ident $ett:ident]) => {
        $offset = {
            let tree = unsafe {
                proto_tree_add_subtree(
                    $tree,
                    $tvb,
                    $offset,
                    -1,
                    $ett,
                    std::ptr::null_mut(),
                    const_cstr!(stringify!($f)).as_ptr(),
                )
            };
            dissect_kafka_array($tvb, $pinfo, tree, $offset, $api_version, $t::dissect)
        };
    };

    // Function call
    ($tree:ident, $tvb:ident, $pinfo:ident, $offset:ident, $api_version:ident, $f:ident, { $_fn:ident : fn }) => {
        $offset = $_fn($tree, $tvb, $pinfo, $offset, $api_version);
    };

    // Function call with one argument
    ($tree:ident, $tvb:ident, $pinfo:ident, $offset:ident, $api_version:ident, $f:ident, { $_fn:ident($arg:ident) }) => {
        unsafe {
            $_fn($arg, $tree, $tvb, $pinfo, $offset);
        }
    };
}

macro_rules! _resolve_version {
    ($api_version:ident, ($from:ident - $to:ident) ) => {
        $api_version >= $from && api_version < $to
    };
    ($api_version:ident, $version:expr) => {
        if $version >= 0 {
            $api_version >= $version
        } else {
            $api_version <= -$version
        }
    };
    () => {
        true
    };
}

macro_rules! protocol {
  ($sname:ident => { $( $f:ident $(/$version:tt)? : $tp:tt ),* } ) => {
      pub(crate) struct $sname {}

      impl $sname {
          pub(crate) fn dissect(tvb: *mut tvbuff_t, pinfo: *mut packet_info, tree: *mut proto_tree, mut offset: i32, api_version: i16) -> i32 {
              $(
                  let version_match = _resolve_version!($(api_version, $version)? );
                  if version_match {
                      dissect_field!(tree, tvb, pinfo, offset, api_version, $f, $tp);
                  }
              )*
              offset
          }
      }
  };
}
