mod clipboard;
mod logging;
mod render;

use std::collections::HashMap;
use std::fmt::Display;
use std::io::Read as _;
use std::io::Seek as _;
use std::io::Write as _;
use std::path::PathBuf;

use clap::Parser as _;
use enumflags2::BitFlags;
use imgui::Context;
use logging::init_logging;
use tracing::debug;
use tracing::error;
use tracing::info;
use tracing::instrument;

fn ask_for_filename() -> color_eyre::Result<Option<PathBuf>> {
    // ask for file with a windows file dialog. use the windows crate
    use std::path::Path;

    use windows::Win32::System::Com::CLSCTX_INPROC_SERVER;
    use windows::Win32::System::Com::CoCreateInstance;
    use windows::Win32::System::Com::CoInitialize;
    use windows::Win32::System::Com::CoUninitialize;
    use windows::Win32::UI::Shell::Common::COMDLG_FILTERSPEC;
    use windows::Win32::UI::Shell::FOS_FILEMUSTEXIST;
    use windows::Win32::UI::Shell::IFileOpenDialog;
    use windows::Win32::UI::Shell::IShellItem;
    use windows::core::w;

    #[allow(non_snake_case)]
    let CLSID_FileOpenDialog: windows::core::GUID = "DC1C5A9C-E88A-4dde-A5A1-60F82A20AEF7".into();

    let path = unsafe {
        CoInitialize(None).unwrap();
        let dialog: IFileOpenDialog = CoCreateInstance(&CLSID_FileOpenDialog, None, CLSCTX_INPROC_SERVER)?;
        let options = dialog.GetOptions()?;

        dialog.SetOptions(options | FOS_FILEMUSTEXIST)?;
        dialog.SetDefaultExtension(w!("umd"))?;
        dialog.SetFileTypes(&[
            COMDLG_FILTERSPEC {
                pszName: w!("UMD"),
                pszSpec: w!("*.umd"),
            },
            // COMDLG_FILTERSPEC {
            //     pszName: w!("UMD Asset"),
            //     pszSpec: w!("*.ass"),
            // },
        ])?;
        dialog.SetTitle(w!("Select a file"))?;
        let path = if dialog.Show(None).is_ok() {
            let item: IShellItem = dialog.GetResult()?;
            let path_ptr = item.GetDisplayName(windows::Win32::UI::Shell::SIGDN_FILESYSPATH)?;
            let path = Path::new(path_ptr.to_string().unwrap().as_str()).to_path_buf();
            windows::Win32::System::Com::CoTaskMemFree(Some(path_ptr.as_ptr() as _));
            Some(path)
        } else {
            None
        };
        CoUninitialize();
        path
    };
    Ok(path)
}

trait Reader {
    fn read<R: std::io::Read>(&mut self, reader: &mut R) -> std::io::Result<()>;
}

macro_rules! num_reader {
    ($ty:ty) => {
        impl Reader for $ty {
            fn read<R: std::io::Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
                let mut tmp = [0u8; std::mem::size_of::<$ty>()];
                reader.read_exact(&mut tmp)?;
                *self = <$ty>::from_le_bytes(tmp).into();
                Ok(())
            }
        }
    };
}

num_reader!(u8);
num_reader!(u16);
num_reader!(u32);
num_reader!(u64);
num_reader!(usize);
num_reader!(i8);
num_reader!(i16);
num_reader!(i32);
num_reader!(i64);
num_reader!(isize);

impl Reader for String {
    fn read<R: std::io::Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
        let len = read_compressed_index(reader)?;
        let mut tmp = vec![0u8; len as usize];
        reader.read_exact(&mut tmp)?;
        if tmp.last() == Some(&0) {
            tmp.pop();
        }
        *self = String::from_utf8(tmp).unwrap();
        Ok(())
    }
}

#[repr(u8)]
#[enumflags2::bitflags]
#[derive(Clone, Copy, Debug)]
enum Flags {
    Zlib = 0x01,
    XMem = 0x02,
    CRC1 = 0x10,
    CRC2 = 0x20,
    CompressTables = 0x80,
}

impl Reader for BitFlags<Flags> {
    fn read<R: std::io::Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
        let mut tmp = [0u8; 1];
        reader.read_exact(&mut tmp)?;
        *self = Self::from_bits_truncate(tmp[0]);
        Ok(())
    }
}

#[derive(Debug, Default)]
struct UmdHeader {
    tag: [u8; 4],
    buffer_size: u32,
    file_size: u32,
    directory_offset: u32,
    flags: BitFlags<Flags>,
    data_skip: Option<u32>,
    table_size_uncompr: Option<u32>,
    table_size_compr: Option<u32>,
}

impl Reader for UmdHeader {
    fn read<R: std::io::Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
        reader.read_exact(&mut self.tag)?;
        self.buffer_size.read(reader)?;
        self.file_size.read(reader)?;
        self.directory_offset.read(reader)?;
        self.flags.read(reader)?;
        if self.flags.contains(Flags::CompressTables) {
            self.data_skip = Some(0);
            self.table_size_uncompr = Some(0);
            self.table_size_compr = Some(0);
            self.data_skip.as_mut().unwrap().read(reader)?;
            self.table_size_uncompr.as_mut().unwrap().read(reader)?;
            self.table_size_compr.as_mut().unwrap().read(reader)?;
        }
        Ok(())
    }
}

impl UmdHeader {
    fn read_pages<R: std::io::Read + std::io::Seek>(&self, reader: &mut R, mut offset: u64) -> std::io::Result<Vec<Page>> {
        let mut page_reader = if self.table_size_compr.unwrap() > 0 {
            let mut table_compressed_data = vec![0u8; self.table_size_compr.unwrap() as usize];
            reader.read_exact(&mut table_compressed_data)?;

            Box::new(flate2::read::ZlibDecoder::new(std::io::Cursor::new(table_compressed_data))) as Box<dyn std::io::Read>
        } else {
            let mut table_data = vec![0u8; self.table_size_uncompr.unwrap() as usize];
            reader.read_exact(&mut table_data)?;
            Box::new(std::io::Cursor::new(table_data)) as Box<dyn std::io::Read>
        };
        let num_pages = read_compressed_index(&mut page_reader)?;
        debug!("Found {} pages", num_pages);
        offset += self.data_skip.unwrap() as u64;
        let pages: Result<Vec<Page>, _> = (0..num_pages)
            .map(|_| {
                let mut page = Page::default();
                page.read(&mut page_reader)?;
                page.file_offset = offset;
                offset += page.compressed_size;
                Ok(page)
            })
            .collect();
        debug!("Pages: {:#x?}", pages);
        pages
    }
}

fn read_compressed_index<R: std::io::Read>(reader: &mut R) -> std::io::Result<u64> {
    let mut tmp = [0u8; 1];
    reader.read_exact(&mut tmp)?;
    let mut res = tmp[0] as u64 & 0x3f;
    let mut shift = 6;
    if tmp[0] & 0x40 != 0 {
        loop {
            let mut tmp = [0u8; 1];
            reader.read_exact(&mut tmp)?;
            res |= (tmp[0] as u64 & 0x7f) << shift;
            shift += 7;
            if tmp[0] & 0x80 == 0 {
                break;
            }
        }
    }
    Ok(res)
}

#[derive(Default, Debug)]
struct Page {
    compressed_size: u64,
    flags: BitFlags<Flags>,
    file_offset: u64,
}

impl Reader for Page {
    fn read<R: std::io::Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.compressed_size = read_compressed_index(reader)?;
        self.flags.read(reader)?;
        Ok(())
    }
}

impl Page {
    fn decompress<R: std::io::Read + std::io::Seek>(&mut self, reader: &mut R) -> std::io::Result<Vec<u8>> {
        reader.seek(std::io::SeekFrom::Start(self.file_offset))?;
        let mut data = vec![0u8; self.compressed_size as usize];
        reader.read_exact(&mut data)?;
        if self.flags.contains(Flags::Zlib) {
            let mut decompressed = vec![];
            flate2::read::ZlibDecoder::new(std::io::Cursor::new(data)).read_to_end(&mut decompressed)?;
            data = decompressed;
        }
        Ok(data)
    }
}

#[derive(Debug, Default, Clone)]
struct Chunk {
    original_offset: u64,
    uncompressed_offset: u64,
    chunk_size: u64,
}

#[derive(Debug, Default, Clone)]
struct DirEntry {
    id: u32,
    short_file_name: String,
    long_file_name: String,
    file_size: u64,
    chunks: Vec<Chunk>,
}

impl Reader for DirEntry {
    fn read<R: std::io::Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.id.read(reader)?;
        self.short_file_name.read(reader)?;
        self.long_file_name.read(reader)?;
        self.file_size = read_compressed_index(reader)?;
        Ok(())
    }
}

impl DirEntry {
    #[instrument(skip_all)]
    fn check(&self) -> bool {
        for (i, chunks) in self.chunks.windows(2).enumerate() {
            let a = &chunks[0];
            let b = &chunks[1];
            if a.original_offset > b.original_offset {
                error!("Chunks {} and {} are not sorted for {}: {:#x?} {:#x?}", i, i + 1, self.long_file_name, a, b);
                return false;
            }
            if a.original_offset + a.chunk_size > b.original_offset {
                error!("Chunks {} and {} overlap for {}: {:#x?} {:#x?}", i, i + 1, self.long_file_name, a, b);
                return false;
            }
        }
        if let Some(size) = self.chunks.last().map(|c| c.original_offset + c.chunk_size) {
            if size != self.file_size {
                error!("File size does not match chunk sizes for {}: {} != {}", self.long_file_name, self.file_size, size);
                return false;
            }
        }
        true
    }
}

struct UmdReader<R: std::io::Read + std::io::Seek> {
    reader: R,
    header: UmdHeader,
    pages: Vec<Page>,
    current_position: u64,
    buffer: Vec<u8>,
    loaded_pages: usize,
    data_start: u64,
}

impl<R: std::io::Read + std::io::Seek> UmdReader<R> {
    fn new(mut reader: R) -> std::io::Result<Self> {
        let mut header = UmdHeader::default();
        header.read(&mut reader)?;
        debug!("Header: {:#x?}", header);
        let (data_start, pages) = if &header.tag == b"EPCK" {
            let data_start = reader.stream_position()?;
            let pages = header.read_pages(&mut reader, data_start)?;
            // let data_start = reader.stream_position()?;
            (0, pages)
        } else {
            reader.seek(std::io::SeekFrom::Start(0))?;
            (0, vec![])
        };

        pages.iter().map(|p| p.compressed_size).sum::<u64>();
        Ok(Self {
            reader,
            header,
            pages,
            current_position: 0,
            buffer: vec![],
            loaded_pages: 0,
            data_start,
        })
    }

    fn read_file_list(&mut self) -> std::io::Result<Vec<DirEntry>> {
        self.seek(std::io::SeekFrom::End(-20))?;
        let mut tmp = [0u8; 4];
        self.read_exact(&mut tmp)?;
        if tmp == [0xa3, 0xc5, 0xe3, 0x9f] {
            info!("Detected arc style");
            let mut table_offset = 0u32;
            table_offset.read(self)?;
            self.seek(std::io::SeekFrom::Start(table_offset as u64))?;
            self.read_file_list_arc()
        } else {
            info!("Detected other style");
            self.seek(std::io::SeekFrom::Start(0))?;
            self.read_file_list_other()
        }
    }

    fn read_file_list_arc(&mut self) -> std::io::Result<Vec<DirEntry>> {
        let num_entries = read_compressed_index(self)?;
        (0..num_entries)
            .map(|_| {
                let mut filename = String::new();
                filename.read(self)?;
                let mut offset = 0u32;
                offset.read(self)?;
                let mut size = 0u32;
                size.read(self)?;
                let mut flags = 0u32;
                flags.read(self)?;
                let mut unk = 0u32;
                unk.read(self)?;
                Ok(DirEntry {
                    id: 0,
                    short_file_name: filename.clone(),
                    long_file_name: filename,
                    file_size: size as u64,
                    chunks: vec![Chunk {
                        original_offset: 0,
                        uncompressed_offset: offset as u64,
                        chunk_size: size as u64,
                    }],
                })
            })
            .collect()
    }

    fn read_file_list_other(&mut self) -> std::io::Result<Vec<DirEntry>> {
        let mut id = String::new();
        id.read(self)?;
        debug!("ID: {id}");
        let num_entries = read_compressed_index(self)?;
        let mut entries: HashMap<u32, DirEntry> = (0..num_entries)
            .map(|_| {
                let mut entry = DirEntry::default();
                entry.read(self)?;
                debug!("Entry: {:#x?}", entry);
                Ok((entry.id, entry))
            })
            .collect::<std::io::Result<_>>()?;

        let num_chunks_lists = read_compressed_index(self)?;
        for _ in 0..num_chunks_lists {
            let mut id = 0u32;
            id.read(self)?;
            debug!("Chunk List ID: {id:x?}");
            let num_chunks = read_compressed_index(self)?;
            let chunks = (0..num_chunks)
                .map(|_| {
                    let original_offset = read_compressed_index(self)?;
                    let uncompressed_offset = read_compressed_index(self)?;
                    let chunk_size = read_compressed_index(self)?;
                    let chunk = Chunk {
                        original_offset,
                        uncompressed_offset,
                        chunk_size,
                    };
                    debug!("Chunk: {:#x?}", chunk);
                    Ok(chunk)
                })
                .collect::<std::io::Result<Vec<Chunk>>>()?;
            let entry = entries.get_mut(&id).unwrap();
            entry.chunks.extend(chunks);
        }
        self.data_start = self.current_position;
        self.data_start = (self.data_start + 0x20000 - 1) & !(0x20000 - 1);

        let mut entries = entries.into_values().collect::<Vec<_>>();

        for entry in entries.iter_mut() {
            entry.chunks.sort_by_key(|c| c.original_offset);
        }
        entries.sort_by(|a, b| a.long_file_name.cmp(&b.long_file_name));

        debug!("Entries: {:#x?}", entries);

        Ok(entries)
    }

    fn read_file(&mut self, entry: &DirEntry) -> std::io::Result<Vec<u8>> {
        let mut data = vec![0u8; entry.file_size as usize];
        for chunk in &entry.chunks {
            let new_len = chunk.original_offset as usize + chunk.chunk_size as usize;
            if data.len() < new_len {
                data.resize(new_len, 0);
            }
            self.seek(std::io::SeekFrom::Start(self.data_start + chunk.uncompressed_offset))?;
            let chunk_data = &mut data[chunk.original_offset as usize..new_len];
            self.read_exact(chunk_data)?;
        }
        Ok(data)
    }
}

impl<R: std::io::Read + std::io::Seek> std::io::Seek for UmdReader<R> {
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
        if self.pages.is_empty() {
            return self.reader.seek(pos);
        }
        match pos {
            std::io::SeekFrom::Start(pos) => self.current_position = pos,
            std::io::SeekFrom::End(rel) => {
                self.current_position = (self.header.file_size as u64).overflowing_add_signed(rel).0;
            }
            std::io::SeekFrom::Current(rel) => {
                if rel < 0 {
                    self.current_position = self.current_position.saturating_sub(-rel as u64);
                } else {
                    self.current_position = self.current_position.saturating_add(rel as u64);
                }
            }
        }
        Ok(self.current_position)
    }
}

impl<R: std::io::Read + std::io::Seek> std::io::Read for UmdReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        // no pages, read directly from backing storage
        if self.pages.is_empty() {
            return self.reader.read(buf);
        }
        // EOF
        if self.current_position == self.header.file_size as u64 {
            return Ok(0);
        }

        let pos = self.current_position as usize;
        // already buffered, shortcut it
        if pos < self.buffer.len() && buf.len() <= self.buffer.len() - pos {
            buf.copy_from_slice(&self.buffer[pos..][..buf.len()]);
            self.current_position += buf.len() as u64;
            return Ok(buf.len());
        }

        if self.loaded_pages < self.pages.len() {
            while pos >= self.buffer.len() || buf.len() > self.buffer.len() - pos {
                self.buffer.extend(self.pages[self.loaded_pages].decompress(&mut self.reader)?);
                self.loaded_pages += 1;
                if self.loaded_pages >= self.pages.len() {
                    break;
                }
            }
        }
        // looks like we're out of pages
        if pos > self.buffer.len() {
            error!("All pages loaded, but current position is beyond the end of the buffer ({} > {})", pos, self.buffer.len());
            return Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "Unexpected EOF. Missing pages"));
        }

        let len = std::cmp::min(buf.len(), self.buffer.len() - pos);
        buf[..len].copy_from_slice(&self.buffer[pos..][..len]);
        self.current_position += len as u64;
        Ok(len)
    }
}

#[derive(Debug)]
enum FileSystemEntry {
    Dir(String, Vec<FileSystemEntry>),
    File(String, DirEntry),
}

enum FileSystemEntryBuilder {
    Dir(String, HashMap<String, FileSystemEntryBuilder>),
    File(String, DirEntry),
}

impl From<FileSystemEntryBuilder> for FileSystemEntry {
    fn from(value: FileSystemEntryBuilder) -> Self {
        match value {
            FileSystemEntryBuilder::Dir(name, entries) => Self::Dir(name, entries.into_values().map(|entry| entry.into()).collect()),
            FileSystemEntryBuilder::File(name, entry) => Self::File(name, entry),
        }
    }
}

impl FileSystemEntry {
    fn new(entries: Vec<DirEntry>) -> Self {
        let mut root = FileSystemEntryBuilder::Dir(String::from("/"), HashMap::default());

        for entry in entries {
            let file_path = entry.long_file_name.replace('\\', "/");
            let file_path = std::path::PathBuf::from(file_path);
            let mut components = file_path.components().collect::<Vec<_>>();
            let last_component = components.pop().unwrap();

            let mut fs_entry = &mut root;
            for component in components {
                match component {
                    std::path::Component::Prefix(_prefix_component) => fs_entry = &mut root,
                    std::path::Component::RootDir => fs_entry = &mut root,
                    std::path::Component::CurDir => {}
                    std::path::Component::ParentDir => fs_entry = &mut root,
                    std::path::Component::Normal(os_str) => {
                        let name = os_str.to_string_lossy().into_owned();
                        let FileSystemEntryBuilder::Dir(_, entries) = fs_entry else { todo!() };
                        let entry = entries.entry(name.clone()).or_insert_with(|| FileSystemEntryBuilder::Dir(name, HashMap::default()));
                        fs_entry = entry;
                    }
                }
            }
            let FileSystemEntryBuilder::Dir(_, entries) = fs_entry else { todo!() };
            entries.insert(
                last_component.as_os_str().to_string_lossy().into_owned(),
                FileSystemEntryBuilder::File(last_component.as_os_str().to_string_lossy().into_owned(), entry),
            );
        }

        let mut root: FileSystemEntry = root.into();
        root.sort();
        root
    }

    fn render(&self, ui: &imgui::Ui, open: bool) -> Option<&DirEntry> {
        ui.table_next_row();
        ui.table_next_column();
        match self {
            FileSystemEntry::Dir(name, entries) => {
                let clicked = ui.tree_node_config(name).default_open(open).build(|| {
                    let mut res = None;
                    for entry in entries {
                        res = res.or(entry.render(ui, false));
                    }
                    res
                });
                ui.table_next_column();
                ui.table_next_column();
                clicked
            }
            FileSystemEntry::File(name, dir_entry) => {
                let clicked = ui.tree_node_config(name).leaf(true).build(|| if ui.is_item_clicked() { Some(dir_entry) } else { None });
                ui.table_next_column();
                ui.text(format!("{}", dir_entry.chunks.len()));
                ui.table_next_column();
                let sz_txt = human_size(dir_entry.file_size);
                let pos = ui.cursor_pos();
                let x = pos[0] + ui.column_width(1) - ui.calc_text_size(&sz_txt)[0] - ui.scroll_x() - 2.0 * unsafe { ui.style() }.item_spacing[0];
                if x > pos[0] {
                    ui.set_cursor_pos([x, pos[1]]);
                }
                ui.text(sz_txt);
                clicked
            }
        }
        .flatten()
    }

    fn sort(&mut self) {
        let FileSystemEntry::Dir(_, entries) = self else {
            return;
        };
        entries.sort_by(|a, b| match (a, b) {
            (FileSystemEntry::Dir(_, _), FileSystemEntry::File(_, _)) => std::cmp::Ordering::Less,
            (FileSystemEntry::File(_, _), FileSystemEntry::Dir(_, _)) => std::cmp::Ordering::Greater,
            (FileSystemEntry::Dir(name, _), FileSystemEntry::Dir(name2, _)) | (FileSystemEntry::File(name, _), FileSystemEntry::File(name2, _)) => name.cmp(name2),
        });
        for entry in entries {
            entry.sort();
        }
    }

    fn check(&self) -> bool {
        match self {
            FileSystemEntry::Dir(_, entries) => {
                let mut valid = true;
                for entry in entries {
                    valid &= entry.check();
                }
                valid
            }
            FileSystemEntry::File(_, entry) => entry.check(),
        }
    }
}

#[instrument]
fn load(umd_filepath: PathBuf, dump_pages_out_path: Option<PathBuf>) -> (String, UmdReader<std::fs::File>, FileSystemEntry) {
    info!("Loading {}", umd_filepath.display());
    let file_name = umd_filepath.file_name().unwrap().to_string_lossy().into_owned();

    let mut reader = std::fs::File::open(umd_filepath).and_then(UmdReader::new).unwrap();

    if let Some(dump_pages_out_path) = dump_pages_out_path {
        info!("Dumping pages to {}", dump_pages_out_path.display());
        let pos = reader.stream_position().unwrap();
        reader.rewind().unwrap();
        let mut out = std::fs::File::create(dump_pages_out_path).unwrap();
        std::io::copy(&mut reader, &mut out).unwrap();
        reader.seek(std::io::SeekFrom::Start(pos)).unwrap();
    }

    let files = reader.read_file_list().unwrap();
    debug!(
        "Found {} files in archive: {}",
        files.len(),
        files.iter().map(|d| d.long_file_name.clone()).collect::<Vec<_>>().join("\n")
    );
    let fs = FileSystemEntry::new(files);
    if !fs.check() {
        error!("Loading returned inconsistent results");
    }
    (file_name, reader, fs)
}

enum Content {
    Original(Vec<u8>),
    Converted((String, usize)),
}

impl Display for Content {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Content::Original(data) => write!(f, "{}", String::from_utf8_lossy(data)),
            Content::Converted((data, _)) => write!(f, "{}", data),
        }
    }
}

impl Content {
    fn len(&self) -> usize {
        match self {
            Content::Original(data) => data.len(),
            Content::Converted((_, len)) => *len,
        }
    }

    fn convert(&mut self) {
        match self {
            Content::Original(content) => {
                let content = if content[0..3] == [0xEF, 0xBB, 0xBF] {
                    // UTF-8 BOM
                    &content[3..]
                } else {
                    content
                };
                let buf = std::str::from_utf8(content)
                    .map(String::from)
                    .ok()
                    .or_else(|| try_decode_utf16(content))
                    .unwrap_or_else(|| generate_hexdump(content));
                *self = Content::Converted((buf, content.len()));
            }
            Content::Converted(_) => {}
        }
    }
}

#[derive(clap::Parser, Debug)]
struct Args {
    umd_filepath: Option<PathBuf>,
    #[clap(long = "dump-pages")]
    dump_pages_out_path: Option<PathBuf>,
    #[clap(long)]
    export: bool,
}

fn main() {
    init_logging();

    let args = Args::parse();

    let Some(umd_filepath) = args.umd_filepath.or_else(|| ask_for_filename().unwrap()) else {
        return;
    };
    let (mut file_name, mut reader, mut fs) = load(umd_filepath, args.dump_pages_out_path);

    if args.export {
        export_archive(&fs, &mut reader);
        return;
    }

    let imgui = Context::create();
    let mut selected_file = None;
    let mut selected_content = None;
    render::render(imgui, |ui: &mut imgui::Ui, w: f32, h: f32| {
        ui.window("UMD Loader")
            .size([w, h], imgui::Condition::Always)
            .position([0f32, 0f32], imgui::Condition::Always)
            .movable(false)
            .resizable(false)
            .title_bar(false)
            .build(|| {
                let [w, h] = ui.window_content_region_max();
                ui.text(format!("Loaded: {}", file_name));
                ui.same_line();
                ui.text(format!("Uncompressed Size: {} ({})", reader.header.file_size, human_size(reader.header.file_size as u64)));
                ui.same_line();
                if ui.button("Export") {
                    export_archive(&fs, &mut reader);
                }
                ui.same_line();
                if ui.button("Open Other") {
                    if let Some(path) = ask_for_filename().unwrap() {
                        selected_content.take();
                        selected_file.take();
                        let (new_file_name, new_reader, new_fs) = load(path, None);
                        file_name = new_file_name;
                        reader = new_reader;
                        fs = new_fs;
                    }
                }
                let [_, y] = ui.cursor_pos();
                let clicked = ui
                    .child_window("Tree")
                    .border(false)
                    .size([w / 2.0, h - y])
                    .build(|| {
                        let _tbl = ui.begin_table_with_flags("Table", 3, imgui::TableFlags::SIZING_STRETCH_SAME)?;
                        ui.table_setup_column_with(imgui::TableColumnSetup {
                            name: "Name",
                            flags: imgui::TableColumnFlags::empty(),
                            init_width_or_weight: 9.0,
                            user_id: imgui::Id::default(),
                        });
                        ui.table_setup_column_with(imgui::TableColumnSetup {
                            name: "Chunks",
                            flags: imgui::TableColumnFlags::empty(),
                            init_width_or_weight: 1.0,
                            user_id: imgui::Id::default(),
                        });
                        ui.table_setup_column_with(imgui::TableColumnSetup {
                            name: "Size",
                            flags: imgui::TableColumnFlags::empty(),
                            init_width_or_weight: 1.0,
                            user_id: imgui::Id::default(),
                        });
                        let res = fs.render(ui, true);
                        res
                    })
                    .flatten();
                ui.same_line();
                if let Some(entry) = clicked {
                    selected_file.replace(entry.clone());
                    let content = reader.read_file(entry).unwrap();
                    selected_content.replace(Content::Original(content));
                }

                if let Some((dir_entry, content)) = selected_file.as_ref().zip(selected_content.as_mut()) {
                    ui.child_window("Data").border(false).size([w / 2.0, h - y]).build(|| {
                        ui.text(format!("Selected: {}", dir_entry.long_file_name));
                        ui.text(format!("  Short: {}", dir_entry.short_file_name));
                        ui.text(format!("  Chunks: {}", dir_entry.chunks.len()));
                        ui.text(format!("  Original Offset: {}", dir_entry.chunks[0].original_offset));
                        ui.text(format!(
                            "  Uncompressed Offset: {} ({})",
                            dir_entry.chunks[0].uncompressed_offset,
                            reader.data_start + dir_entry.chunks[0].uncompressed_offset
                        ));
                        ui.text(format!("  Size: {} ({})", dir_entry.file_size, human_size(dir_entry.file_size)));
                        if content.len() != dir_entry.file_size as usize {
                            ui.text_colored([1.0, 0.0, 0.0, 1.0], format!("  Content Size: {} ({})", content.len(), human_size(content.len() as u64)));
                        }
                        content.convert();
                        let mut buf = content.to_string();
                        let [_, y] = ui.cursor_pos();

                        let min = ui.window_content_region_min();
                        let max = ui.window_content_region_max();
                        let [w, h] = [max[0] - min[0], max[1] - min[1]];
                        let sbsz = unsafe { ui.style() }.scrollbar_size;
                        ui.input_text_multiline("##content", &mut buf, [w - sbsz, h - y - sbsz])
                            .no_horizontal_scroll(false)
                            .read_only(true)
                            .build();
                    });
                }
            });
    });
}

fn try_decode_utf16(content: &[u8]) -> Option<String> {
    if content.len() < 2 {
        return None;
    }
    let bom = &content[0..2];
    if bom == [0xFE, 0xFF] {
        let content = content.chunks_exact(2).skip(1).map(|b| u16::from_ne_bytes([b[1], b[0]])).collect::<Vec<_>>();
        String::from_utf16(content.as_slice()).ok()
    } else if bom == [0xFF, 0xFE] {
        let content = content.chunks_exact(2).skip(1).map(|b| u16::from_ne_bytes([b[0], b[1]])).collect::<Vec<_>>();
        String::from_utf16(content.as_slice()).ok()
    } else {
        None
    }
}

fn generate_hexdump(content: &[u8]) -> String {
    let mut res = String::from("        0  1  2  3  4  5  6  7   8  9  A  B  C  D  E  F");

    for (i, b) in content.iter().enumerate() {
        if i % 16 == 0 {
            res.push_str(&format!("\n{:06X}: ", i));
        }
        res.push_str(&format!("{:02X} ", b));
        if i % 8 == 7 {
            res.push(' ');
        }
        if i % 16 == 15 {
            res.push_str(" |");
            for c in content.iter().take(i + 1).skip(i - 15) {
                if c.is_ascii_graphic() {
                    res.push(*c as char);
                } else {
                    res.push('.');
                }
            }
            res.push('|');
        }
    }
    if content.len() % 16 != 0 {
        let remaining = content.len() % 16;
        for _ in 0..(16 - remaining) {
            res.push_str("   ");
        }
        if remaining <= 8 {
            res.push(' ');
        }
        res.push_str("  |");
        for c in content.iter().skip(content.len() - remaining) {
            if c.is_ascii_graphic() {
                res.push(*c as char);
            } else {
                res.push('.');
            }
        }
        res.push('|');
    }
    res
}

fn human_size(size: u64) -> String {
    let mut size = size;
    let mut units = 0;
    while size >= 1024 {
        size /= 1024;
        units += 1;
    }
    match units {
        0 => format!("{}B", size),
        1 => format!("{}KB", size),
        2 => format!("{}MB", size),
        3 => format!("{}GB", size),
        4 => format!("{}TB", size),
        _ => format!("{}PB", size),
    }
}

#[instrument(skip_all)]
fn export_archive(fs: &FileSystemEntry, reader: &mut UmdReader<std::fs::File>) {
    let exec = std::env::current_exe().unwrap();
    let exec_dir = exec.parent().unwrap();
    let root_dir = exec_dir.join("exported");
    info!("Exporting to {root_dir:?}");
    std::fs::create_dir_all(&root_dir).unwrap_or_else(|e| {
        panic!("Error creating root dir {root_dir:?}: {e:?}");
    });

    let FileSystemEntry::Dir(_, root) = fs else {
        panic!("unexpected root");
    };
    let root: Vec<&FileSystemEntry> = root.iter().collect();
    let mut current_folder = vec![(root_dir, root)];
    while let Some((current_dir, entries)) = current_folder.pop() {
        for fs in entries {
            match fs {
                FileSystemEntry::Dir(name, entries) => {
                    let path = current_dir.join(name);
                    std::fs::create_dir_all(&path).unwrap_or_else(|e| {
                        panic!("Error creating dir {path:?}: {e:?}");
                    });
                    current_folder.push((path, entries.iter().collect()));
                }
                FileSystemEntry::File(name, dir_entry) => {
                    info!("Saving file {name} to {current_dir:?}");
                    let path = current_dir.join(name);
                    let mut file = std::fs::File::create(&path).unwrap_or_else(|e| {
                        panic!("Error creating file {path:?}: {e:?}");
                    });
                    let content = reader.read_file(dir_entry).unwrap();
                    file.write_all(&content).unwrap_or_else(|e| {
                        panic!("Error writing file {path:?}: {e:?}");
                    });
                }
            }
        }
    }
}
