use std::net::{TcpStream, ToSocketAddrs};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use crate::PortscanOptions;

const TOP_100_PORTS: &[u16] = &[
    21, 22, 23, 25, 53, 80, 110, 115, 135, 139, 143, 194, 443, 445, 465, 554, 587, 993, 995,
    1025, 1433, 1723, 3306, 3389, 5060, 5432, 5900, 8000, 8080, 8443, 8888,
];

fn parse_ports(ports_str: &str) -> Result<Vec<u16>, String> {
    if ports_str == "top100" {
        return Ok(TOP_100_PORTS.to_vec());
    }
    if ports_str == "all" {
        return Ok((1..=65535).collect());
    }

    let mut ports = Vec::new();
    for part in ports_str.split(',') {
        let part = part.trim();
        if part.contains('-') {
            let bounds: Vec<&str> = part.split('-').collect();
            if bounds.len() == 2 {
                let start = bounds[0].trim().parse::<u16>().map_err(|_| format!("Invalid port boundary '{}'", bounds[0]))?;
                let end = bounds[1].trim().parse::<u16>().map_err(|_| format!("Invalid port boundary '{}'", bounds[1]))?;
                if start <= end {
                    ports.extend(start..=end);
                } else {
                    return Err(format!("Invalid port range '{}-{}'", start, end));
                }
            } else {
                return Err(format!("Invalid port range '{}'", part));
            }
        } else {
            let port = part.parse::<u16>().map_err(|_| format!("Invalid port number '{}'", part))?;
            ports.push(port);
        }
    }
    
    ports.sort();
    ports.dedup();
    Ok(ports)
}

fn get_service_name(port: u16) -> &'static str {
    match port {
        21 => "FTP",
        22 => "SSH",
        23 => "Telnet",
        25 => "SMTP",
        53 => "DNS",
        80 => "HTTP",
        110 => "POP3",
        115 => "SFTP",
        135 => "RPC",
        139 => "NetBIOS",
        143 => "IMAP",
        194 => "IRC",
        443 => "HTTPS",
        445 => "SMB",
        465 => "SMTPS",
        554 => "RTSP",
        587 => "SMTP (Submission)",
        993 => "IMAPS",
        995 => "POP3S",
        1433 => "MSSQL",
        1723 => "PPTP",
        3306 => "MySQL",
        3389 => "RDP",
        5060 => "SIP",
        5432 => "PostgreSQL",
        5900 => "VNC",
        8000 => "Common HTTP Alt",
        8080 => "HTTP Proxy / Tomcat",
        8443 => "HTTPS Alt",
        8888 => "Common HTTP Alt",
        _ => "Unknown",
    }
}

pub fn run_portscan(options: PortscanOptions) {
    let ports = match parse_ports(&options.ports) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    if !options.json {
        println!("Resolving target '{}'...", options.host);
    }

    let ip_addr = match format!("{}:0", options.host).to_socket_addrs() {
        Ok(mut addrs) => {
            if let Some(addr) = addrs.next() {
                addr.ip()
            } else {
                eprintln!("Error: Resolved target '{}' returned no addresses.", options.host);
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Error: Failed to resolve target '{}': {}", options.host, e);
            std::process::exit(1);
        }
    };

    if !options.json {
        println!("Target IP: {}", ip_addr);
    }

    if options.ping_first {
        if !options.json {
            println!("Validating host is online...");
        }
        let ping_ports = &[80, 443, 22, 53, 445, 3389];
        let mut online = false;
        for &port in ping_ports {
            let addr = std::net::SocketAddr::new(ip_addr, port);
            if TcpStream::connect_timeout(&addr, Duration::from_millis(300)).is_ok() {
                online = true;
                break;
            }
        }
        if !online {
            if options.json {
                println!("{{\"host\": \"{}\", \"status\": \"offline\", \"open_ports\": []}}", options.host);
            } else {
                eprintln!("Error: Target host '{}' appears to be offline. (No response on common ports)", options.host);
            }
            std::process::exit(1);
        }
    }

    if !options.json {
        println!("Scanning {} ports on {} with concurrency limit {}...", ports.len(), ip_addr, options.concurrency);
    }

    let ports_queue = Arc::new(Mutex::new(ports));
    let open_ports = Arc::new(Mutex::new(Vec::new()));
    let timeout = Duration::from_millis(options.timeout_ms);
    let mut workers = Vec::new();

    let concurrency = options.concurrency.min(1000); // safety cap

    for _ in 0..concurrency {
        let ports_queue = Arc::clone(&ports_queue);
        let open_ports = Arc::clone(&open_ports);
        let host_ip = ip_addr;
        
        workers.push(thread::spawn(move || {
            loop {
                let port = {
                    let mut lock = ports_queue.lock().unwrap();
                    if lock.is_empty() {
                        break;
                    }
                    lock.remove(0)
                };

                let addr = std::net::SocketAddr::new(host_ip, port);
                if TcpStream::connect_timeout(&addr, timeout).is_ok() {
                    let mut lock = open_ports.lock().unwrap();
                    lock.push(port);
                }
            }
        }));
    }

    for worker in workers {
        let _ = worker.join();
    }

    let mut final_open_ports = open_ports.lock().unwrap().clone();
    final_open_ports.sort();

    if options.json {
        let ports_json: Vec<String> = final_open_ports.iter().map(|p| p.to_string()).collect();
        println!("{{\"host\": \"{}\", \"ip\": \"{}\", \"status\": \"online\", \"open_ports\": [{}]}}", 
            options.host, ip_addr, ports_json.join(", "));
    } else {
        println!("\nPORT      STATE    SERVICE");
        println!("--------------------------");
        for port in final_open_ports {
            println!("{:<9} open     {}", format!("{}/tcp", port), get_service_name(port));
        }
        println!("\nScan finished successfully.");
    }
}
