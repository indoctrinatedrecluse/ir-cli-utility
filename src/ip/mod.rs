use crate::IpOptions;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
fn print_local_adapters(options: &IpOptions) {
    linux::print_local_adapters(options);
}

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
fn print_local_adapters(options: &IpOptions) {
    windows::print_local_adapters(options);
}

fn get_json_field(json: &str, field: &str) -> Option<String> {
    let pattern = format!("\"{}\"", field);
    if let Some(pos) = json.find(&pattern) {
        let rest = &json[pos + pattern.len()..];
        if let Some(colon_pos) = rest.find(':') {
            let val_part = &rest[colon_pos + 1..];
            let val_part_trimmed = val_part.trim();
            if val_part_trimmed.starts_with('"') {
                if let Some(end_quote) = val_part_trimmed[1..].find('"') {
                    return Some(val_part_trimmed[1..1 + end_quote].to_string());
                }
            } else {
                let end_pos = val_part_trimmed.find(|c: char| c == ',' || c == '}' || c.is_whitespace()).unwrap_or(val_part_trimmed.len());
                return Some(val_part_trimmed[..end_pos].to_string());
            }
        }
    }
    None
}

fn print_public_ip() {
    println!("Querying public IP details...");
    // Retrieve public IP using ipapi.co (fallback to standard GET request)
    match ureq::get("https://ipapi.co/json/").call() {
        Ok(res) => {
            if let Ok(body) = res.into_string() {
                let ip = get_json_field(&body, "ip").unwrap_or_else(|| "Unknown".to_string());
                let city = get_json_field(&body, "city").unwrap_or_else(|| "Unknown".to_string());
                let region = get_json_field(&body, "region").unwrap_or_else(|| "Unknown".to_string());
                let country = get_json_field(&body, "country_name").unwrap_or_else(|| "Unknown".to_string());
                let org = get_json_field(&body, "org").unwrap_or_else(|| "Unknown".to_string());
                
                println!("Public IP:  {}", ip);
                println!("Location:   {}, {} ({})", city, region, country);
                println!("Provider:   {}", org);
            } else {
                eprintln!("Error: Failed to read public IP response body.");
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Error: Public IP lookup failed: {}", e);
            std::process::exit(1);
        }
    }
}

pub fn run_ip(options: IpOptions) {
    if options.public {
        print_public_ip();
    } else {
        print_local_adapters(&options);
    }
}
