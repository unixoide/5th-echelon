use std::collections::HashSet;
use std::ffi::CStr;
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::net::Ipv6Addr;

use anyhow::bail;
use imgui_winit_support::winit;
use raw_window_handle::HasWindowHandle as _;
use windows::core::PWSTR;
use windows::Win32::Foundation::GetLastError;
use windows::Win32::Foundation::LocalFree;
use windows::Win32::Foundation::ERROR_BUFFER_OVERFLOW;
use windows::Win32::Foundation::ERROR_SUCCESS;
use windows::Win32::Foundation::HLOCAL;
use windows::Win32::Foundation::HWND;
use windows::Win32::Foundation::WIN32_ERROR;
use windows::Win32::NetworkManagement::IpHelper::GetAdaptersAddresses;
use windows::Win32::NetworkManagement::IpHelper::GetAdaptersInfo;
use windows::Win32::NetworkManagement::IpHelper::GAA_FLAG_SKIP_ANYCAST;
use windows::Win32::NetworkManagement::IpHelper::GAA_FLAG_SKIP_DNS_SERVER;
use windows::Win32::NetworkManagement::IpHelper::GAA_FLAG_SKIP_MULTICAST;
use windows::Win32::NetworkManagement::IpHelper::GAA_FLAG_SKIP_UNICAST;
use windows::Win32::NetworkManagement::IpHelper::IP_ADAPTER_ADDRESSES_LH;
use windows::Win32::NetworkManagement::IpHelper::IP_ADAPTER_INFO;
use windows::Win32::Networking::WinSock::AF_INET;
use windows::Win32::Networking::WinSock::SOCKADDR_IN;
use windows::Win32::Networking::WinSock::SOCKADDR_IN6;
use windows::Win32::System::Diagnostics::Debug::FormatMessageW;
use windows::Win32::System::Diagnostics::Debug::FORMAT_MESSAGE_ALLOCATE_BUFFER;

mod clipboard_win;

pub fn clipboard_backend(window: &winit::window::Window) -> clipboard_win::WindowsClipboard {
    let raw_handle = window.window_handle().expect("raw window handle").as_raw();
    let hwnd = match raw_handle {
        winit::raw_window_handle::RawWindowHandle::Win32(win32_window_handle) => HWND(win32_window_handle.hwnd.into()),
        _ => unreachable!(),
    };
    clipboard_win::WindowsClipboard { hwnd }
}

pub fn find_adapter_names() -> Vec<(String, IpAddr)> {
    let res = if cfg!(feature = "GetAdapterInfos") {
        get_adapter_infos(|adapterinfo| {
            let mut res = HashSet::new();
            let mut next_adapter: *mut IP_ADAPTER_INFO = adapterinfo;
            while let Some(current_adapter) = unsafe { next_adapter.as_ref() } {
                if let Some(name) = CStr::from_bytes_until_nul(unsafe { &*(current_adapter.Description.as_slice() as *const [i8] as *const [u8]) })
                    .ok()
                    .and_then(|cs| cs.to_str().ok())
                {
                    let ip_raw = current_adapter.IpAddressList.IpAddress.String;
                    let ip = IpAddr::V4(Ipv4Addr::new(ip_raw[0] as u8, ip_raw[1] as u8, ip_raw[2] as u8, ip_raw[3] as u8));
                    res.insert((name.to_owned(), ip));
                }
                next_adapter = current_adapter.Next;
            }
            Ok(res)
        })
    } else {
        get_adapter_addresses(|adapter_addresses| {
            let mut res = HashSet::new();
            let mut next_adapter: *mut IP_ADAPTER_ADDRESSES_LH = adapter_addresses;
            while let Some(current_adapter) = unsafe { next_adapter.as_ref() } {
                if let Ok(adapter_name) = unsafe { current_adapter.FriendlyName.to_string() } {
                    if let Some(addr) = unsafe { current_adapter.FirstUnicastAddress.as_ref() } {
                        let addr = if addr.Address.iSockaddrLength as usize == std::mem::size_of::<SOCKADDR_IN>() {
                            let sockaddr = unsafe { addr.Address.lpSockaddr.cast::<SOCKADDR_IN>().as_ref() }.unwrap();
                            unsafe {
                                IpAddr::V4(Ipv4Addr::new(
                                    sockaddr.sin_addr.S_un.S_un_b.s_b1,
                                    sockaddr.sin_addr.S_un.S_un_b.s_b2,
                                    sockaddr.sin_addr.S_un.S_un_b.s_b3,
                                    sockaddr.sin_addr.S_un.S_un_b.s_b4,
                                ))
                            }
                        } else {
                            let sockaddr = unsafe { addr.Address.lpSockaddr.cast::<SOCKADDR_IN6>().as_ref() }.unwrap();
                            unsafe {
                                IpAddr::V6(Ipv6Addr::new(
                                    sockaddr.sin6_addr.u.Word[0],
                                    sockaddr.sin6_addr.u.Word[1],
                                    sockaddr.sin6_addr.u.Word[2],
                                    sockaddr.sin6_addr.u.Word[3],
                                    sockaddr.sin6_addr.u.Word[4],
                                    sockaddr.sin6_addr.u.Word[5],
                                    sockaddr.sin6_addr.u.Word[6],
                                    sockaddr.sin6_addr.u.Word[7],
                                ))
                            }
                        };
                        res.insert((adapter_name.to_owned(), addr));
                    }
                }
                next_adapter = current_adapter.Next;
            }
            Ok(res)
        })
    };

    let mut res: Vec<(String, IpAddr)> = res
        .unwrap_or_default()
        .into_iter()
        .filter(|(_, ip)| match ip {
            // ignore windows default IPs (168.254.x.x)
            IpAddr::V4(v4) => !v4.is_link_local(),
            _ => true,
        })
        .collect();
    res.sort_by(|a, b| a.0.cmp(&b.0));
    res
}

fn get_adapter_infos<T>(f: impl Fn(*mut windows::Win32::NetworkManagement::IpHelper::IP_ADAPTER_INFO) -> anyhow::Result<T>) -> anyhow::Result<T> {
    #![allow(clippy::cast_possible_truncation, clippy::crosspointer_transmute)]

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
            if !LocalFree(std::mem::transmute::<PWSTR, HLOCAL>(buffer_ptr)).0.is_null() {
                bail!("Error freeing buffer: {}", GetLastError().to_hresult());
            }
            bail!("Couldn't enumerate adapters: {}", msg);
        },
    }
}

fn get_adapter_addresses<T>(f: impl Fn(*mut windows::Win32::NetworkManagement::IpHelper::IP_ADAPTER_ADDRESSES_LH) -> anyhow::Result<T>) -> anyhow::Result<T> {
    #![allow(clippy::cast_possible_truncation, clippy::crosspointer_transmute, clippy::cast_lossless)]
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
            if !LocalFree(std::mem::transmute::<PWSTR, HLOCAL>(buffer_ptr)).0.is_null() {
                bail!("Error freeing buffer: {}", GetLastError().to_hresult());
            }
            bail!("Couldn't enumerate adapters: {}", msg);
        },
    }
}
