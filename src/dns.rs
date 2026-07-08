use std::net::UdpSocket;
use crate::DnsOptions;

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

#[derive(Clone)]
struct DnsRecord {
    name: String,
    rtype: u16,
    _rclass: u16,
    _ttl: u32,
    rdata_offset: usize,
    rdata_len: usize,
}

fn parse_record(packet: &[u8], pos: &mut usize) -> Result<DnsRecord, String> {
    let name = parse_name(packet, pos)?;
    if *pos + 10 > packet.len() {
        return Err("Record header goes out of bounds".to_string());
    }
    let rtype = ((packet[*pos] as u16) << 8) | (packet[*pos + 1] as u16);
    let rclass = ((packet[*pos + 2] as u16) << 8) | (packet[*pos + 3] as u16);
    let ttl = ((packet[*pos + 4] as u32) << 24) | ((packet[*pos + 5] as u32) << 16) | ((packet[*pos + 6] as u32) << 8) | (packet[*pos + 7] as u32);
    let rdlength = ((packet[*pos + 8] as u16) << 8) | (packet[*pos + 9] as u16);
    *pos += 10;
    
    let rdata_offset = *pos;
    if *pos + (rdlength as usize) > packet.len() {
        return Err("Record data goes out of bounds".to_string());
    }
    *pos += rdlength as usize;
    
    Ok(DnsRecord {
        name,
        rtype,
        _rclass: rclass,
        _ttl: ttl,
        rdata_offset,
        rdata_len: rdlength as usize,
    })
}

struct ParsedDnsPacket {
    raw_packet: Vec<u8>,
    answers: Vec<DnsRecord>,
    authorities: Vec<DnsRecord>,
    additionals: Vec<DnsRecord>,
}

fn parse_dns_packet(packet: &[u8]) -> Result<ParsedDnsPacket, String> {
    if packet.len() < 12 {
        return Err("Packet too short".to_string());
    }
    let qdcount = ((packet[4] as u16) << 8) | (packet[5] as u16);
    let ancount = ((packet[6] as u16) << 8) | (packet[7] as u16);
    let nscount = ((packet[8] as u16) << 8) | (packet[9] as u16);
    let arcount = ((packet[10] as u16) << 8) | (packet[11] as u16);
    
    let mut pos = 12;
    for _ in 0..qdcount {
        parse_name(packet, &mut pos)?;
        pos += 4;
    }
    
    let mut answers = Vec::new();
    for _ in 0..ancount {
        answers.push(parse_record(packet, &mut pos)?);
    }
    
    let mut authorities = Vec::new();
    for _ in 0..nscount {
        authorities.push(parse_record(packet, &mut pos)?);
    }
    
    let mut additionals = Vec::new();
    for _ in 0..arcount {
        additionals.push(parse_record(packet, &mut pos)?);
    }
    
    Ok(ParsedDnsPacket {
        raw_packet: packet.to_vec(),
        answers,
        authorities,
        additionals,
    })
}

fn get_record_string(packet: &[u8], rec: &DnsRecord) -> Result<String, String> {
    let pos = rec.rdata_offset;
    let len = rec.rdata_len;
    match rec.rtype {
        1 => { // A
            if len == 4 {
                Ok(format!("{}.{}.{}.{}", packet[pos], packet[pos+1], packet[pos+2], packet[pos+3]))
            } else {
                Err("Invalid A record length".to_string())
            }
        }
        28 => { // AAAA
            if len == 16 {
                let mut parts = Vec::new();
                for i in 0..8 {
                    let word = ((packet[pos + i*2] as u16) << 8) | (packet[pos + i*2 + 1] as u16);
                    parts.push(format!("{:x}", word));
                }
                Ok(parts.join(":"))
            } else {
                Err("Invalid AAAA record length".to_string())
            }
        }
        2 | 5 | 12 => { // NS, CNAME, PTR
            let mut temp_pos = pos;
            parse_name(packet, &mut temp_pos)
        }
        15 => { // MX
            if len >= 2 {
                let preference = ((packet[pos] as u16) << 8) | (packet[pos + 1] as u16);
                let mut temp_pos = pos + 2;
                let exchange = parse_name(packet, &mut temp_pos)?;
                Ok(format!("(Preference: {}) {}", preference, exchange))
            } else {
                Err("Invalid MX record length".to_string())
            }
        }
        16 => { // TXT
            let mut text = String::new();
            let mut txt_pos = pos;
            let rd_end = pos + len;
            while txt_pos < rd_end {
                let txt_len = packet[txt_pos] as usize;
                txt_pos += 1;
                if txt_pos + txt_len > rd_end {
                    break;
                }
                text.push_str(&String::from_utf8_lossy(&packet[txt_pos .. txt_pos + txt_len]));
                txt_pos += txt_len;
            }
            Ok(text)
        }
        6 => { // SOA
            let mut temp_pos = pos;
            let mname = parse_name(packet, &mut temp_pos)?;
            let rname = parse_name(packet, &mut temp_pos)?;
            let rd_end = pos + len;
            if temp_pos + 20 <= rd_end {
                let serial = ((packet[temp_pos] as u32) << 24) | ((packet[temp_pos+1] as u32) << 16) | ((packet[temp_pos+2] as u32) << 8) | (packet[temp_pos+3] as u32);
                let refresh = ((packet[temp_pos+4] as u32) << 24) | ((packet[temp_pos+5] as u32) << 16) | ((packet[temp_pos+6] as u32) << 8) | (packet[temp_pos+7] as u32);
                let retry = ((packet[temp_pos+8] as u32) << 24) | ((packet[temp_pos+9] as u32) << 16) | ((packet[temp_pos+10] as u32) << 8) | (packet[temp_pos+11] as u32);
                let expire = ((packet[temp_pos+12] as u32) << 24) | ((packet[temp_pos+13] as u32) << 16) | ((packet[temp_pos+14] as u32) << 8) | (packet[temp_pos+15] as u32);
                let minimum = ((packet[temp_pos+16] as u32) << 24) | ((packet[temp_pos+17] as u32) << 16) | ((packet[temp_pos+18] as u32) << 8) | (packet[temp_pos+19] as u32);
                Ok(format!(
                    "Primary NS: {}, Admin Mailbox: {}, Serial: {}, Refresh: {}, Retry: {}, Expire: {}, Minimum TTL: {}",
                    mname, rname, serial, refresh, retry, expire, minimum
                ))
            } else {
                Err("Invalid SOA record length".to_string())
            }
        }
        _ => Ok(format!("Unsupported type {}", rec.rtype)),
    }
}

fn query_dns_raw(host: &str, qtype: u16, server: &str) -> Result<ParsedDnsPacket, String> {
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
        if server.contains(':') {
            server.to_string()
        } else {
            format!("{}:53", server)
        }
    };

    socket.send_to(&query, &remote_addr).map_err(|e| e.to_string())?;
    
    let mut response = vec![0u8; 512];
    let (len, _) = socket.recv_from(&mut response).map_err(|e| e.to_string())?;
    
    parse_dns_packet(&response[..len])
}

fn get_reverse_ip_domain(ip_str: &str) -> Result<String, String> {
    use std::net::IpAddr;
    let ip: IpAddr = ip_str.parse().map_err(|_| format!("Invalid IP address '{}' for reverse lookup", ip_str))?;
    match ip {
        IpAddr::V4(ipv4) => {
            let octets = ipv4.octets();
            Ok(format!("{}.{}.{}.{}.in-addr.arpa", octets[3], octets[2], octets[1], octets[0]))
        }
        IpAddr::V6(ipv6) => {
            let segments = ipv6.segments();
            let mut nibbles = Vec::new();
            for &seg in segments.iter().rev() {
                for i in 0..4 {
                    let nibble = (seg >> (i * 4)) & 0x0F;
                    nibbles.push(format!("{:x}", nibble));
                }
            }
            Ok(format!("{}.ip6.arpa", nibbles.join(".")))
        }
    }
}

fn record_type_str_to_u16(t: &str) -> u16 {
    match t {
        "A" => 1,
        "NS" => 2,
        "CNAME" => 5,
        "SOA" => 6,
        "MX" => 15,
        "TXT" => 16,
        "AAAA" => 28,
        "PTR" => 12,
        _ => 1,
    }
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
            let flags = 0x0002 | 0x0004;
            
            let mut ret = GetAdaptersAddresses(0, flags, ptr::null(), buf.as_mut_ptr() as *mut _, &mut size);
            if ret == 111 {
                buf.resize(size as usize, 0);
                ret = GetAdaptersAddresses(0, flags, ptr::null(), buf.as_mut_ptr() as *mut _, &mut size);
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
                                if family == 2 {
                                    let ptr = (sockaddr as *const u8).add(4);
                                    servers.push(format!("{}.{}.{}.{}", *ptr, *ptr.add(1), *ptr.add(2), *ptr.add(3)));
                                } else if family == 23 {
                                    let ptr = (sockaddr as *const u8).add(8);
                                    let mut parts = Vec::new();
                                    for i in 0..8 {
                                        let word = ((*ptr.add(i*2) as u16) << 8) | (*ptr.add(i*2 + 1) as u16);
                                        parts.push(format!("{:x}", word));
                                    }
                                    servers.push(parts.join(":"));
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
                        servers.push(ip.to_string());
                    }
                }
            }
        }
    }

    servers.push("1.1.1.1".to_string());
    servers.push("8.8.8.8".to_string());
    servers
}

fn query_system_resolve(host: &str, qtype: u16) -> Result<String, String> {
    let servers = get_system_dns_servers();
    for server in &servers {
        if let Ok(packet) = query_dns_raw(host, qtype, server) {
            for ans in packet.answers {
                if ans.rtype == qtype {
                    if let Ok(val) = get_record_string(&packet.raw_packet, &ans) {
                        return Ok(val);
                    }
                }
            }
        }
    }
    Err("Failed to resolve".to_string())
}

fn trace_dns_resolver(host: &str, qtype: u16) -> Result<Vec<String>, String> {
    let mut current_server = "198.41.0.4".to_string(); // a.root-servers.net
    let mut current_zone = ".".to_string();
    let mut step = 1;
    let mut iterations = 0;

    println!("Tracing delegation path for '{}' (type {}):", host, qtype);

    loop {
        iterations += 1;
        if iterations > 15 {
            return Err("Trace exceeded maximum iterations limit (15)".to_string());
        }

        println!("\nStep {}: Querying {} nameserver '{}' for '{}'...", step, current_zone, current_server, host);
        step += 1;

        let res = query_dns_raw(host, qtype, &current_server);
        match res {
            Ok(packet) => {
                // If we got answers, trace is finished!
                let mut answers = Vec::new();
                for ans in &packet.answers {
                    if ans.rtype == qtype {
                        if let Ok(val) = get_record_string(&packet.raw_packet, ans) {
                            answers.push(val);
                        }
                    }
                }
                if !answers.is_empty() {
                    println!("  Authoritative Answer Received from '{}'!", current_server);
                    return Ok(answers);
                }

                // If no answers but we have CNAME, print and resolve target instead
                for ans in &packet.answers {
                    if ans.rtype == 5 { // CNAME
                        if let Ok(cname_val) = get_record_string(&packet.raw_packet, ans) {
                            println!("  CNAME Referral: {} -> {}", host, cname_val);
                            return trace_dns_resolver(&cname_val, qtype);
                        }
                    }
                }

                // If no answers, check authority section for delegation records (NS)
                if packet.authorities.is_empty() {
                    return Err("No authority or answers received. Resolution failed.".to_string());
                }

                let mut ns_records = Vec::new();
                for auth in &packet.authorities {
                    if auth.rtype == 2 { // NS
                        if let Ok(ns_name) = get_record_string(&packet.raw_packet, auth) {
                            ns_records.push(ns_name);
                            if auth.name != "." {
                                current_zone = auth.name.clone();
                            }
                        }
                    }
                }

                if ns_records.is_empty() {
                    return Err("No NS authority records found in delegation.".to_string());
                }

                // Try to match NS name to IP from additional section
                let mut next_server = None;
                for ns in &ns_records {
                    for add in &packet.additionals {
                        if add.name.eq_ignore_ascii_case(ns) && (add.rtype == 1 || add.rtype == 28) {
                            if let Ok(ip) = get_record_string(&packet.raw_packet, add) {
                                println!("  Referral NS: '{}' -> IP: {}", ns, ip);
                                next_server = Some(ip);
                                break;
                            }
                        }
                    }
                    if next_server.is_some() {
                        break;
                    }
                }

                // If no glue records in Additional section, resolve one of the NS servers using system resolver
                if next_server.is_none() {
                    let ns_target = &ns_records[0];
                    println!("  Referral NS: '{}' (No glue record in Additional. Resolving via system...)", ns_target);
                    if let Ok(resolved_ip) = query_system_resolve(ns_target, 1) {
                        println!("    Resolved '{}' to IP: {}", ns_target, resolved_ip);
                        next_server = Some(resolved_ip);
                    } else if let Ok(resolved_ip6) = query_system_resolve(ns_target, 28) {
                        println!("    Resolved '{}' to IP: {}", ns_target, resolved_ip6);
                        next_server = Some(resolved_ip6);
                    }
                }

                if let Some(ip) = next_server {
                    current_server = ip;
                } else {
                    return Err(format!("Failed to resolve IP for any delegation NS servers: {:?}", ns_records));
                }
            }
            Err(e) => {
                return Err(format!("Query failed to nameserver {}: {}", current_server, e));
            }
        }
    }
}

pub fn run_dns(options: DnsOptions) {
    let host = if options.reverse {
        match get_reverse_ip_domain(&options.host) {
            Ok(rev) => rev,
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        options.host.clone()
    };

    // Validate host QNAME labels
    for label in host.split('.') {
        if label.is_empty() {
            eprintln!("Error: Invalid host name '{}' (contains empty labels)", options.host);
            std::process::exit(1);
        }
        if label.len() > 63 {
            eprintln!("Error: Label too long (maximum 63 characters)");
            std::process::exit(1);
        }
    }

    let qtype = if options.reverse {
        12 // PTR
    } else {
        record_type_str_to_u16(&options.record_type)
    };

    if options.trace {
        match trace_dns_resolver(&host, qtype) {
            Ok(ref records) if !records.is_empty() => {
                println!("\nTrace Complete. Found records:");
                for record in records {
                    println!("  {}", record);
                }
            }
            Ok(_) => {
                eprintln!("\nError: Trace completed but no matching records found.");
                std::process::exit(1);
            }
            Err(e) => {
                eprintln!("\nError during trace: {}", e);
                std::process::exit(1);
            }
        }
        return;
    }

    // Normal query mode
    let servers = if let Some(ref custom_server) = options.server {
        vec![custom_server.clone()]
    } else {
        get_system_dns_servers()
    };

    let mut resolved_any = false;
    let query_types = if options.reverse {
        vec![("PTR", 12)]
    } else if options.record_type == "ANY" {
        vec![
            ("A (IPv4)", 1),
            ("AAAA (IPv6)", 28),
            ("CNAME", 5),
            ("NS (Name Server)", 2),
            ("MX (Mail Server)", 15),
            ("TXT (Text)", 16),
            ("SOA (Start of Authority)", 6),
        ]
    } else {
        vec![(options.record_type.as_str(), qtype)]
    };

    if !options.short && options.server.is_none() {
        println!("Querying DNS records for '{}' using system servers: {:?}...", options.host, servers);
    } else if !options.short {
        println!("Querying DNS records for '{}' using custom server: {:?}...", options.host, servers);
    }

    for (label, qt) in query_types {
        let mut response = Err("No servers available".to_string());
        for server in &servers {
            response = query_dns_raw(&host, qt, server);
            if response.is_ok() {
                break;
            }
        }

        match response {
            Ok(packet) => {
                let mut printed_header = false;
                for ans in &packet.answers {
                    if ans.rtype == qt {
                        if let Ok(record_str) = get_record_string(&packet.raw_packet, ans) {
                            resolved_any = true;
                            if options.short {
                                println!("{}", record_str);
                            } else {
                                if !printed_header {
                                    println!("\n{}:", label);
                                    printed_header = true;
                                }
                                println!("  {}", record_str);
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    if !resolved_any {
        if !options.short {
            eprintln!("\nError: Failed to resolve records for host '{}' or no records found.", options.host);
        }
        std::process::exit(1);
    }
}
