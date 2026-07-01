use crate::PsOptions;

use windows_sys::Win32::Foundation::{CloseHandle, INVALID_HANDLE_VALUE};
use windows_sys::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, Process32FirstW, Process32NextW, PROCESSENTRY32W, TH32CS_SNAPPROCESS,
};
use windows_sys::Win32::System::Threading::{
    OpenProcess, GetProcessTimes, PROCESS_QUERY_LIMITED_INFORMATION,
};
use windows_sys::Win32::System::ProcessStatus::{
    K32GetProcessMemoryInfo, PROCESS_MEMORY_COUNTERS,
};

struct ProcessInfo {
    pid: u32,
    name: String,
    cpu_time_secs: u64,
    rss_bytes: u64,
}

// Convert wide char array to String
fn string_from_wide(wide: &[u16]) -> String {
    let len = wide.iter().position(|&c| c == 0).unwrap_or(wide.len());
    String::from_utf16_lossy(&wide[..len])
}

fn get_process_metrics(pid: u32) -> (u64, u64) {
    let mut cpu_time = 0;
    let mut rss_bytes = 0;

    unsafe {
        let h_process = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, pid);
        if h_process != 0 {
            // Get CPU times
            let mut creation_ft = std::mem::zeroed();
            let mut exit_ft = std::mem::zeroed();
            let mut kernel_ft = std::mem::zeroed();
            let mut user_ft = std::mem::zeroed();
            if GetProcessTimes(h_process, &mut creation_ft, &mut exit_ft, &mut kernel_ft, &mut user_ft) != 0 {
                let kernel_time = ((kernel_ft.dwHighDateTime as u64) << 32) | (kernel_ft.dwLowDateTime as u64);
                let user_time = ((user_ft.dwHighDateTime as u64) << 32) | (user_ft.dwLowDateTime as u64);
                cpu_time = (kernel_time + user_time) / 10_000_000;
            }

            // Get Memory info
            let mut pmc = std::mem::zeroed::<PROCESS_MEMORY_COUNTERS>();
            pmc.cb = std::mem::size_of::<PROCESS_MEMORY_COUNTERS>() as u32;
            if K32GetProcessMemoryInfo(h_process, &mut pmc, pmc.cb) != 0 {
                rss_bytes = pmc.WorkingSetSize as u64;
            }

            CloseHandle(h_process);
        }
    }

    (cpu_time, rss_bytes)
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
    let mut processes = Vec::new();

    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        if snapshot == INVALID_HANDLE_VALUE {
            eprintln!("Error: Failed to create Toolhelp snapshot.");
            std::process::exit(1);
        }

        let mut entry = std::mem::zeroed::<PROCESSENTRY32W>();
        entry.dwSize = std::mem::size_of::<PROCESSENTRY32W>() as u32;

        if Process32FirstW(snapshot, &mut entry) != 0 {
            loop {
                let name = string_from_wide(&entry.szExeFile);
                
                let mut should_include = true;
                if let Some(ref filter) = options.filter {
                    if !name.to_lowercase().contains(&filter.to_lowercase()) {
                        should_include = false;
                    }
                }

                if should_include {
                    let pid = entry.th32ProcessID;
                    let (cpu_time, rss_bytes) = get_process_metrics(pid);
                    processes.push(ProcessInfo {
                        pid,
                        name,
                        cpu_time_secs: cpu_time,
                        rss_bytes,
                    });
                }

                if Process32NextW(snapshot, &mut entry) == 0 {
                    break;
                }
            }
        }

        CloseHandle(snapshot);
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
    println!("{:>8}  {:>8}  {:>10}  {}", "PID", "TIME", "MEM", "COMMAND");
    for proc in processes.iter().take(display_count) {
        println!(
            "{:>8}  {:>8}  {:>10}  {}",
            proc.pid,
            format_time(proc.cpu_time_secs),
            format_mem(proc.rss_bytes),
            proc.name
        );
    }
}
