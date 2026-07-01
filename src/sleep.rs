use std::time::Duration;
use std::thread;

fn parse_duration(s: &str) -> Result<u64, String> {
    let mut num_part = String::new();
    let mut unit_part = String::new();

    for c in s.chars() {
        if c.is_ascii_digit() || c == '.' {
            num_part.push(c);
        } else {
            unit_part.push(c);
        }
    }

    if num_part.is_empty() {
        return Err("No numeric value found in duration".to_string());
    }

    let val = num_part.parse::<f64>().map_err(|e| format!("Invalid number '{}': {}", num_part, e))?;
    if val < 0.0 {
        return Err("Duration cannot be negative".to_string());
    }

    let factor = match unit_part.trim().to_lowercase().as_str() {
        "ms" => 1.0,
        "s" | "" => 1000.0,
        "m" => 60000.0,
        "h" => 3600000.0,
        other => return Err(format!("Unknown unit '{}'. Supported units: ms, s, m, h", other)),
    };

    Ok((val * factor) as u64)
}

pub fn run_sleep(duration_str: &str) {
    match parse_duration(duration_str) {
        Ok(ms) => {
            thread::sleep(Duration::from_millis(ms));
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
