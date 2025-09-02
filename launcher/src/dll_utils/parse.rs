use std::ffi;
use std::io;
use std::os::windows::ffi::OsStringExt as _;

use crate::version::Version;

/// Represents a parsed DLL file, containing its version information.
pub struct Dll {
    pub version: Version,
}

/// A macro for implementing methods to read primitive types from a byte stream.
macro_rules! read_impl {
    ($ty:ident) => {
        fn $ty(&mut self) -> std::io::Result<$ty> {
            let mut tmp = [0u8; std::mem::size_of::<$ty>()];
            self.read_exact(&mut tmp)?;
            Ok($ty::from_le_bytes(tmp))
        }
    };
}

/// A trait for reading primitive types from a byte stream.
trait BytesRead
where
    Self: io::Read,
{
    fn u8(&mut self) -> std::io::Result<u8> {
        let mut tmp = [0u8; 1];
        self.read_exact(&mut tmp)?;
        Ok(tmp[0])
    }

    read_impl!(u16);
    read_impl!(u32);
}

impl<T: AsRef<[u8]>> BytesRead for std::io::Cursor<T> {}

/// Represents a named entry in a resource directory.
#[derive(Debug)]
#[allow(dead_code)]
struct NameEntry {
    name_offset: u32,
    offset: usize,
}

/// Represents an ID entry in a resource directory.
#[derive(Debug)]
struct IdEntry {
    offset: usize,
    id: u32,
}

/// Represents a resource directory in a PE file.
#[derive(Debug)]
struct Directory {
    #[allow(dead_code)]
    name_entries: Vec<NameEntry>,
    id_entries: Vec<IdEntry>,
}

impl Directory {
    /// Parses a resource directory from a byte stream.
    fn parse<R: BytesRead>(rdr: &mut R) -> io::Result<Self> {
        let _characteristics = rdr.u32()?;
        let _time_date_stamp = rdr.u32()?;
        let _major = rdr.u16()?;
        let _minor = rdr.u16()?;
        let name_entries_count = rdr.u16()?;
        let id_entries_count = rdr.u16()?;

        let name_entries: Vec<NameEntry> = (0..name_entries_count)
            .map(|_| {
                Ok(NameEntry {
                    name_offset: rdr.u32()?,
                    offset: (rdr.u32()? & 0x7fff_ffff) as usize,
                })
            })
            .collect::<io::Result<_>>()?;
        let id_entries: Vec<IdEntry> = (0..id_entries_count)
            .map(|_| {
                Ok(IdEntry {
                    id: rdr.u32()?,
                    offset: (rdr.u32()? & 0x7fff_ffff) as usize,
                })
            })
            .collect::<io::Result<_>>()?;

        Ok(Directory { name_entries, id_entries })
    }
}

/// The resource type for version information.
const RT_VERSION: u32 = 0x10;

/// Parses a DLL file from a byte slice and returns a `Dll` struct.
pub fn parse(data: &[u8]) -> io::Result<Dll> {
    let dll = goblin::pe::PE::parse(data).unwrap();
    // Find the resource section.
    let rsrc = dll
        .sections
        .iter()
        .find(|s| &s.name == b".rsrc\0\0\0")
        .ok_or(io::Error::other("resource section not found"))?;
    let rsrc_data = rsrc.data(data).unwrap().unwrap();

    // Parse the root resource directory.
    let root_dir = Directory::parse(&mut io::Cursor::new(&rsrc_data))?;
    let version = if let Some(entry) = root_dir.id_entries.iter().find(|entry| entry.id == RT_VERSION) {
        // Navigate through the resource directory to find the version information.
        let entry_data = &rsrc_data[entry.offset..];
        let resource_dir = Directory::parse(&mut io::Cursor::new(entry_data))?;
        let resource_data = &rsrc_data[resource_dir.id_entries.first().unwrap().offset..];
        let lang_dir = Directory::parse(&mut io::Cursor::new(&resource_data))?;
        let mut version_data = io::Cursor::new(&rsrc_data[lang_dir.id_entries.first().unwrap().offset..]);

        let rva = version_data.u32()? as usize;
        let size = version_data.u32()? as usize;
        let _codepage = version_data.u32()?;
        let _reserved = version_data.u32()?;
        // should do some checks here ¯\_(ツ)_/¯

        // Find the file offset of the version data using the RVA.
        let offset = goblin::pe::utils::find_offset(
            rva,
            &dll.sections,
            dll.header.optional_header.unwrap().windows_fields.file_alignment,
            &goblin::pe::options::ParseOptions::default(),
        )
        .unwrap();

        let mut version_data = io::Cursor::new(&data[offset..][..size]);

        // Parse the VS_VERSION_INFO struct.
        let _length = version_data.u16()?;
        let _value_length = version_data.u16()?;
        let _type = version_data.u16()?;

        let name = std::iter::from_fn(|| version_data.u16().ok()).take_while(|&c| c != 0).collect::<Vec<u16>>();
        let pad = (4 - (name.len() * 2 % 4)) % 4;

        for _ in 0..pad {
            version_data.u8()?;
        }

        assert_eq!(ffi::OsString::from_wide(&name).to_str().unwrap(), "VS_VERSION_INFO");
        let _signature = version_data.u32()?;
        let _struct_version = version_data.u32()?;
        let file_version_minor = version_data.u16()? as usize;
        let file_version_major = version_data.u16()? as usize;
        let _file_version_x = version_data.u16()?;
        let file_version_patch = version_data.u16()? as usize;
        (file_version_major, file_version_minor, file_version_patch)
    } else {
        (0, 0, 0)
    }
    .into();

    Ok(Dll { version })
}
