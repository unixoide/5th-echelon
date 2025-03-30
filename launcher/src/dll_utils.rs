use std::fs;
use std::path::Path;

#[cfg(feature = "embed-dll")]
mod drop;
mod parse;

#[cfg(feature = "embed-dll")]
pub use drop::drop_dll;
pub use parse::parse;

use crate::version::Version;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("reading file failed")]
    ReadingFileFailed,
    #[error("parsing file failed")]
    ParsingFileFailed,
}

pub fn get_dll_version(dll_path: impl AsRef<Path>) -> Result<Version, Error> {
    let Ok(data) = fs::read(dll_path) else {
        return Err(Error::ReadingFileFailed);
    };
    let Ok(dll) = parse(&data) else {
        return Err(Error::ParsingFileFailed);
    };
    Ok(dll.version)
}
