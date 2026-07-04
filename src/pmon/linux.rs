use crate::pmon::ProcessInfo;
use std::collections::HashMap;
use std::io::Result;

pub enum RawInput {
    Char(char),
    Enter,
    Backspace,
}

pub struct RawModeGuard {
    orig: libc::termios,
}

impl Drop for RawModeGuard {
    fn drop(&mut self) {
        unsafe {
            libc::tcsetattr(0, libc::TCSAFLUSH, &self.orig);
        }
    }
}

pub fn set_raw_mode() -> Option<RawModeGuard> {
    unsafe {
        let mut orig = std::mem::zeroed();
        if libc::tcgetattr(0, &mut orig) == 0 {
            let mut raw = orig;
            raw.c_lflag &= !(libc::ECHO | libc::ICANON);
            raw.c_cc[libc::VMIN] = 0;
            raw.c_cc[libc::VTIME] = 0;
            if libc::tcsetattr(0, libc::TCSAFLUSH, &raw) == 0 {
                return Some(RawModeGuard { orig });
            }
        }
        None
    }
}

pub fn poll_keyboard_input() -> Option<RawInput> {
    let mut buf = [0u8; 1];
    unsafe {
        let n = libc::read(0, buf.as_mut_ptr() as *mut libc::c_void, 1);
        if n > 0 {
            let ch = buf[0];
            if ch == 10 || ch == 13 {
                Some(RawInput::Enter)
            } else if ch == 127 || ch == 8 {
                Some(RawInput::Backspace)
            } else if ch > 0 {
                Some(RawInput::Char(ch as char))
            } else {
                None
            }
        } else {
            None
        }
    }
}

static mut LAST_TOTAL: u64 = 0;
static mut LAST_IDLE: u64 = 0;

pub fn get_system_stats() -> Result<(f64, u64, u64)> {
    // Read CPU
    let stat_content = std::fs::read_to_string("/proc/stat")?;
    let first_line = stat_content.lines().next().unwrap_or("");
    let parts: Vec<&str> = first_line.split_whitespace().skip(1).collect();
    let mut total = 0;
    let mut idle = 0;
    for (i, p) in parts.iter().enumerate() {
        if let Ok(val) = p.parse::<u64>() {
            total += val;
            if i == 3 || i == 4 { // idle and iowait
                idle += val;
            }
        }
    }

    let total_diff = total.saturating_sub(unsafe { LAST_TOTAL });
    let idle_diff = idle.saturating_sub(unsafe { LAST_IDLE });
    
    unsafe {
        LAST_TOTAL = total;
        LAST_IDLE = idle;
    }

    let cpu_pct = if total_diff > 0 {
        ((total_diff.saturating_sub(idle_diff)) as f64 / total_diff as f64) * 100.0
    } else {
        0.0
    };

    // Read Memory
    let mem_content = std::fs::read_to_string("/proc/meminfo")?;
    let mut total_mem = 0;
    let mut avail_mem = 0;
    for line in mem_content.lines() {
        if line.starts_with("MemTotal:") {
            total_mem = line.split_whitespace().nth(1).and_then(|v| v.parse::<u64>().ok()).unwrap_or(0) * 1024;
        } else if line.starts_with("MemAvailable:") {
            avail_mem = line.split_whitespace().nth(1).and_then(|v| v.parse::<u64>().ok()).unwrap_or(0) * 1024;
        }
    }
    let used_mem = total_mem.saturating_sub(avail_mem);

    Ok((cpu_pct, used_mem, total_mem))
}

static mut LAST_SYS_TOTAL: u64 = 0;

pub fn get_processes(cpu_cache: &mut HashMap<u32, u64>) -> Result<Vec<ProcessInfo>> {
    let mut processes = Vec::new();
    let page_size = unsafe { libc::sysconf(libc::_SC_PAGESIZE) as u64 };
    let clk_tck = unsafe { libc::sysconf(libc::_SC_CLK_TCK) as u64 };

    // Get current system ticks to calculate delta system time
    let stat_content = std::fs::read_to_string("/proc/stat").unwrap_or_default();
    let first_line = stat_content.lines().next().unwrap_or("");
    let parts: Vec<&str> = first_line.split_whitespace().skip(1).collect();
    let mut sys_total = 0;
    for p in parts {
        if let Ok(val) = p.parse::<u64>() {
            sys_total += val;
        }
    }
    
    let sys_diff = sys_total.saturating_sub(unsafe { LAST_SYS_TOTAL });
    unsafe { LAST_SYS_TOTAL = sys_total; }

    for entry_res in std::fs::read_dir("/proc")? {
        let entry = entry_res?;
        let name_str = entry.file_name().to_string_lossy().into_owned();
        if let Ok(pid) = name_str.parse::<u32>() {
            // Read stat
            let stat_path = format!("/proc/{}/stat", pid);
            let content = match std::fs::read_to_string(stat_path) {
                Ok(c) => c,
                Err(_) => continue,
            };

            let last_paren = match content.rfind(')') {
                Some(idx) => idx,
                None => continue,
            };
            let first_paren = match content.find('(') {
                Some(idx) => idx,
                None => continue,
            };
            let name = content[first_paren + 1..last_paren].to_string();

            let rest = &content[last_paren + 2..];
            let fields: Vec<&str> = rest.split_whitespace().collect();
            if fields.len() < 15 {
                continue;
            }

            let state = fields[0].chars().next().unwrap_or('?');
            let utime = fields[11].parse::<u64>().unwrap_or(0);
            let stime = fields[12].parse::<u64>().unwrap_or(0);
            let proc_cpu = utime + stime;

            // Calculate CPU %
            let mut cpu_usage = 0.0;
            if let Some(prev_cpu) = cpu_cache.get(&pid) {
                let proc_diff = proc_cpu.saturating_sub(*prev_cpu);
                if sys_diff > 0 {
                    cpu_usage = (proc_diff as f64 / sys_diff as f64) * 100.0;
                }
            }
            cpu_cache.insert(pid, proc_cpu);

            // Read memory (statm)
            let mut rss_bytes = 0;
            let statm_path = format!("/proc/{}/statm", pid);
            if let Ok(statm_content) = std::fs::read_to_string(statm_path) {
                if let Some(rss_pages_str) = statm_content.split_whitespace().nth(1) {
                    if let Ok(rss_pages) = rss_pages_str.parse::<u64>() {
                        rss_bytes = rss_pages * page_size;
                    }
                }
            }

            processes.push(ProcessInfo {
                pid,
                name,
                cpu_usage,
                rss_bytes,
                state,
                cpu_time_secs: proc_cpu / clk_tck,
            });
        }
    }

    Ok(processes)
}

pub fn kill_process(pid: u32) -> Result<()> {
    unsafe {
        let res = libc::kill(pid as libc::pid_t, libc::SIGKILL);
        if res == 0 {
            Ok(())
        } else {
            Err(std::io::Error::last_os_error())
        }
    }
}
