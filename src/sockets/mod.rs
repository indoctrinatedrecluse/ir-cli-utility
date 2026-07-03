use crate::SocketsOptions;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
fn get_sockets(options: &SocketsOptions) -> Vec<SocketInfo> {
    linux::get_sockets(options)
}

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
fn get_sockets(options: &SocketsOptions) -> Vec<SocketInfo> {
    windows::get_sockets(options)
}

#[derive(Clone)]
pub struct SocketInfo {
    pub protocol: String,
    pub local_addr: String,
    pub remote_addr: String,
    pub state: String,
    pub pid: Option<u32>,
    pub program: Option<String>,
}

pub fn sockets(options: SocketsOptions) {
    let mut list = get_sockets(&options);

    // Apply filtering based on options
    list.retain(|s| {
        let is_tcp = s.protocol.starts_with("tcp");
        let is_udp = s.protocol.starts_with("udp");

        // Protocol filters
        if options.tcp_only && !is_tcp {
            return false;
        }
        if options.udp_only && !is_udp {
            return false;
        }

        // State/Connection filters
        if options.listening_only {
            if is_tcp {
                s.state == "LISTEN"
            } else {
                true // UDP is implicitly listening/bound
            }
        } else if !options.show_all {
            // Default: connected/active sockets only
            if is_tcp {
                s.state != "LISTEN" && s.state != "CLOSE" && s.state != "UNKNOWN"
            } else {
                false // Hide UDP by default (since UDP is connectionless)
            }
        } else {
            true
        }
    });

    println!(
        "{:<6} {:<30} {:<30} {:<12} {:<20}",
        "Proto", "Local Address", "Remote Address", "State", "PID/Program"
    );

    for s in list {
        let program_str = match (s.pid, s.program) {
            (Some(pid), Some(prog)) => format!("{}/{}", pid, prog),
            (Some(pid), None) => format!("{}/-", pid),
            _ => "-".to_string(),
        };

        println!(
            "{:<6} {:<30} {:<30} {:<12} {:<20}",
            s.protocol, s.local_addr, s.remote_addr, s.state, program_str
        );
    }
}
