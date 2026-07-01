use crate::HexOptions;
use std::fs::File;
use std::io::{Read, BufReader};

pub fn hex(file_path: &str, options: HexOptions) {
    let file = match File::open(file_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error: Failed to open file '{}': {}", file_path, e);
            std::process::exit(1);
        }
    };

    let mut reader = BufReader::new(file);
    let cols = if options.cols == 0 { 16 } else { options.cols };
    let mut buffer = vec![0u8; cols];
    let mut offset = 0usize;
    let mut total_read = 0usize;

    loop {
        // Calculate how much we should read if limit is set
        let to_read = if let Some(limit) = options.limit {
            if total_read >= limit {
                break;
            }
            let remaining = limit - total_read;
            cols.min(remaining)
        } else {
            cols
        };

        match reader.read(&mut buffer[..to_read]) {
            Ok(0) => break,
            Ok(n) => {
                // Print offset
                print!("{:08x}  ", offset);

                // Print hex bytes
                for i in 0..cols {
                    if i < n {
                        print!("{:02x} ", buffer[i]);
                    } else {
                        print!("   "); // Alignment padding
                    }
                    // Additional spacing at the middle for readability (only if 16 columns)
                    if cols == 16 && i == 7 {
                        print!(" ");
                    }
                }

                print!(" |");

                // Print ASCII representation
                for i in 0..n {
                    let c = buffer[i];
                    if c >= 32 && c <= 126 {
                        print!("{}", c as char);
                    } else {
                        print!(".");
                    }
                }

                println!("|");

                offset += n;
                total_read += n;
            }
            Err(e) => {
                eprintln!("Error reading file: {}", e);
                std::process::exit(1);
            }
        }
    }
}
