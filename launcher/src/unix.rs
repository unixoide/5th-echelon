use std::collections::HashMap;
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::net::Ipv6Addr;

use libc::sockaddr_in;
use libc::sockaddr_in6;

pub fn find_adapter_names() -> Vec<(String, IpAddr)> {
    let mut addrs: *mut libc::ifaddrs = std::ptr::null_mut();
    if unsafe { libc::getifaddrs(&mut addrs) } != 0 {
        return vec![];
    }

    let mut res = HashMap::new();
    let mut current_ifaddr = unsafe { addrs.as_mut() };
    while let Some(ifaddr) = current_ifaddr {
        if let Ok(name) = unsafe { std::ffi::CStr::from_ptr(ifaddr.ifa_name) }.to_str() {
            if !ifaddr.ifa_addr.is_null() {
                if unsafe { (*ifaddr.ifa_addr).sa_family } == libc::AF_INET as u16 {
                    let sockaddr =
                        unsafe { ifaddr.ifa_addr.cast::<sockaddr_in>().as_ref().unwrap() };
                    let ip = sockaddr.sin_addr.s_addr;
                    let ip = IpAddr::V4(Ipv4Addr::new(
                        (ip & 0xff) as u8,
                        ((ip >> 8) & 0xff) as u8,
                        ((ip >> 16) & 0xff) as u8,
                        ((ip >> 24) & 0xff) as u8,
                    ));
                    res.insert(name.to_owned(), ip);
                } else if false {
                    let sockaddr =
                        unsafe { ifaddr.ifa_addr.cast::<sockaddr_in6>().as_ref().unwrap() };
                    let ip = sockaddr.sin6_addr.s6_addr;
                    let ip = IpAddr::V6(Ipv6Addr::from(ip));
                    res.insert(name.to_owned(), ip);
                }
            }
        }
        current_ifaddr = unsafe { ifaddr.ifa_next.as_mut() };
    }
    unsafe {
        libc::freeifaddrs(addrs);
    }
    let mut res: Vec<(String, IpAddr)> = res.into_iter().collect();
    res.sort_by(|a, b| a.0.cmp(&b.0));
    res
}
