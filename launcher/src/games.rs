//! Defines game-related data structures and logic, such as supported game
//! versions and functions for locating the game installation.

use std::ffi::OsStr;
use std::path::Path;
use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;

use crate::registry;

/// Represents the available versions of the game.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum GameVersion {
    SplinterCellBlacklistDx9,
    SplinterCellBlacklistDx11,
}

impl GameVersion {
    /// Returns the name of the executable for the game version.
    pub fn executable(self) -> &'static str {
        match self {
            GameVersion::SplinterCellBlacklistDx9 => "Blacklist_game.exe",
            GameVersion::SplinterCellBlacklistDx11 => "Blacklist_DX11_game.exe",
        }
    }

    /// Returns the full path to the game executable, given a base directory.
    pub fn full_path(self, base_dir: &Path) -> PathBuf {
        base_dir.join(self.executable())
    }

    /// Returns a user-friendly label for the game version.
    pub fn label(&self) -> &'static str {
        match self {
            GameVersion::SplinterCellBlacklistDx9 => "DirectX 9",
            GameVersion::SplinterCellBlacklistDx11 => "DirectX 11",
        }
    }

    /// Returns a short label for the game version.
    pub fn label_short(&self) -> &'static str {
        match self {
            GameVersion::SplinterCellBlacklistDx9 => "DX9",
            GameVersion::SplinterCellBlacklistDx11 => "DX11",
        }
    }
}

/// Retrieves the game's installation directory from the Windows Registry.
fn get_install_dir() -> Option<PathBuf> {
    registry::read_string(registry::Key::LocaLMachine, r"SOFTWARE\Ubisoft\Splinter Cell Blacklist", "installdir")
        .as_deref()
        .and_then(OsStr::to_str)
        .map(PathBuf::from)
}

/// Finds the target directory for the game.
///
/// It searches for the game executable in a list of candidate paths, starting
/// from the current executable's directory and also checking the registry.
pub fn find_target_dir() -> Option<PathBuf> {
    let mut target_dir = std::env::current_exe().unwrap().parent().unwrap().to_owned();

    // Try to find the installation directory from the registry.
    if let Some(install_dir) = get_install_dir() {
        target_dir = install_dir;
    }

    // A list of possible relative paths to the game executable.
    let candidates: Vec<_> = ["Blacklist_game.exe", "SYSTEM\\Blacklist_game.exe", "src\\SYSTEM\\Blacklist_game.exe"]
        .into_iter()
        .map(|p| target_dir.join(p))
        .collect();

    // Check each candidate path to see if the executable exists.
    for path in candidates {
        if path.exists() {
            if let Some(dir) = path.parent() {
                // When found, set the current directory to the executable's location.
                std::env::set_current_dir(dir).unwrap();
                return Some(dir.canonicalize().unwrap());
            }
        }
    }
    None
}

/// Represents the state of the game (e.g., whether it's ready to be launched).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    Ready,
    NotReady,
}

impl From<bool> for GameState {
    /// Converts a boolean to a `GameState`.
    fn from(b: bool) -> Self {
        if b {
            GameState::Ready
        } else {
            GameState::NotReady
        }
    }
}
