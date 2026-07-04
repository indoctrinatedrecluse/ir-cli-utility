use crate::pmon::ProcessInfo;
use std::collections::HashMap;
use std::io::Result;
use windows_sys::Win32::Foundation::{CloseHandle, INVALID_HANDLE_VALUE, FILETIME};
use windows_sys::Win32::System::SystemInformation::{GlobalMemoryStatusEx, MEMORYSTATUSEX};
use windows_sys::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, Process32FirstW, Process32NextW, PROCESSENTRY32W, TH32CS_SNAPPROCESS,
};
use windows_sys::Win32::System::Threading::{
    OpenProcess, GetProcessTimes, PROCESS_QUERY_LIMITED_INFORMATION, PROCESS_TERMINATE, TerminateProcess, GetSystemTimes,
};
use windows_sys::Win32::System::ProcessStatus::{
    K32GetProcessMemoryInfo, PROCESS_MEMORY_COUNTERS,
};

extern "C" {
    fn _kbhit() -> std::os::raw::c_int;
    fn _getch() -> std::os::raw::c_int;
}

pub enum RawInput {
    Char(char),
    Enter,
    Backspace,
}

pub fn poll_keyboard_input() -> Option<RawInput> {
    unsafe {
        if _kbhit() != 0 {
            let ch = _getch();
            if ch == 13 || ch == 10 {
                Some(RawInput::Enter)
            } else if ch == 8 {
                Some(RawInput::Backspace)
            } else if ch > 0 && ch < 256 {
                if let Some(c) = std::char::from_u32(ch as u32) {
                    Some(RawInput::Char(c))
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }
}

fn filetime_to_u64(ft: FILETIME) -> u64 {
    ((ft.dwHighDateTime as u64) << 32) | (ft.dwLowDateTime as u64)
}

static mut LAST_IDLE: u64 = 0;
static mut LAST_KERNEL: u64 = 0;
static mut LAST_USER: u64 = 0;

pub fn get_system_stats() -> Result<(f64, u64, u64)> {
    let mut mem_status = unsafe { std::mem::zeroed::<MEMORYSTATUSEX>() };
    mem_status.dwLength = std::mem::size_of::<MEMORYSTATUSEX>() as u32;
    
    let mut mem_used = 0;
    let mut mem_total = 1;
    unsafe {
        if GlobalMemoryStatusEx(&mut mem_status) != 0 {
            mem_total = mem_status.ullTotalPhys;
            mem_used = mem_total.saturating_sub(mem_status.ullAvailPhys);
        }
    }

    let mut idle_ft = unsafe { std::mem::zeroed::<FILETIME>() };
    let mut kernel_ft = unsafe { std::mem::zeroed::<FILETIME>() };
    let mut user_ft = unsafe { std::mem::zeroed::<FILETIME>() };

    let mut cpu_pct = 0.0;
    unsafe {
        if GetSystemTimes(&mut idle_ft, &mut kernel_ft, &mut user_ft) != 0 {
            let idle = filetime_to_u64(idle_ft);
            let kernel = filetime_to_u64(kernel_ft);
            let user = filetime_to_u64(user_ft);

            let idle_diff = idle.saturating_sub(LAST_IDLE);
            let kernel_diff = kernel.saturating_sub(LAST_KERNEL);
            let user_diff = user.saturating_sub(LAST_USER);
            let system_diff = kernel_diff.saturating_add(user_diff);

            if system_diff > 0 {
                cpu_pct = ((system_diff.saturating_sub(idle_diff)) as f64 / system_diff as f64) * 100.0;
            }

            LAST_IDLE = idle;
            LAST_KERNEL = kernel;
            LAST_USER = user;
        }
    }

    Ok((cpu_pct, mem_used, mem_total))
}

static mut LAST_SYS_TOTAL: u64 = 0;

fn string_from_wide(wide: &[u16]) -> String {
    let len = wide.iter().position(|&c| c == 0).unwrap_or(wide.len());
    String::from_utf16_lossy(&wide[..len])
}

pub fn get_processes(cpu_cache: &mut HashMap<u32, u64>) -> Result<Vec<ProcessInfo>> {
    let mut processes = Vec::new();

    unsafe {
        // Get current system times to calculate delta system time
        let mut idle_ft = std::mem::zeroed::<FILETIME>();
        let mut kernel_ft = std::mem::zeroed::<FILETIME>();
        let mut user_ft = std::mem::zeroed::<FILETIME>();
        let sys_total = if GetSystemTimes(&mut idle_ft, &mut kernel_ft, &mut user_ft) != 0 {
            filetime_to_u64(kernel_ft) + filetime_to_u64(user_ft)
        } else {
            0
        };
        let sys_diff = sys_total.saturating_sub(LAST_SYS_TOTAL);
        LAST_SYS_TOTAL = sys_total;

        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        if snapshot == INVALID_HANDLE_VALUE {
            return Err(std::io::Error::last_os_error());
        }

        let mut entry = std::mem::zeroed::<PROCESSENTRY32W>();
        entry.dwSize = std::mem::size_of::<PROCESSENTRY32W>() as u32;

        let mut new_cache = HashMap::new();

        if Process32FirstW(snapshot, &mut entry) != 0 {
            loop {
                let pid = entry.th32ProcessID;
                if pid != 0 {
                    let name = string_from_wide(&entry.szExeFile);
                    
                    let mut proc_cpu = 0;
                    let mut rss_bytes = 0;

                    let h_process = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, pid);
                    if h_process != 0 {
                        let mut creation_ft = std::mem::zeroed();
                        let mut exit_ft = std::mem::zeroed();
                        let mut kernel_ft = std::mem::zeroed();
                        let mut user_ft = std::mem::zeroed();
                        if GetProcessTimes(h_process, &mut creation_ft, &mut exit_ft, &mut kernel_ft, &mut user_ft) != 0 {
                            proc_cpu = filetime_to_u64(kernel_ft) + filetime_to_u64(user_ft);
                        }

                        let mut pmc = std::mem::zeroed::<PROCESS_MEMORY_COUNTERS>();
                        pmc.cb = std::mem::size_of::<PROCESS_MEMORY_COUNTERS>() as u32;
                        if K32GetProcessMemoryInfo(h_process, &mut pmc, pmc.cb) != 0 {
                            rss_bytes = pmc.WorkingSetSize as u64;
                        }

                        CloseHandle(h_process);
                    }

                    // Calculate CPU usage
                    let mut cpu_usage = 0.0;
                    if let Some(&prev_cpu) = cpu_cache.get(&pid) {
                        let proc_diff = proc_cpu.saturating_sub(prev_cpu);
                        if sys_diff > 0 {
                            cpu_usage = (proc_diff as f64 / sys_diff as f64) * 100.0;
                        }
                    }
                    
                    new_cache.insert(pid, proc_cpu);

                    processes.push(ProcessInfo {
                        pid,
                        name,
                        cpu_usage,
                        rss_bytes,
                        state: if cpu_usage > 0.1 { 'R' } else { 'S' }, // Approximate state
                        cpu_time_secs: proc_cpu / 10_000_000, // FILETIME is in 100ns units
                    });
                }

                if Process32NextW(snapshot, &mut entry) == 0 {
                    break;
                }
            }
        }

        CloseHandle(snapshot);
        *cpu_cache = new_cache;
    }

    Ok(processes)
}

pub fn kill_process(pid: u32) -> Result<()> {
    unsafe {
        let h_process = OpenProcess(PROCESS_TERMINATE, 0, pid);
        if h_process != 0 {
            let res = TerminateProcess(h_process, 1);
            CloseHandle(h_process);
            if res != 0 {
                Ok(())
            } else {
                Err(std::io::Error::last_os_error())
            }
        } else {
            Err(std::io::Error::last_os_error())
        }
    }
}
