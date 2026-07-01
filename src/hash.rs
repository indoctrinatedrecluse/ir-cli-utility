use crate::HashOptions;
use std::fs::File;
use std::io::{Read, BufReader, BufRead};
use sha1::Sha1;
use sha2::{Sha256, Sha512, Digest};

fn compute_hash(file_path: &str, algorithm: &str) -> Result<String, std::io::Error> {
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = [0u8; 65536]; // 64KB buffer

    let algo = algorithm.to_lowercase();
    match algo.as_str() {
        "md5" => {
            let mut context = md5::Context::new();
            loop {
                let bytes_read = reader.read(&mut buffer)?;
                if bytes_read == 0 { break; }
                context.consume(&buffer[..bytes_read]);
            }
            let digest = context.compute();
            Ok(format!("{:x}", digest))
        }
        "sha1" => {
            let mut hasher = Sha1::new();
            loop {
                let bytes_read = reader.read(&mut buffer)?;
                if bytes_read == 0 { break; }
                hasher.update(&buffer[..bytes_read]);
            }
            let result = hasher.finalize();
            Ok(format!("{:x}", result))
        }
        "sha512" => {
            let mut hasher = Sha512::new();
            loop {
                let bytes_read = reader.read(&mut buffer)?;
                if bytes_read == 0 { break; }
                hasher.update(&buffer[..bytes_read]);
            }
            let result = hasher.finalize();
            Ok(format!("{:x}", result))
        }
        _ => {
            // Default to sha256
            let mut hasher = Sha256::new();
            loop {
                let bytes_read = reader.read(&mut buffer)?;
                if bytes_read == 0 { break; }
                hasher.update(&buffer[..bytes_read]);
            }
            let result = hasher.finalize();
            Ok(format!("{:x}", result))
        }
    }
}

pub fn hash(file_path: &str, options: HashOptions) {
    let algo = if options.algorithm.is_empty() {
        "sha256".to_string()
    } else {
        options.algorithm.to_lowercase()
    };

    if options.checksum_file {
        // Read checksum file and verify each line
        let file = match File::open(file_path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Error: Failed to open checksum file '{}': {}", file_path, e);
                std::process::exit(1);
            }
        };

        let reader = BufReader::new(file);
        let mut failed_count = 0;
        let mut success_count = 0;

        for (line_num, line_result) in reader.lines().enumerate() {
            let line = match line_result {
                Ok(l) => l,
                Err(e) => {
                    eprintln!("Error reading checksum file at line {}: {}", line_num + 1, e);
                    failed_count += 1;
                    continue;
                }
            };

            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue; // Skip comments and empty lines
            }

            // Parse: expected_hash  file_path
            // Can be separated by spaces, or check for ' *'
            let parts: Vec<&str> = trimmed.splitn(2, |c: char| c.is_whitespace()).collect();
            if parts.len() < 2 {
                eprintln!("Error: Invalid line format in checksum file at line {}: '{}'", line_num + 1, trimmed);
                failed_count += 1;
                continue;
            }

            let expected_hash = parts[0].trim();
            // File path might start with * (indicating binary mode in sha256sum)
            let mut target_path = parts[1].trim();
            if target_path.starts_with('*') {
                target_path = &target_path[1..];
            }

            // Compute hash of the target file
            match compute_hash(target_path, &algo) {
                Ok(computed) => {
                    if computed.eq_ignore_ascii_case(expected_hash) {
                        println!("{}: OK", target_path);
                        success_count += 1;
                    } else {
                        println!("{}: FAILED", target_path);
                        failed_count += 1;
                    }
                }
                Err(e) => {
                    println!("{}: FAILED (Could not open/read file: {})", target_path, e);
                    failed_count += 1;
                }
            }
        }

        if failed_count > 0 {
            eprintln!("ir-hash: WARNING: {} computed checksums did NOT match", failed_count);
            std::process::exit(1);
        } else if success_count == 0 {
            eprintln!("ir-hash: WARNING: No valid checksum lines found in '{}'", file_path);
        }
    } else if let Some(ref expected) = options.verify {
        // Verify a single file
        match compute_hash(file_path, &algo) {
            Ok(computed) => {
                if computed.eq_ignore_ascii_case(expected.trim()) {
                    println!("{}: OK", file_path);
                } else {
                    println!("{}: FAILED", file_path);
                    eprintln!("Expected: {}\nComputed: {}", expected, computed);
                    std::process::exit(1);
                }
            }
            Err(e) => {
                eprintln!("Error: Failed to hash '{}': {}", file_path, e);
                std::process::exit(1);
            }
        }
    } else {
        // Just print the hash
        match compute_hash(file_path, &algo) {
            Ok(computed) => {
                println!("{}  {}", computed, file_path);
            }
            Err(e) => {
                eprintln!("Error: Failed to hash '{}': {}", file_path, e);
                std::process::exit(1);
            }
        }
    }
}
