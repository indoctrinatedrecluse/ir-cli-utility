use std::path::Path;

fn strip_ansi(s: &str) -> String {
    let mut result = String::new();
    let mut in_escape = false;
    for c in s.chars() {
        if c == '\x1b' {
            in_escape = true;
        } else if in_escape {
            if c == 'm' {
                in_escape = false;
            }
        } else {
            result.push(c);
        }
    }
    result
}

fn print_side_by_side(logo: &[&str], info: &[String]) {
    let max_lines = logo.len().max(info.len());
    let logo_visual_width = logo.iter().map(|line| strip_ansi(line).chars().count()).max().unwrap_or(0);

    for i in 0..max_lines {
        let logo_line = if i < logo.len() { logo[i] } else { "" };
        let info_line = if i < info.len() { &info[i] } else { "" };
        
        let visual_len = strip_ansi(logo_line).chars().count();
        let padding = " ".repeat(logo_visual_width.saturating_sub(visual_len) + 4);
        
        println!("{}{}{}", logo_line, padding, info_line);
    }
}

fn get_os() -> Option<String> {
    let content = std::fs::read_to_string("/etc/os-release").ok()?;
    for line in content.lines() {
        if line.starts_with("PRETTY_NAME=") {
            let val = line.trim_start_matches("PRETTY_NAME=").trim_matches('"');
            return Some(val.to_string());
        }
    }
    None
}

fn get_kernel() -> Option<String> {
    std::fs::read_to_string("/proc/sys/kernel/osrelease")
        .map(|s| s.trim().to_string())
        .ok()
}

fn get_uptime() -> u64 {
    if let Ok(s) = std::fs::read_to_string("/proc/uptime") {
        if let Some(first) = s.split_whitespace().next() {
            if let Ok(f) = first.parse::<f64>() {
                return f as u64;
            }
        }
    }
    0
}

fn format_uptime(secs: u64) -> String {
    let days = secs / 86400;
    let hours = (secs % 86400) / 3600;
    let mins = (secs % 3600) / 60;
    
    if days > 0 {
        format!("{} days, {} hours, {} mins", days, hours, mins)
    } else if hours > 0 {
        format!("{} hours, {} mins", hours, mins)
    } else {
        format!("{} mins", mins)
    }
}

fn get_host() -> Option<String> {
    std::fs::read_to_string("/sys/class/dmi/id/product_name")
        .or_else(|_| std::fs::read_to_string("/sys/devices/virtual/dmi/id/product_name"))
        .map(|s| s.trim().to_string())
        .ok()
}

fn get_shell() -> String {
    std::env::var("SHELL")
        .ok()
        .and_then(|s| {
            Path::new(&s)
                .file_name()
                .map(|f| f.to_string_lossy().to_string())
        })
        .unwrap_or_else(|| "bash".to_string())
}

fn get_cpu() -> Option<String> {
    let content = std::fs::read_to_string("/proc/cpuinfo").ok()?;
    for line in content.lines() {
        if line.starts_with("model name") {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() > 1 {
                return Some(parts[1].trim().to_string());
            }
        }
    }
    None
}

fn get_gpu() -> Option<String> {
    let output = std::process::Command::new("lspci").output().ok()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        if line.contains("VGA compatible controller") || line.contains("3D controller") {
            let parts: Vec<&str> = line.split(" controller:").collect();
            if parts.len() > 1 {
                return Some(parts[1].trim().to_string());
            }
            let parts_colon: Vec<&str> = line.split(':').collect();
            if parts_colon.len() > 2 {
                return Some(parts_colon[2].trim().to_string());
            }
        }
    }
    None
}

fn get_mem() -> Option<(u64, u64)> {
    let content = std::fs::read_to_string("/proc/meminfo").ok()?;
    let mut total = 0;
    let mut available = 0;
    for line in content.lines() {
        if line.starts_with("MemTotal:") {
            total = parse_meminfo_line(line)?;
        } else if line.starts_with("MemAvailable:") {
            available = parse_meminfo_line(line)?;
        }
    }
    if total > 0 {
        let used = total - available;
        Some((used * 1024, total * 1024))
    } else {
        None
    }
}

fn parse_meminfo_line(line: &str) -> Option<u64> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() > 1 {
        parts[1].parse::<u64>().ok()
    } else {
        None
    }
}

fn get_disks() -> Vec<(String, u64, u64)> {
    let mut disks = Vec::new();
    let file = match std::fs::File::open("/proc/self/mounts") {
        Ok(f) => f,
        Err(_) => return disks,
    };
    use std::io::{BufRead, BufReader};
    let reader = BufReader::new(file);
    let mut seen_devices = std::collections::HashSet::new();

    for line_res in reader.lines() {
        let line = match line_res {
            Ok(l) => l,
            Err(_) => continue,
        };
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 3 {
            continue;
        }
        let device = parts[0];
        let mount_point = parts[1];
        
        if !device.starts_with("/dev/") {
            continue;
        }

        if seen_devices.contains(device) {
            continue;
        }
        seen_devices.insert(device.to_string());

        unsafe {
            let mut stat = std::mem::zeroed::<libc::statvfs>();
            if let Ok(c_path) = std::ffi::CString::new(mount_point) {
                if libc::statvfs(c_path.as_ptr(), &mut stat) == 0 {
                    let block_size = if stat.f_frsize > 0 {
                        stat.f_frsize as u64
                    } else {
                        stat.f_bsize as u64
                    };
                    let total = stat.f_blocks as u64 * block_size;
                    let free = stat.f_bfree as u64 * block_size;
                    let used = total.saturating_sub(free);
                    disks.push((mount_point.to_string(), used, total));
                }
            }
        }
    }
    disks
}

fn format_bytes(bytes: u64) -> String {
    let gib = bytes as f64 / 1024.0 / 1024.0 / 1024.0;
    format!("{:.2} GiB", gib)
}

pub fn fastfetch() {
    let logo = [
        "\x1b[38;5;208m      .--.",
        "\x1b[38;5;208m     |o_o |",
        "\x1b[38;5;208m     |:_/ |",
        "\x1b[38;5;208m    //   \\ \\",
        "\x1b[38;5;208m   (|     | )",
        "\x1b[38;5;208m  /'\\_   _/`\\",
        "\x1b[38;5;208m  \\___)=(___/\x1b[0m",
    ];

    let os = get_os().unwrap_or_else(|| "Linux".to_string());
    let host = get_host().unwrap_or_else(|| "Generic PC".to_string());
    let kernel = get_kernel().unwrap_or_else(|| "Unknown".to_string());
    let uptime = format_uptime(get_uptime());
    let shell = get_shell();
    let cpu = get_cpu().unwrap_or_else(|| "Unknown".to_string());
    let gpu = get_gpu().unwrap_or_else(|| "Unknown".to_string());
    
    let mem_str = if let Some((used, total)) = get_mem() {
        let pct = (used as f64 / total as f64) * 100.0;
        format!("{} / {} ({:.0}%)", format_bytes(used), format_bytes(total), pct)
    } else {
        "Unknown".to_string()
    };

    let mut info = Vec::new();
    info.push(format!("\x1b[36mOS\x1b[0m:       {}", os));
    info.push(format!("\x1b[36mHost\x1b[0m:     {}", host));
    info.push(format!("\x1b[36mKernel\x1b[0m:   {}", kernel));
    info.push(format!("\x1b[36mUptime\x1b[0m:   {}", uptime));
    info.push(format!("\x1b[36mShell\x1b[0m:    {}", shell));
    info.push(format!("\x1b[36mCPU\x1b[0m:      {}", cpu));
    info.push(format!("\x1b[36mGPU\x1b[0m:      {}", gpu));
    info.push(format!("\x1b[36mMemory\x1b[0m:   {}", mem_str));

    let disks = get_disks();
    if !disks.is_empty() {
        info.push(format!("\x1b[36mDisks\x1b[0m:"));
        for (name, used, total) in disks {
            let pct = (used as f64 / total as f64) * 100.0;
            info.push(format!("  {}  {} / {} ({:.0}%)", name, format_bytes(used), format_bytes(total), pct));
        }
    } else {
        info.push(format!("\x1b[36mDisk\x1b[0m:     Unknown"));
    }

    print_side_by_side(&logo, &info);
}
