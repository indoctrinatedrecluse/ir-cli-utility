use std::net::UdpSocket;

fn build_dns_query(host: &str, qtype: u16) -> Result<Vec<u8>, String> {
    let mut packet = Vec::new();
    
    // Transaction ID
    packet.push(0x12);
    packet.push(0x34);
    
    // Flags (standard query, recursion desired)
    packet.push(0x01);
    packet.push(0x00);
    
    // Questions (1)
    packet.push(0x00);
    packet.push(0x01);
    
    // Answer RRs (0), Authority RRs (0), Additional RRs (0)
    packet.push(0x00); packet.push(0x00);
    packet.push(0x00); packet.push(0x00);
    packet.push(0x00); packet.push(0x00);
    
    // QNAME
    for label in host.split('.') {
        if label.is_empty() {
            return Err("Invalid host name (contains empty labels)".to_string());
        }
        if label.len() > 63 {
            return Err("Label too long (maximum 63 characters)".to_string());
        }
        packet.push(label.len() as u8);
        packet.extend_from_slice(label.as_bytes());
    }
    packet.push(0x00); // end of QNAME
    
    // QTYPE
    packet.push((qtype >> 8) as u8);
    packet.push((qtype & 0xFF) as u8);
    
    // QCLASS (IN = 1)
    packet.push(0x00);
    packet.push(0x01);
    
    Ok(packet)
}

fn parse_name(packet: &[u8], pos: &mut usize) -> Result<String, String> {
    let mut name = String::new();
    let mut current_pos = *pos;
    let mut jumped = false;
    let mut original_len_consumed = 0;
    
    let mut loop_count = 0;
    loop {
        if loop_count > 50 {
            return Err("Infinite pointer loop in DNS name".to_string());
        }
        loop_count += 1;
        
        if current_pos >= packet.len() {
            return Err("Name position goes out of packet bounds".to_string());
        }
        
        let len = packet[current_pos];
        if (len & 0xC0) == 0xC0 {
            if current_pos + 1 >= packet.len() {
                return Err("Pointer offset goes out of packet bounds".to_string());
            }
            let offset = (((len & 0x3F) as usize) << 8) | (packet[current_pos + 1] as usize);
            if !jumped {
                original_len_consumed += 2;
                jumped = true;
            }
            current_pos = offset;
        } else if len == 0 {
            current_pos += 1;
            if !jumped {
                original_len_consumed += 1;
            }
            break;
        } else {
            if current_pos + 1 + (len as usize) > packet.len() {
                return Err("Name length goes out of bounds".to_string());
            }
            if !name.is_empty() {
                name.push('.');
            }
            name.push_str(std::str::from_utf8(&packet[current_pos + 1 .. current_pos + 1 + (len as usize)])
                .map_err(|e| e.to_string())?);
            current_pos += 1 + (len as usize);
            if !jumped {
                original_len_consumed += 1 + (len as usize);
            }
        }
    }
    
    if jumped {
        *pos += original_len_consumed;
    } else {
        *pos = current_pos;
    }
    
    Ok(name)
}

fn parse_dns_response(packet: &[u8], query_type: u16) -> Result<Vec<String>, String> {
    if packet.len() < 12 {
        return Err("Response packet too short".to_string());
    }
    
    let ancount = ((packet[6] as u16) << 8) | (packet[7] as u16);
    let qdcount = ((packet[4] as u16) << 8) | (packet[5] as u16);
    
    let mut pos = 12;
    
    // Skip questions
    for _ in 0..qdcount {
        parse_name(packet, &mut pos)?;
        pos += 4; // Skip QTYPE and QCLASS
    }
    
    let mut results = Vec::new();
    
    // Parse Answer RRs
    for _ in 0..ancount {
        let _name = parse_name(packet, &mut pos)?;
        if pos + 10 > packet.len() {
            return Err("Answer RR header goes out of packet bounds".to_string());
        }
        let rtype = ((packet[pos] as u16) << 8) | (packet[pos + 1] as u16);
        let rdlength = ((packet[pos + 8] as u16) << 8) | (packet[pos + 9] as u16);
        pos += 10; // Skip Type, Class, TTL, RDLength
        
        let rd_end = pos + (rdlength as usize);
        if rd_end > packet.len() {
            return Err("Answer RR rdlength goes out of packet bounds".to_string());
        }
        
        if rtype == query_type {
            match query_type {
                1 => { // Type A (IPv4)
                    if rdlength == 4 {
                        results.push(format!("{}.{}.{}.{}", packet[pos], packet[pos+1], packet[pos+2], packet[pos+3]));
                    }
                }
                28 => { // Type AAAA (IPv6)
                    if rdlength == 16 {
                        let mut ipv6 = String::new();
                        for i in 0..8 {
                            let val = ((packet[pos + i*2] as u16) << 8) | (packet[pos + i*2 + 1] as u16);
                            if i > 0 { ipv6.push(':'); }
                            ipv6.push_str(&format!("{:x}", val));
                        }
                        results.push(ipv6);
                    }
                }
                5 => { // Type CNAME
                    let mut temp_pos = pos;
                    let cname = parse_name(packet, &mut temp_pos)?;
                    results.push(cname);
                }
                15 => { // Type MX
                    if rdlength >= 2 {
                        let preference = ((packet[pos] as u16) << 8) | (packet[pos + 1] as u16);
                        let mut temp_pos = pos + 2;
                        let mail_exchange = parse_name(packet, &mut temp_pos)?;
                        results.push(format!("(Preference: {}) {}", preference, mail_exchange));
                    }
                }
                16 => { // Type TXT
                    let mut text = String::new();
                    let mut txt_pos = pos;
                    while txt_pos < rd_end {
                        let txt_len = packet[txt_pos] as usize;
                        txt_pos += 1;
                        if txt_pos + txt_len > rd_end {
                            break;
                        }
                        text.push_str(&String::from_utf8_lossy(&packet[txt_pos .. txt_pos + txt_len]));
                        txt_pos += txt_len;
                    }
                    results.push(text);
                }
                _ => {}
            }
        }
        pos = rd_end;
    }
    
    Ok(results)
}

fn query_dns_type(host: &str, qtype: u16, server: &str) -> Result<Vec<String>, String> {
    let is_ipv6 = server.starts_with('[') || server.chars().filter(|&c| c == ':').count() > 1;
    let bind_addr = if is_ipv6 { "[::]:0" } else { "0.0.0.0:0" };
    
    let socket = UdpSocket::bind(bind_addr).map_err(|e| e.to_string())?;
    socket.set_read_timeout(Some(std::time::Duration::from_secs(2))).map_err(|e| e.to_string())?;
    
    let query = build_dns_query(host, qtype)?;
    
    let remote_addr = if is_ipv6 && !server.starts_with('[') {
        if let Some(last_colon) = server.rfind(':') {
            let ip = &server[..last_colon];
            let port = &server[last_colon+1..];
            format!("[{}]:{}", ip, port)
        } else {
            format!("[{}]:53", server)
        }
    } else {
        server.to_string()
    };

    socket.send_to(&query, &remote_addr).map_err(|e| e.to_string())?;
    
    let mut response = vec![0u8; 512];
    let (len, _) = socket.recv_from(&mut response).map_err(|e| e.to_string())?;
    
    parse_dns_response(&response[..len], qtype)
}

fn get_system_dns_servers() -> Vec<String> {
    let mut servers = Vec::new();

    #[cfg(target_os = "windows")]
    {
        use std::ptr;
        use windows_sys::Win32::NetworkManagement::IpHelper::{
            GetAdaptersAddresses, IP_ADAPTER_ADDRESSES_LH,
        };

        unsafe {
            let mut size = 15000;
            let mut buf = vec![0u8; size as usize];
            
            // Do NOT skip DNS servers
            let flags = 0x0002 | 0x0004; // GAA_FLAG_SKIP_ANYCAST | GAA_FLAG_SKIP_MULTICAST
            
            let mut ret = GetAdaptersAddresses(
                0, // AF_UNSPEC (both IPv4 and IPv6)
                flags,
                ptr::null(),
                buf.as_mut_ptr() as *mut _,
                &mut size,
            );
            
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
            
            if ret == 0 {
                let mut curr = buf.as_ptr() as *const IP_ADAPTER_ADDRESSES_LH;
                while !curr.is_null() {
                    let adapter = &*curr;
                    if adapter.OperStatus == 1 {
                        let mut dns_addr = adapter.FirstDnsServerAddress;
                        while !dns_addr.is_null() {
                            let dns = &*dns_addr;
                            let sockaddr = dns.Address.lpSockaddr;
                            if !sockaddr.is_null() {
                                let family = (*sockaddr).sa_family;
                                if family == 2 { // AF_INET
                                    let ptr = (sockaddr as *const u8).add(4);
                                    let ip_str = format!("{}.{}.{}.{}", *ptr, *ptr.add(1), *ptr.add(2), *ptr.add(3));
                                    servers.push(format!("{}:53", ip_str));
                                } else if family == 23 { // AF_INET6
                                    let ptr = (sockaddr as *const u8).add(8);
                                    let mut parts = Vec::new();
                                    for i in 0..8 {
                                        let word = ((*ptr.add(i*2) as u16) << 8) | (*ptr.add(i*2 + 1) as u16);
                                        parts.push(format!("{:x}", word));
                                    }
                                    servers.push(format!("[{}]:53", parts.join(":")));
                                }
                            }
                            dns_addr = dns.Next;
                        }
                    }
                    curr = adapter.Next;
                }
            }
        }
    }

    #[cfg(target_os = "linux")]
    {
        use std::io::BufRead;
        if let Ok(file) = std::fs::File::open("/etc/resolv.conf") {
            let reader = std::io::BufReader::new(file);
            for line in reader.lines().map_while(Result::ok) {
                let trimmed = line.trim();
                if trimmed.starts_with("nameserver ") {
                    let ip = trimmed["nameserver ".len()..].trim();
                    if !ip.is_empty() {
                        servers.push(format!("{}:53", ip));
                    }
                }
            }
        }
    }

    // Add public resolvers as fallbacks
    servers.push("1.1.1.1:53".to_string());
    servers.push("8.8.8.8:53".to_string());
    
    servers
}

pub fn run_dns(host: &str) {
    // Validate host first
    for label in host.split('.') {
        if label.is_empty() {
            eprintln!("Error: Invalid host name (contains empty labels)");
            std::process::exit(1);
        }
        if label.len() > 63 {
            eprintln!("Error: Label too long (maximum 63 characters)");
            std::process::exit(1);
        }
    }

    let servers = get_system_dns_servers();
    let mut resolved_any = false;

    println!("Querying DNS records for '{}' using servers: {:?}...", host, servers);

    let queries = [
        ("A (IPv4)", 1),
        ("AAAA (IPv6)", 28),
        ("CNAME", 5),
        ("MX (Mail Server)", 15),
        ("TXT (Text)", 16),
    ];

    for (label, qtype) in &queries {
        let mut results = Err("Failed to connect".to_string());
        for server in &servers {
            results = query_dns_type(host, *qtype, server);
            if results.is_ok() {
                break;
            }
        }

        match results {
            Ok(ref records) if !records.is_empty() => {
                resolved_any = true;
                println!("\n{}:", label);
                for record in records {
                    println!("  {}", record);
                }
            }
            _ => {}
        }
    }

    if !resolved_any {
        eprintln!("\nError: Failed to resolve records for host '{}' or no records found.", host);
        std::process::exit(1);
    }
}
