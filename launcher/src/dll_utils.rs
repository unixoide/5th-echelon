//! Provides helper functions for working with DLLs.
//!
//! This module includes functionality for parsing DLL files to extract version
//! information and, when the `embed-dll` feature is enabled, for dropping
//! embedded DLLs into the file system.

use std::fs;
use std::path::Path;

#[cfg(feature = "embed-dll")]
mod drop;
mod parse;

#[cfg(feature = "embed-dll")]
pub use drop::drop_dll;
pub use parse::parse;

use crate::version::Version;

/// Represents errors that can occur when working with DLLs.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("reading file failed")]
    ReadingFileFailed,
    #[error("parsing file failed")]
    ParsingFileFailed,
}

/// Reads a DLL file and returns its version information.
///
/// # Arguments
///
/// * `dll_path` - A path to the DLL file.
///
/// # Returns
///
/// A `Result` containing the `Version` of the DLL, or an `Error` if reading
/// or parsing the file fails.
pub fn get_dll_version(dll_path: impl AsRef<Path>) -> Result<Version, Error> {
    // Read the DLL file into a byte vector.
    let Ok(data) = fs::read(dll_path) else {
        return Err(Error::ReadingFileFailed);
    };

    // Parse the DLL data to extract version information.
    let Ok(dll) = parse(&data) else {
        return Err(Error::ParsingFileFailed);
    };

    Ok(dll.version)
}
