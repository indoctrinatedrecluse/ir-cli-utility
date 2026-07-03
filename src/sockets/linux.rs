use crate::SocketsOptions;
use super::SocketInfo;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};

fn get_process_map() -> HashMap<u64, (u32, String)> {
    let mut map = HashMap::new();
    let proc_dir = match fs::read_dir("/proc") {
        Ok(d) => d,
        Err(_) => return map,
    };

    for entry_res in proc_dir {
        let entry = match entry_res {
            Ok(e) => e,
            Err(_) => continue,
        };

        let file_name = entry.file_name();
        let pid_str = file_name.to_string_lossy();
        let pid = match pid_str.parse::<u32>() {
            Ok(p) => p,
            Err(_) => continue,
        };

        let comm_path = format!("/proc/{}/comm", pid);
        let program = fs::read_to_string(comm_path)
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|_| String::new());

        let fd_dir_path = format!("/proc/{}/fd", pid);
        let fd_dir = match fs::read_dir(fd_dir_path) {
            Ok(d) => d,
            Err(_) => continue,
        };

        for fd_entry_res in fd_dir {
            let fd_entry = match fd_entry_res {
                Ok(e) => e,
                Err(_) => continue,
            };

            if let Ok(link) = fs::read_link(fd_entry.path()) {
                let target = link.to_string_lossy();
                if target.starts_with("socket:[") && target.ends_with(']') {
                    let inode_str = &target["socket:[".len()..target.len() - 1];
                    if let Ok(inode) = inode_str.parse::<u64>() {
                        map.insert(inode, (pid, program.clone()));
                    }
                }
            }
        }
    }
    map
}

fn parse_ipv4_endpoint(s: &str) -> Option<String> {
    let (ip_hex, port_hex) = s.split_once(':')?;
    let ip_val = u32::from_str_radix(ip_hex, 16).ok()?;
    let port = u16::from_str_radix(port_hex, 16).ok()?;
    let bytes = ip_val.to_ne_bytes();
    Some(format!("{}.{}.{}.{}:{}", bytes[0], bytes[1], bytes[2], bytes[3], port))
}

fn parse_ipv6_endpoint(s: &str) -> Option<String> {
    let (ip_hex, port_hex) = s.split_once(':')?;
    if ip_hex.len() != 32 { return None; }
    let port = u16::from_str_radix(port_hex, 16).ok()?;
    
    let mut ip_bytes = [0u8; 16];
    for i in 0..4 {
        let chunk = &ip_hex[i * 8..(i + 1) * 8];
        let val = u32::from_str_radix(chunk, 16).ok()?;
        let bytes = val.to_ne_bytes();
        ip_bytes[i * 4..i * 4 + 4].copy_from_slice(&bytes);
    }
    
    let ipv6 = std::net::Ipv6Addr::from(ip_bytes);
    if ipv6.is_unspecified() {
        Some(format!("[::]:{}", port))
    } else {
        Some(format!("[{}]:{}", ipv6, port))
    }
}

fn map_linux_state(state_hex: &str) -> &'static str {
    match state_hex {
        "01" => "ESTABLISHED",
        "02" => "SYN_SENT",
        "03" => "SYN_RECV",
        "04" => "FIN_WAIT1",
        "05" => "FIN_WAIT2",
        "06" => "TIME_WAIT",
        "07" => "CLOSE",
        "08" => "CLOSE_WAIT",
        "09" => "LAST_ACK",
        "0A" => "LISTEN",
        "0B" => "CLOSING",
        _ => "UNKNOWN",
    }
}

fn parse_file(
    file_path: &str,
    protocol: &str,
    is_ipv6: bool,
    sockets: &mut Vec<SocketInfo>,
    pm: &HashMap<u64, (u32, String)>,
) {
    let file = match File::open(file_path) {
        Ok(f) => f,
        Err(_) => return,
    };

    let reader = BufReader::new(file);
    for (idx, line_res) in reader.lines().enumerate() {
        if idx == 0 {
            continue;
        }

        let line = match line_res {
            Ok(l) => l,
            Err(_) => continue,
        };

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 10 {
            continue;
        }

        let local = parts[1];
        let remote = parts[2];
        let state_hex = parts[3];
        let inode_str = parts[9];

        let local_addr = if is_ipv6 {
            parse_ipv6_endpoint(local)
        } else {
            parse_ipv4_endpoint(local)
        }.unwrap_or_else(|| local.to_string());

        let remote_addr = if is_ipv6 {
            parse_ipv6_endpoint(remote)
        } else {
            parse_ipv4_endpoint(remote)
        }.unwrap_or_else(|| remote.to_string());

        let inode = inode_str.parse::<u64>().unwrap_or(0);
        let (pid, program) = if let Some((p, prog)) = pm.get(&inode) {
            (Some(*p), Some(prog.clone()))
        } else {
            (None, None)
        };

        let state = if protocol.starts_with("tcp") {
            map_linux_state(state_hex).to_string()
        } else {
            "-".to_string()
        };

        sockets.push(SocketInfo {
            protocol: protocol.to_string(),
            local_addr,
            remote_addr,
            state,
            pid,
            program,
        });
    }
}

pub fn get_sockets(_options: &SocketsOptions) -> Vec<SocketInfo> {
    let mut sockets = Vec::new();
    let pm = get_process_map();

    parse_file("/proc/net/tcp", "tcp", false, &mut sockets, &pm);
    parse_file("/proc/net/tcp6", "tcp6", true, &mut sockets, &pm);
    parse_file("/proc/net/udp", "udp", false, &mut sockets, &pm);
    parse_file("/proc/net/udp6", "udp6", true, &mut sockets, &pm);

    sockets
}
