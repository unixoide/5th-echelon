//! Provides a high-level interface for reading string values from the Windows Registry.
//!
//! This module abstracts the complexity of interacting with the Windows Registry API,
//! including handling different registry hives and dealing with 32-bit vs. 64-bit
//! registry views.

use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt as _;

use windows::core::PCWSTR;
use windows::Win32::Foundation::ERROR_FILE_NOT_FOUND;
use windows::Win32::Foundation::ERROR_MORE_DATA;
use windows::Win32::Foundation::ERROR_SUCCESS;
use windows::Win32::System::Registry;

/// Represents the root key of a registry path.
#[derive(Clone, Copy)]
pub enum Key {
    LocaLMachine,
    CurrentUser,
}

/// Reads a string value from the Windows Registry.
///
/// # Arguments
///
/// * `key` - The root key to use (`HKEY_LOCAL_MACHINE` or `HKEY_CURRENT_USER`).
/// * `subkey` - The path to the subkey.
/// * `value` - The name of the value to read.
///
/// # Returns
///
/// An `Option` containing the `OsString` value if it exists, or `None` otherwise.
pub fn read_string(key: Key, subkey: &str, value: &str) -> Option<OsString> {
    let mut subkey = subkey.replace('/', "\\");
    let mut subkey_utf16 = subkey.encode_utf16().collect::<Vec<u16>>();
    subkey_utf16.push(0);
    let mut value = value.encode_utf16().collect::<Vec<u16>>();
    value.push(0);

    let mut buf = vec![0u16; 2048];
    let mut bufsz = buf.len() as u32 * 2;
    let buf = loop {
        let e = unsafe {
            Registry::RegGetValueW(
                match key {
                    Key::LocaLMachine => Registry::HKEY_LOCAL_MACHINE,
                    Key::CurrentUser => Registry::HKEY_CURRENT_USER,
                },
                PCWSTR::from_raw(subkey_utf16.as_ptr()),
                PCWSTR::from_raw(value.as_ptr()),
                // RRF_SUBKEY_WOW6432KEY allows accessing 32-bit registry keys from a 64-bit application.
                Registry::RRF_RT_REG_SZ | Registry::RRF_SUBKEY_WOW6432KEY,
                None,
                Some(buf.as_mut_ptr().cast()),
                Some(&mut bufsz),
            )
        };
        match e {
            // If the buffer is too small, resize it and try again.
            ERROR_MORE_DATA => {
                buf.resize(bufsz as usize / 2, 0);
            }
            ERROR_SUCCESS => {
                // The returned size includes the null terminator, so we remove it.
                buf.resize(bufsz as usize / 2 - 1, 0);
                break Some(buf);
            }
            // If the key is not found, and we haven't checked the WOW6432Node yet,
            // modify the subkey path to look in the 32-bit view and try again.
            ERROR_FILE_NOT_FOUND if subkey.starts_with("SOFTWARE\\") && !subkey.starts_with("SOFTWARE\\WOW6432Node") => {
                subkey = subkey.replace("SOFTWARE", "SOFTWARE\\WOW6432Node");
                subkey_utf16 = subkey.encode_utf16().collect::<Vec<u16>>();
                subkey_utf16.push(0);
            }
            _ => {
                break None;
            }
        }
    };

    buf.as_deref().map(OsString::from_wide)
}
