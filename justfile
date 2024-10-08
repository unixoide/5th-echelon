set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

# backticks to evaluate $(PWD)
release_flags := `"-Zremap-cwd-prefix=. --remap-path-prefix=$(PWD)=/ -Clink-arg=/PDBALTPATH:%_PDB%"`

default:
    just --list

serve $RUST_LOG="info":
    cargo run -p dedicated_server

serve_release $RUST_LOG="info":
    cargo run -p dedicated_server --release

uplay $CARGO_TARGET_DIR="target_i686":
    cargo build -p hooks --target i686-pc-windows-msvc
    cp "$env:CARGO_TARGET_DIR\i686-pc-windows-msvc\debug\hooks.dll" "C:\Program Files (x86)\Steam\steamapps\common\Tom Clancy's Splinter Cell Blacklist\src\SYSTEM\uplay_r1_loader.dll"
    cp "$env:CARGO_TARGET_DIR\i686-pc-windows-msvc\debug\hooks.pdb" "C:\Program Files (x86)\Steam\steamapps\common\Tom Clancy's Splinter Cell Blacklist\src\SYSTEM\hooks.pdb"

uplay_release $CARGO_TARGET_DIR="target_i686":
    cargo build -p hooks --target i686-pc-windows-msvc --release
    cp "$env:CARGO_TARGET_DIR\i686-pc-windows-msvc\release\hooks.dll" "C:\Program Files (x86)\Steam\steamapps\common\Tom Clancy's Splinter Cell Blacklist\src\SYSTEM\uplay_r1_loader.dll"
    cp "$env:CARGO_TARGET_DIR\i686-pc-windows-msvc\release\hooks.pdb" "C:\Program Files (x86)\Steam\steamapps\common\Tom Clancy's Splinter Cell Blacklist\src\SYSTEM\hooks.pdb"

launcher:
    cargo build -p launcher    

launch:
    cargo run -p launcher

launch_release:
    cargo run -p launcher --release

run_game: uplay
    cd "C:\Program Files (x86)\Steam\steamapps\common\Tom Clancy's Splinter Cell Blacklist\src\SYSTEM\"; .\Blacklist_game.exe

run_game_release: uplay_release
    cd "C:\Program Files (x86)\Steam\steamapps\common\Tom Clancy's Splinter Cell Blacklist\src\SYSTEM\"; .\Blacklist_game.exe

release: release_server release_uplay release_launcher_embed
    7z a -tzip 5th-echelon.zip data\ .\target\release\dedicated_server.exe .\target\release\launcher.exe .\target_i686\i686-pc-windows-msvc\release\hooks.dll
    7z rn 5th-echelon.zip hooks.dll uplay_r1_loader.dll

release_server $RUSTFLAGS=release_flags:
    cargo build -p dedicated_server --release

release_uplay $CARGO_TARGET_DIR="target_i686" $RUSTFLAGS=release_flags:
    cargo build -p hooks --release

release_launcher $RUSTFLAGS=release_flags: release_uplay
    cargo build -p launcher --release

release_launcher_embed $RUSTFLAGS=release_flags: release_uplay
    cargo build -p launcher --release --features embed-dll

generate_protocols BINARY ID_FILE:
    cargo run -p quazal-tools --bin ddl-parser -- -g ./dedicated_server/sc_bl_protocols/src/protocols -i {{ ID_FILE }} {{ BINARY }}
    cargo fmt -p sc_bl_protocols
    cargo fix -p sc_bl_protocols --allow-dirty
    cargo fmt -p sc_bl_protocols
    cargo check -p sc_bl_protocols

build-cross-msvc-image:
    [ -d cross ] || git clone https://github.com/cross-rs/cross
    cd cross && git submodule update --init --remote && cargo +stable build-docker-image i686-pc-windows-msvc-cross --tag local

build-hooks-linux: build-cross-msvc-image
    RUSTC_WRAPPER="" cross build -p hooks --target i686-pc-windows-msvc --target-dir=target_i686

test-hooks-linux: build-cross-msvc-image
    RUSTC_WRAPPER="" cross test -p hooks --target i686-pc-windows-msvc --target-dir=target_i686

test-linux: && test-hooks-linux
    cargo test --workspace --exclude hooks
