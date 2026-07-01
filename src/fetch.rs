use crate::FetchOptions;
use std::fs::File;
use std::io::{Read, Write};

pub fn fetch(url: &str, options: FetchOptions) {
    let method = if options.method.is_empty() {
        "GET"
    } else {
        options.method.as_str()
    };

    let mut request = ureq::request(method, url);

    // Apply custom headers
    for header in &options.headers {
        let parts: Vec<&str> = header.splitn(2, ':').collect();
        if parts.len() == 2 {
            request = request.set(parts[0].trim(), parts[1].trim());
        } else {
            eprintln!("Warning: Ignoring malformed header '{}'", header);
        }
    }

    // Execute the request
    let response = if let Some(ref data_str) = options.data {
        request.send_string(data_str)
    } else {
        request.call()
    };

    // Handle response or status errors (non-2xx responses)
    let (status, status_text, mut reader, header_list) = match response {
        Ok(res) => {
            let status = res.status();
            let status_text = res.status_text().to_string();
            
            // Build header list before moving the response reader
            let mut headers = Vec::new();
            for name in &res.headers_names() {
                if let Some(val) = res.header(name) {
                    headers.push((name.clone(), val.to_string()));
                }
            }
            
            (status, status_text, res.into_reader(), headers)
        }
        Err(ureq::Error::Status(code, res)) => {
            let status_text = res.status_text().to_string();
            let mut headers = Vec::new();
            for name in &res.headers_names() {
                if let Some(val) = res.header(name) {
                    headers.push((name.clone(), val.to_string()));
                }
            }
            (code, status_text, res.into_reader(), headers)
        }
        Err(e) => {
            eprintln!("Error: Request failed: {}", e);
            std::process::exit(1);
        }
    };

    // Print headers if requested
    if options.include_headers {
        println!("HTTP/1.1 {} {}", status, status_text);
        for (name, val) in &header_list {
            println!("{}: {}", name, val);
        }
        println!();
    }

    // Process response body
    if let Some(ref output_path) = options.output {
        let mut file = match File::create(output_path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Error: Failed to create output file '{}': {}", output_path, e);
                std::process::exit(1);
            }
        };

        let mut temp_buf = [0u8; 8192];
        let mut bytes_written = 0;
        loop {
            match reader.read(&mut temp_buf) {
                Ok(0) => break,
                Ok(n) => {
                    if let Err(e) = file.write_all(&temp_buf[..n]) {
                        eprintln!("Error: Failed to write to output file: {}", e);
                        std::process::exit(1);
                    }
                    bytes_written += n;
                }
                Err(e) => {
                    eprintln!("Error: Failed to read response stream: {}", e);
                    std::process::exit(1);
                }
            }
        }
        println!("Downloaded {} bytes to '{}'.", bytes_written, output_path);
    } else {
        // Output directly to stdout
        let mut stdout = std::io::stdout();
        let mut temp_buf = [0u8; 8192];
        loop {
            match reader.read(&mut temp_buf) {
                Ok(0) => break,
                Ok(n) => {
                    if let Err(e) = stdout.write_all(&temp_buf[..n]) {
                        eprintln!("Error: Failed writing to stdout: {}", e);
                        std::process::exit(1);
                    }
                }
                Err(e) => {
                    eprintln!("Error: Failed to read response stream: {}", e);
                    std::process::exit(1);
                }
            }
        }
        // Print newline if status was non-2xx to separate prompt
        if status >= 400 {
            println!();
            eprintln!("Warning: Server returned status {} {}", status, status_text);
            std::process::exit(1);
        }
    }
}
