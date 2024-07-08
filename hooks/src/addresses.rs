pub use hooks_addresses::Addresses;
use windows::Win32::Foundation::HMODULE;

use crate::get_executable;
use crate::macros::fatal_error;

pub fn get() -> Addresses {
    let Some(path) = get_executable(HMODULE::default()) else {
        fatal_error!("Couldn't find host process");
    };

    match hooks_addresses::get_from_path(&path) {
        Ok(a) => a,
        Err(e) => {
            fatal_error!("{e}");
        }
    }
}
