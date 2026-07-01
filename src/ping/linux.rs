use crate::PingOptions;
use std::net::ToSocketAddrs;
use std::time::{Instant, Duration};

fn calculate_checksum(buffer: &[u8]) -> u16 {
    let mut sum = 0u32;
    let mut i = 0;
    while i < buffer.len() - 1 {
        let word = ((buffer[i] as u32) << 8) | (buffer[i + 1] as u32);
        sum += word;
        i += 2;
    }
    if i < buffer.len() {
        sum += (buffer[i] as u32) << 8;
    }
    while sum >> 16 != 0 {
        sum = (sum & 0xffff) + (sum >> 16);
    }
    !(sum as u16)
}

fn create_ping_socket() -> Result<(libc::c_int, bool), String> {
    unsafe {
        // Try unprivileged DGRAM ICMP socket first
        // IPPROTO_ICMP is 1
        let sock = libc::socket(libc::AF_INET, libc::SOCK_DGRAM, 1);
        if sock >= 0 {
            return Ok((sock, false));
        }

        // Try raw ICMP socket (requires root)
        let sock_raw = libc::socket(libc::AF_INET, libc::SOCK_RAW, 1);
        if sock_raw >= 0 {
            return Ok((sock_raw, true));
        }

        let err = std::io::Error::last_os_error();
        Err(format!("Permission Denied: {}", err))
    }
}

pub fn ping(host: &str, options: PingOptions) {
    let addr = match (host, 0).to_socket_addrs() {
        Ok(mut addrs) => {
            if let Some(std::net::IpAddr::V4(ipv4)) = addrs.find(|a| a.is_ipv4()) {
                ipv4
            } else {
                eprintln!("Error: Could not resolve '{}' to an IPv4 address.", host);
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Error: Resolution failed for '{}': {}", host, e);
            std::process::exit(1);
        }
    };

    let (sock, is_raw) = match create_ping_socket() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error: Failed to create ping socket: {}", e);
            eprintln!("On Linux, ICMP sockets require permissions. Try running as root (sudo) or enabling:");
            eprintln!("    sysctl -w net.ipv4.ping_group_range=\"0 2147483647\"");
            std::process::exit(1);
        }
    };

    println!("PING {} ({}) 56(84) bytes of data.", host, addr);

    unsafe {
        let timeout = libc::timeval {
            tv_sec: (options.timeout_ms / 1000) as libc::time_t,
            tv_usec: ((options.timeout_ms % 1000) * 1000) as libc::suseconds_t,
        };
        libc::setsockopt(
            sock,
            libc::SOL_SOCKET,
            libc::SO_RCVTIMEO,
            &timeout as *const _ as *const libc::c_void,
            std::mem::size_of::<libc::timeval>() as libc::socklen_t,
        );

        let dest_addr = libc::sockaddr_in {
            sin_family: libc::AF_INET as libc::sa_family_t,
            sin_port: 0,
            sin_addr: libc::in_addr {
                s_addr: u32::from_ne_bytes(addr.octets()),
            },
            sin_zero: [0; 8],
        };

        let mut sent = 0;
        let mut received = 0;
        let mut rtts = Vec::new();

        for seq in 1..=options.count {
            // Construct ICMP Echo Request (8 bytes header + 32 bytes data)
            let mut packet = vec![0u8; 40];
            packet[0] = 8; // Type: Echo Request
            packet[1] = 0; // Code: 0
            
            // Identifier (2 bytes)
            let pid = std::process::id() as u16;
            packet[4] = (pid >> 8) as u8;
            packet[5] = (pid & 0xff) as u8;
            
            // Sequence Number (2 bytes)
            packet[6] = (seq >> 8) as u8;
            packet[7] = (seq & 0xff) as u8;

            // Payload data
            for i in 8..40 {
                packet[i] = i as u8;
            }

            // Calculate checksum
            let checksum = calculate_checksum(&packet);
            packet[2] = (checksum >> 8) as u8;
            packet[3] = (checksum & 0xff) as u8;

            let start = Instant::now();
            let bytes_sent = libc::sendto(
                sock,
                packet.as_ptr() as *const libc::c_void,
                packet.len(),
                0,
                &dest_addr as *const _ as *const libc::sockaddr,
                std::mem::size_of::<libc::sockaddr_in>() as libc::socklen_t,
            );

            if bytes_sent < 0 {
                eprintln!("Error: sendto failed.");
                libc::close(sock);
                std::process::exit(1);
            }

            sent += 1;

            let mut recv_buf = vec![0u8; 1024];
            let mut from_addr = std::mem::zeroed::<libc::sockaddr_in>();
            let mut from_len = std::mem::size_of::<libc::sockaddr_in>() as libc::socklen_t;

            let bytes_received = libc::recvfrom(
                sock,
                recv_buf.as_mut_ptr() as *mut libc::c_void,
                recv_buf.len(),
                0,
                &mut from_addr as *mut _ as *mut libc::sockaddr,
                &mut from_len,
            );

            let duration = start.elapsed();

            if bytes_received >= 0 {
                let bytes_rec = bytes_received as usize;
                
                // Parse ICMP header offset
                let mut icmp_offset = 0;
                let mut ttl = 64; // Default fallback

                if is_raw && bytes_rec >= 28 {
                    // Raw socket includes IPv4 header (20 bytes)
                    icmp_offset = 20;
                    ttl = recv_buf[8]; // TTL is at byte 8 of IP header
                } else if !is_raw && bytes_rec >= 8 {
                    icmp_offset = 0;
                } else {
                    println!("Request timed out (invalid packet size: {}).", bytes_rec);
                    continue;
                }

                let icmp_type = recv_buf[icmp_offset];
                if icmp_type == 0 { // Echo Reply
                    let rtt_ms = duration.as_secs_f64() * 1000.0;
                    println!(
                        "64 bytes from {}: icmp_seq={} ttl={} time={:.2} ms",
                        addr, seq, ttl, rtt_ms
                    );
                    received += 1;
                    rtts.push(rtt_ms);
                } else {
                    println!("From {}: icmp_seq={} Destination Unreachable (Type={})", addr, seq, icmp_type);
                }
            } else {
                println!("Request timed out.");
            }

            // Sleep between pings
            if seq < options.count {
                std::thread::sleep(Duration::from_secs(1));
            }
        }

        libc::close(sock);

        println!("\n--- {} ping statistics ---", host);
        let lost = sent - received;
        let lost_pct = (lost as f64 / sent as f64) * 100.0;
        println!(
            "{} packets transmitted, {} received, {:.0}% packet loss, time {}ms",
            sent,
            received,
            lost_pct,
            sent * 1000
        );

        if !rtts.is_empty() {
            let min = rtts.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
            let max = rtts.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
            let sum: f64 = rtts.iter().sum();
            let avg = sum / rtts.len() as f64;
            println!(
                "rtt min/avg/max = {:.3}/{:.3}/{:.3} ms",
                min, avg, max
            );
        }
    }
}
