use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::path::PathBuf;

use windows::Win32::Foundation::HANDLE;
use windows::Win32::UI::Shell;

use crate::Save;
use crate::SaveDir;

pub trait SaveGameExt {
    fn get_savegames_path(&self) -> PathBuf;
    fn get_savegame_path(&self, slot_id: usize) -> PathBuf;
}

impl SaveGameExt for Save {
    fn get_savegames_path(&self) -> PathBuf {
        const SAVE_GAME_FOLDER: &str = "5th-Echelon\\Saves";
        match self.save_dir {
            SaveDir::InstallLocation => todo!(),
            SaveDir::Roaming => known_folder_roaming_app_data()
                .expect("Couldn't find roaming directory")
                .join(SAVE_GAME_FOLDER),
            SaveDir::Custom(ref p) => PathBuf::from(p),
        }
    }

    fn get_savegame_path(&self, slot_id: usize) -> PathBuf {
        self.get_savegames_path().join(format!("{slot_id:08}.sav"))
    }
}

fn known_folder(folder_id: windows::core::GUID) -> Option<PathBuf> {
    unsafe {
        let result = Shell::SHGetKnownFolderPath(&folder_id, Shell::KNOWN_FOLDER_FLAG(0), HANDLE::default());
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
