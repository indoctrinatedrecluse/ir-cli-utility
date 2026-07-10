use std::io::{self, Write};
use std::time::{Duration, Instant};
use crate::tui_util::{self, Key};

pub fn run_sysinfo() -> io::Result<()> {
    // 1. Setup TUI Raw Mode
    #[cfg(not(target_os = "windows"))]
    let _raw_mode = tui_util::set_raw_mode();
    #[cfg(target_os = "windows")]
    unsafe {
        use windows_sys::Win32::System::Console::{
            GetStdHandle, GetConsoleMode, SetConsoleMode, STD_INPUT_HANDLE, ENABLE_PROCESSED_INPUT,
        };
        let h = GetStdHandle(STD_INPUT_HANDLE);
        let mut mode = 0;
        GetConsoleMode(h, &mut mode);
        SetConsoleMode(h, mode & !ENABLE_PROCESSED_INPUT);
    }

    // Enter alternate screen buffer
    print!("\x1B[?1049h\x1B[?25l\x1B[2J\x1B[H");
    let _ = io::stdout().flush();

    let result = sysinfo_loop();

    // Exit alternate screen buffer
    print!("\x1B[?1049l\x1B[?25h\x1B[0m");
    let _ = io::stdout().flush();

    result
}

// ─── Windows FFI definitions ──────────────────────────────────────────────────
#[cfg(target_os = "windows")]
#[repr(C)]
#[allow(non_snake_case)]
struct FILETIME {
    dwLowDateTime: u32,
    dwHighDateTime: u32,
}

#[cfg(target_os = "windows")]
#[repr(C)]
#[allow(non_snake_case)]
struct MEMORYSTATUSEX {
    dwLength: u32,
    dwMemoryLoad: u32,
    ullTotalPhys: u64,
    ullAvailPhys: u64,
    ullTotalPageFile: u64,
    ullAvailPageFile: u64,
    ullTotalVirtual: u64,
    ullAvailVirtual: u64,
    ullAvailExtendedVirtual: u64,
}

#[cfg(target_os = "windows")]
extern "system" {
    fn GetSystemTimes(lpIdleTime: *mut FILETIME, lpKernelTime: *mut FILETIME, lpUserTime: *mut FILETIME) -> i32;
    fn GlobalMemoryStatusEx(lpBuffer: *mut MEMORYSTATUSEX) -> i32;
}

#[cfg(target_os = "windows")]
fn filetime_to_u64(ft: &FILETIME) -> u64 {
    ((ft.dwHighDateTime as u64) << 32) | (ft.dwLowDateTime as u64)
}

struct SysMetrics {
    cpu_percent: f64,
    mem_total: u64,
    mem_used: u64,
    net_in_sec: f64,
    net_out_sec: f64,
}

fn get_system_metrics(
    prev_idle: &mut u64,
    prev_total: &mut u64,
    prev_net_in: &mut u64,
    prev_net_out: &mut u64,
    last_net_check: &mut Instant,
) -> SysMetrics {
    let mut cpu_percent = 0.0;
    let mut mem_total: u64 = 16 * 1024 * 1024 * 1024; // fallback values
    let mut mem_used: u64 = 8 * 1024 * 1024 * 1024;

    // 1. CPU and Memory - OS Specific
    #[cfg(target_os = "windows")]
    unsafe {
        let mut idle_ft = std::mem::zeroed();
        let mut kernel_ft = std::mem::zeroed();
        let mut user_ft = std::mem::zeroed();
        if GetSystemTimes(&mut idle_ft, &mut kernel_ft, &mut user_ft) != 0 {
            let idle = filetime_to_u64(&idle_ft);
            let total = filetime_to_u64(&kernel_ft) + filetime_to_u64(&user_ft);
            let diff_idle = idle.saturating_sub(*prev_idle);
            let diff_total = total.saturating_sub(*prev_total);
            if diff_total > 0 {
                cpu_percent = 100.0 * (1.0 - (diff_idle as f64 / diff_total as f64));
            }
            *prev_idle = idle;
            *prev_total = total;
        }

        let mut mem_info: MEMORYSTATUSEX = std::mem::zeroed();
        mem_info.dwLength = std::mem::size_of::<MEMORYSTATUSEX>() as u32;
        if GlobalMemoryStatusEx(&mut mem_info) != 0 {
            mem_total = mem_info.ullTotalPhys;
            mem_used = mem_info.ullTotalPhys.saturating_sub(mem_info.ullAvailPhys);
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        // Read CPU from /proc/stat
        if let Ok(content) = std::fs::read_to_string("/proc/stat") {
            if let Some(first_line) = content.lines().next() {
                let parts: Vec<&str> = first_line.split_whitespace().collect();
                if parts.len() >= 5 {
                    let user: u64 = parts[1].parse().unwrap_or(0);
                    let nice: u64 = parts[2].parse().unwrap_or(0);
                    let system: u64 = parts[3].parse().unwrap_or(0);
                    let idle: u64 = parts[4].parse().unwrap_or(0);
                    let iowait: u64 = parts[5].parse().unwrap_or(0);
                    let irq: u64 = parts[6].parse().unwrap_or(0);
                    let softirq: u64 = parts[7].parse().unwrap_or(0);

                    let idle_time = idle + iowait;
                    let total_time = user + nice + system + idle_time + irq + softirq;

                    let diff_idle = idle_time.saturating_sub(*prev_idle);
                    let diff_total = total_time.saturating_sub(*prev_total);
                    if diff_total > 0 {
                        cpu_percent = 100.0 * (1.0 - (diff_idle as f64 / diff_total as f64));
                    }
                    *prev_idle = idle_time;
                    *prev_total = total_time;
                }
            }
        }

        // Read Memory from /proc/meminfo
        if let Ok(content) = std::fs::read_to_string("/proc/meminfo") {
            let mut total_kb: u64 = 0;
            let mut free_kb: u64 = 0;
            let mut buffers_kb: u64 = 0;
            let mut cached_kb: u64 = 0;
            for line in content.lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    match parts[0] {
                        "MemTotal:" => total_kb = parts[1].parse().unwrap_or(0),
                        "MemFree:" => free_kb = parts[1].parse().unwrap_or(0),
                        "Buffers:" => buffers_kb = parts[1].parse().unwrap_or(0),
                        "Cached:" => cached_kb = parts[1].parse().unwrap_or(0),
                        _ => {}
                    }
                }
            }
            mem_total = total_kb * 1024;
            let mem_free = (free_kb + buffers_kb + cached_kb) * 1024;
            mem_used = mem_total.saturating_sub(mem_free);
        }
    }

    // 2. Network speeds - Read bytes and compute delta
    let mut net_in_sec = 0.0;
    let mut net_out_sec = 0.0;

    let mut current_net_in = 0u64;
    let mut current_net_out = 0u64;

    #[cfg(target_os = "windows")]
    unsafe {
        use windows_sys::Win32::NetworkManagement::IpHelper::{GetIfTable2, MIB_IF_TABLE2};
        let mut table: *mut MIB_IF_TABLE2 = std::ptr::null_mut();
        if GetIfTable2(&mut table) == 0 && !table.is_null() {
            let count = (*table).NumEntries as usize;
            let table_ptr = table as *const u8;
            // The table has a NumEntries(4B) + Table array of MIB_IF_ROW2 records (each row is ~480B)
            // To be safe and simple, we scan the pointer offset for rows
            // MIB_IF_ROW2 structure offset has OutOctets and InOctets
            // Let's sum up all interface octets
            let row_size = std::mem::size_of::<windows_sys::Win32::NetworkManagement::IpHelper::MIB_IF_ROW2>();
            let table_offset = 8; // skip NumEntries (usually aligned to 8 bytes)
            for i in 0..count {
                let row_ptr = table_ptr.add(table_offset + i * row_size) as *const windows_sys::Win32::NetworkManagement::IpHelper::MIB_IF_ROW2;
                // Accumulate InOctets and OutOctets
                current_net_in += (*row_ptr).InOctets;
                current_net_out += (*row_ptr).OutOctets;
            }
            // Free the table allocated by GetIfTable2
            // Actually, we can use FreeMibTable from IpHelper
            use windows_sys::Win32::NetworkManagement::IpHelper::FreeMibTable;
            FreeMibTable(table as *mut std::ffi::c_void);
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        if let Ok(content) = std::fs::read_to_string("/proc/net/dev") {
            for line in content.lines().skip(2) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 10 {
                    let rx_bytes: u64 = parts[1].parse().unwrap_or(0);
                    let tx_bytes: u64 = parts[9].parse().unwrap_or(0);
                    current_net_in += rx_bytes;
                    current_net_out += tx_bytes;
                }
            }
        }
    }

    let elapsed = last_net_check.elapsed().as_secs_f64();
    if elapsed > 0.0 && *prev_net_in > 0 {
        let diff_in = current_net_in.saturating_sub(*prev_net_in);
        let diff_out = current_net_out.saturating_sub(*prev_net_out);
        net_in_sec = diff_in as f64 / elapsed;
        net_out_sec = diff_out as f64 / elapsed;
    }
    *prev_net_in = current_net_in;
    *prev_net_out = current_net_out;
    *last_net_check = Instant::now();

    SysMetrics {
        cpu_percent: cpu_percent.clamp(0.0, 100.0),
        mem_total,
        mem_used,
        net_in_sec,
        net_out_sec,
    }
}

fn sysinfo_loop() -> io::Result<()> {
    let mut prev_idle = 0u64;
    let mut prev_total = 0u64;
    let mut prev_net_in = 0u64;
    let mut prev_net_out = 0u64;
    let mut last_net_check = Instant::now();

    let mut cpu_history: Vec<f64> = vec![0.0; 40];
    let mut net_in_history: Vec<f64> = vec![0.0; 40];
    let mut net_out_history: Vec<f64> = vec![0.0; 40];

    // Seed initial values
    let _ = get_system_metrics(&mut prev_idle, &mut prev_total, &mut prev_net_in, &mut prev_net_out, &mut last_net_check);
    std::thread::sleep(Duration::from_millis(200));

    let (mut cols, mut rows) = tui_util::terminal_size();
    let mut last_update = Instant::now();

    loop {
        let (new_cols, new_rows) = tui_util::terminal_size();
        if new_cols != cols || new_rows != rows {
            cols = new_cols;
            rows = new_rows;
            print!("\x1B[2J\x1B[H");
            let _ = io::stdout().flush();
        }

        // Fetch updates once per second
        if last_update.elapsed() >= Duration::from_millis(950) {
            let metrics = get_system_metrics(&mut prev_idle, &mut prev_total, &mut prev_net_in, &mut prev_net_out, &mut last_net_check);
            
            cpu_history.remove(0);
            cpu_history.push(metrics.cpu_percent);

            net_in_history.remove(0);
            net_in_history.push(metrics.net_in_sec);

            net_out_history.remove(0);
            net_out_history.push(metrics.net_out_sec);

            last_update = Instant::now();
        }

        let metrics = get_system_metrics(&mut prev_idle, &mut prev_total, &mut prev_net_in, &mut prev_net_out, &mut last_net_check);

        // Render Title
        let mut frame = String::new();
        frame.push_str("\x1B[H");
        frame.push_str("\x1B[1;30;46m ir sysinfo Dashboard \x1B[0m");
        frame.push_str("\x1B[K\n");
        frame.push_str(&"━".repeat(cols as usize));
        frame.push_str("\x1B[K\n");

        let content_height = rows.saturating_sub(5) as usize;

        // Render CPU Usage & Sparkline
        frame.push_str(&format!("  \x1B[1;36mCPU LOAD:\x1B[0m {:.1}%  ", metrics.cpu_percent));
        // CPU Bar
        let cpu_bar_w = (cols as usize).saturating_sub(25).min(40);
        let filled_w = (metrics.cpu_percent / 100.0 * cpu_bar_w as f64).round() as usize;
        frame.push_str("\x1B[32m");
        frame.push_str(&"█".repeat(filled_w));
        frame.push_str("\x1B[90m");
        frame.push_str(&"░".repeat(cpu_bar_w - filled_w));
        frame.push_str("\x1B[0m\n");

        // CPU Sparkline
        frame.push_str("  \x1B[90mCPU History (40s):\x1B[0m ");
        frame.push_str(&render_sparkline(&cpu_history, 0.0, 100.0));
        frame.push_str("\x1B[K\n\n");

        // Render Memory usage
        let mem_percent = 100.0 * (metrics.mem_used as f64 / metrics.mem_total as f64);
        frame.push_str(&format!(
            "  \x1B[1;35mRAM USAGE:\x1B[0m {:.1}% ({:.1} GB / {:.1} GB)  ",
            mem_percent,
            metrics.mem_used as f64 / (1024 * 1024 * 1024) as f64,
            metrics.mem_total as f64 / (1024 * 1024 * 1024) as f64
        ));
        let mem_bar_w = (cols as usize).saturating_sub(42).min(30);
        let mem_filled = (mem_percent / 100.0 * mem_bar_w as f64).round() as usize;
        frame.push_str("\x1B[35m");
        frame.push_str(&"█".repeat(mem_filled));
        frame.push_str("\x1B[90m");
        frame.push_str(&"░".repeat(mem_bar_w - mem_filled));
        frame.push_str("\x1B[0m\x1B[K\n\n");

        // Render Network speed graphs
        let max_net_in = net_in_history.iter().fold(0.1f64, |a, &b| a.max(b));
        let max_net_out = net_out_history.iter().fold(0.1f64, |a, &b| a.max(b));
        
        frame.push_str(&format!(
            "  \x1B[1;32mNET IN:\x1B[0m  {} /s  |  Sparkline: ",
            format_speed(metrics.net_in_sec)
        ));
        frame.push_str(&render_sparkline(&net_in_history, 0.0, max_net_in));
        frame.push_str("\x1B[K\n");

        frame.push_str(&format!(
            "  \x1B[1;31mNET OUT:\x1B[0m {} /s  |  Sparkline: ",
            format_speed(metrics.net_out_sec)
        ));
        frame.push_str(&render_sparkline(&net_out_history, 0.0, max_net_out));
        frame.push_str("\x1B[K\n\n");

        // Padding
        for _ in 9..content_height {
            frame.push_str("\x1B[K\n");
        }

        // Render Footer
        frame.push_str(&"━".repeat(cols as usize));
        frame.push_str("\x1B[K\n");
        frame.push_str(" \x1B[1;30;47m Q / Esc \x1B[0m Quit  |  Dashboard updates live in real-time...");

        print!("{}", frame);
        let _ = io::stdout().flush();

        // Keyboard poll
        let key = tui_util::poll_key();
        match key {
            Key::Esc | Key::Char('q') | Key::Char('Q') => {
                break;
            }
            _ => {}
        }

        std::thread::sleep(Duration::from_millis(200));
    }

    Ok(())
}

fn render_sparkline(history: &[f64], min: f64, max: f64) -> String {
    let spark_chars = [' ', ' ', '▂', '▃', '▄', '▅', '▆', '▇', '█'];
    let range = max - min;
    let mut spark = String::new();
    for &val in history {
        let normalized = if range > 0.0 {
            ((val - min) / range * 8.0).round() as usize
        } else {
            0
        };
        let idx = normalized.min(8);
        spark.push(spark_chars[idx]);
    }
    spark
}

fn format_speed(bytes_sec: f64) -> String {
    if bytes_sec >= 1024.0 * 1024.0 {
        format!("{:.1} MB", bytes_sec / (1024.0 * 1024.0))
    } else if bytes_sec >= 1024.0 {
        format!("{:.1} KB", bytes_sec / 1024.0)
    } else {
        format!("{:.0} B", bytes_sec)
    }
}
