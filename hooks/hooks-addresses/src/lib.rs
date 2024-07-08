use std::collections::HashMap;
use std::io::Read;
use std::ops::Range;
use std::path::Path;

use hooks_config::Hook;
use sha2::digest::crypto_common::BlockSizeUser;
use sha2::Digest;
use tracing::info;

#[derive(Clone)]
pub struct Addresses {
    pub global_onlineconfig_client: usize,
    pub onlineconfig_url: usize,
    pub unreal_commandline: Option<usize>,

    pub debug_print: Option<(usize, Vec<Range<usize>>)>,

    // hooks
    pub func_printer: Option<usize>,
    pub func_net_finite_state_machine_next_state: Option<usize>,
    pub func_net_finite_state_leave_state: Option<usize>,
    pub func_net_result_base: Option<usize>,
    pub func_something_with_goal: Option<usize>,
    pub func_quazal_stepsequencejob_setstep: Option<usize>,
    pub func_thread_starter: Option<usize>,
    pub func_goal_change_state: Option<usize>,
    pub func_net_core: Option<usize>,
    pub func_net_result_core: Option<usize>,
    pub func_net_result_session: Option<usize>,
    pub func_net_result_lobby: Option<usize>,
    pub func_storm_host_port_to_str: Option<usize>,
    pub func_generate_id: Option<usize>,
    pub func_storm_maybe_set_state: Option<usize>,
    pub func_storm_statemachineaction_execute: Option<usize>,
    pub func_storm_some_error_formatter: Option<usize>,
    pub func_gear_str_destructor: Option<usize>,
    pub func_storm_event_dispatch: Option<usize>,
    pub func_storm_event_dispatch2: Option<usize>,
    pub func_storm_event_maybe_queue_pop: Option<usize>,
    pub func_some_gear_str_constructor: Option<usize>,
    pub func_net_result_rdv_session: Option<usize>,
    pub func_rmc_init_message: Option<usize>,
    pub func_rmc_add_method_id: Option<usize>,
    pub func_rmc_send_message: Option<usize>,
    pub func_storm_event_handler: Option<usize>,
    pub func_another_gear_str_destructor: Option<usize>,
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
    pub fn hook_addr(&self, hook: Hook) -> Option<Vec<usize>> {
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
    };
    HashMap::from(dx11_hashes.map(|h| (h, dx11_addrs.clone())))
}

fn build_game_map() -> HashMap<String, HashMap<[u8; 32], Addresses>> {
    HashMap::from([
        (String::from("blacklist_game.exe"), dx9_addresses()),
        (String::from("blacklist_dx11_game.exe"), dx11_addresses()),
    ])
}

fn hash_file(path: impl AsRef<Path>) -> anyhow::Result<[u8; 32]> {
    let path = path.as_ref();
    let mut f = std::fs::File::open(path)?;
    let mut hasher = sha2::Sha256::new();
    let mut buf = vec![0u8; sha2::Sha256::block_size()];

    while let Ok(n) = f.read(&mut buf) {
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }

    Ok(hasher.finalize().into())
}

pub fn get_from_path(filepath: &Path) -> anyhow::Result<Addresses> {
    let file_name = filepath
        .file_name()
        .ok_or_else(|| anyhow::anyhow!("no filename"))?
        .to_ascii_lowercase()
        .to_string_lossy()
        .into_owned();
    info!("Loaded by {file_name}");

    let digest = hash_file(filepath);

    let game_map = build_game_map();
    let Some(game_map) = game_map.get(&file_name) else {
        anyhow::bail!("Unknown binary {file_name}");
    };

    if let Some(addr) = digest.ok().and_then(|digest| game_map.get(&digest)) {
        Ok(addr.clone())
    } else {
        anyhow::bail!("{file_name} was modified or the version is not supported.\n\nPlease share {file_name} with the project, so that support can be implemented.");
    }
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
