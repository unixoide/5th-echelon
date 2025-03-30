static COMPRESSED_DLL: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/uplay_r1_loader.dll.brotli"));

pub fn drop_dll(dir: &std::path::Path) {
    use std::fs;
    use std::fs::File;
    use std::io::Cursor;

    let dll_path = dir.join("uplay_r1_loader.dll");
    if !dll_path.exists() {
        panic!("uplay_r1_loader.dll not found. Make sure the launcher is placed in the right directory");
    }
    let backup_path = dir.join("uplay_r1_loader.orig.dll");
    if !backup_path.exists() && fs::copy(&dll_path, backup_path).is_err() {
        panic!("Backup failed");
    }

    let decompress = || {
        let mut dll_file = File::create(dll_path).ok()?;
        brotli::BrotliDecompress(&mut Cursor::new(COMPRESSED_DLL), &mut dll_file).ok()
    };

    if (decompress)().is_none() {
        panic!("Update failed");
    }
}
