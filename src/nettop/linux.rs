use std::fs::File;
use std::io::{BufRead, BufReader, Result};

pub struct NetInterfaceStats {
    pub name: String,
    pub rx_bytes: u64,
    pub tx_bytes: u64,
}

pub fn get_net_interfaces() -> Result<Vec<NetInterfaceStats>> {
    let mut interfaces = Vec::new();
    let file = File::open("/proc/net/dev")?;
    let reader = BufReader::new(file);

    for (idx, line_res) in reader.lines().enumerate() {
        if idx < 2 {
            continue; // Skip headers
        }
        let line = line_res?;
        if let Some((name_part, stats_part)) = line.split_once(':') {
            let name = name_part.trim().to_string();
            let fields: Vec<&str> = stats_part.split_whitespace().collect();
            if fields.len() >= 9 {
                let rx_bytes = fields[0].parse::<u64>().unwrap_or(0);
                let tx_bytes = fields[8].parse::<u64>().unwrap_or(0);
                interfaces.push(NetInterfaceStats {
                    name,
                    rx_bytes,
                    tx_bytes,
                });
            }
        }
    }
    Ok(interfaces)
}

pub struct RawModeGuard {
    orig: libc::termios,
}

impl Drop for RawModeGuard {
    fn drop(&mut self) {
        unsafe {
            libc::tcsetattr(0, libc::TCSAFLUSH, &self.orig);
        }
    }
}

pub fn set_raw_mode() -> Option<RawModeGuard> {
    unsafe {
        let mut orig = std::mem::zeroed();
        if libc::tcgetattr(0, &mut orig) == 0 {
            let mut raw = orig;
            raw.c_lflag &= !(libc::ECHO | libc::ICANON);
            raw.c_cc[libc::VMIN] = 0;
            raw.c_cc[libc::VTIME] = 0;
            if libc::tcsetattr(0, libc::TCSAFLUSH, &raw) == 0 {
                return Some(RawModeGuard { orig });
            }
        }
        None
    }
}

pub fn poll_keyboard_input() -> Option<char> {
    let mut buf = [0u8; 1];
    unsafe {
        let n = libc::read(0, buf.as_mut_ptr() as *mut libc::c_void, 1);
        if n > 0 {
            Some(buf[0] as char)
        } else {
            None
        }
    }
}
