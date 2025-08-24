//! Provides a Windows-specific implementation of the `imgui::ClipboardBackend`.
//!
//! This module uses the Windows API to interact with the system clipboard,
//! allowing `imgui` to copy and paste text.

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
use windows::Win32::System::Memory::GlobalAlloc;
use windows::Win32::System::Memory::GlobalLock;
use windows::Win32::System::Memory::GlobalUnlock;
use windows::Win32::System::Memory::GMEM_MOVEABLE;

// Clipboard formats
const CF_TEXT: u32 = 1;
const CF_UNICODETEXT: u32 = 13;

/// A struct that holds the necessary state for Windows clipboard interaction,
/// specifically the window handle (`HWND`).
pub struct WindowsClipboard {
    pub hwnd: HWND,
}

impl imgui::ClipboardBackend for WindowsClipboard {
    /// Gets the clipboard content as a `String`.
    ///
    /// This function opens the clipboard, retrieves the text data,
    /// and then closes the clipboard.
    fn get(&mut self) -> Option<String> {
        unsafe {
            // Open the clipboard, returning None on failure.
            OpenClipboard(self.hwnd).ok()?;

            // Get clipboard data as a handle.
            let data = GetClipboardData(CF_TEXT).ok().and_then(|hndl| {
                let hgbl = HGLOBAL(hndl.0 as _);

                // Lock the global memory handle to get a pointer to the data.
                let data: *mut core::ffi::c_char = GlobalLock(hgbl).cast();
                if data.is_null() {
                    return None;
                }

                // Create a CStr from the pointer and convert it to a Rust String.
                let data = std::ffi::CStr::from_ptr(data);
                let data = data.to_str().ok()?.to_owned();

                // Unlock the global memory handle.
                #[allow(unused_must_use)]
                GlobalUnlock(hgbl);
                Some(data)
            });

            // Close the clipboard.
            CloseClipboard().ok()?;

            data.inspect(|d| {
                info!("Pasting {} chars from clipboard", d.len());
            })
        }
    }

    /// Sets the clipboard content to the given `&str`.
    ///
    /// This function allocates global memory for the string, copies the string into it,
    /// and then sets it as the clipboard data.
    #[instrument(skip(self))]
    fn set(&mut self, value: &str) {
        unsafe {
            // Convert the string to a wide string (UTF-16) with a null terminator.
            let osstr = OsStr::new(value);
            let mut cstr: Vec<u16> = osstr.encode_wide().collect();
            cstr.push(0);

            // Allocate global memory for the wide string.
            let hgbl = GlobalAlloc(GMEM_MOVEABLE, cstr.len() * 2).expect("allocation failed");
            let data: *mut u16 = GlobalLock(hgbl).cast();
            if data.is_null() {
                return;
            }

            // Copy the string data into the allocated memory.
            std::ptr::copy_nonoverlapping(cstr.as_ptr(), data, cstr.len());

            // Unlock the global memory.
            #[allow(unused_must_use)]
            GlobalUnlock(hgbl);

            // Open the clipboard and set the data.
            OpenClipboard(self.hwnd).expect("opening clipboard failed");
            if SetClipboardData(CF_UNICODETEXT, HANDLE(hgbl.0 as _)).is_err() {
                error!("Setting clipboard failed");
                // Free the global memory if setting the clipboard data failed.
                #[allow(unused_must_use)]
                GlobalFree(hgbl);
            } else {
                info!("Copied {} bytes to clipboard", cstr.len());
            }

            // Close the clipboard.
            CloseClipboard().expect("closing clipboard failed");
        }
    }
}
