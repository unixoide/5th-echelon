#![deny(clippy::pedantic)]

/// shim-gen
#[derive(argh::FromArgs)]
struct Args {
    /// library to parse
    #[argh(positional)]
    library: String,
}

fn parse_pe(pe: goblin::pe::PE) -> Vec<String> {
    pe.exports
        .into_iter()
        //.map(|e| format!("{:?}",e))
        .filter_map(|e| e.name.map(String::from))
        .collect()
}

fn main() {
    let args: Args = argh::from_env();
    let data = std::fs::read(&args.library).expect("valid file");
    let library = goblin::Object::parse(&data).expect("valid library");
    let exports = match library {
        goblin::Object::PE(pe) => parse_pe(pe),
        _ => unimplemented!(),
    };
    eprintln!("{exports:?}");

    let preamble = r"lazy_static::lazy_static! {
       static ref DLL_handle = unsafe { windows::Win32::System::LibraryLoader::LoadLibraryA(ORIGINAL_LIBRARY).unwrap() };
    }";
    println!("const ORIGINAL_LIBRARY: &str = {:?}", args.library);
    println!("{preamble}");

    for e in exports {
        if e.starts_with('?') {
            continue;
        }

        println!(
            r#"#[no_mangle]
unsafe extern "system" fn {e}() -> isize {{
   static mut func: Option<unsafe extern "system" fn() -> isize> = None;
   static mut ONCE: std::sync::Once = std::sync::Once::new();
   ONCE.call_once(|| {{
    func = Some(windows::Win32::System::LibraryLoader::GetProcAddress(DLL_handle, b"{e}\0").unwrap());
   }});
   (func.unwrap())()
}}

"#
        );
    }
}
