use dll_syringe::process::OwnedProcess;
use dll_syringe::Syringe;

fn get_or_spawn() -> OwnedProcess {
    if let Some(proc) = OwnedProcess::find_first_by_name("Blacklist_game.exe") {
        return proc;
    }

    // let proc = std::process::Command::new(r"C:\Program Files (x86)\Ubisoft\Ubisoft Game Launcher\UbisoftGameLauncher.exe")
    // .args([
    //     "-gamelauncher_wait_handle", "1376",
    //     "-upc_uplay_id","449",
    //     "-upc_game_version", "1",
    //     "-upc_exe_path", "QzpcUHJvZ3JhbSBGaWxlcyAoeDg2KVxTdGVhbVxzdGVhbWFwcHNcY29tbW9uXFRvbSBDbGFuY3kncyBTcGxpbnRlciBDZWxsIEJsYWNrbGlzdFxzcmNcU1lTVEVNXEJsYWNrbGlzdF9nYW1lLmV4ZQ== -upc_working_directory QzpcUHJvZ3JhbSBGaWxlcyAoeDg2KVxTdGVhbVxzdGVhbWFwcHNcY29tbW9uXFRvbSBDbGFuY3kncyBTcGxpbnRlciBDZWxsIEJsYWNrbGlzdFxzcmNcU1lTVEVN",
    //     "-upc_arguments"
    //     ])
    // .spawn().unwrap();

    println!("Game not running. Starting launch.bat");
    let _proc = std::process::Command::new(r".\launch.bat").spawn().unwrap();

    loop {
        if let Some(proc) = OwnedProcess::find_first_by_name("Blacklist_game.exe") {
            break proc;
        }
    }
}

fn main() {
    let target_process = get_or_spawn();
    println!("Found process");
    let syringe = Syringe::for_process(target_process);

    let _injected_payload = syringe
        .inject("target/i686-pc-windows-msvc/release/hooks.dll")
        .unwrap();

    println!("Injectect dll into process");

    // syringe.eject(injected_payload).unwrap();
}

#[cfg(test)]
mod tests {

    const HASHES: &str = include_str!("../../maphashes.txt");

    fn hashes() -> std::collections::HashMap<usize, &'static str> {
        HASHES
            .split('\n')
            .flat_map(|line| line.split_once('\t'))
            .flat_map(|(txt, id)| {
                Some((
                    dbg!(usize::from_str_radix(dbg!(id.trim_end()), 16)).ok()?,
                    txt,
                ))
            })
            .collect()
    }
    #[test]
    fn hash() {
        assert!(!hashes().is_empty());
    }
}
