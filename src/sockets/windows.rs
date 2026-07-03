use crate::SocketsOptions;
use super::SocketInfo;
use std::collections::HashMap;
use windows_sys::Win32::Foundation::{CloseHandle, INVALID_HANDLE_VALUE};
use windows_sys::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, Process32FirstW, Process32NextW, PROCESSENTRY32W, TH32CS_SNAPPROCESS,
};

const TCP_TABLE_OWNER_PID_ALL: i32 = 5;
const UDP_TABLE_OWNER_PID: i32 = 1;
const AF_INET: u32 = 2;
const AF_INET6: u32 = 23;

#[repr(C)]
struct MIB_TCPROW_OWNER_PID {
    dwState: u32,
    dwLocalAddr: u32,
    dwLocalPort: u32,
    dwRemoteAddr: u32,
    dwRemotePort: u32,
    dwOwningPid: u32,
}

#[repr(C)]
struct MIB_TCPTABLE_OWNER_PID {
    dwNumEntries: u32,
    table: [MIB_TCPROW_OWNER_PID; 1],
}

#[repr(C)]
struct MIB_TCP6ROW_OWNER_PID {
    ucLocalAddr: [u8; 16],
    dwLocalScopeId: u32,
    dwLocalPort: u32,
    ucRemoteAddr: [u8; 16],
    dwRemoteScopeId: u32,
    dwRemotePort: u32,
    dwState: u32,
    dwOwningPid: u32,
}

#[repr(C)]
struct MIB_TCP6TABLE_OWNER_PID {
    dwNumEntries: u32,
    table: [MIB_TCP6ROW_OWNER_PID; 1],
}

#[repr(C)]
struct MIB_UDPROW_OWNER_PID {
    dwLocalAddr: u32,
    dwLocalPort: u32,
    dwOwningPid: u32,
}

#[repr(C)]
struct MIB_UDPTABLE_OWNER_PID {
    dwNumEntries: u32,
    table: [MIB_UDPROW_OWNER_PID; 1],
}

#[repr(C)]
struct MIB_UDP6ROW_OWNER_PID {
    ucLocalAddr: [u8; 16],
    dwLocalScopeId: u32,
    dwLocalPort: u32,
    dwOwningPid: u32,
}

#[repr(C)]
struct MIB_UDP6TABLE_OWNER_PID {
    dwNumEntries: u32,
    table: [MIB_UDP6ROW_OWNER_PID; 1],
}

fn map_state(state: u32) -> &'static str {
    match state {
        1 => "CLOSED",
        2 => "LISTEN",
        3 => "SYN_SENT",
        4 => "SYN_RECV",
        5 => "ESTABLISHED",
        6 => "FIN_WAIT1",
        7 => "FIN_WAIT2",
        8 => "CLOSE_WAIT",
        9 => "CLOSING",
        10 => "LAST_ACK",
        11 => "TIME_WAIT",
        _ => "UNKNOWN",
    }
}

fn string_from_wide(wide: &[u16]) -> String {
    let len = wide.iter().position(|&c| c == 0).unwrap_or(wide.len());
    String::from_utf16_lossy(&wide[..len])
}

fn get_process_map() -> HashMap<u32, String> {
    let mut map = HashMap::new();
    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        if snapshot != INVALID_HANDLE_VALUE {
            let mut entry = std::mem::zeroed::<PROCESSENTRY32W>();
            entry.dwSize = std::mem::size_of::<PROCESSENTRY32W>() as u32;

            if Process32FirstW(snapshot, &mut entry) != 0 {
                loop {
                    let pid = entry.th32ProcessID;
                    let name = string_from_wide(&entry.szExeFile);
                    map.insert(pid, name);
                    if Process32NextW(snapshot, &mut entry) == 0 {
                        break;
                    }
                }
            }
            CloseHandle(snapshot);
        }
    }
    map
}

fn get_tcp_ipv4(sockets: &mut Vec<SocketInfo>, pm: &HashMap<u32, String>) {
    unsafe {
        let mut size = 0u32;
        let _ = windows_sys::Win32::NetworkManagement::IpHelper::GetExtendedTcpTable(
            std::ptr::null_mut(),
            &mut size,
            1,
            AF_INET,
            TCP_TABLE_OWNER_PID_ALL,
            0,
        );

        if size == 0 {
            return;
        }

        let mut buf = vec![0u8; size as usize];
        let ret = windows_sys::Win32::NetworkManagement::IpHelper::GetExtendedTcpTable(
            buf.as_mut_ptr() as *mut _,
            &mut size,
            1,
            AF_INET,
            TCP_TABLE_OWNER_PID_ALL,
            0,
        );

        if ret == 0 {
            let table = &*(buf.as_ptr() as *const MIB_TCPTABLE_OWNER_PID);
            let entries = std::slice::from_raw_parts(
                table.table.as_ptr(),
                table.dwNumEntries as usize,
            );

            for entry in entries {
                let local_ip_bytes = entry.dwLocalAddr.to_ne_bytes();
                let local_ip = format!(
                    "{}.{}.{}.{}",
                    local_ip_bytes[0], local_ip_bytes[1], local_ip_bytes[2], local_ip_bytes[3]
                );
                let local_port = u16::from_be(entry.dwLocalPort as u16);

                let remote_ip_bytes = entry.dwRemoteAddr.to_ne_bytes();
                let remote_ip = format!(
                    "{}.{}.{}.{}",
                    remote_ip_bytes[0], remote_ip_bytes[1], remote_ip_bytes[2], remote_ip_bytes[3]
                );
                let remote_port = u16::from_be(entry.dwRemotePort as u16);

                let pid = entry.dwOwningPid;
                let prog = pm.get(&pid).cloned();

                sockets.push(SocketInfo {
                    protocol: "tcp".to_string(),
                    local_addr: format!("{}:{}", local_ip, local_port),
                    remote_addr: if entry.dwRemoteAddr == 0 {
                        "*:*".to_string()
                    } else {
                        format!("{}:{}", remote_ip, remote_port)
                    },
                    state: map_state(entry.dwState).to_string(),
                    pid: Some(pid),
                    program: prog,
                });
            }
        }
    }
}

fn get_tcp_ipv6(sockets: &mut Vec<SocketInfo>, pm: &HashMap<u32, String>) {
    unsafe {
        let mut size = 0u32;
        let _ = windows_sys::Win32::NetworkManagement::IpHelper::GetExtendedTcpTable(
            std::ptr::null_mut(),
            &mut size,
            1,
            AF_INET6,
            TCP_TABLE_OWNER_PID_ALL,
            0,
        );

        if size == 0 {
            return;
        }

        let mut buf = vec![0u8; size as usize];
        let ret = windows_sys::Win32::NetworkManagement::IpHelper::GetExtendedTcpTable(
            buf.as_mut_ptr() as *mut _,
            &mut size,
            1,
            AF_INET6,
            TCP_TABLE_OWNER_PID_ALL,
            0,
        );

        if ret == 0 {
            let table = &*(buf.as_ptr() as *const MIB_TCP6TABLE_OWNER_PID);
            let entries = std::slice::from_raw_parts(
                table.table.as_ptr(),
                table.dwNumEntries as usize,
            );

            for entry in entries {
                let local_ipv6 = std::net::Ipv6Addr::from(entry.ucLocalAddr);
                let local_port = u16::from_be(entry.dwLocalPort as u16);

                let remote_ipv6 = std::net::Ipv6Addr::from(entry.ucRemoteAddr);
                let remote_port = u16::from_be(entry.dwRemotePort as u16);

                let pid = entry.dwOwningPid;
                let prog = pm.get(&pid).cloned();

                let is_unspecified = remote_ipv6.is_unspecified();

                sockets.push(SocketInfo {
                    protocol: "tcp6".to_string(),
                    local_addr: format!("[{}]:{}", local_ipv6, local_port),
                    remote_addr: if is_unspecified {
                        "[::]:*".to_string()
                    } else {
                        format!("[{}]:{}", remote_ipv6, remote_port)
                    },
                    state: map_state(entry.dwState).to_string(),
                    pid: Some(pid),
                    program: prog,
                });
            }
        }
    }
}

fn get_udp_ipv4(sockets: &mut Vec<SocketInfo>, pm: &HashMap<u32, String>) {
    unsafe {
        let mut size = 0u32;
        let _ = windows_sys::Win32::NetworkManagement::IpHelper::GetExtendedUdpTable(
            std::ptr::null_mut(),
            &mut size,
            1,
            AF_INET,
            UDP_TABLE_OWNER_PID,
            0,
        );

        if size == 0 {
            return;
        }

        let mut buf = vec![0u8; size as usize];
        let ret = windows_sys::Win32::NetworkManagement::IpHelper::GetExtendedUdpTable(
            buf.as_mut_ptr() as *mut _,
            &mut size,
            1,
            AF_INET,
            UDP_TABLE_OWNER_PID,
            0,
        );

        if ret == 0 {
            let table = &*(buf.as_ptr() as *const MIB_UDPTABLE_OWNER_PID);
            let entries = std::slice::from_raw_parts(
                table.table.as_ptr(),
                table.dwNumEntries as usize,
            );

            for entry in entries {
                let local_ip_bytes = entry.dwLocalAddr.to_ne_bytes();
                let local_ip = format!(
                    "{}.{}.{}.{}",
                    local_ip_bytes[0], local_ip_bytes[1], local_ip_bytes[2], local_ip_bytes[3]
                );
                let local_port = u16::from_be(entry.dwLocalPort as u16);

                let pid = entry.dwOwningPid;
                let prog = pm.get(&pid).cloned();

                sockets.push(SocketInfo {
                    protocol: "udp".to_string(),
                    local_addr: format!("{}:{}", local_ip, local_port),
                    remote_addr: "*:*".to_string(),
                    state: "-".to_string(),
                    pid: Some(pid),
                    program: prog,
                });
            }
        }
    }
}

fn get_udp_ipv6(sockets: &mut Vec<SocketInfo>, pm: &HashMap<u32, String>) {
    unsafe {
        let mut size = 0u32;
        let _ = windows_sys::Win32::NetworkManagement::IpHelper::GetExtendedUdpTable(
            std::ptr::null_mut(),
            &mut size,
            1,
            AF_INET6,
            UDP_TABLE_OWNER_PID,
            0,
        );

        if size == 0 {
            return;
        }

        let mut buf = vec![0u8; size as usize];
        let ret = windows_sys::Win32::NetworkManagement::IpHelper::GetExtendedUdpTable(
            buf.as_mut_ptr() as *mut _,
            &mut size,
            1,
            AF_INET6,
            UDP_TABLE_OWNER_PID,
            0,
        );

        if ret == 0 {
            let table = &*(buf.as_ptr() as *const MIB_UDP6TABLE_OWNER_PID);
            let entries = std::slice::from_raw_parts(
                table.table.as_ptr(),
                table.dwNumEntries as usize,
            );

            for entry in entries {
                let local_ipv6 = std::net::Ipv6Addr::from(entry.ucLocalAddr);
                let local_port = u16::from_be(entry.dwLocalPort as u16);

                let pid = entry.dwOwningPid;
                let prog = pm.get(&pid).cloned();

                sockets.push(SocketInfo {
                    protocol: "udp6".to_string(),
                    local_addr: format!("[{}]:{}", local_ipv6, local_port),
                    remote_addr: "[::]:*".to_string(),
                    state: "-".to_string(),
                    pid: Some(pid),
                    program: prog,
                });
            }
        }
    }
}

pub fn get_sockets(_options: &SocketsOptions) -> Vec<SocketInfo> {
    let mut sockets = Vec::new();
    let pm = get_process_map();

    get_tcp_ipv4(&mut sockets, &pm);
    get_tcp_ipv6(&mut sockets, &pm);
    get_udp_ipv4(&mut sockets, &pm);
    get_udp_ipv6(&mut sockets, &pm);

    sockets
}
