use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;
use std::ops::Range;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::LazyLock;

use hooks_config::Hook;
use iced_x86::Decoder;
use iced_x86::DecoderOptions;
use patterns::Pattern;
use serde::Deserialize;
use serde::Serialize;
use sha2::digest::crypto_common::BlockSizeUser;
use sha2::Digest;
use tracing::error;
use tracing::info;
use tracing::instrument;

pub mod patterns;

pub type Address = usize;
pub type NopRange = Range<Address>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("no file provided in {0:?}")]
    NoFileName(PathBuf),
    #[error("{0} is unknown")]
    UnknownBinary(String),
    #[error("{0} was modified or the version is not supported.\n\nPlease share {0} with the project, so that support can be implemented.\n\nHash: {1}")]
    BinaryMismatch(String, String),
    #[error("couldn't read binary")]
    IO(#[from] std::io::Error),
    #[error("couldn't identify binary")]
    IdFailed,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Addresses {
    pub global_onlineconfig_client: Address,
    pub onlineconfig_url: Address,
    pub unreal_commandline: Option<Address>,

    pub debug_print: Option<(Address, Vec<NopRange>)>,

    // hooks
    pub func_printer: Option<Address>,
    pub func_net_finite_state_machine_next_state: Option<Address>,
    pub func_net_finite_state_leave_state: Option<Address>,
    pub func_net_result_base: Option<Address>,
    pub func_something_with_goal: Option<Address>,
    pub func_quazal_stepsequencejob_setstep: Option<Address>,
    pub func_thread_starter: Option<Address>,
    pub func_goal_change_state: Option<Address>,
    pub func_net_core: Option<Address>,
    pub func_net_result_core: Option<Address>,
    pub func_net_result_session: Option<Address>,
    pub func_net_result_lobby: Option<Address>,
    pub func_storm_host_port_to_str: Option<Address>,
    pub func_generate_id: Option<Address>,
    pub func_storm_maybe_set_state: Option<Address>,
    pub func_storm_statemachineaction_execute: Option<Address>,
    pub func_storm_some_error_formatter: Option<Address>,
    pub func_gear_str_destructor: Option<Address>,
    pub func_storm_event_dispatch: Option<Address>,
    pub func_storm_event_dispatch2: Option<Address>,
    pub func_storm_event_maybe_queue_pop: Option<Address>,
    pub func_some_gear_str_constructor: Option<Address>,
    pub func_net_result_rdv_session: Option<Address>,
    pub func_rmc_init_message: Option<Address>,
    pub func_rmc_add_method_id: Option<Address>,
    pub func_rmc_send_message: Option<Address>,
    pub func_storm_event_handler: Option<Address>,
    pub func_another_gear_str_destructor: Option<Address>,
    pub func_open_file_from_archive: Option<Address>,
}

#[derive(Serialize, Deserialize)]
struct SavedAddresses {
    dx9: HashMap<String, Addresses>,
    dx11: HashMap<String, Addresses>,
}

macro_rules! option2vec {
    ($($e:expr),+) => {
        match vec![$($e),+].into_iter().flatten().collect::<Vec<_>>() {
            x if !x.is_empty() => Some(x),
            _ => None,
        }
    }
}

impl Addresses {
    /// Returns a vector of addresses for this hook or None if no addresses are available.
    /// The vector can be empty, for example when the addresses are calculated at runtime.
    pub fn hook_addr(&self, hook: Hook) -> Option<Vec<Address>> {
        match hook {
            Hook::Printer => option2vec!(self.func_printer),
            Hook::LeaveState => option2vec!(self.func_net_finite_state_leave_state),
            Hook::NextState => option2vec!(self.func_net_finite_state_machine_next_state),
            Hook::NetResultBase => option2vec!(self.func_net_result_base),
            Hook::Goal => option2vec!(self.func_something_with_goal),
            Hook::SetStep => option2vec!(self.func_quazal_stepsequencejob_setstep),
            Hook::Thread => option2vec!(self.func_thread_starter),
            Hook::ChangeState => option2vec!(self.func_goal_change_state),
            Hook::NetCore => option2vec!(self.func_net_core),
            Hook::NetResultCore => option2vec!(self.func_net_result_core),
            Hook::NetResultSession => option2vec!(self.func_net_result_session),
            Hook::NetResultRdvSession => option2vec!(self.func_net_result_rdv_session),
            Hook::NetResultLobby => option2vec!(self.func_net_result_lobby),
            Hook::StormHostPortToString => option2vec!(self.func_storm_host_port_to_str),
            Hook::GenerateID => option2vec!(self.func_generate_id),
            Hook::StormSetState => option2vec!(self.func_storm_maybe_set_state),
            Hook::StormStateMachineActionExecute => {
                option2vec!(self.func_storm_statemachineaction_execute)
            }
            Hook::StormErrorFormatter => option2vec!(self.func_storm_some_error_formatter),
            Hook::GearStrDestructor => option2vec!(
                self.func_gear_str_destructor,
                self.func_another_gear_str_destructor,
                self.func_some_gear_str_constructor
            ),
            Hook::StormEventDispatcher => option2vec!(
                self.func_storm_event_dispatch,
                self.func_storm_event_dispatch2,
                self.func_storm_event_maybe_queue_pop,
                self.func_storm_event_handler
            ),
            Hook::RMCMessages => option2vec!(
                self.func_rmc_init_message,
                self.func_rmc_add_method_id,
                self.func_rmc_send_message
            ),
            // dynamically found at runtime
            Hook::GetAdaptersInfo | Hook::Gethostbyname | Hook::StormPackets => Some(vec![]),
            #[cfg(feature = "modding")]
            Hook::OverridePackaged => option2vec!(self.func_open_file_from_archive),
        }
    }

    pub fn set_hook_addrs(&mut self, hook: Hook, addrs: Vec<Address>) {
        let mut addrs = addrs.into_iter();
        match hook {
            Hook::Printer => self.func_printer = addrs.next(),
            Hook::LeaveState => self.func_net_finite_state_leave_state = addrs.next(),
            Hook::NextState => self.func_net_finite_state_machine_next_state = addrs.next(),
            Hook::NetResultBase => self.func_net_result_base = addrs.next(),
            Hook::Goal => self.func_something_with_goal = addrs.next(),
            Hook::SetStep => self.func_quazal_stepsequencejob_setstep = addrs.next(),
            Hook::Thread => self.func_thread_starter = addrs.next(),
            Hook::ChangeState => self.func_goal_change_state = addrs.next(),
            Hook::NetCore => self.func_net_core = addrs.next(),
            Hook::NetResultCore => self.func_net_result_core = addrs.next(),
            Hook::NetResultSession => self.func_net_result_session = addrs.next(),
            Hook::NetResultRdvSession => self.func_net_result_rdv_session = addrs.next(),
            Hook::NetResultLobby => self.func_net_result_lobby = addrs.next(),
            Hook::StormHostPortToString => self.func_storm_host_port_to_str = addrs.next(),
            Hook::GenerateID => self.func_generate_id = addrs.next(),
            Hook::StormSetState => self.func_storm_maybe_set_state = addrs.next(),
            Hook::StormStateMachineActionExecute => {
                self.func_storm_statemachineaction_execute = addrs.next()
            }
            Hook::StormErrorFormatter => self.func_storm_some_error_formatter = addrs.next(),
            Hook::GearStrDestructor => {
                self.func_gear_str_destructor = addrs.next();
                self.func_another_gear_str_destructor = addrs.next();
                self.func_some_gear_str_constructor = addrs.next();
            }
            Hook::StormEventDispatcher => {
                self.func_storm_event_dispatch = addrs.next();
                self.func_storm_event_dispatch2 = addrs.next();
                self.func_storm_event_maybe_queue_pop = addrs.next();
                self.func_storm_event_handler = addrs.next();
            }
            Hook::RMCMessages => {
                self.func_rmc_init_message = addrs.next();
                self.func_rmc_add_method_id = addrs.next();
                self.func_rmc_send_message = addrs.next();
            }
            // dynamically found at runtime
            Hook::GetAdaptersInfo | Hook::Gethostbyname | Hook::StormPackets => {}
            #[cfg(feature = "modding")]
            Hook::OverridePackaged => self.func_open_file_from_archive = addrs.next(),
        }
    }
}

fn dx9_addresses() -> HashMap<[u8; 32], Addresses> {
    #![allow(clippy::unreadable_literal)]

    let dx9_addrs = Addresses {
        global_onlineconfig_client: 0x032bf5bc,
        onlineconfig_url: 0x02cc0650,
        unreal_commandline: Some(0x0323b97c),

        debug_print: Some((
            0x033d5c94,
            vec![0x021f1144..0x021f1144 + 5, 0x021f1167..0x021f1167 + 4],
        )),

        // hooks
        func_printer: Some(0x04b19e0),
        func_net_finite_state_machine_next_state: Some(0x00ad0260),
        func_net_finite_state_leave_state: Some(0x00a9aa40),
        func_net_result_base: Some(0x00a9a180),
        func_something_with_goal: Some(0x0ae5130),
        func_quazal_stepsequencejob_setstep: Some(0x02138f10),
        func_thread_starter: Some(0x07be840),
        func_goal_change_state: Some(0x0af3e20),
        func_net_core: Some(0x0b1cc10),
        func_net_result_core: Some(0x00ab9a80),
        func_net_result_session: Some(0x0ab98b0),
        func_net_result_lobby: Some(0x0a9d7f0),
        func_storm_host_port_to_str: Some(0x020b9860),
        func_generate_id: Some(0x020b0240),
        func_storm_maybe_set_state: Some(0x020cad40),
        func_storm_statemachineaction_execute: Some(0x020dc860),
        func_storm_some_error_formatter: Some(0x020bb250),
        func_gear_str_destructor: Some(0x04c58a0),
        func_storm_event_dispatch: Some(0x02055600),
        func_storm_event_dispatch2: Some(0x020ce030),
        func_storm_event_maybe_queue_pop: Some(0x20c46e0),
        func_some_gear_str_constructor: Some(0x004b19e0),
        func_net_result_rdv_session: Some(0x00a9c4b0),
        func_rmc_init_message: Some(0x021a6550),
        func_rmc_add_method_id: Some(0x021a6210),
        func_rmc_send_message: Some(0x021b7050),
        func_storm_event_handler: Some(0x020ca850),
        func_another_gear_str_destructor: Some(0x004c58a0),
        func_open_file_from_archive: Some(0x070_9040),
    };

    let dx9_hashes = [
        [
            0x15, 0x8e, 0xfc, 0x5d, 0x9, 0x40, 0xfc, 0xaf, 0x3e, 0x4b, 0x16, 0x95, 0x2c, 0x8f,
            0x88, 0x61, 0xe1, 0x60, 0x61, 0x50, 0x9d, 0x9e, 0xb8, 0xeb, 0x5f, 0xf4, 0xae, 0x32,
            0x49, 0xbb, 0x5a, 0x5,
        ],
        // Same binary except of 2 bytes of the COFF header checksum?? (offset 0x1a9 and 0x1aa)
        [
            0x7f, 0xcd, 0x3a, 0x18, 0xd4, 0xdc, 0xc6, 0x92, 0x71, 0x99, 0x84, 0xb0, 0x72, 0x68,
            0xbd, 0x42, 0x76, 0x41, 0x8, 0xe7, 0xdf, 0x37, 0x4, 0x9f, 0x14, 0x90, 0xf2, 0x9, 0x29,
            0xf9, 0x92, 0x5d,
        ],
        /*
        Byte Diffs :
            Address   Program1  Program2
            004001a8    0xe7       0x26
            004001a9    0x68       0x3c
            Address   Program1  Program2
            004d0c73    0xf2       0xe9
            004d0c74    0x0f       0x42
            004d0c75    0x59       0x08
            004d0c76    0x05       0x0c
            004d0c77    0x80       0x02
            004d0c78    0x70       0x90
            004d0c79    0xd5       0x90
            004d0c7a    0x02       0x90
            Address   Program1  Program2
            004d0c91    0xf3       0xe9
            004d0c92    0x0f       0x3b
            004d0c93    0x10       0x08
            004d0c94    0x46       0x0c
            004d0c95    0x40       0x02
            Address   Program1  Program2
            006c08e7    0xf3       0xe9
            006c08e8    0x0f       0x66
            006c08e9    0x10       0x0b
            006c08ea    0x05       0xed
            006c08eb    0xc8       0x01
            006c08ec    0x4e       0x90
            006c08ed    0x23       0x90
            006c08ee    0x03       0x90
            Address   Program1  Program2
            006e17d6    0xf3       0xe9
            006e17d7    0x0f       0x7e
            006e17d8    0x10       0x64
            006e17d9    0x45       0x25
            006e17da    0x20       0x02
            Address   Program1  Program2
            0071a4f0    0x0f       0xe9
            0071a4f1    0x84       0xcb
            0071a4f2    0xca       0x01
            0071a4f3    0x01       0x00
            0071a4f5    0x00       0x90
            Address   Program1  Program2
            007eb33a    0xf2       0xe9
            007eb33b    0x0f       0x77
            007eb33c    0x10       0xc9
            007eb33d    0x0d       0x14
            007eb33e    0xe8       0x02
            007eb33f    0x70       0x90
            007eb340    0xd5       0x90
            007eb341    0x02       0x90
            Address   Program1  Program2
            0090eafa    0x83       0xe9
            0090eafb    0xbb       0x68
            0090eafc    0x34       0x29
            0090eafd    0x01       0xc8
            0090eafe    0x00       0x01
            0090eaff    0x00       0x90
            0090eb00    0x01       0x90
            Address   Program1  Program2
            00b27b19    0x00       0x11
            Address   Program1  Program2
            00d18bdf    0x76       0xeb
            Address   Program1  Program2
            00d198e0    0x10       0x00
            Address   Program1  Program2
            00fd1423    0xf3       0xe9
            00fd1424    0x0f       0x5d
            00fd1425    0x10       0x00
            00fd1426    0x46       0x5c
            00fd1427    0x34       0x01
            Address   Program1  Program2
            0195aff5    0x07       0x08
            0195aff6    0xaf       0xcc
            0195aff7    0x07       0xfd
            Address   Program1  Program2
            019d8040    0xf3       0xe9
            019d8041    0x0f       0xf5
            019d8042    0x10       0xfb
            019d8043    0x0d       0xf5
            019d8044    0x6c       0x00
            019d8045    0xf3       0x90
            019d8046    0x94       0x90
            019d8047    0x02       0x90
         */
        [
            0xaf, 0x06, 0x72, 0x09, 0x45, 0x88, 0x04, 0xe6, 0xad, 0xce, 0xca, 0x43, 0x2b, 0x50,
            0xfb, 0x25, 0x52, 0x08, 0x71, 0x6f, 0xb2, 0x30, 0x37, 0xe8, 0x25, 0x24, 0x4d, 0xbc,
            0x9f, 0x3b, 0x19, 0x5c,
        ],
    ];

    HashMap::from(dx9_hashes.map(|h| (h, dx9_addrs.clone())))
}

fn dx11_addresses() -> HashMap<[u8; 32], Addresses> {
    #![allow(clippy::unreadable_literal)]

    let dx11_hashes = [
        [
            0xc6, 0xb9, 0xf3, 0x30, 0xfa, 0xc1, 0x41, 0x2f, 0x19, 0xf3, 0x2a, 0x6f, 0xd8, 0x6e,
            0xdb, 0x4c, 0x66, 0x29, 0x1a, 0x69, 0x2, 0x61, 0x1e, 0x94, 0x33, 0xb9, 0xb0, 0xea,
            0x65, 0x9e, 0xb4, 0xbc,
        ],
        /*
        Byte Diffs :
            Address   Program1  Program2
            00400198    0x69       0x51
            00400199    0x97       0x98
         */
        [
            0xc5, 0x2b, 0x3d, 0x09, 0x27, 0x59, 0x1e, 0x47, 0x74, 0x24, 0xf3, 0x89, 0xff, 0x0b,
            0x13, 0x14, 0xa3, 0x00, 0x93, 0x8e, 0x19, 0xce, 0x61, 0xa7, 0xb0, 0xa7, 0xbc, 0x09,
            0xf8, 0x1c, 0x2c, 0x89,
        ],
        /*
        Byte Diffs :
            Address   Program1  Program2
            00400198    0x51       0x69
            00400199    0x98       0x97
            Address   Program1  Program2
            0042ccbe    0xf3       0xe9
            0042ccbf    0x0f       0xea
            0042ccc0    0x10       0x5c
            0042ccc1    0x45       0x54
            0042ccc2    0x20       0x02
            Address   Program1  Program2
            0046c860    0x0f       0xe9
            0046c861    0x84       0xcb
            0046c862    0xca       0x01
            0046c863    0x01       0x00
            0046c865    0x00       0x90
            Address   Program1  Program2
            00542aea    0xf2       0xe9
            00542aeb    0x0f       0x58
            00542aec    0x10       0x77
            00542aed    0x0d       0x08
            00542aee    0x60       0x02
            00542aef    0x5c       0x90
            00542af0    0xd7       0x90
            00542af1    0x02       0x90
            Address   Program1  Program2
            0066fe91    0x83       0xe9
            0066fe92    0xbb       0x51
            0066fe93    0x34       0x14
            0066fe94    0x01       0x30
            0066fe95    0x00       0x02
            0066fe96    0x00       0x90
            0066fe97    0x01       0x90
            Address   Program1  Program2
            0088cfc9    0x00       0x11
            Address   Program1  Program2
            00a7f7ff    0x76       0xeb
            Address   Program1  Program2
            00a80670    0x10       0x00
            Address   Program1  Program2
            00d3b853    0xf3       0xe9
            00d3b854    0x0f       0xc4
            00d3b855    0x10       0x5a
            00d3b856    0x46       0xc3
            00d3b857    0x34       0x01
            Address   Program1  Program2
            016db3c8    0x54       0xc5
            016db3c9    0x2b       0x5e
            016db3ca    0x08       0x29
            016db3cb    0x00       0x01
            Address   Program1  Program2
            0175f4b0    0xf3       0xe9
            0175f4b1    0x0f       0xfc
            0175f4b2    0x10       0x1e
            0175f4b3    0x0d       0x21
            0175f4b4    0x00       0x01
            0175f4b5    0x5c       0x90
            0175f4b6    0xd7       0x90
            0175f4b7    0x02       0x90
            Address   Program1  Program2
            018f20a3    0xf2       0xe9
            018f20a4    0x0f       0xc7
            018f20a5    0x59       0xf2
            018f20a6    0x05       0x07
            018f20a7    0x50       0x01
            018f20a8    0xf9       0x90
            018f20a9    0xc1       0x90
            018f20aa    0x02       0x90
            Address   Program1  Program2
            018f20c1    0xf3       0xe9
            018f20c2    0x0f       0xc0
            018f20c3    0x10       0xf2
            018f20c4    0x46       0x07
            018f20c5    0x40       0x01
            Address   Program1  Program2
            01acb93a    0xf3       0xe9
            01acb93b    0x0f       0xc6
            01acb93c    0x10       0x59
            01acb93d    0x46       0xea
            01acb93e    0x50       0x00

         */
        [
            0xb5, 0x21, 0xc4, 0xb0, 0x23, 0x62, 0x72, 0xe7, 0x3f, 0xcb, 0xf9, 0x35, 0xdb, 0x33,
            0x80, 0xa8, 0x4b, 0x7c, 0x0f, 0xa9, 0x80, 0x47, 0x66, 0x43, 0x77, 0xdb, 0x1c, 0xda,
            0xaf, 0xde, 0x06, 0x7f,
        ],
    ];

    let dx11_addrs = Addresses {
        global_onlineconfig_client: 0x0338d5fc,
        onlineconfig_url: 0x02d12b60,
        unreal_commandline: Some(0x33099B4),

        debug_print: Some((
            0x34CA4D4,
            vec![0x02218634..0x02218634 + 5, 0x02218657..0x02218657 + 4],
        )),

        // hooks
        func_printer: Some(0x636D20),
        func_net_finite_state_machine_next_state: Some(0x834FF0),
        func_net_finite_state_leave_state: Some(0x7FE000),
        func_net_result_base: Some(0x7FD740),
        func_something_with_goal: Some(0x849F80),
        func_quazal_stepsequencejob_setstep: Some(0x2160400),
        func_thread_starter: Some(0x5132C0),
        func_goal_change_state: Some(0x858ED0),
        func_net_core: Some(0x8820C0),
        func_net_result_core: Some(0x81E020),
        func_net_result_session: Some(0x81DE50),
        func_net_result_lobby: Some(0x801260),
        func_storm_host_port_to_str: Some(0x020E0D50),
        func_generate_id: Some(0x020D7730),
        func_storm_maybe_set_state: Some(0x020F2230),
        func_storm_statemachineaction_execute: Some(0x02103D50),
        func_storm_some_error_formatter: Some(0x020E2740),
        func_gear_str_destructor: Some(0x41F630),
        func_storm_event_dispatch: Some(0x0207CAF0),
        func_storm_event_dispatch2: Some(0x020F5520),
        func_storm_event_maybe_queue_pop: Some(0x020EBBD0),
        func_some_gear_str_constructor: Some(0x636D20),
        func_net_result_rdv_session: Some(0x7FFD30),
        func_rmc_init_message: Some(0x021CDA40),
        func_rmc_add_method_id: Some(0x021CD700),
        func_rmc_send_message: Some(0x021DE540),
        func_storm_event_handler: Some(0x20f1d40),
        func_another_gear_str_destructor: Some(0x41F630),
        func_open_file_from_archive: Some(0x45ab20),
    };
    HashMap::from(dx11_hashes.map(|h| (h, dx11_addrs.clone())))
}

#[instrument]
fn load_custom_addresses_default() -> HashMap<String, HashMap<[u8; 32], Addresses>> {
    let Ok(exec_path) = env::current_exe() else {
        error!("Unable to find current exectuable");
        return HashMap::default();
    };
    load_custom_addresses(exec_path.parent().unwrap())
}

#[instrument]
pub fn load_custom_addresses(dir: &Path) -> HashMap<String, HashMap<[u8; 32], Addresses>> {
    let fname = "5th-echelon-addresses.json";

    let filepath = dir.join(fname);

    info!("Trying to load addresses from {filepath:?}");

    let f = match File::open(&filepath) {
        Ok(f) => f,
        Err(e) => {
            error!("Error reading {filepath:?}: {e}");
            return HashMap::default();
        }
    };

    let saved: SavedAddresses = match serde_json::from_reader(f) {
        Ok(s) => s,
        Err(e) => {
            error!("Error parsing {filepath:?}: {e}");
            return HashMap::default();
        }
    };

    let dehex = |s: String| -> anyhow::Result<[u8; 32]> {
        anyhow::ensure!(s.len() == 64, "hash should be 64 hex chars");
        anyhow::ensure!(s.is_ascii(), "hash should be a hex string");
        anyhow::ensure!(
            s.chars().all(|c| c.is_ascii_hexdigit()),
            "hash should be a hex string"
        );
        let mut out = [0u8; 32];
        for i in 0..s.len() / 2 {
            out[i] = u8::from_str_radix(&s[i * 2..i * 2 + 2], 16)?;
        }
        Ok(out)
    };

    let convert = |hm: HashMap<String, Addresses>| -> anyhow::Result<HashMap<[u8; 32], Addresses>> {
        hm.into_iter()
            .map(|(key, value)| Ok((dehex(key)?, value)))
            .collect()
    };

    let build = || -> anyhow::Result<_> {
        Ok(HashMap::from([
            (String::from("blacklist_game.exe"), convert(saved.dx9)?),
            (
                String::from("blacklist_dx11_game.exe"),
                convert(saved.dx11)?,
            ),
        ]))
    };

    match build() {
        Ok(res) => {
            let loaded = res
                .iter()
                .map(|(k, v)| format!("{k} ({})", v.keys().map(hex).collect::<Vec<_>>().join(", ")))
                .collect::<Vec<_>>()
                .join(", ");
            info!("Loaded custom addresses for {loaded}");
            res
        }
        Err(e) => {
            error!("Error parsing {filepath:?}: {e}");
            return HashMap::default();
        }
    }
}

#[instrument]
fn save_builtin_addresses() {
    let Ok(exec_path) = env::current_exe() else {
        error!("Unable to find current exectuable");
        return;
    };
    let mut inbuilt = inbuilt_addresses();
    save_addresses_impl(
        exec_path.parent().unwrap(),
        inbuilt.remove("blacklist_game.exe").unwrap_or_default(),
        inbuilt
            .remove("blacklist_dx11_game.exe")
            .unwrap_or_default(),
        false,
    )
}

fn save_addresses_impl(
    dir: &Path,
    dx9: HashMap<[u8; 32], Addresses>,
    dx11: HashMap<[u8; 32], Addresses>,
    overwrite: bool,
) {
    let fname = "5th-echelon-addresses.json";

    let filepath = dir.join(fname);

    if !overwrite && filepath.exists() {
        info!("{filepath:?} already exists, no need to save");
        return;
    }

    info!("Trying to save internal addresses to {filepath:?}");

    let f = match File::create(&filepath) {
        Ok(f) => f,
        Err(e) => {
            error!("Error opening {filepath:?}: {e}");
            return;
        }
    };

    let hex = |b: [u8; 32]| -> String {
        b.into_iter()
            .map(|b| format!("{b:02x}"))
            .collect::<Vec<_>>()
            .join("")
    };

    let convert = |a: HashMap<[u8; 32], Addresses>| -> HashMap<String, Addresses> {
        a.into_iter()
            .map(|(key, value)| {
                let key = hex(key);
                (key, value)
            })
            .collect()
    };

    let dx9 = convert(dx9);
    let dx11 = convert(dx11);

    let saved = SavedAddresses { dx9, dx11 };
    if let Err(e) = serde_json::to_writer_pretty(f, &saved) {
        error!("Error saving inbuilt addresses: {e:?}");
    }
}

#[instrument]
pub fn save_addresses(
    dir: &Path,
    dx9: HashMap<[u8; 32], Addresses>,
    dx11: HashMap<[u8; 32], Addresses>,
) {
    save_addresses_impl(dir, dx9, dx11, true);
}

fn inbuilt_addresses() -> HashMap<String, HashMap<[u8; 32], Addresses>> {
    HashMap::from([
        (String::from("blacklist_game.exe"), dx9_addresses()),
        (String::from("blacklist_dx11_game.exe"), dx11_addresses()),
    ])
}

fn build_game_map(dir: &Path) -> HashMap<String, HashMap<[u8; 32], Addresses>> {
    save_builtin_addresses();
    let mut map = inbuilt_addresses();
    let mut custom = load_custom_addresses(dir);

    // make sure that the inbuilt addresses always take precedence
    for (key, value) in map.iter_mut() {
        if let Some(addrs) = custom.remove(key) {
            for (k, v) in addrs.into_iter() {
                value.entry(k).or_insert(v);
            }
        }
    }

    map
}

pub fn hash_file(path: impl AsRef<Path>) -> anyhow::Result<[u8; 32]> {
    let path = path.as_ref();
    let mut f = std::fs::File::open(path)?;
    let mut hasher = sha2::Sha256::new();
    let mut buf = vec![0u8; sha2::Sha256::block_size() * 16];

    while let Ok(n) = f.read(&mut buf) {
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }

    Ok(hasher.finalize().into())
}

fn hex(data: impl AsRef<[u8]>) -> String {
    #[allow(clippy::format_collect)]
    data.as_ref().iter().map(|b| format!("{b:02x}")).collect()
}

pub fn get_from_path(filepath: &Path) -> Result<Addresses, Error> {
    let file_name = filepath
        .file_name()
        .ok_or_else(|| Error::NoFileName(filepath.to_path_buf()))?
        .to_ascii_lowercase()
        .to_string_lossy()
        .into_owned();
    info!("Loaded by {file_name}");

    let digest = hash_file(filepath);

    let game_map = build_game_map(filepath.parent().unwrap());
    let game_map = game_map
        .get(&file_name)
        .ok_or(Error::UnknownBinary(file_name.clone()))?;

    digest
        .ok()
        .map(|digest| {
            game_map
                .get(&digest)
                .ok_or(Error::BinaryMismatch(file_name.clone(), hex(digest)))
        })
        .unwrap_or(Err(Error::BinaryMismatch(file_name, Default::default())))
        .cloned()
}

fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}

static ONLINE_CLIENT_CFG_PATTERN: LazyLock<Pattern> = LazyLock::new(|| {
    Pattern::from_str(
        "A1 ?? ?? ?? ?? 50 8D 4? ?? 51 8B 0D ?? ?? ?? ?? 8D 5? ?? 52 8D 4? ?? 50 E8 ?? ?? ?? ??",
    )
    .unwrap()
});

static CMDLINE_PATTERN: LazyLock<Pattern> = LazyLock::new(|| {
    Pattern::from_str(
"68 ?? ?? ?? ?? E8 ?? ?? ?? ?? 83 C4 ?? A3 ?? ?? ?? ?? 3B FB 0F 85 ?? ?? ?? ?? 68 ?? ?? ?? ?? 8D 9? ?? ?? ?? ?? 53 52 C6 8? ?? ?? ?? ?? ??"
)
.unwrap()
});

static HOOK_PATTERNS: LazyLock<Vec<(Hook, Vec<Pattern>)>> = LazyLock::new(|| {
    vec![
        (Hook::Printer, vec![Pattern::from_str("55 8B EC 56 57 8B F1 E8 ?? ?? ?? ?? 8B 7D ?? 89 06 33 C0 C6 46 ?? ?? 89 46 ?? C7 46 ?? ?? ?? ?? ?? C7 46 ?? ?? ?? ?? ??").unwrap()]),
        (Hook::LeaveState, vec![Pattern::from_str("55 8B EC 56 57 8B F1 8D 7E ?? 57 FF 15 ?? ?? ?? ?? 8B 4D ?? 8B 06").unwrap()]),
        (Hook::NextState, vec![Pattern::from_str("55 8B EC 6A ?? 68 ?? ?? ?? ?? 64 A1 ?? ?? ?? ?? 50 83 EC ?? 53 56 57 A1 ?? ?? ?? ?? 33 C5 50 8D 45 ?? 64 A3 ?? ?? ?? ?? 8B F9 8B 45 ?? 33 DB 89 5D ?? 8B 08").unwrap()]),
        (Hook::NetResultBase, vec![Pattern::from_str("55 8B EC 6A ?? 68 ?? ?? ?? ?? 64 A1 ?? ?? ?? ?? 50 51 56 A1 ?? ?? ?? ?? 33 C5 50 8D 45 ?? 64 A3 ?? ?? ?? ?? 8B C1 89 45 ?? 8B 4D ?? C7 00 ?? ?? ?? ?? 8B 11 89 50 ?? 8B 51 ?? 89 50 ?? C7 40 ?? ?? ?? ?? ?? C7 40 ?? ?? ?? ?? ?? 8B 51 ?? 85 D2 74 ?? BE ?? ?? ?? ?? F0 0F C1 32 8B 49 ?? 89 48 ?? EB ?? C7 40 ?? ?? ?? ?? ?? C7 45 ?? ?? ?? ?? ?? 8B 4D ??").unwrap()]),
        (Hook::Goal, vec![]),
        (Hook::SetStep, vec![Pattern::from_str("55 8B EC 83 EC ?? 56 8D 45 ?? 6A ?? 50 8B F1 E8 ?? ?? ?? ?? 8B 4D ?? 8B 55 ?? 8B 45 ??").unwrap()]),
        (Hook::Thread, vec![Pattern::from_str("56 8B F1 8B 46 ?? C6 86 ?? ?? ?? ?? ?? 83 F8 ??").unwrap()]),
        (Hook::ChangeState, vec![Pattern::from_str("55 8B EC 56 8B F1 8B 4D ?? 8B 01 57 8B 3D ?? ?? ?? ??").unwrap()]),
        (Hook::NetCore, vec![Pattern::from_str("55 8B EC 6A ?? 68 ?? ?? ?? ?? 64 A1 ?? ?? ?? ?? 50 83 EC ?? 53 56 57 A1 ?? ?? ?? ?? 33 C5 50 8D 45 ?? 64 A3 ?? ?? ?? ?? 8B F1 89 75 ?? E8 ?? ?? ?? ?? 33 DB 89 5D ?? 8D 8E ?? ?? ?? ?? C7 06 ?? ?? ?? ?? E8 ?? ?? ?? ??").unwrap()]),
        (Hook::NetResultCore, vec![]),
        (Hook::NetResultSession, vec![]),
        (Hook::NetResultRdvSession, vec![]),
        (Hook::NetResultLobby, vec![]),
        (Hook::StormHostPortToString, vec![Pattern::from_str("55 8B EC 83 EC ?? 53 56 6A ?? 8D 45 ?? 6A ??").unwrap()]),
        (Hook::GenerateID, vec![Pattern::from_str("55 8B EC 8B 45 ?? 56 8B F1 85 C0 74 ?? 80 38 ?? 74 ?? 80 7D ?? ??").unwrap()]),
        (Hook::StormSetState, vec![Pattern::from_str("55 8B EC 8B 41 ?? 8B 55 ?? 85 C0 74 ?? 8B 40 ??").unwrap()]),
        (Hook::StormStateMachineActionExecute, vec![]),
        (Hook::StormErrorFormatter, vec![Pattern::from_str("55 8B EC 83 EC ?? 56 8B F1 57 8D 4D ?? E8 ?? ?? ?? ?? 68 ?? ?? ?? ??").unwrap()]),
        (Hook::GearStrDestructor, vec![Pattern::from_str("55 8B EC 56 57 8B F1 E8 ?? ?? ?? ?? 8B 7D ?? 89 06 33 C0 C6 46 ?? ?? 89 46 ?? C7 46 ?? ?? ?? ?? ?? C7 46 ?? ?? ?? ?? ??").unwrap()]),
        (Hook::StormEventDispatcher, vec![Pattern::from_str("55 8B EC 51 53 56 57 8B F9 8D 4D ??").unwrap(), Pattern::from_str("55 8B EC 8B 89 ?? ?? ?? ?? 56 85 C9 74 ?? 80 79 ?? ?? 74 ?? 8B 45 ?? 8B 75 ?? 83 F8 ?? 75 ?? 68 ?? ?? ?? ?? 8B CE E8 ?? ?? ?? ?? 8B C6 5E 5D C2 ?? ?? 8B 55 ?? 52").unwrap(), Pattern::from_str("55 8B EC 83 EC ?? 53 56 8B F1 57 33 DB").unwrap(), Pattern::from_str("55 8B EC 8B 45 ?? 53 56 57 8B F1 85 C0 0F 84 ?? ?? ?? ??").unwrap()]),
        (Hook::RMCMessages, vec![Pattern::from_str("55 8B EC 8B 45 ?? 8B 4D ?? 8B 55 ?? 50 51 52 E8 ?? ?? ?? ?? 83 C4 ?? 5D C3 CC CC CC CC CC CC CC C3").unwrap(), Pattern::from_str("55 8B EC 8B 4D ?? 6A ?? 6A ?? 8D 45 ?? 50 E8 ?? ?? ?? ?? 5D C3 CC CC CC CC CC CC CC CC CC CC CC 55 8B EC 51 8B 4D ??").unwrap(), Pattern::from_str("55 8B EC 83 EC ?? 53 8B 5D ?? 56 57 8B F9 85 DB 0F 84 ?? ?? ?? ?? 80 7F ?? ??").unwrap()]),
        #[cfg(feature = "modding")]
        (Hook::OverridePackaged, vec![Pattern::from_str("55 8B EC 6A ?? 68 ?? ?? ?? ?? 64 A1 ?? ?? ?? ?? 50 83 EC ?? 53 56 57 A1 ?? ?? ?? ?? 33 C5 50 8D 45 ?? 64 A3 ?? ?? ?? ?? 8B F1 8B 45 ?? 50 8D 4D ?? 51").unwrap()]),
        ]
});

pub fn search_patterns(filepath: &Path) -> Result<Addresses, Error> {
    let content = std::fs::read(filepath)?;

    let pe =
        goblin::pe::PE::parse(&content).map_err(|_| std::io::Error::other("parsing failed"))?;

    let rdata_section = pe
        .sections
        .iter()
        .find(|s| &s.name == b".rdata\0\0")
        .ok_or(std::io::Error::other("rdata not found"))?;

    let rdata_content = rdata_section
        .data(&content)
        .map_err(|_| std::io::Error::other("parsing failed"))?
        .ok_or(std::io::Error::other("parsing failed"))?;
    let data_section = pe
        .sections
        .iter()
        .find(|s| &s.name == b".data\0\0\0")
        .ok_or(std::io::Error::other("data not found"))?;
    let text_section = pe
        .sections
        .iter()
        .find(|s| &s.name == b".text\0\0\0")
        .ok_or(std::io::Error::other("text not found"))?;

    let text_content = text_section
        .data(&content)
        .map_err(|_| std::io::Error::other("parsing failed"))?
        .ok_or(std::io::Error::other("parsing failed"))?;

    let image_base = pe.image_base;

    let Some(onlineconfig_url_offset) =
        find_subsequence(&rdata_content, b"onlineconfigservice.ubi.com\0")
    else {
        return Err(Error::IdFailed);
    };

    let Some(onlinecfg_client_addr) = ONLINE_CLIENT_CFG_PATTERN.search(&text_content).map(|idx| {
        let mut decoder = Decoder::with_ip(
            if pe.is_64 { 64 } else { 32 },
            text_content.as_ref(),
            text_section.virtual_address as u64 + image_base as u64,
            DecoderOptions::NONE,
        );
        decoder.set_ip(text_section.virtual_address as u64 + image_base as u64 + idx as u64);
        decoder.set_position(idx).unwrap();
        let instr = decoder.decode();
        instr.memory_displacement32() as usize
    }) else {
        return Err(Error::IdFailed);
    };

    let unreal_commandline = CMDLINE_PATTERN.search(&text_content).map(|idx| {
        let mut decoder = Decoder::with_ip(
            if pe.is_64 { 64 } else { 32 },
            text_content.as_ref(),
            text_section.virtual_address as u64 + image_base as u64,
            DecoderOptions::NONE,
        );
        decoder.set_ip(text_section.virtual_address as u64 + image_base as u64 + idx as u64);
        decoder.set_position(idx).unwrap();
        let instr = decoder.decode();
        instr.immediate32() as usize
    });

    let mut addr = Addresses {
        global_onlineconfig_client: onlinecfg_client_addr,
        onlineconfig_url: image_base
            + rdata_section.virtual_address as usize
            + onlineconfig_url_offset,
        unreal_commandline,
        debug_print: None,
        func_printer: None,
        func_net_finite_state_machine_next_state: None,
        func_net_finite_state_leave_state: None,
        func_net_result_base: None,
        func_something_with_goal: None,
        func_quazal_stepsequencejob_setstep: None,
        func_thread_starter: None,
        func_goal_change_state: None,
        func_net_core: None,
        func_net_result_core: None,
        func_net_result_session: None,
        func_net_result_lobby: None,
        func_storm_host_port_to_str: None,
        func_generate_id: None,
        func_storm_maybe_set_state: None,
        func_storm_statemachineaction_execute: None,
        func_storm_some_error_formatter: None,
        func_gear_str_destructor: None,
        func_storm_event_dispatch: None,
        func_storm_event_dispatch2: None,
        func_storm_event_maybe_queue_pop: None,
        func_some_gear_str_constructor: None,
        func_net_result_rdv_session: None,
        func_rmc_init_message: None,
        func_rmc_add_method_id: None,
        func_rmc_send_message: None,
        func_storm_event_handler: None,
        func_another_gear_str_destructor: None,
        func_open_file_from_archive: None,
    };

    for (hook, patterns) in HOOK_PATTERNS.iter() {
        let addresses = patterns
            .iter()
            .flat_map(|pattern| {
                let idx = pattern.search(&text_content)?;
                Some(text_section.virtual_address as usize + image_base as usize + idx)
            })
            .collect::<Vec<_>>();
        if addresses.is_empty() {
            continue;
        }
        addr.set_hook_addrs(*hook, addresses);
    }

    Ok(addr)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hashing() {
        let fname = r"C:\Program Files (x86)\Steam\steamapps\common\Tom Clancy's Splinter Cell Blacklist\src\SYSTEM\Blacklist_game.exe";
        if std::fs::metadata(fname).is_err() {
            return;
        }
        hash_file(fname).unwrap();
    }
}
