use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

use tracing::error;
use tracing::info;
use tracing::instrument;
use windows::Win32::Foundation::GlobalFree;
use windows::Win32::Foundation::HANDLE;
use windows::Win32::Foundation::HGLOBAL;
use windows::Win32::Foundation::HWND;
use windows::Win32::System::DataExchange::CloseClipboard;
use windows::Win32::System::DataExchange::GetClipboardData;
use windows::Win32::System::DataExchange::OpenClipboard;
use windows::Win32::System::DataExchange::SetClipboardData;
use windows::Win32::System::Memory::GMEM_MOVEABLE;
use windows::Win32::System::Memory::GlobalAlloc;
use windows::Win32::System::Memory::GlobalLock;
use windows::Win32::System::Memory::GlobalUnlock;

const CF_TEXT: u32 = 1;
const CF_UNICODETEXT: u32 = 13;

pub struct WindowsClipboard {
    pub hwnd: HWND,
}

impl imgui::ClipboardBackend for WindowsClipboard {
    fn get(&mut self) -> Option<String> {
        unsafe {
            OpenClipboard(self.hwnd).ok()?;
            let data = GetClipboardData(CF_TEXT).ok().and_then(|hndl| {
                let hgbl = HGLOBAL(hndl.0 as _);
                let data: *mut core::ffi::c_char = GlobalLock(hgbl).cast();
                if data.is_null() {
                    return None;
                }

                let data = std::ffi::CStr::from_ptr(data);
                let data = data.to_str().ok()?.to_owned();
                #[allow(unused_must_use)]
                GlobalUnlock(hgbl);
                Some(data)
            });
            CloseClipboard().ok()?;
            data.inspect(|d| {
                info!("Pasting {} chars from clipboard", d.len());
            })
        }
    }

    #[instrument(skip(self))]
    fn set(&mut self, value: &str) {
        unsafe {
            let osstr = OsStr::new(value);
            let mut cstr: Vec<u16> = osstr.encode_wide().collect();
            cstr.push(0);
            let hgbl = GlobalAlloc(GMEM_MOVEABLE, cstr.len() * 2).expect("allocation failed");
            let data: *mut u16 = GlobalLock(hgbl).cast();
            if data.is_null() {
                return;
            }
            std::ptr::copy_nonoverlapping(cstr.as_ptr(), data, cstr.len());
            #[allow(unused_must_use)]
            GlobalUnlock(hgbl);
            OpenClipboard(self.hwnd).expect("opening clipboard failed");
            if SetClipboardData(CF_UNICODETEXT, HANDLE(hgbl.0 as _)).is_err() {
                error!("Setting clipboard failed");
                #[allow(unused_must_use)]
                GlobalFree(hgbl);
            } else {
                info!("Copied {} bytes to clipboard", cstr.len());
            }
            CloseClipboard().expect("closing clipboard failed");
        }
    }
}
