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

fn query_registry(path: &str, value: &str) -> Option<String> {
    use std::process::Command;
    // Set standard cmd flags to hide the window / run silently
    let output = Command::new("reg")
        .args(["query", path, "/v", value])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        if line.contains(value) {
            let parts: Vec<&str> = line.split("REG_SZ").collect();
            if parts.len() > 1 {
                return Some(parts[1].trim().to_string());
            }
            let parts_dword: Vec<&str> = line.split("REG_DWORD").collect();
            if parts_dword.len() > 1 {
                return Some(parts_dword[1].trim().to_string());
            }
        }
    }
    None
}

fn get_os() -> Option<String> {
    query_registry("HKLM\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion", "ProductName")
}

fn get_kernel() -> Option<String> {
    query_registry("HKLM\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion", "CurrentBuild")
}

fn get_uptime() -> u64 {
    unsafe {
        windows_sys::Win32::System::SystemInformation::GetTickCount64() / 1000
    }
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
    query_registry("HKLM\\SYSTEM\\CurrentControlSet\\Control\\SystemInformation", "SystemProductName")
}

fn get_shell() -> String {
    std::env::var("ComSpec")
        .ok()
        .and_then(|s| {
            Path::new(&s)
                .file_name()
                .map(|f| f.to_string_lossy().to_string())
        })
        .unwrap_or_else(|| "cmd.exe".to_string())
}

fn get_cpu() -> Option<String> {
    query_registry("HKLM\\HARDWARE\\DESCRIPTION\\System\\CentralProcessor\\0", "ProcessorNameString")
}

fn get_gpu() -> Option<String> {
    query_registry("HKLM\\SYSTEM\\CurrentControlSet\\Control\\Class\\{4d36e968-e325-11ce-bfc1-08002be10318}\\0000", "DriverDesc")
}

fn get_mem() -> Option<(u64, u64)> {
    use windows_sys::Win32::System::SystemInformation::{GlobalMemoryStatusEx, MEMORYSTATUSEX};
    unsafe {
        let mut mem_status = std::mem::zeroed::<MEMORYSTATUSEX>();
        mem_status.dwLength = std::mem::size_of::<MEMORYSTATUSEX>() as u32;
        if GlobalMemoryStatusEx(&mut mem_status) != 0 {
            let total = mem_status.ullTotalPhys;
            let used = total - mem_status.ullAvailPhys;
            Some((used, total))
        } else {
            None
        }
    }
}

fn get_disk() -> Option<(u64, u64)> {
    use windows_sys::Win32::Storage::FileSystem::GetDiskFreeSpaceExW;
    let mut free_bytes = 0u64;
    let mut total_bytes = 0u64;
    let mut total_free = 0u64;
    let path = [b'C' as u16, b':' as u16, b'\\' as u16, 0u16];
    unsafe {
        if GetDiskFreeSpaceExW(path.as_ptr(), &mut free_bytes, &mut total_bytes, &mut total_free) != 0 {
            let used = total_bytes - total_free;
            Some((used, total_bytes))
        } else {
            None
        }
    }
}

fn format_bytes(bytes: u64) -> String {
    let gib = bytes as f64 / 1024.0 / 1024.0 / 1024.0;
    format!("{:.2} GiB", gib)
}

pub fn fastfetch() {
    let logo = [
        "\x1b[36m    ██████████████  ██████████████",
        "\x1b[36m    ██████████████  ██████████████",
        "\x1b[36m    ██████████████  ██████████████",
        "\x1b[36m    ██████████████  ██████████████",
        "\x1b[36m    ██████████████  ██████████████",
        "\x1b[36m",
        "\x1b[36m    ██████████████  ██████████████",
        "\x1b[36m    ██████████████  ██████████████",
        "\x1b[36m    ██████████████  ██████████████",
        "\x1b[36m    ██████████████  ██████████████",
        "\x1b[36m    ██████████████  ██████████████\x1b[0m",
    ];

    let os = get_os().unwrap_or_else(|| "Windows".to_string());
    let host = get_host().unwrap_or_else(|| "PC".to_string());
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

    let disk_str = if let Some((used, total)) = get_disk() {
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
    info.push(format!("\x1b[36mDisk\x1b[0m:     {}", disk_str));

    print_side_by_side(&logo, &info);
}
