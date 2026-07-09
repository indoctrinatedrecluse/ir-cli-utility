use std::fs::{self, File};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::time::Instant;
use crate::ServeOptions;

fn percent_decode(s: &str) -> String {
    let mut decoded = String::new();
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '%' {
            if let (Some(h1), Some(h2)) = (chars.next(), chars.next()) {
                if let Ok(val) = u8::from_str_radix(&format!("{}{}", h1, h2), 16) {
                    decoded.push(val as char);
                    continue;
                }
            }
        }
        decoded.push(c);
    }
    decoded
}

fn get_mime_type(path: &Path) -> &'static str {
    match path.extension().and_then(|s| s.to_str()) {
        Some(ext) => match ext.to_lowercase().as_str() {
            "html" | "htm" => "text/html; charset=utf-8",
            "css" => "text/css; charset=utf-8",
            "js" => "text/javascript; charset=utf-8",
            "json" => "application/json; charset=utf-8",
            "txt" => "text/plain; charset=utf-8",
            "xml" => "text/xml; charset=utf-8",
            "png" => "image/png",
            "jpg" | "jpeg" => "image/jpeg",
            "gif" => "image/gif",
            "svg" => "image/svg+xml",
            "ico" => "image/x-icon",
            "pdf" => "application/pdf",
            "zip" => "application/zip",
            _ => "application/octet-stream",
        },
        None => "application/octet-stream",
    }
}

fn format_size(bytes: u64) -> String {
    if bytes >= 1024 * 1024 * 1024 {
        format!("{:.2} GB", bytes as f64 / (1024 * 1024 * 1024) as f64)
    } else if bytes >= 1024 * 1024 {
        format!("{:.2} MB", bytes as f64 / (1024 * 1024) as f64)
    } else if bytes >= 1024 {
        format!("{:.2} KB", bytes as f64 / 1024.0)
    } else {
        format!("{} B", bytes)
    }
}

fn send_error(mut stream: TcpStream, code: u16, message: &str, method: &str) {
    let body = format!(
        "<!DOCTYPE html><html><head><title>{} {}</title><style>\
         body{{font-family:system-ui,-apple-system,sans-serif;background:#0d0e12;color:#f3f4f6;display:flex;flex-direction:column;align-items:center;justify-content:center;height:100vh;margin:0;}}\
         h1{{font-size:3rem;margin:0;color:#ef4444;}}\
         p{{color:#9ca3af;font-size:1.2rem;}}\
         hr{{border-color:#1f2937;width:100px;margin:20px;}}\
         </style></head><body><h1>{}</h1><p>{}</p><hr><p style='font-size:0.9rem;color:#4b5563;'>ir serve local engine</p></body></html>",
        code, message, code, message
    );

    let response = format!(
        "HTTP/1.1 {} {}\r\nContent-Length: {}\r\nContent-Type: text/html; charset=utf-8\r\nConnection: close\r\n\r\n",
        code, message, body.len()
    );

    let _ = stream.write_all(response.as_bytes());
    if method != "HEAD" {
        let _ = stream.write_all(body.as_bytes());
    }
}

fn log_request(client_ip: &str, method: &str, path: &str, status: u16, duration: std::time::Duration) {
    let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
    let status_color = match status {
        200..=299 => "\x1B[32m", // Green
        300..=399 => "\x1B[36m", // Cyan
        400..=499 => "\x1B[33m", // Yellow
        _ => "\x1B[31m",         // Red
    };
    println!(
        "[{}] {} {} {} -> {}{} \x1B[0m({:.2?})",
        now, client_ip, method, path, status_color, status, duration
    );
}

fn handle_client(mut stream: TcpStream, options: &ServeOptions, base_canonical: &Path) {
    let start_time = Instant::now();
    let client_ip = stream.peer_addr().map(|addr| addr.ip().to_string()).unwrap_or_else(|_| "unknown".to_string());
    let mut buffer = [0u8; 4096];
    let mut bytes_read = 0;

    // Read until headers end
    loop {
        match stream.read(&mut buffer[bytes_read..]) {
            Ok(0) => break,
            Ok(n) => {
                bytes_read += n;
                if buffer[..bytes_read].windows(4).any(|w| w == b"\r\n\r\n") || bytes_read >= buffer.len() {
                    break;
                }
            }
            Err(_) => return,
        }
    }

    if bytes_read == 0 {
        return;
    }

    let req_str = String::from_utf8_lossy(&buffer[..bytes_read]);
    let mut lines = req_str.lines();
    let req_line = match lines.next() {
        Some(l) => l,
        None => return,
    };

    let parts: Vec<&str> = req_line.split_whitespace().collect();
    if parts.len() < 2 {
        send_error(stream, 400, "Bad Request", "GET");
        return;
    }

    let method = parts[0];
    let mut raw_path = parts[1];

    if method != "GET" && method != "HEAD" {
        send_error(stream.try_clone().unwrap(), 405, "Method Not Allowed", method);
        log_request(&client_ip, method, raw_path, 405, start_time.elapsed());
        return;
    }

    // Strip query parameters and hash fragments
    if let Some(pos) = raw_path.find('?') {
        raw_path = &raw_path[..pos];
    }
    if let Some(pos) = raw_path.find('#') {
        raw_path = &raw_path[..pos];
    }

    let path_str = percent_decode(raw_path);

    // Build requested file path
    let rel_path = path_str.trim_start_matches('/');
    let requested_path = if rel_path.is_empty() {
        PathBuf::from(&options.directory)
    } else {
        Path::new(&options.directory).join(rel_path)
    };

    // Traversal check
    if path_str.contains("..") {
        send_error(stream.try_clone().unwrap(), 403, "Forbidden", method);
        log_request(&client_ip, method, &path_str, 403, start_time.elapsed());
        return;
    }

    let safe_path = match requested_path.canonicalize() {
        Ok(p) => {
            if p.starts_with(base_canonical) {
                p
            } else {
                send_error(stream.try_clone().unwrap(), 403, "Forbidden", method);
                log_request(&client_ip, method, &path_str, 403, start_time.elapsed());
                return;
            }
        }
        Err(_) => {
            send_error(stream.try_clone().unwrap(), 404, "Not Found", method);
            log_request(&client_ip, method, &path_str, 404, start_time.elapsed());
            return;
        }
    };

    // If directory, check for index.html
    if safe_path.is_dir() {
        let index_html = safe_path.join("index.html");
        if index_html.exists() && index_html.is_file() {
            serve_file(stream, method, &index_html, options, &path_str, &client_ip, start_time);
        } else {
            serve_directory(stream, method, &safe_path, base_canonical, &path_str, &client_ip, start_time);
        }
    } else if safe_path.is_file() {
        serve_file(stream, method, &safe_path, options, &path_str, &client_ip, start_time);
    } else {
        send_error(stream.try_clone().unwrap(), 404, "Not Found", method);
        log_request(&client_ip, method, &path_str, 404, start_time.elapsed());
    }
}

fn serve_file(mut stream: TcpStream, method: &str, path: &Path, options: &ServeOptions, path_str: &str, client_ip: &str, start_time: Instant) {
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(_) => {
            send_error(stream.try_clone().unwrap(), 500, "Internal Server Error", method);
            log_request(client_ip, method, path_str, 500, start_time.elapsed());
            return;
        }
    };

    let metadata = match path.metadata() {
        Ok(m) => m,
        Err(_) => {
            send_error(stream.try_clone().unwrap(), 500, "Internal Server Error", method);
            log_request(client_ip, method, path_str, 500, start_time.elapsed());
            return;
        }
    };

    let size = metadata.len();
    let mime = get_mime_type(path);
    let cache_hdr = if options.cache_seconds > 0 {
        format!("Cache-Control: max-age={}\r\n", options.cache_seconds)
    } else {
        "Cache-Control: no-store\r\n".to_string()
    };

    let header = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: {}\r\n{}Connection: close\r\n\r\n",
        size, mime, cache_hdr
    );

    if stream.write_all(header.as_bytes()).is_err() {
        return;
    }

    if method != "HEAD" {
        let mut buffer = [0u8; 8192];
        loop {
            match file.read(&mut buffer) {
                Ok(0) => break,
                Ok(n) => {
                    if stream.write_all(&buffer[..n]).is_err() {
                        break;
                    }
                }
                Err(_) => break,
            }
        }
    }

    log_request(client_ip, method, path_str, 200, start_time.elapsed());
}

fn serve_directory(mut stream: TcpStream, method: &str, dir_path: &Path, base_canonical: &Path, path_str: &str, client_ip: &str, start_time: Instant) {
    let dir_entries = match fs::read_dir(dir_path) {
        Ok(entries) => entries,
        Err(_) => {
            send_error(stream.try_clone().unwrap(), 500, "Internal Server Error", method);
            log_request(client_ip, method, path_str, 500, start_time.elapsed());
            return;
        }
    };

    let mut rows = String::new();
    
    // Add link to parent directory if not at served root
    if dir_path != base_canonical {
        rows.push_str(
            "<tr><td>📁 <a href='../'>..</a></td><td>-</td><td>-</td></tr>"
        );
    }

    for entry in dir_entries.map_while(Result::ok) {
        let name = entry.file_name().to_string_lossy().into_owned();
        let metadata = entry.metadata().ok();
        let is_dir = metadata.as_ref().map(|m| m.is_dir()).unwrap_or(false);
        
        let size_str = if is_dir {
            "-".to_string()
        } else {
            metadata.as_ref().map(|m| format_size(m.len())).unwrap_or_else(|| "-".to_string())
        };

        let mod_time = metadata.as_ref()
            .and_then(|m| m.modified().ok())
            .map(|t| {
                let datetime: chrono::DateTime<chrono::Local> = t.into();
                datetime.format("%Y-%m-%d %H:%M:%S").to_string()
            })
            .unwrap_or_else(|| "-".to_string());

        let display_name = if is_dir { format!("{}/", name) } else { name.clone() };
        let icon = if is_dir { "📁" } else { "📄" };
        
        // Ensure path matches properly (with trailing slash for directory references)
        let href = if is_dir { format!("{}/", name) } else { name };

        rows.push_str(&format!(
            "<tr><td>{} <a href='{}'>{}</a></td><td>{}</td><td>{}</td></tr>",
            icon, href, display_name, size_str, mod_time
        ));
    }

    let body = format!(
        "<!DOCTYPE html><html><head><title>Index of {}</title><style>\
         body{{font-family:system-ui,-apple-system,sans-serif;background:#0d0e12;color:#f3f4f6;margin:0;padding:40px;display:flex;justify-content:center;}}\
         .container{{width:100%;max-width:1000px;background:#16171d;border:1px solid #23242e;border-radius:12px;box-shadow:0 10px 30px rgba(0,0,0,0.5);padding:30px;}}\
         h1{{font-size:1.8rem;margin-top:0;color:#10b981;border-bottom:1px solid #23242e;padding-bottom:15px;}}\
         table{{width:100%;border-collapse:collapse;margin-top:15px;}}\
         th,td{{text-align:left;padding:12px;border-bottom:1px solid #1f2029;}}\
         th{{color:#9ca3af;font-weight:600;font-size:0.9rem;text-transform:uppercase;}}\
         a{{color:#3b82f6;text-decoration:none;transition:color 0.2s;}}\
         a:hover{{color:#60a5fa;text-decoration:underline;}}\
         tr:hover{{background:#1d1f27;}}\
         footer{{margin-top:30px;border-top:1px solid #23242e;padding-top:15px;font-size:0.85rem;color:#4b5563;text-align:center;}}\
         </style></head><body><div class='container'><h1>Index of {}</h1>\
         <table><thead><tr><th>Name</th><th>Size</th><th>Last Modified</th></tr></thead><tbody>\
         {}</tbody></table><footer>ir serve local engine</footer></div></body></html>",
        path_str, path_str, rows
    );

    let header = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/html; charset=utf-8\r\nConnection: close\r\n\r\n",
        body.len()
    );

    if stream.write_all(header.as_bytes()).is_err() {
        return;
    }

    if method != "HEAD" {
        let _ = stream.write_all(body.as_bytes());
    }

    log_request(client_ip, method, path_str, 200, start_time.elapsed());
}

pub fn run_serve(options: ServeOptions) {
    let base_path = Path::new(&options.directory);
    let base_canonical = match base_path.canonicalize() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error: Serving directory '{}' does not exist: {}", options.directory, e);
            std::process::exit(1);
        }
    };

    println!("\x1B[32mHost Server Starting...\x1B[0m");
    println!("Serving Folder:  {}", base_canonical.display());
    println!("Listening on:    http://{}:{}", options.bind, options.port);
    println!("Press Ctrl+C to stop.\n");

    let listener = match TcpListener::bind(format!("{}:{}", options.bind, options.port)) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Error: Failed to bind to {}:{}: {}", options.bind, options.port, e);
            std::process::exit(1);
        }
    };

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            let opt = options.clone();
            let base_c = base_canonical.clone();
            std::thread::spawn(move || {
                handle_client(stream, &opt, &base_c);
            });
        }
    }
}
