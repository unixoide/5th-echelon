use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::sync::OnceLock;

use serde::Deserialize;
use serde::Serialize;
use tracing::info;
use tracing::instrument;
use url::Url;

#[cfg(target_os = "windows")]
mod saves;
#[cfg(target_os = "windows")]
pub use saves::SaveGameExt;

#[cfg(target_os = "windows")]
mod msgbox {
    use std::ffi::CString;

    use windows::core::PCSTR;
    use windows::Win32::UI::WindowsAndMessaging::MessageBoxA;
    use windows::Win32::UI::WindowsAndMessaging::IDOK;
    use windows::Win32::UI::WindowsAndMessaging::MB_ICONQUESTION;
    use windows::Win32::UI::WindowsAndMessaging::MB_OK;
    use windows::Win32::UI::WindowsAndMessaging::MB_OKCANCEL;

    pub fn show_msgbox(msg: &str, caption: &str) {
        let msg = CString::new(msg).unwrap();
        let caption = CString::new(caption).unwrap();
        unsafe {
            MessageBoxA(None, PCSTR(msg.as_ptr().cast::<u8>()), PCSTR(caption.as_ptr().cast::<u8>()), MB_OK);
        }
    }

    pub fn show_msgbox_ok_cancel(msg: &str, caption: &str) -> bool {
        let msg = CString::new(msg).unwrap();
        let caption = CString::new(caption).unwrap();
        unsafe { MessageBoxA(None, PCSTR(msg.as_ptr().cast::<u8>()), PCSTR(caption.as_ptr().cast::<u8>()), MB_OKCANCEL | MB_ICONQUESTION) == IDOK }
    }
}

static CONFIG: OnceLock<Config> = OnceLock::new();
pub static URL: OnceLock<Url> = OnceLock::new();

fn default_password() -> String {
    String::from("password1234")
}
fn default_username() -> String {
    String::from("sam_the_fisher")
}
fn default_account_id() -> String {
    String::from("00000000-0000-4000-0000-000000000000")
}
fn default_overlay() -> bool {
    true
}

macro_rules! enum_gui {
    (
        $(#[$attr:meta])*
        $vis:vis enum $name:ident {
            $(
                $(#[cfg(feature = $feature:literal)])?
                #[label=$label:literal]
                $field:ident,
            )*
        }
    ) => {
        $(#[$attr])*
        $vis enum $name {
            $(
                $(#[cfg(feature = $feature)])?
                $field,
            )*
        }

        impl $name {
            enum_gui!(@1 $name, [$($(#[cfg(feature = $feature)])?$name::$field),*]);
            enum_gui!(@2 [$($(#[cfg(feature = $feature)])?$label),*]);
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(
                        $(#[cfg(feature = $feature)])?
                        $name::$field => write!(f, "{}::{}", stringify!($name), stringify!($field)),
                    )*
                }
            }
        }
    };

    (@1 $name:ident, $value:expr) => {
        pub const VARIANTS: [$name; $value.len()] = $value;
    };

    (@2 $value:expr) => {
        pub const LABELS: [&'static str; $value.len()] = $value;
    };
}

enum_gui! {
    #[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Hash)]
    #[serde(rename_all = "PascalCase")]
    pub enum Hook {
        #[label="Log internal messages"]
        Printer,
        #[label="Log when a state is exiting"]
        LeaveState,
        #[label="Log what state is coming up next"]
        NextState,
        #[label="Log NetResultBase"]
        NetResultBase,
        #[label="Log goals"]
        Goal,
        #[label="Log next step"]
        SetStep,
        #[label="Log new threads"]
        Thread,
        #[label="Log state changes"]
        ChangeState,
        #[label="Enforce LAN mode?"]
        NetCore,
        #[label="Log NetResultCore"]
        NetResultCore,
        #[label="Log NetResultSession"]
        NetResultSession,
        #[label="Log NetResultRdvSession"]
        NetResultRdvSession,
        #[label="Log NetResultLobby"]
        NetResultLobby,
        #[label="Log IP:PORT used by Storm"]
        StormHostPortToString,
        #[label="Enforce IP returned by GetAdaptersInfo"]
        GetAdaptersInfo,
        #[label="Enforce IP returned by gethostbyname"]
        Gethostbyname,
        #[label="Log generated IDs"]
        GenerateID,
        #[label="Log storm states"]
        StormSetState,
        #[label="Log storm state transitions"]
        StormStateMachineActionExecute,
        #[label="Log storm errors"]
        StormErrorFormatter,
        #[label="Log gear destructors (LARGE)"]
        GearStrDestructor,
        #[label="Log storm events"]
        StormEventDispatcher,
        #[label="Log storm udp packets"]
        StormPackets,
        #[label="Log RMC messages"]
        RMCMessages,
        #[cfg(feature = "modding")]
        #[label="Override packaged files"]
        OverridePackaged,
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct Config {
    pub user: User,
    #[serde(default)]
    #[serde(skip_serializing_if = "Save::is_default")]
    pub save: Save,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub forward_calls: Vec<String>,
    #[serde(default)]
    pub forward_all_calls: bool,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub internal_command_line: String,
    #[serde(default, skip_serializing_if = "HashSet::is_empty")]
    pub enable_hooks: HashSet<Hook>,
    #[serde(default)]
    pub enable_all_hooks: bool,
    #[serde(default = "default_overlay")]
    pub enable_overlay: bool,
    pub config_server: Option<String>,
    pub api_server: url::Url,
    #[serde(default)]
    #[serde(skip_serializing_if = "Networking::is_default")]
    pub networking: Networking,
    #[serde(default)]
    #[serde(skip_serializing_if = "Logging::is_default")]
    pub logging: Logging,
    #[serde(default)]
    pub auto_join_invite: bool,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct User {
    #[serde(default = "default_username")]
    pub username: String,
    #[serde(default = "default_password")]
    pub password: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cd_keys: Vec<String>,
    #[serde(default = "default_account_id")]
    pub account_id: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum SaveDir {
    InstallLocation,
    #[default]
    Roaming,
    Custom(String),
}

impl SaveDir {
    pub fn is_default(&self) -> bool {
        matches!(self, SaveDir::Roaming)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Save {
    #[serde(default)]
    #[serde(skip_serializing_if = "SaveDir::is_default")]
    pub save_dir: SaveDir,
}

impl Save {
    pub fn is_default(&self) -> bool {
        self.save_dir.is_default()
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct SaveGame {
    pub slot_id: usize,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct Networking {
    pub ip_address: Option<std::net::Ipv4Addr>,
    pub adapter: Option<String>,
}

impl Networking {
    pub fn is_default(&self) -> bool {
        self.ip_address.is_none() && self.adapter.is_none()
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq, PartialOrd, Eq, Ord, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LogLevel {
    Trace,
    Debug,
    #[default]
    Info,
    Warning,
    Error,
}

impl AsRef<str> for LogLevel {
    fn as_ref(&self) -> &str {
        match self {
            LogLevel::Trace => "Trace",
            LogLevel::Debug => "Debug",
            LogLevel::Info => "Info",
            LogLevel::Warning => "Warning",
            LogLevel::Error => "Error",
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct Logging {
    #[serde(default)]
    pub level: LogLevel,
}

impl Logging {
    pub fn is_default(&self) -> bool {
        matches!(self.level, LogLevel::Info)
    }
}

const DEFAULT_CONFIG: &str = r#"
# Where to find the config server
ConfigServer = "127.0.0.1"
# Where to find the api server (typically the same as the config server)
ApiServer = "http://127.0.0.1:50051"
# Automatically join invites without user intervention
AutoJoinInvite = false

[User]
# Username for the community server
Username = "sam_the_fisher"
# Password for the community server
Password = "password1234"

[Save]
SaveDir = "Roaming"
"#;

pub fn get() -> Option<&'static Config> {
    CONFIG.get()
}

#[cfg(target_os = "windows")]
fn get_adapter_infos<T>(f: impl Fn(*mut windows::Win32::NetworkManagement::IpHelper::IP_ADAPTER_INFO) -> anyhow::Result<T>) -> anyhow::Result<T> {
    #![allow(clippy::cast_possible_truncation, clippy::crosspointer_transmute)]

    use anyhow::bail;
    use windows::core::PWSTR;
    use windows::Win32::Foundation::LocalFree;
    use windows::Win32::Foundation::ERROR_BUFFER_OVERFLOW;
    use windows::Win32::Foundation::ERROR_SUCCESS;
    use windows::Win32::Foundation::HLOCAL;
    use windows::Win32::Foundation::WIN32_ERROR;
    use windows::Win32::NetworkManagement::IpHelper::GetAdaptersInfo;
    use windows::Win32::NetworkManagement::IpHelper::IP_ADAPTER_INFO;
    use windows::Win32::System::Diagnostics::Debug::FormatMessageW;
    use windows::Win32::System::Diagnostics::Debug::FORMAT_MESSAGE_ALLOCATE_BUFFER;

    let mut adapterinfo = vec![0u8; std::mem::size_of::<IP_ADAPTER_INFO>()];
    let mut size = adapterinfo.len() as u32;
    let mut res = WIN32_ERROR(unsafe { GetAdaptersInfo(Some(adapterinfo.as_mut_ptr().cast()), &mut size) });
    if res == ERROR_BUFFER_OVERFLOW {
        adapterinfo.resize(size as usize, 0);
        res = WIN32_ERROR(unsafe { GetAdaptersInfo(Some(adapterinfo.as_mut_ptr().cast()), &mut size) });
    }
    match res {
        ERROR_BUFFER_OVERFLOW => {
            bail!("Couldn't allocate enough memory for network adapters");
        }
        ERROR_SUCCESS => f(adapterinfo.as_mut_ptr().cast()),
        _ => unsafe {
            let mut buffer_ptr: PWSTR = PWSTR::null();
            let ptr_ptr: *mut PWSTR = &mut buffer_ptr;
            let chars = FormatMessageW(FORMAT_MESSAGE_ALLOCATE_BUFFER, None, res.0, 0, std::mem::transmute::<*mut PWSTR, PWSTR>(ptr_ptr), 256, None);
            let msg = if chars > 0 {
                String::from_utf16(std::slice::from_raw_parts(buffer_ptr.0, chars as _))?
            } else {
                String::from("unknown")
            };
            LocalFree(std::mem::transmute::<PWSTR, HLOCAL>(buffer_ptr))?;
            bail!("Couldn't enumerate adapters: {}", msg);
        },
    }
}

#[cfg(target_os = "windows")]
fn get_adapter_addresses<T>(f: impl Fn(*mut windows::Win32::NetworkManagement::IpHelper::IP_ADAPTER_ADDRESSES_LH) -> anyhow::Result<T>) -> anyhow::Result<T> {
    #![allow(clippy::cast_possible_truncation, clippy::crosspointer_transmute, clippy::cast_lossless)]

    use anyhow::bail;
    use windows::core::PWSTR;
    use windows::Win32::Foundation::LocalFree;
    use windows::Win32::Foundation::ERROR_BUFFER_OVERFLOW;
    use windows::Win32::Foundation::ERROR_SUCCESS;
    use windows::Win32::Foundation::HLOCAL;
    use windows::Win32::Foundation::WIN32_ERROR;
    use windows::Win32::NetworkManagement::IpHelper::GetAdaptersAddresses;
    use windows::Win32::NetworkManagement::IpHelper::GAA_FLAG_SKIP_ANYCAST;
    use windows::Win32::NetworkManagement::IpHelper::GAA_FLAG_SKIP_DNS_SERVER;
    use windows::Win32::NetworkManagement::IpHelper::GAA_FLAG_SKIP_MULTICAST;
    use windows::Win32::NetworkManagement::IpHelper::GAA_FLAG_SKIP_UNICAST;
    use windows::Win32::NetworkManagement::IpHelper::IP_ADAPTER_ADDRESSES_LH;
    use windows::Win32::Networking::WinSock::AF_INET;
    use windows::Win32::System::Diagnostics::Debug::FormatMessageW;
    use windows::Win32::System::Diagnostics::Debug::FORMAT_MESSAGE_ALLOCATE_BUFFER;

    let mut adapter_addresses = vec![0u8; std::mem::size_of::<IP_ADAPTER_ADDRESSES_LH>()];
    let mut size = adapter_addresses.len() as u32;
    let mut res = WIN32_ERROR(unsafe {
        GetAdaptersAddresses(
            AF_INET.0 as u32,
            GAA_FLAG_SKIP_UNICAST | GAA_FLAG_SKIP_ANYCAST | GAA_FLAG_SKIP_MULTICAST | GAA_FLAG_SKIP_DNS_SERVER,
            None,
            Some(adapter_addresses.as_mut_ptr().cast()),
            &mut size,
        )
    });
    if res == ERROR_BUFFER_OVERFLOW {
        adapter_addresses.resize(size as usize, 0);
        res = WIN32_ERROR(unsafe {
            GetAdaptersAddresses(
                AF_INET.0 as u32,
                GAA_FLAG_SKIP_ANYCAST | GAA_FLAG_SKIP_MULTICAST | GAA_FLAG_SKIP_DNS_SERVER,
                None,
                Some(adapter_addresses.as_mut_ptr().cast()),
                &mut size,
            )
        });
    }
    match res {
        ERROR_BUFFER_OVERFLOW => {
            bail!("Couldn't allocate enough memory for network adapters");
        }
        ERROR_SUCCESS => f(adapter_addresses.as_mut_ptr().cast()),
        _ => unsafe {
            let mut buffer_ptr: PWSTR = PWSTR::null();
            let ptr_ptr: *mut PWSTR = &mut buffer_ptr;
            let chars = FormatMessageW(FORMAT_MESSAGE_ALLOCATE_BUFFER, None, res.0, 0, std::mem::transmute::<*mut PWSTR, PWSTR>(ptr_ptr), 256, None);
            let msg = if chars > 0 {
                String::from_utf16(std::slice::from_raw_parts(buffer_ptr.0, chars as _))?
            } else {
                String::from("unknown")
            };
            LocalFree(std::mem::transmute::<PWSTR, HLOCAL>(buffer_ptr))?;
            bail!("Couldn't enumerate adapters: {}", msg);
        },
    }
}

#[cfg(target_os = "windows")]
fn find_ipaddress_for_adapter(target_adapter: &str) -> anyhow::Result<Option<std::net::Ipv4Addr>> {
    use std::ffi::CStr;
    use std::net::Ipv4Addr;

    use tracing::debug;
    use tracing::warn;
    use windows::Win32::NetworkManagement::IpHelper::IP_ADAPTER_ADDRESSES_LH;
    use windows::Win32::NetworkManagement::IpHelper::IP_ADAPTER_INFO;
    use windows::Win32::Networking::WinSock::AF_INET;
    use windows::Win32::Networking::WinSock::SOCKADDR_IN;

    if cfg!(feature = "GetAdapterInfos") {
        get_adapter_infos(|adapterinfo| {
            let mut result = None;
            let mut next_adapter: *mut IP_ADAPTER_INFO = adapterinfo;
            while let Some(current_adapter) = unsafe { next_adapter.as_ref() } {
                let adapter_name = CStr::from_bytes_until_nul(&current_adapter.Description)?;
                debug!("{adapter_name:?} == {target_adapter:?}");
                if adapter_name.to_str()? == target_adapter {
                    result = Some(CStr::from_bytes_until_nul(&current_adapter.IpAddressList.IpAddress.String)?.to_str()?.parse()?);
                    break;
                }
                next_adapter = current_adapter.Next;
            }
            if result.is_none() {
                warn!("No adapter {target_adapter:?} found");
            }
            Ok(result)
        })
    } else {
        get_adapter_addresses(|adapter_addresses| {
            let mut result = None;
            let mut next_adapter: *mut IP_ADAPTER_ADDRESSES_LH = adapter_addresses;
            while let Some(current_adapter) = unsafe { next_adapter.as_ref() } {
                let adapter_name = unsafe { current_adapter.FriendlyName.to_string()? };
                debug!("{adapter_name:?} == {target_adapter:?}");
                if adapter_name.as_str() == target_adapter {
                    if let Some(ip) = unsafe { current_adapter.FirstUnicastAddress.as_ref() } {
                        #[allow(clippy::cast_ptr_alignment)]
                        let sockaddr = unsafe { ip.Address.lpSockaddr.cast::<SOCKADDR_IN>().as_ref().unwrap() };
                        if sockaddr.sin_family == AF_INET {
                            result = Some(Ipv4Addr::new(
                                unsafe { sockaddr.sin_addr.S_un.S_un_b.s_b1 },
                                unsafe { sockaddr.sin_addr.S_un.S_un_b.s_b2 },
                                unsafe { sockaddr.sin_addr.S_un.S_un_b.s_b3 },
                                unsafe { sockaddr.sin_addr.S_un.S_un_b.s_b4 },
                            ));
                            break;
                        }
                        warn!("Can't handle family {:?}", sockaddr.sin_family);
                    }
                }
                next_adapter = current_adapter.Next;
            }
            if result.is_none() {
                warn!("No adapter {target_adapter:?} found");
            }
            Ok(result)
        })
    }
}

pub fn get_or_load(path: impl AsRef<Path>) -> anyhow::Result<&'static Config> {
    _get_or_load(path.as_ref())
}

#[instrument]
fn _get_or_load(path: &Path) -> anyhow::Result<&'static Config> {
    if let Some(cfg) = get() {
        return Ok(cfg);
    }
    let content = match fs::read_to_string(path) {
        Ok(content) => content,
        #[cfg(target_os = "windows")]
        Err(ref err) if err.kind() == std::io::ErrorKind::NotFound && msgbox::show_msgbox_ok_cancel("Configuration not found, generate and exit?", "Configuration not found") => {
            fs::write(path, DEFAULT_CONFIG)?;
            msgbox::show_msgbox(&format!("Config file placed at {}", path.to_str().unwrap()), "Done");
            std::process::exit(0);
        }
        Err(err) => return Err(err.into()),
    };
    let mut cfg: Config = toml::from_str(&content)?;
    if cfg.user.cd_keys.is_empty() {
        info!("Passing startup to original dll");
        cfg.forward_calls.push("UPLAY_Startup".into());
        cfg.forward_calls.push("UPLAY_Quit".into());
        //        cfg.forward_calls.push("UPLAY_USER_GetCdKeys".into());
        cfg.user.cd_keys.push("ABCD-EFGH-IJKL-MNOP".into());
    }

    #[cfg(target_os = "windows")]
    if cfg.networking.ip_address.is_none() && cfg.networking.adapter.is_some() {
        let target_adapter = cfg.networking.adapter.as_ref().unwrap();
        info!("Getting IP address of adapter {target_adapter:?}");
        cfg.networking.ip_address = find_ipaddress_for_adapter(target_adapter)?;
    }

    if let Some(ip) = &cfg.networking.ip_address {
        info!("Enforcing {ip} for networking");
    }

    // if let Some(ref api_server) = cfg.api_server {
    crate::URL.set(cfg.api_server.clone()).map_err(|cfg| anyhow::anyhow!("Couldn't store api url {:?}", cfg))?;
    // }
    CONFIG.set(cfg).map_err(|cfg| anyhow::anyhow!("Couldn't store config {:?}", cfg))?;

    get().ok_or_else(|| anyhow::anyhow!("Config not loaded"))
}

pub fn get_config_path(path: impl AsRef<Path>) -> PathBuf {
    path.as_ref().join("uplay.toml")
}

pub fn default() -> Config {
    toml::from_str(DEFAULT_CONFIG).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_default_config() {
        let cfg: Config = toml::from_str(DEFAULT_CONFIG).unwrap();
        println!("{}", toml::to_string_pretty(&cfg).unwrap());
    }
}
