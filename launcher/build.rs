use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

fn root_manifest_dir() -> PathBuf {
    let output = &String::from_utf8(
        Command::new("cargo")
            .arg("locate-project")
            .arg("--workspace")
            .arg("--message-format")
            .arg("plain")
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap();
    let path = Path::new(output.trim());
    assert_eq!(path.file_name().unwrap(), "Cargo.toml");
    path.parent().unwrap().to_owned()
}

fn embed_dll() {
    use brotli::enc::BrotliEncoderParams;
    use json::JsonValue;

    let is_release = env::var("PROFILE").unwrap() == "release";
    let dir = root_manifest_dir();
    let target_dir = dir.join("target_i686");
    let mut cmd = Command::new("cargo");
    cmd.arg("build")
        .arg("-p")
        .arg("hooks")
        .arg("--target")
        .arg("i686-pc-windows-msvc")
        .arg("--target-dir")
        .arg(target_dir.to_str().unwrap())
        .arg("--message-format")
        .arg("json");

    if is_release {
        cmd.arg("--release").env(
            "RUSTFLAGS",
            "-Zremap-cwd-prefix=. --remap-path-prefix=$(PWD)=/ -Clink-arg=/PDBALTPATH:%_PDB%",
        );
    }

    let results = String::from_utf8(cmd.output().unwrap().stdout).unwrap();

    let artifacts = results
        .lines()
        .map(json::parse)
        .map(Result::unwrap)
        .filter_map(|event| {
            let JsonValue::Object(obj) = event else {
                return None;
            };
            let JsonValue::Short(reason) = obj.get("reason")? else {
                return None;
            };
            if reason != "compiler-artifact" {
                return None;
            }
            let JsonValue::Object(target) = obj.get("target")? else {
                return None;
            };
            if !matches!(target.get("name")?, JsonValue::Short(s) if s == "hooks") {
                return None;
            }
            let JsonValue::Array(filenames) = obj.get("filenames")? else {
                return None;
            };
            let filenames = filenames
                .iter()
                .filter_map(|entry| {
                    let JsonValue::String(name) = entry else {
                        return None;
                    };
                    Some(PathBuf::from(name))
                })
                .collect::<Vec<PathBuf>>();
            Some(filenames)
        })
        .flatten()
        .filter(|artifact| {
            artifact
                .extension()
                .map(|ext| ext == "dll")
                .unwrap_or_default()
        })
        .collect::<Vec<PathBuf>>();

    assert_eq!(artifacts.len(), 1);

    let dll_path: PathBuf = artifacts.into_iter().next().unwrap();

    println!("cargo:warning=Dll path: {dll_path:?}");

    let data = fs::read(&dll_path).unwrap();
    let dll = dll::parse(&data).unwrap();

    println!(
        "cargo:warning=Dll version: {}.{}.{}",
        dll.version.0, dll.version.1, dll.version.2
    );
    println!(
        "cargo:rustc-env=HOOKS_VERSION={}.{}.{}",
        dll.version.0, dll.version.1, dll.version.2
    );

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let payload_path = out_dir.join("uplay_r1_loader.dll.brotli");

    let mut in_file = fs::File::open(dll_path).unwrap();
    let mut out_file = fs::File::create(payload_path).unwrap();
    let params = BrotliEncoderParams {
        quality: 5,
        ..Default::default()
    };
    brotli::BrotliCompress(&mut in_file, &mut out_file, &params).unwrap();
}

mod dll {
    include!("src/dll_utils.rs");
}

fn main() {
    let dir = root_manifest_dir();
    let hooks_dir = dir.join("hooks");
    println!("cargo:rerun-if-changed={}", hooks_dir.to_str().unwrap());
    embed_dll();

    #[cfg(target_os = "windows")]
    {
        winres::WindowsResource::new()
            .set_icon("logo.ico")
            .compile()
            .unwrap();
        println!("cargo:rerun-if-changed=logo.ico");
    }

    let img = image::open("../docs/logo.png").unwrap();
    let mut f =
        fs::File::create(PathBuf::from(env::var("OUT_DIR").unwrap()).join("logo.dat")).unwrap();
    f.write_all(img.as_rgba8().unwrap()).unwrap();
    println!("cargo:rerun-if-changed=../docs/logo.png");
}
