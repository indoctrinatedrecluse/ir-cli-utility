use std::io::Result;
use windows_sys::Win32::NetworkManagement::IpHelper::{GetIfTable2, FreeMibTable, MIB_IF_TABLE2, MIB_IF_ROW2};

const IF_TYPE_ETHERNET_CSMACD: u32 = 6;
const IF_TYPE_SOFTWARE_LOOPBACK: u32 = 24;
const IF_TYPE_IEEE80211: u32 = 71;

pub struct NetInterfaceStats {
    pub name: String,
    pub rx_bytes: u64,
    pub tx_bytes: u64,
}

pub fn get_net_interfaces() -> Result<Vec<NetInterfaceStats>> {
    let mut interfaces = Vec::new();
    unsafe {
        let mut p_table: *mut MIB_IF_TABLE2 = std::ptr::null_mut();
        if GetIfTable2(&mut p_table) == 0 && !p_table.is_null() {
            let num_entries = (*p_table).NumEntries as usize;
            let table_start = &(*p_table).Table[0] as *const MIB_IF_ROW2;

            for i in 0..num_entries {
                let row = &*table_start.add(i);
                // Filter interface types (Ethernet, WiFi, or Loopback/others)
                // Typically we want Ethernet (6) and WiFi (71), or Software Loopback (24)
                let if_type = row.Type;
                if if_type == IF_TYPE_ETHERNET_CSMACD || if_type == IF_TYPE_IEEE80211 || if_type == IF_TYPE_SOFTWARE_LOOPBACK {
                    // Extract alias/description
                    let len = row.Description.iter().position(|&c| c == 0).unwrap_or(row.Description.len());
                    let desc = String::from_utf16_lossy(&row.Description[..len]);
                    
                    interfaces.push(NetInterfaceStats {
                        name: desc,
                        rx_bytes: row.InOctets,
                        tx_bytes: row.OutOctets,
                    });
                }
            }
            FreeMibTable(p_table as *const std::ffi::c_void);
        }
    }
    Ok(interfaces)
}

#[cfg(target_os = "windows")]
extern "C" {
    fn _kbhit() -> std::os::raw::c_int;
    fn _getch() -> std::os::raw::c_int;
}

pub fn poll_keyboard_input() -> Option<char> {
    unsafe {
        if _kbhit() != 0 {
            let ch = _getch();
            Some(ch as u8 as char)
        } else {
            None
        }
    }
}
