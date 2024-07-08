use std::collections::HashMap;
use std::io::Read;
use std::ops::Range;
use std::path::Path;

use sha2::digest::crypto_common::BlockSizeUser;
use sha2::Digest;
use tracing::info;
use windows::Win32::Foundation::HMODULE;

use crate::get_executable;
use crate::macros::fatal_error;

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

fn build_game_map() -> HashMap<String, HashMap<[u8; 32], Addresses>> {
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
    ];

    HashMap::from([
        (
            String::from("blacklist_game.exe"),
            HashMap::from(dx9_hashes.map(|h| (h, dx9_addrs.clone()))),
        ),
        (
            String::from("blacklist_dx11_game.exe"),
            HashMap::from([(
                [
                    0xc6, 0xb9, 0xf3, 0x30, 0xfa, 0xc1, 0x41, 0x2f, 0x19, 0xf3, 0x2a, 0x6f, 0xd8,
                    0x6e, 0xdb, 0x4c, 0x66, 0x29, 0x1a, 0x69, 0x2, 0x61, 0x1e, 0x94, 0x33, 0xb9,
                    0xb0, 0xea, 0x65, 0x9e, 0xb4, 0xbc,
                ],
                Addresses {
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
                },
            )]),
        ),
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

pub fn get() -> Addresses {
    let Some(path) = get_executable(HMODULE::default()) else {
        fatal_error!("Couldn't find host process");
    };

    let file_name = path
        .file_name()
        .expect("Valid filename")
        .to_ascii_lowercase();
    let file_name = file_name.to_str().expect("Valid filename");

    info!("Loaded by {file_name}");

    let digest = hash_file(&path);

    let game_map = build_game_map();
    let Some(game_map) = game_map.get(file_name) else {
        fatal_error!("Unknown binary {file_name}");
    };

    if let Some(addr) = digest.ok().and_then(|digest| game_map.get(&digest)) {
        addr.clone()
    } else {
        fatal_error!("{file_name} was modified or the version is not supported.\n\nPlease share {file_name} with the project, so that support can be implemented.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hashing() {
        hash_file(r"C:\Program Files (x86)\Steam\steamapps\common\Tom Clancy's Splinter Cell Blacklist\src\SYSTEM\Blacklist_game.exe").unwrap();
    }
}
