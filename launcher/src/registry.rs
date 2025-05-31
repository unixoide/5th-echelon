use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt as _;

use windows::core::PCWSTR;
use windows::Win32::Foundation::ERROR_FILE_NOT_FOUND;
use windows::Win32::Foundation::ERROR_MORE_DATA;
use windows::Win32::Foundation::ERROR_SUCCESS;
use windows::Win32::System::Registry;

#[derive(Clone, Copy)]
pub enum Key {
    LocaLMachine,
    CurrentUser,
}

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
                Registry::RRF_RT_REG_SZ | Registry::RRF_SUBKEY_WOW6432KEY,
                None,
                Some(buf.as_mut_ptr().cast()),
                Some(&mut bufsz),
            )
        };
        match e {
            ERROR_MORE_DATA => {
                buf.resize(bufsz as usize / 2, 0);
            }
            ERROR_SUCCESS => {
                // RegGetValue returns null terminated data
                buf.resize(bufsz as usize / 2 - 1, 0);
                break Some(buf);
            }
            ERROR_FILE_NOT_FOUND
                if subkey.starts_with("SOFTWARE\\") && !subkey.starts_with("SOFTWARE\\WOW6432Node") =>
            {
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
