use std::collections::HashMap;
use std::io::Read;
use std::path::Path;

use sha2::digest::crypto_common::BlockSizeUser;
use sha2::Digest;
use tracing::info;
use windows::Win32::Foundation::HMODULE;

use crate::get_executable;
use crate::macros::fatal_error;

#[derive(Clone, Copy)]
pub struct Addresses {
    pub global_onlineconfig_client: usize,
    pub onlineconfig_url: usize,
    pub unreal_commandline: Option<usize>,

    // hooks
    pub func_printer: Option<usize>,
    pub func_net_finite_state_machine_next_state: Option<usize>,
    pub func_net_finite_state_leave_state: Option<usize>,
    pub func_net_result_base: Option<usize>,
    pub func_something_with_goal: Option<usize>,
    pub func_quazal_stepsequencejob_setstep: Option<usize>,
}

fn build_game_map() -> HashMap<String, HashMap<[u8; 32], Addresses>> {
    #![allow(clippy::unreadable_literal)]

    HashMap::from([
        (
            String::from("blacklist_game.exe"), 
            HashMap::from([
                (
                    *b"\x15\x8e\xfc]\t@\xfc\xaf>K\x16\x95,\x8f\x88a\xe1`aP\x9d\x9e\xb8\xeb_\xf4\xae2I\xbbZ\x05",
                    Addresses {
                        global_onlineconfig_client: 0x032bf5bc,
                        onlineconfig_url: 0x02cc0650,
                        unreal_commandline: Some(0x0323b97c),

                        // hooks
                        func_printer: Some(0x04b19e0),
                        func_net_finite_state_machine_next_state: Some(0x00ad0260),
                        func_net_finite_state_leave_state: Some(0x00a9aa40),
                        func_net_result_base: Some(0x00a9a180),
                        func_something_with_goal: Some(0x0ae5130),
                        func_quazal_stepsequencejob_setstep: Some(0x02138f10),
                    }
                ),
                (
                    // Same binary except of 2 bytes of the COFF header checksum?? (offset 0x1a9 and 0x1aa)
                    *b"\x7f\xcd:\x18\xd4\xdc\xc6\x92q\x99\x84\xb0rh\xbdBvA\x08\xe7\xdf7\x04\x9f\x14\x90\xf2\t)\xf9\x92]",
                    Addresses {
                        global_onlineconfig_client: 0x032bf5bc,
                        onlineconfig_url: 0x02cc0650,
                        unreal_commandline: Some(0x0323b97c),

                        // hooks
                        func_printer: Some(0x04b19e0),
                        func_net_finite_state_machine_next_state: Some(0x00ad0260),
                        func_net_finite_state_leave_state: Some(0x00a9aa40),
                        func_net_result_base: Some(0x00a9a180),
                        func_something_with_goal: Some(0x0ae5130),
                        func_quazal_stepsequencejob_setstep: Some(0x02138f10),
                    }
                )
            ])
        ),
        (
            String::from("blacklist_dx11_game.exe"), 
            HashMap::new()
        )
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
        *addr
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
