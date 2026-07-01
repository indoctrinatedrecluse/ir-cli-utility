use crate::Base64Options;
use base64::Engine;
use std::fs::File;
use std::io::{Read, Write};

pub fn run_base64(input_path: Option<&str>, options: Base64Options) {
    // Select the engine based on URL-safe and padding options
    let engine = if options.url {
        if options.no_padding {
            &base64::prelude::BASE64_URL_SAFE_NO_PAD
        } else {
            &base64::prelude::BASE64_URL_SAFE
        }
    } else {
        if options.no_padding {
            &base64::prelude::BASE64_STANDARD_NO_PAD
        } else {
            &base64::prelude::BASE64_STANDARD
        }
    };

    // Read input bytes
    let input_bytes = if let Some(path) = input_path {
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
        // Read from standard input
        let mut buf = Vec::new();
        if let Err(e) = std::io::stdin().read_to_end(&mut buf) {
            eprintln!("Error: Failed to read from stdin: {}", e);
            std::process::exit(1);
        }
        buf
    };

    // Process encode/decode
    let output_bytes = if options.decode {
        // Trim whitespace for base64 decoding
        let input_str = String::from_utf8_lossy(&input_bytes);
        let cleaned_str: String = input_str.chars().filter(|c| !c.is_whitespace()).collect();
        match engine.decode(cleaned_str.as_bytes()) {
            Ok(bytes) => bytes,
            Err(e) => {
                eprintln!("Error: Base64 decode failed: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        // Encode
        engine.encode(&input_bytes).into_bytes()
    };

    // Write output bytes
    if let Some(ref out_path) = options.output {
        let mut file = match File::create(out_path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Error: Failed to create output file '{}': {}", out_path, e);
                std::process::exit(1);
            }
        };
        if let Err(e) = file.write_all(&output_bytes) {
            eprintln!("Error: Failed to write to output file: {}", e);
            std::process::exit(1);
        }
    } else {
        // Write to stdout
        let mut stdout = std::io::stdout();
        if let Err(e) = stdout.write_all(&output_bytes) {
            eprintln!("Error: Failed to write to stdout: {}", e);
            std::process::exit(1);
        }
        // Print trailing newline only if encoding and output is to a tty/stdout
        if !options.decode {
            println!();
        }
    }
}
