use std::ffi;
use std::io;
use std::os::windows::ffi::OsStringExt as _;

use tracing::debug;
use tracing::instrument;

/// Represents a parsed DLL file, containing its version information.
#[derive(Default, Debug)]
pub struct Dll {
    pub version: Option<String>,
    pub company: Option<String>,
    pub product: Option<String>,
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
    read_impl!(u64);
}

impl<T: AsRef<[u8]>> BytesRead for std::io::Cursor<T> {}

trait SubCursor {
    // fn sub_cursor(&mut self, size: usize) -> std::io::Result<std::io::Cursor<&[u8]>>;
    fn sub_cursor_from_length(&mut self) -> std::io::Result<std::io::Cursor<&[u8]>>;
}

impl SubCursor for std::io::Cursor<&[u8]> {
    // fn sub_cursor(&mut self, size: usize) -> std::io::Result<std::io::Cursor<&[u8]>> {
    //     #[allow(clippy::cast_possible_truncation)]
    //     let pos = self.position() as usize;
    //     if pos + size > self.get_ref().len() {
    //         return Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "not enough bytes"));
    //     }
    //     self.set_position((pos + size) as u64);
    //     let inner = self.get_ref();
    //     Ok(std::io::Cursor::new(&inner[pos..][..size]))
    // }

    fn sub_cursor_from_length(&mut self) -> std::io::Result<std::io::Cursor<&[u8]>>
    where
        Self: BytesRead,
    {
        #[allow(clippy::cast_possible_truncation)]
        let pos = self.position() as usize;
        let size = self.u16()? as usize;
        let end_pos = pos.saturating_add(size);
        debug!("sub cursor of size {} ({} -> {})", size, pos, end_pos);
        if end_pos > self.get_ref().len() {
            debug!("end_pos > max_pos");
            return Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "not enough bytes"));
        }
        self.set_position(end_pos as u64);
        let inner = self.get_ref();
        Ok(std::io::Cursor::new(&inner[pos..end_pos]))
    }
}

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

#[instrument]
fn parse_fixed_file_info(version_data: &mut io::Cursor<&[u8]>) -> io::Result<Dll> {
    let signature = version_data.u32()?;
    assert_eq!(signature, 0xFEEF_04BD);
    let _struct_version = version_data.u32()?;
    let file_version_minor = version_data.u16()? as usize;
    let file_version_major = version_data.u16()? as usize;
    let _file_version_x = version_data.u16()?;
    let file_version_patch = version_data.u16()? as usize;
    let _product_version = version_data.u64()?;
    let _file_flags_mask = version_data.u64()?;
    let _file_flags = version_data.u32()?;
    let _file_os = version_data.u32()?;
    let _file_type = version_data.u32()?;
    let _file_sub_type = version_data.u32()?;
    let _file_timestamp = version_data.u32()?;

    let dll = Dll {
        version: Some(format!("{file_version_major}.{file_version_minor}.{file_version_patch}")),
        ..Default::default()
    };
    debug!("Parsed version info: {:?}", dll);
    Ok(dll)
}

#[instrument(skip_all)]
fn parse_version(version_data: &mut io::Cursor<&[u8]>) -> io::Result<Dll> {
    let mut version_data = version_data.sub_cursor_from_length()?;
    let length = version_data.u16()?;
    let _value_length = version_data.u16()?;
    let _type = version_data.u16()?;

    let key = (0..16).map(|_| version_data.u16()).collect::<Result<Vec<u16>, _>>()?;
    let end = key.iter().position(|p| *p == 0);

    assert_eq!(ffi::OsString::from_wide(&key[..end.unwrap_or_default()]).to_str().unwrap(), "VS_VERSION_INFO");
    while version_data.position() % 4 != 0 {
        version_data.u8()?;
    }
    let mut dll = parse_fixed_file_info(&mut version_data)?;
    while version_data.position() % 4 != 0 {
        version_data.u8()?;
    }

    loop {
        match parse_version_entry(&mut version_data, &mut dll) {
            Ok(()) => {}
            Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => {
                break;
            }
            Err(e) => return Err(e),
        }
    }
    Ok(dll)
}

#[instrument(skip_all)]
fn parse_string(version_data: &mut io::Cursor<&[u8]>, dll: &mut Dll) -> io::Result<()> {
    let mut version_data = version_data.sub_cursor_from_length()?;
    let length = version_data.u16()?;
    let _value_length = version_data.u16()?;
    let _type = version_data.u16()?;
    let key = std::iter::from_fn(|| version_data.u16().ok()).take_while(|&c| c != 0).collect::<Vec<u16>>();
    while version_data.position() % 4 != 0 {
        version_data.u8()?;
    }

    let key = ffi::OsString::from_wide(&key).into_string().unwrap();

    let value = std::iter::from_fn(|| version_data.u16().ok()).take_while(|&c| c != 0).collect::<Vec<u16>>();

    let value = ffi::OsString::from_wide(&value).into_string().unwrap();

    debug!("{key}: {value}");

    if key == "CompanyName" {
        dll.company = Some(value);
    } else if key == "ProductName" {
        dll.product = Some(value);
    }

    Ok(())
}

#[instrument(skip_all)]
fn parse_string_table(version_data: &mut io::Cursor<&[u8]>, dll: &mut Dll) -> io::Result<()> {
    let mut version_data = version_data.sub_cursor_from_length()?;
    let length = version_data.u16()?;
    let _value_length = version_data.u16()?;
    let _type = version_data.u16()?;
    let key = std::iter::from_fn(|| version_data.u16().ok()).take_while(|&c| c != 0).collect::<Vec<u16>>();

    loop {
        while version_data.position() % 4 != 0 {
            version_data.u8()?;
        }
        match parse_string(&mut version_data, dll) {
            Ok(()) => {}
            Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => {
                break;
            }
            Err(e) => return Err(e),
        }
    }

    Ok(())
}

#[instrument(skip_all)]
fn parse_version_entry(version_data: &mut io::Cursor<&[u8]>, dll: &mut Dll) -> io::Result<()> {
    let mut version_data = version_data.sub_cursor_from_length()?;
    let length = version_data.u16()?;
    let _value_length = version_data.u16()?;
    let _type = version_data.u16()?;

    let key = std::iter::from_fn(|| version_data.u16().ok()).take_while(|&c| c != 0).collect::<Vec<u16>>();
    while version_data.position() % 4 != 0 {
        version_data.u8()?;
    }

    let key = ffi::OsString::from_wide(&key);
    let key = key.to_str().unwrap();

    // skip entries we're not interested in
    if key != "StringFileInfo" {
        return Ok(());
    }

    parse_string_table(&mut version_data, dll)
}

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
    let Some(entry) = root_dir.id_entries.iter().find(|entry| entry.id == RT_VERSION) else {
        return Ok(Dll::default());
    };
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
    parse_version(&mut version_data)
}
