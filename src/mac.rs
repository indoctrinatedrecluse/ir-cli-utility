use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use crate::MacOptions;

const EMBEDDED_OUIS: &[(&str, &str)] = &[
    ("00000C", "Cisco Systems, Inc."),
    ("0001C1", "Huawei Technologies Co., Ltd."),
    ("000AF7", "Broadcom Inc."),
    ("001422", "Dell Inc."),
    ("001A11", "Google LLC"),
    ("001C42", "Parallels, Inc."),
    ("0021CC", "Intel Corporate"),
    ("002590", "Super Micro Computer, Inc."),
    ("002687", "Realtek Semiconductor Corp."),
    ("005056", "VMware, Inc."),
    ("00E04C", "Realtek Semiconductor Corp."),
    ("0418D6", "Ubiquiti Networks, Inc."),
    ("080027", "Oracle Corporation (VirtualBox)"),
    ("0C5415", "Apple, Inc."),
    ("107B44", "ASUSTek Computer Inc."),
    ("1C1B0D", "GIGA-BYTE TECHNOLOGY CO., LTD."),
    ("2C56DC", "Intel Corporate"),
    ("3C7C3F", "Apple, Inc."),
    ("3C8C93", "Intel Corporate"),
    ("3CD92B", "Hewlett Packard"),
    ("408D5C", "Apple, Inc."),
    ("482C6A", "Intel Corporate"),
    ("54B203", "Intel Corporate"),
    ("705A0F", "Hewlett Packard"),
    ("70B3D5", "Linksys"),
    ("8C8590", "Apple, Inc."),
    ("90FBA6", "Xiaomi Communications Co., Ltd."),
    ("A0CEC8", "Apple, Inc."),
    ("A45E60", "Apple, Inc."),
    ("A483E7", "Intel Corporate"),
    ("B42E99", "Intel Corporate"),
    ("B4A9FC", "Apple, Inc."),
    ("B827EB", "Raspberry Pi Foundation"),
    ("C02568", "Microsoft Corporation"),
    ("C8D9D2", "Intel Corporate"),
    ("D03745", "Intel Corporate"),
    ("D05099", "ASUSTek Computer Inc."),
    ("D83ADD", "Intel Corporate"),
    ("D88083", "Xiaomi Communications Co., Ltd."),
    ("E0D55E", "Intel Corporate"),
    ("E4B97A", "Intel Corporate"),
    ("E86F38", "Apple, Inc."),
    ("EC8EB5", "Intel Corporate"),
    ("FC3497", "Apple, Inc."),
];

fn get_db_path() -> PathBuf {
    let mut path = if cfg!(target_os = "windows") {
        PathBuf::from(std::env::var("APPDATA").unwrap_or_else(|_| ".".to_string()))
    } else {
        let mut p = PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| ".".to_string()));
        p.push(".config");
        p
    };
    path.push("ir");
    path.push("oui.txt");
    path
}

fn normalize_mac(mac: &str) -> String {
    mac.chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .collect::<String>()
        .to_uppercase()
}

fn load_oui_db() -> HashMap<String, String> {
    let mut db = HashMap::new();
    
    // First load embedded
    for &(oui, vendor) in EMBEDDED_OUIS {
        db.insert(oui.to_string(), vendor.to_string());
    }

    // Attempt to load from file
    let path = get_db_path();
    if path.exists() {
        if let Ok(file) = File::open(&path) {
            let reader = BufReader::new(file);
            for line in reader.lines().map_while(Result::ok) {
                let parts: Vec<&str> = line.split('|').collect();
                if parts.len() == 2 {
                    db.insert(parts[0].trim().to_string(), parts[1].trim().to_string());
                }
            }
        }
    }
    db
}

fn lookup_vendor(mac: &str, db: &HashMap<String, String>) -> String {
    let clean = normalize_mac(mac);
    if clean.len() < 6 {
        return "Invalid Address".to_string();
    }
    let oui = &clean[0..6];
    db.get(oui).cloned().unwrap_or_else(|| "Unknown Vendor".to_string())
}

fn update_oui_db() {
    println!("Downloading OUI registry from IEEE...");
    
    let agent = ureq::AgentBuilder::new()
        .timeout_connect(Duration::from_secs(10))
        .timeout_read(Duration::from_secs(30))
        .build();

    let response = match agent.get("https://standards-oui.ieee.org/oui/oui.txt")
        .set("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .call() {
        Ok(res) => res,
        Err(e) => {
            eprintln!("Error: Failed to fetch OUI database: {}", e);
            std::process::exit(1);
        }
    };

    let path = get_db_path();
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    let mut out_file = match File::create(&path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error: Failed to create local file database: {}", e);
            std::process::exit(1);
        }
    };

    let reader = BufReader::new(response.into_reader());
    let mut count = 0;

    for line in reader.lines().map_while(Result::ok) {
        if line.contains("(hex)") {
            let parts: Vec<&str> = line.split("(hex)").collect();
            if parts.len() == 2 {
                let oui = normalize_mac(parts[0]);
                let vendor = parts[1].trim().to_string();
                if oui.len() == 6 && !vendor.is_empty() {
                    let _ = writeln!(out_file, "{}|{}", oui, vendor);
                    count += 1;
                }
            }
        }
    }

    println!("OUI Database successfully updated (stored {} entries at '{}').", count, path.display());
}

#[cfg(target_os = "windows")]
fn get_local_mac_addresses() -> Vec<(String, String)> {
    use std::ptr;
    use windows_sys::Win32::NetworkManagement::IpHelper::{
        GetAdaptersAddresses, IP_ADAPTER_ADDRESSES_LH,
    };

    let mut list = Vec::new();
    unsafe {
        let mut size = 15000;
        let mut buf = vec![0u8; size as usize];
        let flags = 0x0002 | 0x0004 | 0x0008;
        
        let mut ret = GetAdaptersAddresses(0, flags, ptr::null(), buf.as_mut_ptr() as *mut _, &mut size);
        if ret == 111 {
            buf.resize(size as usize, 0);
            ret = GetAdaptersAddresses(0, flags, ptr::null(), buf.as_mut_ptr() as *mut _, &mut size);
        }
        
        if ret == 0 {
            let mut curr = buf.as_ptr() as *const IP_ADAPTER_ADDRESSES_LH;
            while !curr.is_null() {
                let adapter = &*curr;
                if adapter.OperStatus == 1 && adapter.PhysicalAddressLength > 0 {
                    let mut name_utf16 = Vec::new();
                    let mut p = adapter.FriendlyName;
                    while !p.is_null() && *p != 0 {
                        name_utf16.push(*p);
                        p = p.add(1);
                    }
                    let name = String::from_utf16_lossy(&name_utf16);

                    let mut mac_parts = Vec::new();
                    for i in 0..(adapter.PhysicalAddressLength as usize) {
                        mac_parts.push(format!("{:02X}", adapter.PhysicalAddress[i]));
                    }
                    list.push((name, mac_parts.join(":")));
                }
                curr = adapter.Next;
            }
        }
    }
    list
}

#[cfg(target_os = "linux")]
fn get_local_mac_addresses() -> Vec<(String, String)> {
    let mut list = Vec::new();
    unsafe {
        let mut ifaddrs_ptr: *mut libc::ifaddrs = std::ptr::null_mut();
        if libc::getifaddrs(&mut ifaddrs_ptr) == 0 {
            let mut curr = ifaddrs_ptr;
            while !curr.is_null() {
                let ifa = &*curr;
                if !ifa.ifa_name.is_null() {
                    let name = std::ffi::CStr::from_ptr(ifa.ifa_name).to_string_lossy().into_owned();
                    let addr = ifa.ifa_addr;
                    if !addr.is_null() && (*addr).sa_family as i32 == libc::AF_PACKET {
                        let sll = addr as *const libc::sockaddr_ll;
                        let len = (*sll).sll_halen as usize;
                        if len > 0 {
                            let mut mac_parts = Vec::new();
                            for i in 0..len {
                                mac_parts.push(format!("{:02X}", (*sll).sll_addr[i]));
                            }
                            list.push((name, mac_parts.join(":")));
                        }
                    }
                }
                curr = ifa.ifa_next;
            }
            libc::freeifaddrs(ifaddrs_ptr);
        }
    }
    list
}

use std::time::Duration;

pub fn run_mac(options: MacOptions) {
    if options.update {
        update_oui_db();
        return;
    }

    let db = load_oui_db();

    if let Some(ref query_mac) = options.query {
        let vendor = lookup_vendor(query_mac, &db);
        println!("MAC Address: {}", query_mac);
        println!("Vendor:      {}", vendor);
    } else if options.local {
        let list = get_local_mac_addresses();
        if list.is_empty() {
            println!("No active local adapters found with MAC addresses.");
        } else {
            println!("{:<25} {:<20} {}", "INTERFACE", "MAC ADDRESS", "VENDOR");
            println!("----------------------------------------------------------------------");
            for (name, mac) in list {
                let vendor = lookup_vendor(&mac, &db);
                println!("{:<25} {:<20} {}", name, mac, vendor);
            }
        }
    }
}
