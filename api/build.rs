use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("friends_descriptor.bin"))
        .compile_protos(&["proto/friends.proto"], &["proto/"])?;

    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("users_descriptor.bin"))
        .compile_protos(&["proto/users.proto"], &["proto/"])?;

    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("misc_descriptor.bin"))
        .compile_protos(&["proto/misc.proto"], &["proto/"])?;

    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("games_descriptor.bin"))
        .compile_protos(&["proto/games.proto"], &["proto/"])?;
    Ok(())
}
