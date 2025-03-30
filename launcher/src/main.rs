#![windows_subsystem = "windows"]

#[cfg(target_os = "windows")]
#[path = "windows.rs"]
mod sys;

#[cfg(not(target_os = "windows"))]
#[path = "unix.rs"]
mod sys;

mod config;
mod dll_utils;
mod games;
mod logging;
mod network;
mod render;
mod ui;
mod updater;
mod version;

static VERSION: std::sync::LazyLock<version::Version> = std::sync::LazyLock::new(|| version::Version {
    major: env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap(),
    minor: env!("CARGO_PKG_VERSION_MINOR").parse().unwrap(),
    patch: env!("CARGO_PKG_VERSION_PATCH").parse().unwrap(),
});

fn ask_for_ui_version() -> config::UIVersion {
    use windows::core::w;
    use windows::Win32::UI::Controls::TaskDialogIndirect;
    use windows::Win32::UI::Controls::TASKDIALOGCONFIG;
    use windows::Win32::UI::Controls::TASKDIALOG_BUTTON;
    use windows::Win32::UI::WindowsAndMessaging::IDNO;
    use windows::Win32::UI::WindowsAndMessaging::IDYES;

    unsafe {
        let buttons = [
            TASKDIALOG_BUTTON {
                nButtonID: IDYES.0,
                pszButtonText: w!("New"),
            },
            TASKDIALOG_BUTTON {
                nButtonID: IDNO.0,
                pszButtonText: w!("Old"),
            },
        ];
        let task_dlg_cfg = TASKDIALOGCONFIG {
            cbSize: std::mem::size_of::<TASKDIALOGCONFIG>() as u32,
            pButtons: buttons.as_ptr(),
            cButtons: buttons.len() as u32,
            pszWindowTitle: w!("Select UI Version"),
            pszMainInstruction: w!("Please select the UI version you want to use."),
            pszContent: w!("You can change the selection at any time in the settings submenus"),
            ..Default::default()
        };
        let mut selected_button = 0;
        TaskDialogIndirect(&task_dlg_cfg, Some(&mut selected_button), None, None).unwrap();
        if selected_button == IDYES.0 {
            config::UIVersion::New
        } else {
            config::UIVersion::Old
        }
    }
}

fn main() {
    logging::init();

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    let assets = runtime.block_on(<updater::Updater>::check_for_updates());
    let launcher_asset = assets.into_iter().find(|a| a.name == "launcher.exe");

    eprintln!("Launcher asset: {:?}", launcher_asset);

    if let Some("update") = std::env::args().nth(1).as_deref() {
        runtime.block_on(<updater::Updater>::update_self(
            launcher_asset.expect("Launcher asset not found"),
        ));
        return;
    }
    let update_available = launcher_asset.map(|a| a.version > *VERSION).unwrap_or(false);

    drop(runtime);

    let target_dir = games::find_target_dir().expect("Game not found. Try to place the launcher in the games folder.");
    println!("Found target dir {target_dir:?}");

    #[cfg(feature = "embed-dll")]
    dll_utils::drop_dll(&target_dir);

    let mut cfg = config::Config::load(&target_dir);
    let adapters = sys::find_adapter_names();
    let (adapter_names, adapter_ips) = adapters.clone().into_iter().unzip::<_, _, Vec<_>, Vec<_>>();

    let ui_version = cfg.ui_version.unwrap_or_else(ask_for_ui_version);
    cfg.update(|cfg| cfg.ui_version = Some(ui_version));

    match ui_version {
        config::UIVersion::New => ui::new::run(target_dir, cfg, &adapters, update_available),
        config::UIVersion::Old => ui::old::run(
            target_dir,
            cfg.into_inner(),
            &adapter_names,
            &adapter_ips,
            update_available,
        ),
    }
}
