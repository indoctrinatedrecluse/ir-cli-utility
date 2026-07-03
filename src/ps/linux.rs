use crate::PsOptions;
use std::fs;

struct ProcessInfo {
    pid: u32,
    name: String,
    state: char,
    cpu_time_secs: u64,
    rss_bytes: u64,
}

fn get_clk_tck() -> u64 {
    unsafe {
        let tck = libc::sysconf(libc::_SC_CLK_TCK);
        if tck > 0 { tck as u64 } else { 100 }
    }
}

fn get_page_size() -> u64 {
    unsafe {
        let sz = libc::sysconf(libc::_SC_PAGESIZE);
        if sz > 0 { sz as u64 } else { 4096 }
    }
}

fn parse_stat_file(pid_str: &str, clk_tck: u64) -> Option<(String, char, u64)> {
    let stat_path = format!("/proc/{}/stat", pid_str);
    let content = fs::read_to_string(stat_path).ok()?;
    
    // Find the last index of ')' to handle process names with spaces/parentheses
    let last_paren = content.rfind(')')?;
    let first_paren = content.find('(')?;
    let name = content[first_paren + 1..last_paren].to_string();

    let rest = &content[last_paren + 2..];
    let fields: Vec<&str> = rest.split_whitespace().collect();
    if fields.len() < 13 {
        return None;
    }

    let state = fields[0].chars().next().unwrap_or('?');
    let utime = fields[11].parse::<u64>().unwrap_or(0);
    let stime = fields[12].parse::<u64>().unwrap_or(0);
    let cpu_time_secs = (utime + stime) / clk_tck;

    Some((name, state, cpu_time_secs))
}

fn parse_statm_file(pid_str: &str, page_size: u64) -> u64 {
    let statm_path = format!("/proc/{}/statm", pid_str);
    if let Ok(content) = fs::read_to_string(statm_path) {
        let parts: Vec<&str> = content.split_whitespace().collect();
        if parts.len() > 1 {
            if let Ok(rss_pages) = parts[1].parse::<u64>() {
                return rss_pages * page_size;
            }
        }
    }
    0
}

fn format_mem(bytes: u64) -> String {
    let kib = bytes as f64 / 1024.0;
    if kib >= 1024.0 {
        format!("{:.1} MiB", kib / 1024.0)
    } else {
        format!("{:.1} KiB", kib)
    }
}

fn format_time(secs: u64) -> String {
    let h = secs / 3600;
    let m = (secs % 3600) / 60;
    let s = secs % 60;
    format!("{:02}:{:02}:{:02}", h, m, s)
}

pub fn ps(options: PsOptions) {
    let clk_tck = get_clk_tck();
    let page_size = get_page_size();
    
    let mut processes = Vec::new();
    
    let proc_dir = match fs::read_dir("/proc") {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("Error: Failed to read /proc: {}", e);
            std::process::exit(1);
        }
    };

    for entry_result in proc_dir {
        let entry = match entry_result {
            Ok(e) => e,
            Err(_) => continue,
        };
        let file_name = entry.file_name();
        let name_str = file_name.to_string_lossy();
        
        // Check if directory name is numerical (representing a PID)
        if let Ok(pid) = name_str.parse::<u32>() {
            if let Some((name, state, cpu_time)) = parse_stat_file(&name_str, clk_tck) {
                // Apply name filter if specified
                if let Some(ref filter) = options.filter {
                    if !name.to_lowercase().contains(&filter.to_lowercase()) {
                        continue;
                    }
                }
                
                let rss_bytes = parse_statm_file(&name_str, page_size);
                processes.push(ProcessInfo {
                    pid,
                    name,
                    state,
                    cpu_time_secs: cpu_time,
                    rss_bytes,
                });
            }
        }
    }

    // Sort processes
    let sort_by = options.sort_by.to_lowercase();
    match sort_by.as_str() {
        "name" => {
            processes.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
        }
        "cpu" => {
            processes.sort_by(|a, b| b.cpu_time_secs.cmp(&a.cpu_time_secs));
        }
        "mem" => {
            processes.sort_by(|a, b| b.rss_bytes.cmp(&a.rss_bytes));
        }
        _ => {
            // Default to pid ascending
            processes.sort_by(|a, b| a.pid.cmp(&b.pid));
        }
    }

    // Apply limit if specified
    let display_count = if let Some(limit) = options.limit {
        processes.len().min(limit)
    } else {
        processes.len()
    };

    // Print table header
    println!("{:>8}  {:^5}  {:>8}  {:>10}  {}", "PID", "STATE", "TIME", "MEM", "COMMAND");
    for proc in processes.iter().take(display_count) {
        println!(
            "{:>8}  {:^5}  {:>8}  {:>10}  {}",
            proc.pid,
            proc.state,
            format_time(proc.cpu_time_secs),
            format_mem(proc.rss_bytes),
            proc.name
        );
    }
}
