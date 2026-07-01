use crate::UuidOptions;

fn get_random_bytes(buf: &mut [u8]) {
    if let Err(e) = getrandom::getrandom(buf) {
        eprintln!("Error: Failed to obtain secure random bytes: {}", e);
        std::process::exit(1);
    }
}

fn generate_uuid_v4() -> [u8; 16] {
    let mut bytes = [0u8; 16];
    get_random_bytes(&mut bytes);
    
    // Set version to 4
    bytes[6] = (bytes[6] & 0x0f) | 0x40;
    // Set variant to RFC 4122
    bytes[8] = (bytes[8] & 0x3f) | 0x80;
    
    bytes
}

fn generate_uuid_v7() -> [u8; 16] {
    let mut bytes = [0u8; 16];
    
    // Obtain Unix epoch timestamp in milliseconds (48-bit)
    let ts_ms = match std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH) {
        Ok(d) => d.as_millis() as u64,
        Err(_) => 0u64,
    };
    
    bytes[0] = ((ts_ms >> 40) & 0xff) as u8;
    bytes[1] = ((ts_ms >> 32) & 0xff) as u8;
    bytes[2] = ((ts_ms >> 24) & 0xff) as u8;
    bytes[3] = ((ts_ms >> 16) & 0xff) as u8;
    bytes[4] = ((ts_ms >> 8) & 0xff) as u8;
    bytes[5] = (ts_ms & 0xff) as u8;
    
    // Fill the remaining 10 bytes with random data
    let mut rand_bytes = [0u8; 10];
    get_random_bytes(&mut rand_bytes);
    bytes[6..16].copy_from_slice(&rand_bytes);
    
    // Set version to 7
    bytes[6] = (bytes[6] & 0x0f) | 0x70;
    // Set variant to RFC 4122
    bytes[8] = (bytes[8] & 0x3f) | 0x80;
    
    bytes
}

fn format_uuid(bytes: &[u8; 16], uppercase: bool, no_hyphens: bool) -> String {
    let hex_chars = if uppercase {
        b"0123456789ABCDEF"
    } else {
        b"0123456789abcdef"
    };

    let mut result = String::with_capacity(36);
    for (i, &byte) in bytes.iter().enumerate() {
        if !no_hyphens && (i == 4 || i == 6 || i == 8 || i == 10) {
            result.push('-');
        }
        let high = (byte >> 4) as usize;
        let low = (byte & 0x0f) as usize;
        result.push(hex_chars[high] as char);
        result.push(hex_chars[low] as char);
    }
    result
}

pub fn run_uuid(options: UuidOptions) {
    let count = if options.count == 0 { 1 } else { options.count };
    
    for _ in 0..count {
        let bytes = if options.version == 7 {
            generate_uuid_v7()
        } else {
            generate_uuid_v4()
        };
        
        let formatted = format_uuid(&bytes, options.uppercase, options.no_hyphens);
        println!("{}", formatted);
    }
}
