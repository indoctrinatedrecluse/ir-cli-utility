use crate::FetchOptions;
use std::fs::File;
use std::io::{Read, Write};
use std::time::Duration;

pub fn fetch(url: &str, options: FetchOptions) {
    let method = if options.method.is_empty() { "GET" } else { options.method.as_str() };

    // Build agent with timeout and redirect settings
    let mut agent_builder = ureq::AgentBuilder::new();

    if options.timeout_secs > 0 {
        let dur = Duration::from_secs(options.timeout_secs);
        agent_builder = agent_builder
            .timeout_connect(dur)
            .timeout_read(dur)
            .timeout_write(dur);
    }

    if options.no_follow_redirects {
        agent_builder = agent_builder.redirects(0);
    }

    let agent = agent_builder.build();
    let mut request = agent.request(method, url);

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

    // Extract Content-Length from headers for progress reporting
    let content_length: Option<u64> = header_list.iter()
        .find(|(k, _)| k.eq_ignore_ascii_case("content-length"))
        .and_then(|(_, v)| v.parse::<u64>().ok());

    // Process response body
    if let Some(ref output_path) = options.output {
        let mut file = match File::create(output_path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Error: Failed to create output file '{}': {}", output_path, e);
                std::process::exit(1);
            }
        };

        let show_progress = options.progress;
        let mut temp_buf = [0u8; 8192];
        let mut bytes_written: u64 = 0;

        loop {
            match reader.read(&mut temp_buf) {
                Ok(0) => break,
                Ok(n) => {
                    if let Err(e) = file.write_all(&temp_buf[..n]) {
                        eprintln!("Error: Failed to write to output file: {}", e);
                        std::process::exit(1);
                    }
                    bytes_written += n as u64;

                    if show_progress {
                        print_progress(bytes_written, content_length);
                    }
                }
                Err(e) => {
                    eprintln!("Error: Failed to read response stream: {}", e);
                    std::process::exit(1);
                }
            }
        }

        if show_progress {
            // Clear progress line
            eprint!("\r{:80}\r", "");
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

        if status >= 400 {
            println!();
            eprintln!("Warning: Server returned status {} {}", status, status_text);
            std::process::exit(1);
        }
    }
}

/// Print a compact progress indicator to stderr.
fn print_progress(downloaded: u64, total: Option<u64>) {
    match total {
        Some(t) if t > 0 => {
            let pct = (downloaded * 100 / t).min(100);
            let bar_filled = (pct / 5) as usize; // 20-char bar
            let bar: String = format!(
                "[{}{}] {}% ({}/{})",
                "#".repeat(bar_filled),
                "-".repeat(20 - bar_filled),
                pct,
                human_bytes(downloaded),
                human_bytes(t),
            );
            eprint!("\r{}", bar);
        }
        _ => {
            eprint!("\r{} downloaded", human_bytes(downloaded));
        }
    }
    let _ = std::io::stderr().flush();
}

fn human_bytes(b: u64) -> String {
    match b {
        b if b >= 1024 * 1024 * 1024 => format!("{:.2} GiB", b as f64 / (1024.0 * 1024.0 * 1024.0)),
        b if b >= 1024 * 1024        => format!("{:.2} MiB", b as f64 / (1024.0 * 1024.0)),
        b if b >= 1024               => format!("{:.2} KiB", b as f64 / 1024.0),
        b                            => format!("{} B", b),
    }
}
