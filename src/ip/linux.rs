use crate::IpOptions;
use std::collections::HashMap;

struct InterfaceInfo {
    name: String,
    is_up: bool,
    mac_address: Option<String>,
    ipv4_addresses: Vec<String>,
    ipv6_addresses: Vec<String>,
}

pub fn print_local_adapters(options: &IpOptions) {
    unsafe {
        let mut ifaddrs_ptr: *mut libc::ifaddrs = std::ptr::null_mut();
        if libc::getifaddrs(&mut ifaddrs_ptr) != 0 {
            let err = std::io::Error::last_os_error();
            eprintln!("Error: getifaddrs failed: {}", err);
            std::process::exit(1);
        }

        let mut interfaces: HashMap<String, InterfaceInfo> = HashMap::new();
        let mut order: Vec<String> = Vec::new();

        let mut curr = ifaddrs_ptr;
        while !curr.is_null() {
            let ifa = &*curr;
            if !ifa.ifa_name.is_null() {
                let name = std::ffi::CStr::from_ptr(ifa.ifa_name)
                    .to_string_lossy()
                    .into_owned();

                if !interfaces.contains_key(&name) {
                    order.push(name.clone());
                }

                // IFF_UP flag is 0x1
                let is_up = (ifa.ifa_flags & libc::IFF_UP as libc::c_uint) != 0;

                let info = interfaces.entry(name.clone()).or_insert_with(|| InterfaceInfo {
                    name,
                    is_up,
                    mac_address: None,
                    ipv4_addresses: Vec::new(),
                    ipv6_addresses: Vec::new(),
                });

                let addr = ifa.ifa_addr;
                if !addr.is_null() {
                    let family = (*addr).sa_family as i32;
                    if family == libc::AF_INET {
                        let ptr = (addr as *const u8).add(4);
                        let ip_str = format!("{}.{}.{}.{}", *ptr, *ptr.add(1), *ptr.add(2), *ptr.add(3));
                        info.ipv4_addresses.push(ip_str);
                    } else if family == libc::AF_INET6 {
                        let ptr = (addr as *const u8).add(8);
                        let mut parts = Vec::new();
                        for i in 0..8 {
                            let word = ((*ptr.add(i*2) as u16) << 8) | (*ptr.add(i*2 + 1) as u16);
                            parts.push(format!("{:x}", word));
                        }
                        info.ipv6_addresses.push(parts.join(":"));
                    } else if family == libc::AF_PACKET {
                        let sll = addr as *const libc::sockaddr_ll;
                        let len = (*sll).sll_halen as usize;
                        if len > 0 {
                            let mut mac_parts = Vec::new();
                            for i in 0..len {
                                mac_parts.push(format!("{:02X}", (*sll).sll_addr[i]));
                            }
                            info.mac_address = Some(mac_parts.join(":"));
                        }
                    }
                }
            }
            curr = ifa.ifa_next;
        }

        libc::freeifaddrs(ifaddrs_ptr);

        // Display results
        for name in order {
            if let Some(info) = interfaces.get(&name) {
                if options.all || info.is_up {
                    println!("{}", info.name);
                    println!("  Status:      {}", if info.is_up { "Up" } else { "Down" });
                    if let Some(ref mac) = info.mac_address {
                        println!("  MAC Address: {}", mac);
                    }
                    if !info.ipv4_addresses.is_empty() {
                        println!("  IPv4:        {}", info.ipv4_addresses.join(", "));
                    }
                    if !info.ipv6_addresses.is_empty() {
                        println!("  IPv6:        {}", info.ipv6_addresses.join(", "));
                    }
                    println!();
                }
            }
        }
    }
}
