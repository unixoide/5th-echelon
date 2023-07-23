use std::ffi::OsString;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Seek;
use std::io::Write;
use std::os::windows::prelude::FromRawHandle;
use std::os::windows::prelude::IntoRawHandle;
use std::os::windows::prelude::OsStringExt;
use std::os::windows::prelude::RawHandle;
use std::path::PathBuf;
use std::slice;

use hooks_proc::forwardable_export;
use tracing::error;
use tracing::info;
use windows::Win32;
use windows::Win32::UI::Shell;

use super::List;
use super::Result;
use super::UplayList;
use super::UplayOverlapped;
use super::UplaySave;
use crate::config::get;
use crate::config::Save;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(usize)]
enum OpenMode {
    Read = 0,
    Write = 1,
}

#[derive(Debug, PartialEq, Eq)]
struct SaveHandle {
    slot_id: usize,
    raw_handle: Option<RawHandle>,
}

impl SaveHandle {
    fn into_mut_ptr(self) -> *mut SaveHandle {
        let boxed = Box::new(self);
        Box::into_raw(boxed)
    }

    #[allow(clippy::unnecessary_box_returns)]
    fn from_mut_ptr(ptr: *mut SaveHandle) -> Box<SaveHandle> {
        if ptr.is_null() || !ptr.is_aligned() {
            error!("{ptr:?} is invalid");
            panic!("{ptr:?} is invalid");
        }
        unsafe { Box::from_raw(ptr) }
    }
}

#[forwardable_export]
unsafe extern "cdecl" fn UPLAY_SAVE_Close(out_save_handle: *mut SaveHandle) -> bool {
    let out_save_handle = out_save_handle.as_mut().unwrap();
    let mut out_save_handle = SaveHandle::from_mut_ptr(out_save_handle);
    let _file = out_save_handle
        .raw_handle
        .take()
        .map(|h| File::from_raw_handle(h));
    true
}

#[forwardable_export]
unsafe extern "cdecl" fn UPLAY_SAVE_GetSavegames(
    out_games_list: *mut *mut List,
    overlapped: *mut UplayOverlapped,
) -> bool {
    if out_games_list.is_null() {
        error!("Invalid out_games_list");
        if !overlapped.is_null() {
            (*overlapped).set_result(1);
        }
        return false;
    }
    let saves = match get_savegames() {
        Ok(sg) => sg,
        Err(e) => {
            error!("Couldn't get savegames: {e}");
            if !overlapped.is_null() {
                (*overlapped).set_result(6);
            }
            return false;
        }
    };

    info!("Savegames: {saves:?}");

    let list = UplayList::Saves(saves);
    *out_games_list = Box::into_raw(Box::new(list.into()));

    if !overlapped.is_null() {
        (*overlapped).set_success();
    }
    true
}

#[forwardable_export]
unsafe extern "cdecl" fn UPLAY_SAVE_Open(
    slot_id: usize,
    mode: OpenMode,
    out_save_handle: *mut *mut SaveHandle,
    overlapped: *mut UplayOverlapped,
) -> bool {
    if out_save_handle.is_null() {
        error!("Invalid handle {out_save_handle:?}");
        return false;
    }
    let path = cfg.save.get_savegame_path(slot_id);
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            if let Err(e) = std::fs::create_dir_all(parent) {
                error!("Couldn't create save game dir {parent:?}: {e}");
                return false;
            }
        }
    }
    let file = match mode {
        OpenMode::Read => File::options().read(true).open(path),
        OpenMode::Write => File::options()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path),
    };
    let file = match file {
        Ok(f) => f,
        Err(e) => {
            error!("Couldn't open slot {slot_id} for {mode:?}: {e}");
            if !overlapped.is_null() {
                (*overlapped).set_result(6);
            }
            return false;
        }
    };

    let handle = SaveHandle {
        slot_id,
        raw_handle: Some(file.into_raw_handle()),
    };

    *out_save_handle = handle.into_mut_ptr();
    if !overlapped.is_null() {
        (*overlapped).set_success();
    }

    true
}

#[forwardable_export]
unsafe extern "cdecl" fn UPLAY_SAVE_Read(
    save_handle: *mut SaveHandle,
    num_of_bytes_to_read: usize,
    offset: usize,
    out_buffer: *mut *mut u8,
    out_num_of_bytes_read: *mut usize,
    overlapped: *mut UplayOverlapped,
) -> bool {
    // if save_handle.is_null() || !save_handle.is_aligned() {
    //     error!("Invalid save_handle {save_handle:?}");
    //     return false;
    // }
    let save_handle = save_handle.as_mut().unwrap();

    let res = if let Some(raw_handle) = (save_handle).raw_handle.take() {
        let mut file = File::from_raw_handle(raw_handle);
        let res = || -> std::io::Result<()> {
            file.seek(std::io::SeekFrom::Start(offset as u64))?;
            let mut buf = vec![0u8; num_of_bytes_to_read];
            let read = file.read(&mut buf)?;
            info!("read {read} bytes from slot {}", save_handle.slot_id);
            std::ptr::copy(buf.as_ptr(), *out_buffer, read);
            *out_num_of_bytes_read = read;
            Ok(())
        }();
        // return handle ownership again
        (save_handle).raw_handle.replace(file.into_raw_handle());
        res
    } else {
        Ok(())
    };
    let exit_code = res.is_ok();
    if !overlapped.is_null() {
        if let Err(e) = res {
            error!("Couldn't read from file: {e}");
            (*overlapped).set_result(6);
        } else {
            (*overlapped).set_success();
        }
    }
    exit_code
}

#[forwardable_export]
unsafe extern "cdecl" fn UPLAY_SAVE_Remove(
    slot_id: usize,
    overlapped: *mut UplayOverlapped,
) -> bool {
    let path: PathBuf = cfg.save.get_savegame_path(slot_id);
    if path.exists() {
        if let Err(e) = std::fs::remove_file(&path) {
            error!("Couldn't remove {path:?}: {e}");
            if !overlapped.is_null() {
                (*overlapped).set_result(6);
            }
            return false;
        }
    }
    if !overlapped.is_null() {
        (*overlapped).set_success();
    }
    true
}

#[forwardable_export]
unsafe extern "cdecl" fn UPLAY_SAVE_SetName(save_handle: *mut SaveHandle, name: *const i8) -> bool {
    // if save_handle.is_null() || !save_handle.is_aligned() {
    //     error!("Invalid save_handle {save_handle:?}");
    //     return false;
    // }
    let save_handle = save_handle.as_mut().unwrap();
    let slot_id = (save_handle).slot_id;
    let mut path: PathBuf = cfg.save.get_savegame_path(slot_id);
    path.set_extension("meta");
    let name = std::ffi::CStr::from_ptr(name);
    info!("Save game name: {path:?} = {name:?}");
    if let Err(e) = fs::write(path, name.to_bytes()) {
        error!("Couldn't set save name for slot {slot_id}: {e}");
        false
    } else {
        true
    }
}

#[forwardable_export]
unsafe extern "cdecl" fn UPLAY_SAVE_Write(
    save_handle: *mut SaveHandle,
    num_of_bytes_to_write: usize,
    buffer: *const *const u8,
    overlapped: *mut UplayOverlapped,
) -> bool {
    // if save_handle.is_null() || !save_handle.is_aligned() {
    //     error!("Invalid save_handle {save_handle:?}");
    //     return false;
    // }
    let save_handle = save_handle.as_mut().unwrap();

    let res = if let Some(raw_handle) = (save_handle).raw_handle.take() {
        let mut file = File::from_raw_handle(raw_handle);
        let res = || -> std::io::Result<()> {
            let buf = slice::from_raw_parts(*buffer, num_of_bytes_to_write);
            file.write_all(buf)?;
            Ok(())
        }();
        // return handle ownership again
        (save_handle).raw_handle.replace(file.into_raw_handle());
        res
    } else {
        Ok(())
    };
    let exit_code = res.is_ok();
    if !overlapped.is_null() {
        if let Err(e) = res {
            error!("Couldn't write to file: {e}");
            (*overlapped).set_result(6);
        } else {
            (*overlapped).set_success();
        }
    }
    exit_code
}

fn get_savegames() -> Result<Vec<UplaySave>> {
    let cfg = get().ok_or(anyhow::anyhow!("config not loaded"))?;
    let path = cfg.save.get_savegames_path();
    if !path.exists() {
        return Ok(vec![]);
    }

    let files = std::fs::read_dir(&path)?;
    let saves = files
        .into_iter()
        .filter_map(std::result::Result::ok)
        .filter(|f| f.file_type().map(|ft| ft.is_file()).unwrap_or_default())
        .filter(|f| {
            f.path()
                .extension()
                .map(|e| e == std::ffi::OsStr::new("sav"))
                .unwrap_or_default()
        })
        .filter_map(|f| {
            let size = f.metadata().unwrap().len();
            #[allow(clippy::cast_possible_truncation)]
            let size = size as usize;
            f.path()
                .file_stem()
                .and_then(std::ffi::OsStr::to_str)
                .and_then(|n| n.parse().ok())
                .map(|slot_id| (slot_id, f.path(), size))
        })
        .map(|(slot_id, mut fpath, size)| {
            fpath.set_extension("meta");
            let name = if fpath.exists() {
                std::fs::read_to_string(fpath).unwrap_or_default()
            } else {
                String::new()
            };
            UplaySave {
                slot_id,
                name,
                size,
            }
        })
        .collect();

    Ok(saves)
}

impl Save {
    fn get_savegames_path(&self) -> PathBuf {
        const SAVE_GAME_FOLDER: &str = "5th-Echolon\\Saves";
        match self.save_dir {
            crate::config::SaveDir::InstallLocation => todo!(),
            crate::config::SaveDir::Roaming => known_folder_roaming_app_data()
                .expect("Couldn't find roaming directory")
                .join(SAVE_GAME_FOLDER),
            crate::config::SaveDir::Custom(ref p) => PathBuf::from(p),
        }
    }

    fn get_savegame_path(&self, slot_id: usize) -> PathBuf {
        self.get_savegames_path().join(format!("{slot_id:08}.sav"))
    }
}

fn known_folder(folder_id: windows::core::GUID) -> Option<PathBuf> {
    unsafe {
        let result = Shell::SHGetKnownFolderPath(
            &folder_id,
            Shell::KNOWN_FOLDER_FLAG(0),
            Win32::Foundation::HANDLE::default(),
        );
        if let Ok(result) = result {
            let path = result.as_wide();
            let ostr: OsString = OsStringExt::from_wide(path);
            windows::Win32::System::Com::CoTaskMemFree(Some(result.as_ptr() as *const _));
            Some(PathBuf::from(ostr))
        } else {
            None
        }
    }
}

fn known_folder_roaming_app_data() -> Option<PathBuf> {
    known_folder(Shell::FOLDERID_RoamingAppData)
}
