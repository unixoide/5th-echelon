use std::io::Read;
use std::path::Path;

use sha2::digest::crypto_common::BlockSizeUser;
use sha2::Digest;
use tracing::info;
use windows::Win32::Foundation::HMODULE;

use crate::get_executable;
use crate::macros::fatal_error;

const DX11_HASH: &[u8] =
    b"\xc6\xb9\xf30\xfa\xc1A/\x19\xf3*o\xd8n\xdbLf)\x1ai\x02a\x1e\x943\xb9\xb0\xeae\x9e\xb4\xbc";

const DX9_HASH: &[u8] =
    b"\x15\x8e\xfc]\t@\xfc\xaf>K\x16\x95,\x8f\x88a\xe1`aP\x9d\x9e\xb8\xeb_\xf4\xae2I\xbbZ\x05";

pub struct Addresses {
    pub global_onlineconfig_client: usize,
    pub onlineconfig_url: usize,
    pub unreal_commandline: usize,

    // hooks
    pub func_printer: usize,
    pub func_net_finite_state_machine_next_state: usize,
    pub func_net_finite_state_leave_state: usize,
    pub func_net_result_base: usize,
    pub func_something_with_goal: usize,
    pub func_quazal_stepsequencejob_setstep: usize,
}

fn splinter_cell_blacklist_dx9() -> Addresses {
    #![allow(clippy::unreadable_literal)]

    Addresses {
        global_onlineconfig_client: 0x032bf5bc,
        onlineconfig_url: 0x02cc0650,
        unreal_commandline: 0x0323b97c,

        // hooks
        func_printer: 0x04b19e0,
        func_net_finite_state_machine_next_state: 0x00ad0260,
        func_net_finite_state_leave_state: 0x00a9aa40,
        func_net_result_base: 0x00A9A180,
        func_something_with_goal: 0x0ae5130,
        func_quazal_stepsequencejob_setstep: 0x02138f10,
    }
}

fn splinter_cell_blacklist_dx11() -> Addresses {
    todo!("DX11 binary is not supported yet")
    // Addresses {}
}

fn hash_file(path: impl AsRef<Path>) -> anyhow::Result<Vec<u8>> {
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

    Ok(hasher.finalize().to_vec())
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

    match file_name {
        "blacklist_dx11_game.exe" => {
            if let Ok(digest) = digest {
                if digest != DX11_HASH {
                    fatal_error!("Host process was modified/is not supported");
                }
            }
            splinter_cell_blacklist_dx11()
        }
        "blacklist_game.exe" => {
            if let Ok(digest) = digest {
                if digest != DX9_HASH {
                    fatal_error!("Host process was modified/is not supported");
                }
            }
            splinter_cell_blacklist_dx9()
        }
        x => panic!("Unexpected module name {x}"),
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
