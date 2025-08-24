use std::ffi::OsStr;
use std::path::Path;
use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;

use crate::registry;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum GameVersion {
    SplinterCellBlacklistDx9,
    SplinterCellBlacklistDx11,
}

impl GameVersion {
    pub fn executable(self) -> &'static str {
        match self {
            GameVersion::SplinterCellBlacklistDx9 => "Blacklist_game.exe",
            GameVersion::SplinterCellBlacklistDx11 => "Blacklist_DX11_game.exe",
        }
    }

    pub fn full_path(self, base_dir: &Path) -> PathBuf {
        base_dir.join(self.executable())
    }

    pub fn label(&self) -> &'static str {
        match self {
            GameVersion::SplinterCellBlacklistDx9 => "DirectX 9",
            GameVersion::SplinterCellBlacklistDx11 => "DirectX 11",
        }
    }

    pub fn label_short(&self) -> &'static str {
        match self {
            GameVersion::SplinterCellBlacklistDx9 => "DX9",
            GameVersion::SplinterCellBlacklistDx11 => "DX11",
        }
    }
}

fn get_install_dir() -> Option<PathBuf> {
    registry::read_string(registry::Key::LocaLMachine, r"SOFTWARE\Ubisoft\Splinter Cell Blacklist", "installdir")
        .as_deref()
        .and_then(OsStr::to_str)
        .map(PathBuf::from)
}

pub fn find_target_dir() -> Option<PathBuf> {
    let mut target_dir = std::env::current_exe().unwrap().parent().unwrap().to_owned();

    if let Some(install_dir) = get_install_dir() {
        target_dir = install_dir;
    }

    let candidates: Vec<_> = ["Blacklist_game.exe", "SYSTEM\\Blacklist_game.exe", "src\\SYSTEM\\Blacklist_game.exe"]
        .into_iter()
        .map(|p| target_dir.join(p))
        .collect();

    for path in candidates {
        if path.exists() {
            if let Some(dir) = path.parent() {
                std::env::set_current_dir(dir).unwrap();
                return Some(dir.canonicalize().unwrap());
            }
        }
    }
    None
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    Ready,
    NotReady,
}

impl From<bool> for GameState {
    fn from(b: bool) -> Self {
        if b {
            GameState::Ready
        } else {
            GameState::NotReady
        }
    }
}
