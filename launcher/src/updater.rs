use std::ffi::OsString;
use std::future::Future;
use std::path::Path;
use std::sync::atomic::AtomicPtr;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use bytes::Bytes;
use futures::Stream;
use futures::StreamExt;
use jzon::JsonValue;
use tokio::io::AsyncWriteExt;
use tracing::debug;

use crate::version::Version;

unsafe extern "system" fn dialog_cb(
    hwnd: windows::Win32::Foundation::HWND,
    msg: windows::Win32::UI::Controls::TASKDIALOG_NOTIFICATIONS,
    _wparam: windows::Win32::Foundation::WPARAM,
    _lparam: windows::Win32::Foundation::LPARAM,
    lprefdata: isize,
) -> windows::core::HRESULT {
    use windows::Win32::Foundation::LPARAM;
    use windows::Win32::Foundation::WPARAM;
    use windows::Win32::UI::Controls::TDM_SET_PROGRESS_BAR_POS;
    use windows::Win32::UI::Controls::TDN_CREATED;
    use windows::Win32::UI::Controls::TDN_TIMER;
    use windows::Win32::UI::WindowsAndMessaging::SendMessageW;
    use windows::Win32::UI::WindowsAndMessaging::WM_CLOSE;

    let total_size = lprefdata as usize;

    if msg == TDN_CREATED {
        TASKDIALOG_HWND.store(Box::into_raw(Box::new(hwnd)), Ordering::SeqCst);
    } else if msg == TDN_TIMER {
        let sz = DOWNLOADED_SIZE.load(Ordering::SeqCst);
        SendMessageW(
            hwnd,
            TDM_SET_PROGRESS_BAR_POS.0 as u32,
            WPARAM(sz * 100 / total_size),
            LPARAM(0),
        );
        if sz >= total_size {
            SendMessageW(hwnd, WM_CLOSE, WPARAM(0), LPARAM(0));
        }
    }
    windows::core::HRESULT(0)
}

// struct AsyncChunks {
//     resp: reqwest::Response,
//     chunk_fut:
//         Option<Pin<Box<dyn std::future::Future<Output = reqwest::Result<Option<bytes::Bytes>>>>>>,
// }

// impl std::future::Future for AsyncChunks {
//     type Output = reqwest::Result<Option<bytes::Bytes>>;

//     fn poll(
//         mut self: std::pin::Pin<&mut Self>,
//         cx: &mut std::task::Context<'_>,
//     ) -> std::task::Poll<Self::Output> {
//         let mut fut = if let Some(fut) = self.chunk_fut.take() {
//             fut
//         } else {
//             let chunks = self.resp.chunk();
//             Box::pin(chunks)
//         };

//         let r = match fut.as_mut().poll(cx) {
//             std::task::Poll::Ready(r) => std::task::Poll::Ready(r),
//             std::task::Poll::Pending => std::task::Poll::Pending,
//         };

//         if matches!(r, std::task::Poll::Pending) {
//             self.chunk_fut = Some(fut);
//         }
//         r
//     }
// }

pub trait UpdaterClientFactory {
    fn new() -> impl UpdaterClient;
}

pub struct GitHubClient;

impl UpdaterClientFactory for GitHubClient {
    fn new() -> impl UpdaterClient {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("X-GitHub-Api-Version", "2022-11-28".parse().unwrap());
        headers.insert("Accept", "application/vnd.github+json".parse().unwrap());
        headers.insert("User-Agent", "5th-echelon/launcher".parse().unwrap());
        reqwest::Client::builder().default_headers(headers).build().unwrap()
    }
}

pub trait UpdaterClient: Send {
    fn fetch_latest_assets(&self) -> impl Future<Output = reqwest::Result<Vec<Asset>>> + Send;
    fn download<U: reqwest::IntoUrl + Send>(
        &self,
        url: U,
    ) -> impl Future<Output = reqwest::Result<impl Stream<Item = reqwest::Result<Bytes>> + Send>> + Send;
}

impl UpdaterClient for reqwest::Client {
    async fn fetch_latest_assets(&self) -> reqwest::Result<Vec<Asset>> {
        let resp = self
            .get(" https://api.github.com/repos/unixoide/5th-echelon/releases/latest")
            .send()
            .await?;
        let body = resp.text().await.unwrap();
        let resp = jzon::parse(&body).unwrap();
        let JsonValue::Object(obj) = resp else {
            debug!("Unexpected response from server: {}", body);
            return Ok(Vec::new());
        };
        let Some(JsonValue::Short(tag_name)) = obj.get("tag_name") else {
            debug!("tag_name not found ({:?})", obj.get("tag_name"));
            return Ok(Vec::new());
        };

        let version = if let Some(tag_name) = tag_name.strip_prefix("v") {
            tag_name.parse::<Version>().ok()
        } else {
            None
        };

        let Some(version) = version else {
            debug!("unexpected tag_name {tag_name}");
            return Ok(Vec::new());
        };

        let Some(JsonValue::Array(assets)) = obj.get("assets") else {
            debug!("assets not found");
            return Ok(Vec::new());
        };

        Ok(assets
            .iter()
            .filter_map(|asset| {
                let JsonValue::Object(obj) = asset else {
                    debug!("asset is not an object");
                    return None;
                };
                let Some(JsonValue::Short(name)) = obj.get("name") else {
                    debug!("asset has no name");
                    return None;
                };
                let Some(JsonValue::String(url)) = obj.get("browser_download_url") else {
                    debug!("asset has no browser_download_url");
                    return None;
                };
                let Some(JsonValue::Number(size)) = obj.get("size") else {
                    debug!("asset has no size");
                    return None;
                };
                Some(Asset {
                    version,
                    name: name.as_str().to_string(),
                    url: url.clone(),
                    size: f64::from(*size) as usize,
                })
            })
            .collect())
    }

    async fn download<U: reqwest::IntoUrl + Send>(
        &self,
        url: U,
    ) -> reqwest::Result<impl Stream<Item = reqwest::Result<Bytes>> + Send> {
        match self.get(url).send().await {
            Ok(resp) => Ok(resp.bytes_stream()),
            Err(err) => Err(err),
        }
    }
}

pub struct MockUpdaterClient;

impl UpdaterClientFactory for MockUpdaterClient {
    fn new() -> impl UpdaterClient {
        MockUpdaterClient
    }
}

impl UpdaterClient for MockUpdaterClient {
    fn fetch_latest_assets(&self) -> impl Future<Output = reqwest::Result<Vec<Asset>>> + Send {
        async {
            Ok(vec![
                Asset {
                    version: Version {
                        major: 1,
                        minor: 2,
                        patch: 3,
                    },
                    name: "launcher.exe".to_string(),
                    url: "http://localhost/launcher.exe".to_string(),
                    size: 1024,
                },
                Asset {
                    version: Version {
                        major: 1,
                        minor: 2,
                        patch: 3,
                    },
                    name: "dedicated_server.exe".to_string(),
                    url: "http://localhost/dedicated_server.exe".to_string(),
                    size: 2048,
                },
            ])
        }
    }

    async fn download<U: reqwest::IntoUrl + Send>(
        &self,
        url: U,
    ) -> reqwest::Result<impl Stream<Item = reqwest::Result<Bytes>> + Send> {
        let stream = if url.as_str() == "http://localhost/launcher.exe" {
            futures::stream::iter(
                vec![Bytes::from_owner([0u8; 512]); 2]
                    .into_iter()
                    .map(Ok)
                    .collect::<Vec<_>>(),
            )
        } else if url.as_str() == "http://localhost/dedicated_server.exe" {
            futures::stream::iter(
                vec![Bytes::from_owner([0u8; 512]); 4]
                    .into_iter()
                    .map(Ok)
                    .collect::<Vec<_>>(),
            )
        } else {
            unreachable!()
        };

        Ok(stream.then(|x| async {
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            x
        }))
    }
}

static DOWNLOADED_SIZE: AtomicUsize = AtomicUsize::new(0);
static TASKDIALOG_HWND: AtomicPtr<windows::Win32::Foundation::HWND> = AtomicPtr::new(std::ptr::null_mut());

pub struct Updater<CF: UpdaterClientFactory = GitHubClient> {
    phantom: std::marker::PhantomData<CF>,
}

impl<CF: UpdaterClientFactory + 'static> Updater<CF> {
    pub async fn download_with_progress(asset: Asset, target_path: impl AsRef<Path>, progress: &AtomicUsize) {
        let target_path = target_path.as_ref();
        let mut launcher_exe_file = tokio::fs::File::create(target_path.with_extension("download"))
            .await
            .unwrap();
        let client = CF::new();
        let resp = client.download(asset.url).await.unwrap();
        tokio::pin!(resp);
        let mut downloaded_size = 0;
        let total_size = asset.size;
        while let Some(Ok(chunk)) = resp.next().await {
            launcher_exe_file.write_all(&chunk).await.unwrap();
            downloaded_size += chunk.len();
            progress.store(downloaded_size * 10000 / total_size, Ordering::SeqCst);
        }
        launcher_exe_file.flush().await.unwrap();
        progress.store(10000, Ordering::SeqCst);
        drop(launcher_exe_file);
        tokio::fs::rename(target_path.with_extension("download"), target_path)
            .await
            .unwrap();
    }

    pub async fn update_self(asset: Asset) {
        use std::os::windows::ffi::OsStrExt as _;

        use windows::core::w;
        use windows::Win32::Foundation::LPARAM;
        use windows::Win32::Foundation::WPARAM;
        use windows::Win32::UI::Controls::TaskDialogIndirect;
        use windows::Win32::UI::Controls::TASKDIALOGCONFIG;
        use windows::Win32::UI::Controls::TASKDIALOG_BUTTON;
        use windows::Win32::UI::Controls::TDF_CALLBACK_TIMER;
        use windows::Win32::UI::Controls::TDF_SHOW_PROGRESS_BAR;
        use windows::Win32::UI::WindowsAndMessaging::PostMessageW;
        use windows::Win32::UI::WindowsAndMessaging::IDCANCEL;
        use windows::Win32::UI::WindowsAndMessaging::WM_CLOSE;

        let updater_exe = std::env::current_exe().unwrap();
        let launcher_exe = updater_exe.parent().unwrap().join(&asset.name);

        tokio::join!(tokio::task::spawn_blocking(move || {
            windows_wait_parent::wait_for_parent_exit().unwrap();
        }))
        .0
        .unwrap();
        DOWNLOADED_SIZE.store(0, Ordering::SeqCst);
        let downloader = tokio::spawn(Self::download_with_progress(
            asset.clone(),
            launcher_exe.clone(),
            &DOWNLOADED_SIZE,
        ));

        let progress = tokio::task::spawn_blocking(move || unsafe {
            let buttons = [TASKDIALOG_BUTTON {
                nButtonID: IDCANCEL.0, // must be cancel to work with PostMessage(WM_CLOSE)
                pszButtonText: w!("Cancel"),
            }];
            let mut content = OsString::from(format!("Fetching {} bytes from the server", asset.size))
                .encode_wide()
                .collect::<Vec<u16>>();
            content.push(0);
            let task_dlg_cfg = TASKDIALOGCONFIG {
                cbSize: std::mem::size_of::<TASKDIALOGCONFIG>() as u32,
                pButtons: buttons.as_ptr(),
                cButtons: buttons.len() as u32,
                pszWindowTitle: w!("Updating launcher"),
                pszMainInstruction: w!("Updating..."),
                pszContent: windows::core::PCWSTR(content.as_ptr()),
                pfCallback: Some(dialog_cb),
                lpCallbackData: asset.size as isize,
                dwFlags: TDF_SHOW_PROGRESS_BAR | TDF_CALLBACK_TIMER,
                ..Default::default()
            };
            let mut selected_button = 0;
            TaskDialogIndirect(&task_dlg_cfg, Some(&mut selected_button), None, None).unwrap();
        });

        let done = tokio::select! {
            _ = downloader => {
                unsafe {
                    let hwnd_ptr = TASKDIALOG_HWND.load(Ordering::SeqCst);
                    if !hwnd_ptr.is_null() {
                        let hwnd = *hwnd_ptr;
                        PostMessageW(hwnd, WM_CLOSE, WPARAM(0), LPARAM(0)).unwrap();
                    }
                }
                true
            }
            _ = progress => false
        };
        let hwnd_ptr = TASKDIALOG_HWND.swap(std::ptr::null_mut(), Ordering::SeqCst);
        if !hwnd_ptr.is_null() {
            unsafe {
                let _ = Box::from_raw(hwnd_ptr);
            }
        }

        if done {
            let mut child = std::process::Command::new(launcher_exe).spawn().unwrap();
            child.try_wait().unwrap();
        }
    }

    pub async fn check_for_updates() -> Vec<Asset> {
        CF::new().fetch_latest_assets().await.unwrap()
    }
}

pub fn start_update_process_and_terminate() {
    let myself = std::env::current_exe().unwrap();
    let dest_dir = myself.parent().unwrap();
    let updater = dest_dir.join("launcher_updater.exe");
    std::fs::copy(&myself, &updater).unwrap();
    let mut child = std::process::Command::new(updater)
        .arg("update")
        .arg(myself.as_os_str())
        .spawn()
        .unwrap();
    child.try_wait().unwrap();
    std::process::exit(0);
}

pub fn remove_updater_if_needed() {
    let myself = std::env::current_exe().unwrap();
    let dest_dir = myself.parent().unwrap();
    let updater = dest_dir.join("launcher_updater.exe");
    if updater.exists() {
        std::fs::remove_file(updater).unwrap();
    }
}

#[derive(Debug, Clone)]
pub struct Asset {
    pub name: String,
    pub version: Version,
    url: String,
    size: usize,
}

#[cfg(target_os = "windows")]
mod windows_wait_parent {
    use std::mem;
    use std::process;

    use windows::core::Result;
    use windows::Win32::Foundation::CloseHandle;
    use windows::Win32::Foundation::WAIT_OBJECT_0;
    use windows::Win32::System::Diagnostics::ToolHelp::CreateToolhelp32Snapshot;
    use windows::Win32::System::Diagnostics::ToolHelp::Process32First;
    use windows::Win32::System::Diagnostics::ToolHelp::Process32Next;
    use windows::Win32::System::Diagnostics::ToolHelp::PROCESSENTRY32;
    use windows::Win32::System::Diagnostics::ToolHelp::TH32CS_SNAPPROCESS;
    use windows::Win32::System::Threading::OpenProcess;
    use windows::Win32::System::Threading::WaitForSingleObject;
    use windows::Win32::System::Threading::INFINITE;
    use windows::Win32::System::Threading::PROCESS_QUERY_INFORMATION;
    use windows::Win32::System::Threading::PROCESS_SYNCHRONIZE;

    pub fn get_parent_pid() -> Option<u32> {
        unsafe {
            let current_pid = process::id();
            let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0).unwrap();
            if snapshot.is_invalid() {
                return None;
            }
            let mut entry = mem::zeroed::<PROCESSENTRY32>();
            entry.dwSize = mem::size_of::<PROCESSENTRY32>() as u32;
            if Process32First(snapshot, &mut entry).is_err() {
                CloseHandle(snapshot).unwrap();
                return None;
            }
            while Process32Next(snapshot, &mut entry).is_ok() {
                if entry.th32ProcessID == current_pid {
                    let parent_pid = entry.th32ParentProcessID;
                    CloseHandle(snapshot).unwrap();
                    return Some(parent_pid);
                }
            }
            CloseHandle(snapshot).unwrap();
            None
        }
    }

    pub fn wait_for_parent_exit() -> Result<()> {
        if let Some(ppid) = get_parent_pid() {
            println!("Parent PID: {}", ppid);
            unsafe {
                let process_handle = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_SYNCHRONIZE, false, ppid).unwrap();
                if process_handle.is_invalid() {
                    eprintln!("Failed to open handle to parent process.");
                    return Err(windows::core::Error::from_win32());
                }

                println!("Waiting for parent process to exit...");
                let result = WaitForSingleObject(process_handle, INFINITE);
                CloseHandle(process_handle).unwrap();

                if result == WAIT_OBJECT_0 {
                    println!("Parent process exited.");
                    Ok(())
                } else {
                    eprintln!("Error waiting for parent process (code: {:?}).", result);
                    Err(windows::core::Error::from_win32()) // Consider a more specific error
                }
            }
        } else {
            eprintln!("Could not determine parent process ID.");
            Ok(()) // Or return an error if this is critical
        }
    }
}
