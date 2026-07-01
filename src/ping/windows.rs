use crate::PingOptions;
use std::net::ToSocketAddrs;
use windows_sys::Win32::NetworkManagement::IpHelper::{
    IcmpCloseHandle, IcmpCreateFile, IcmpSendEcho, ICMP_ECHO_REPLY,
};

pub fn ping(host: &str, options: PingOptions) {
    // Resolve host to IPv4
    let addr = match (host, 0).to_socket_addrs() {
        Ok(mut addrs) => {
            if let Some(sock_addr) = addrs.find(|a| a.is_ipv4()) {
                if let std::net::IpAddr::V4(ipv4) = sock_addr.ip() {
                    ipv4
                } else {
                    eprintln!("Error: Resolved IP is not IPv4.");
                    std::process::exit(1);
                }
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

    println!("Pinging {} [{}] with 32 bytes of data:", host, addr);

    unsafe {
        let handle = IcmpCreateFile();
        if handle == -1 || handle == 0 {
            eprintln!("Error: Failed to create ICMP handle.");
            std::process::exit(1);
        }

        let ip_addr = u32::from_ne_bytes(addr.octets());
        let request_data = [0u8; 32];
        
        let reply_size = std::mem::size_of::<ICMP_ECHO_REPLY>() + 32 + 8;
        let mut reply_buffer = vec![0u8; reply_size];

        let mut sent = 0;
        let mut received = 0;
        let mut rtts = Vec::new();

        for seq in 1..=options.count {
            let replies = IcmpSendEcho(
                handle,
                ip_addr,
                request_data.as_ptr() as *const _,
                request_data.len() as u16,
                std::ptr::null(),
                reply_buffer.as_mut_ptr() as *mut _,
                reply_size as u32,
                options.timeout_ms as u32,
            );

            sent += 1;

            if replies > 0 {
                let reply = &*(reply_buffer.as_ptr() as *const ICMP_ECHO_REPLY);
                if reply.Status == 0 { // IP_SUCCESS
                    let rtt = reply.RoundTripTime;
                    println!(
                        "Reply from {}: bytes={} time={}ms TTL={}",
                        addr,
                        reply.DataSize,
                        rtt,
                        reply.Options.Ttl
                    );
                    received += 1;
                    rtts.push(rtt);
                } else {
                    println!("Request timed out (Status={}).", reply.Status);
                }
            } else {
                println!("Request timed out.");
            }

            // Sleep 1 second between pings if not the last one
            if seq < options.count {
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        }

        IcmpCloseHandle(handle);

        println!("\nPing statistics for {}:", addr);
        let lost = sent - received;
        let lost_pct = (lost as f64 / sent as f64) * 100.0;
        println!(
            "    Packets: Sent = {}, Received = {}, Lost = {} ({:.0}% loss),",
            sent, received, lost, lost_pct
        );

        if !rtts.is_empty() {
            let min = rtts.iter().min().unwrap();
            let max = rtts.iter().max().unwrap();
            let sum: u32 = rtts.iter().sum();
            let avg = sum as f64 / rtts.len() as f64;
            println!(
                "Approximate round trip times in milli-seconds:\n    Minimum = {}ms, Maximum = {}ms, Average = {:.1}ms",
                min, max, avg
            );
        }
    }
}
