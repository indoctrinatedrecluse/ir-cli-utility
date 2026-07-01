use crate::IpOptions;
use std::ptr;
use windows_sys::Win32::NetworkManagement::IpHelper::{
    GetAdaptersAddresses, IP_ADAPTER_ADDRESSES_LH,
};

unsafe fn pwstr_to_string(ptr: *const u16) -> String {
    if ptr.is_null() {
        return String::new();
    }
    let mut len = 0;
    while *ptr.add(len) != 0 {
        len += 1;
    }
    let slice = std::slice::from_raw_parts(ptr, len);
    String::from_utf16_lossy(slice)
}

pub fn print_local_adapters(options: &IpOptions) {
    unsafe {
        let mut size = 15000;
        let mut buf = vec![0u8; size as usize];
        
        let flags = 0x0002 | 0x0004 | 0x0008; // GAA_FLAG_SKIP_ANYCAST | GAA_FLAG_SKIP_MULTICAST | GAA_FLAG_SKIP_DNS_SERVER
        
        let mut ret = GetAdaptersAddresses(
            0, // AF_UNSPEC (both IPv4 and IPv6)
            flags,
            ptr::null(),
            buf.as_mut_ptr() as *mut _,
            &mut size,
        );
        
        // If buffer was too small, resize and try again
        if ret == 111 { // ERROR_BUFFER_OVERFLOW
            buf.resize(size as usize, 0);
            ret = GetAdaptersAddresses(
                0,
                flags,
                ptr::null(),
                buf.as_mut_ptr() as *mut _,
                &mut size,
            );
        }
        
        if ret != 0 {
            eprintln!("Error: GetAdaptersAddresses failed with code {}.", ret);
            std::process::exit(1);
        }
        
        let mut curr = buf.as_ptr() as *const IP_ADAPTER_ADDRESSES_LH;
        
        while !curr.is_null() {
            let adapter = &*curr;
            
            // Check status (1 is IfOperStatusUp)
            let is_up = adapter.OperStatus == 1;
            
            if options.all || is_up {
                let name = pwstr_to_string(adapter.FriendlyName);
                let desc = pwstr_to_string(adapter.Description);
                
                // Format MAC Address
                let mut mac_parts = Vec::new();
                for i in 0..(adapter.PhysicalAddressLength as usize) {
                    mac_parts.push(format!("{:02X}", adapter.PhysicalAddress[i]));
                }
                let mac_str = if mac_parts.is_empty() {
                    "None".to_string()
                } else {
                    mac_parts.join(":")
                };
                
                println!("{}", name);
                println!("  Description: {}", desc);
                println!("  Status:      {}", if is_up { "Up (Connected)" } else { "Down (Disconnected)" });
                println!("  MAC Address: {}", mac_str);
                
                // Collect IP addresses
                let mut ipv4_list = Vec::new();
                let mut ipv6_list = Vec::new();
                
                let mut unicast = adapter.FirstUnicastAddress;
                while !unicast.is_null() {
                    let addr = &*unicast;
                    let sockaddr = addr.Address.lpSockaddr;
                    if !sockaddr.is_null() {
                        let family = (*sockaddr).sa_family;
                        if family == 2 { // AF_INET
                            let ptr = (sockaddr as *const u8).add(4);
                            let ip_str = format!("{}.{}.{}.{}", *ptr, *ptr.add(1), *ptr.add(2), *ptr.add(3));
                            ipv4_list.push(ip_str);
                        } else if family == 23 { // AF_INET6
                            let ptr = (sockaddr as *const u8).add(8);
                            let mut parts = Vec::new();
                            for i in 0..8 {
                                let word = ((*ptr.add(i*2) as u16) << 8) | (*ptr.add(i*2 + 1) as u16);
                                parts.push(format!("{:x}", word));
                            }
                            ipv6_list.push(parts.join(":"));
                        }
                    }
                    unicast = addr.Next;
                }
                
                if !ipv4_list.is_empty() {
                    println!("  IPv4:        {}", ipv4_list.join(", "));
                }
                if !ipv6_list.is_empty() {
                    println!("  IPv6:        {}", ipv6_list.join(", "));
                }
                println!();
            }
            
            curr = adapter.Next;
        }
    }
}
