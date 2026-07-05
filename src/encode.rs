use crate::{EncodeOptions, DecodeOptions};
use base64::Engine;
use std::fs::File;
use std::io::{Read, Write};

pub fn run_encode(input_path: Option<&str>, options: EncodeOptions) {
    let input_bytes = read_input(input_path);
    let format = options.format.to_lowercase();

    let output_bytes = match format.as_str() {
        "base64" => {
            let engine = if options.no_padding {
                &base64::prelude::BASE64_STANDARD_NO_PAD
            } else {
                &base64::prelude::BASE64_STANDARD
            };
            engine.encode(&input_bytes).into_bytes()
        }
        "base64url" => {
            let engine = if options.no_padding {
                &base64::prelude::BASE64_URL_SAFE_NO_PAD
            } else {
                &base64::prelude::BASE64_URL_SAFE
            };
            engine.encode(&input_bytes).into_bytes()
        }
        "base32" => {
            base32_encode(&input_bytes, options.no_padding).into_bytes()
        }
        "hex" | "base16" => {
            hex_encode(&input_bytes, options.hex_upper, options.hex_separator.as_deref()).into_bytes()
        }
        "url" => {
            url_encode(&input_bytes, options.url_encode_all).into_bytes()
        }
        "rot13" => {
            rot13(&input_bytes)
        }
        other => {
            eprintln!("Error: Unknown encode format '{}'", other);
            std::process::exit(1);
        }
    };

    write_output(&output_bytes, options.output.as_deref(), true);
}

pub fn run_decode(input_path: Option<&str>, options: DecodeOptions) {
    let input_bytes = read_input(input_path);
    let format = options.format.to_lowercase();

    let output_bytes = match format.as_str() {
        "base64" => {
            let engine = if options.no_padding {
                &base64::prelude::BASE64_STANDARD_NO_PAD
            } else {
                &base64::prelude::BASE64_STANDARD
            };
            let cleaned = clean_whitespace(&input_bytes);
            match engine.decode(cleaned.as_bytes()) {
                Ok(bytes) => bytes,
                Err(e) => {
                    eprintln!("Error: Base64 decode failed: {}", e);
                    std::process::exit(1);
                }
            }
        }
        "base64url" => {
            let engine = if options.no_padding {
                &base64::prelude::BASE64_URL_SAFE_NO_PAD
            } else {
                &base64::prelude::BASE64_URL_SAFE
            };
            let cleaned = clean_whitespace(&input_bytes);
            match engine.decode(cleaned.as_bytes()) {
                Ok(bytes) => bytes,
                Err(e) => {
                    eprintln!("Error: Base64Url decode failed: {}", e);
                    std::process::exit(1);
                }
            }
        }
        "base32" => {
            let input_str = String::from_utf8_lossy(&input_bytes);
            match base32_decode(&input_str) {
                Ok(bytes) => bytes,
                Err(e) => {
                    eprintln!("Error: Base32 decode failed: {}", e);
                    std::process::exit(1);
                }
            }
        }
        "hex" | "base16" => {
            let input_str = String::from_utf8_lossy(&input_bytes);
            match hex_decode(&input_str, options.hex_separator.as_deref()) {
                Ok(bytes) => bytes,
                Err(e) => {
                    eprintln!("Error: Hex decode failed: {}", e);
                    std::process::exit(1);
                }
            }
        }
        "url" => {
            let input_str = String::from_utf8_lossy(&input_bytes);
            match url_decode(&input_str) {
                Ok(bytes) => bytes,
                Err(e) => {
                    eprintln!("Error: URL decode failed: {}", e);
                    std::process::exit(1);
                }
            }
        }
        "rot13" => {
            rot13(&input_bytes)
        }
        other => {
            eprintln!("Error: Unknown decode format '{}'", other);
            std::process::exit(1);
        }
    };

    write_output(&output_bytes, options.output.as_deref(), false);
}

// ---------------------------------------------------------------------------
// Helper functions
// ---------------------------------------------------------------------------

fn read_input(input_path: Option<&str>) -> Vec<u8> {
    if let Some(path) = input_path {
        let mut file = match File::open(path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Error: Failed to open input file '{}': {}", path, e);
                std::process::exit(1);
            }
        };
        let mut buf = Vec::new();
        if let Err(e) = file.read_to_end(&mut buf) {
            eprintln!("Error: Failed to read input file: {}", e);
            std::process::exit(1);
        }
        buf
    } else {
        let mut buf = Vec::new();
        if let Err(e) = std::io::stdin().read_to_end(&mut buf) {
            eprintln!("Error: Failed to read from stdin: {}", e);
            std::process::exit(1);
        }
        buf
    }
}

fn write_output(bytes: &[u8], output_path: Option<&str>, is_encode: bool) {
    if let Some(path) = output_path {
        let mut file = match File::create(path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Error: Failed to create output file '{}': {}", path, e);
                std::process::exit(1);
            }
        };
        if let Err(e) = file.write_all(bytes) {
            eprintln!("Error: Failed to write output file: {}", e);
            std::process::exit(1);
        }
    } else {
        let mut stdout = std::io::stdout();
        if let Err(e) = stdout.write_all(bytes) {
            eprintln!("Error: Failed to write to stdout: {}", e);
            std::process::exit(1);
        }
        // Print trailing newline only when encoding to standard stdout (non-file)
        if is_encode {
            println!();
        }
    }
}

fn clean_whitespace(bytes: &[u8]) -> String {
    let s = String::from_utf8_lossy(bytes);
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

// ---------------------------------------------------------------------------
// Codecs implementation
// ---------------------------------------------------------------------------

fn hex_encode(bytes: &[u8], upper: bool, separator: Option<&str>) -> String {
    let mut parts = Vec::with_capacity(bytes.len());
    for &b in bytes {
        parts.push(if upper {
            format!("{:02X}", b)
        } else {
            format!("{:02x}", b)
        });
    }
    parts.join(separator.unwrap_or(""))
}

fn hex_decode(s: &str, separator: Option<&str>) -> Result<Vec<u8>, String> {
    let mut cleaned = s.to_string();
    if let Some(sep) = separator {
        if !sep.is_empty() {
            cleaned = cleaned.replace(sep, "");
        }
    }
    cleaned = cleaned.chars().filter(|c| !c.is_whitespace()).collect();
    if cleaned.len() % 2 != 0 {
        return Err("Hex string must have an even length".to_string());
    }

    let mut bytes = Vec::with_capacity(cleaned.len() / 2);
    for i in (0..cleaned.len()).step_by(2) {
        let byte_str = &cleaned[i..i + 2];
        match u8::from_str_radix(byte_str, 16) {
            Ok(b) => bytes.push(b),
            Err(_) => return Err(format!("Invalid hex byte: '{}'", byte_str)),
        }
    }
    Ok(bytes)
}

fn url_encode(bytes: &[u8], encode_all: bool) -> String {
    let mut result = String::new();
    for &b in bytes {
        let c = b as char;
        if !encode_all && (c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.' || c == '~') {
            result.push(c);
        } else {
            result.push_str(&format!("%{:02X}", b));
        }
    }
    result
}

fn url_decode(s: &str) -> Result<Vec<u8>, String> {
    let mut bytes = Vec::new();
    let mut chars = s.as_bytes().iter().copied().peekable();
    while let Some(b) = chars.next() {
        if b == b'%' {
            let mut hex_buf = Vec::new();
            if let Some(h1) = chars.next() { hex_buf.push(h1); }
            if let Some(h2) = chars.next() { hex_buf.push(h2); }
            if hex_buf.len() != 2 {
                return Err("Invalid percent-encoded sequence: truncated %".to_string());
            }
            let hex_str = std::str::from_utf8(&hex_buf).map_err(|_| "Invalid UTF-8 in percent encoding")?;
            let val = u8::from_str_radix(hex_str, 16)
                .map_err(|_| format!("Invalid hex byte in percent encoding: '%{}'", hex_str))?;
            bytes.push(val);
        } else if b == b'+' {
            bytes.push(b' '); // URL space decoder helper
        } else {
            bytes.push(b);
        }
    }
    Ok(bytes)
}

fn base32_encode(bytes: &[u8], no_padding: bool) -> String {
    let alphabet = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
    let mut result = String::new();
    let mut buffer: u64 = 0;
    let mut bits = 0;
    for &b in bytes {
        buffer = (buffer << 8) | (b as u64);
        bits += 8;
        while bits >= 5 {
            bits -= 5;
            let idx = ((buffer >> bits) & 0x1F) as usize;
            result.push(alphabet[idx] as char);
        }
    }
    if bits > 0 {
        let idx = ((buffer << (5 - bits)) & 0x1F) as usize;
        result.push(alphabet[idx] as char);
    }
    if !no_padding {
        while result.len() % 8 != 0 {
            result.push('=');
        }
    }
    result
}

fn base32_decode(s: &str) -> Result<Vec<u8>, String> {
    let alphabet = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
    let cleaned: String = s.chars()
        .filter(|&c| c != '=' && !c.is_whitespace())
        .map(|c| c.to_ascii_uppercase())
        .collect();

    let mut bytes = Vec::new();
    let mut buffer: u64 = 0;
    let mut bits = 0;
    for c in cleaned.chars() {
        let val = match alphabet.iter().position(|&x| x == c as u8) {
            Some(pos) => pos as u64,
            None => return Err(format!("Invalid Base32 character: '{}'", c)),
        };
        buffer = (buffer << 5) | val;
        bits += 5;
        if bits >= 8 {
            bits -= 8;
            bytes.push((buffer >> bits) as u8);
        }
    }
    Ok(bytes)
}

fn rot13(bytes: &[u8]) -> Vec<u8> {
    bytes.iter().map(|&b| {
        if b >= b'a' && b <= b'z' {
            b'a' + (b - b'a' + 13) % 26
        } else if b >= b'A' && b <= b'Z' {
            b'A' + (b - b'A' + 13) % 26
        } else {
            b
        }
    }).collect()
}
