//! Handles the decompression and deployment of the embedded `uplay_r1_loader.dll`.
//!
//! This module is used when the `embed-dll` feature is enabled.

/// The compressed `uplay_r1_loader.dll` data, embedded at compile time.
static COMPRESSED_DLL: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/uplay_r1_loader.dll.brotli"));

/// Decompresses and drops the embedded `uplay_r1_loader.dll` into the specified directory.
///
/// This function will first back up the original `uplay_r1_loader.dll` to
/// `uplay_r1_loader.orig.dll` if a backup does not already exist. It then
/// decompresses the embedded DLL data and overwrites the original file.
///
/// # Panics
///
/// This function will panic if:
/// - The original `uplay_r1_loader.dll` is not found.
/// - The backup of the original DLL fails.
/// - The decompression and writing of the new DLL fails.
pub fn drop_dll(dir: &std::path::Path) {
    use std::fs;
    use std::fs::File;
    use std::io::Cursor;

    let dll_path = dir.join("uplay_r1_loader.dll");
    if !dll_path.exists() {
        panic!("uplay_r1_loader.dll not found. Make sure the launcher is placed in the right directory");
    }

    // Back up the original DLL if a backup doesn't already exist.
    let backup_path = dir.join("uplay_r1_loader.orig.dll");
    if !backup_path.exists() && fs::copy(&dll_path, backup_path).is_err() {
        panic!("Backup failed");
    }

    // Decompress the embedded DLL and write it to disk.
    let decompress = || {
        let mut dll_file = File::create(dll_path).ok()?;
        brotli::BrotliDecompress(&mut Cursor::new(COMPRESSED_DLL), &mut dll_file).ok()
    };

    if (decompress)().is_none() {
        panic!("Update failed");
    }
}
