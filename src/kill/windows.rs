use crate::KillOptions;
use windows_sys::Win32::Foundation::{CloseHandle, INVALID_HANDLE_VALUE};
use windows_sys::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, Process32FirstW, Process32NextW, PROCESSENTRY32W, TH32CS_SNAPPROCESS,
};
use windows_sys::Win32::System::Threading::{
    OpenProcess, TerminateProcess, PROCESS_TERMINATE,
};

// Convert wide char array to String
fn string_from_wide(wide: &[u16]) -> String {
    let len = wide.iter().position(|&c| c == 0).unwrap_or(wide.len());
    String::from_utf16_lossy(&wide[..len])
}

fn get_pids_by_name(name_filter: &str) -> Vec<(u32, String)> {
    let mut results = Vec::new();

    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        if snapshot == INVALID_HANDLE_VALUE {
            return results;
        }

        let mut entry = std::mem::zeroed::<PROCESSENTRY32W>();
        entry.dwSize = std::mem::size_of::<PROCESSENTRY32W>() as u32;

        if Process32FirstW(snapshot, &mut entry) != 0 {
            loop {
                let name = string_from_wide(&entry.szExeFile);
                let matches_direct = name.eq_ignore_ascii_case(name_filter);
                let matches_with_exe = !name_filter.to_lowercase().ends_with(".exe")
                    && name.eq_ignore_ascii_case(&(name_filter.to_owned() + ".exe"));

                if matches_direct || matches_with_exe {
                    results.push((entry.th32ProcessID, name));
                }
                if Process32NextW(snapshot, &mut entry) == 0 {
                    break;
                }
            }
        }
        CloseHandle(snapshot);
    }

    results
}

fn kill_pid(pid: u32) -> Result<(), String> {
    unsafe {
        let h_process = OpenProcess(PROCESS_TERMINATE, 0, pid);
        if h_process != 0 {
            let result = TerminateProcess(h_process, 1);
            CloseHandle(h_process);
            if result != 0 {
                Ok(())
            } else {
                let err = std::io::Error::last_os_error();
                Err(format!("{}", err))
            }
        } else {
            let err = std::io::Error::last_os_error();
            Err(format!("{}", err))
        }
    }
}

pub fn kill(target: &str, options: KillOptions) {
    if let Ok(pid) = target.parse::<u32>() {
        // Target is a PID
        match kill_pid(pid) {
            Ok(_) => println!("Successfully terminated process {}", pid),
            Err(e) => {
                eprintln!("Error: Failed to terminate process {}: {}", pid, e);
                std::process::exit(1);
            }
        }
    } else {
        // Target is a process name
        let matching = get_pids_by_name(target);
        if matching.is_empty() {
            eprintln!("Error: No running process found matching name '{}'.", target);
            std::process::exit(1);
        }

        if matching.len() > 1 && !options.all {
            eprintln!("Error: Multiple processes match name '{}':", target);
            for (pid, name) in &matching {
                eprintln!("  PID: {} ({})", pid, name);
            }
            eprintln!("Use '-a' / '--all' to terminate all matching processes, or specify the PID directly.");
            std::process::exit(1);
        }

        // Perform termination
        let mut failed = false;
        for (pid, name) in matching {
            match kill_pid(pid) {
                Ok(_) => println!("Successfully terminated process {} ({})", pid, name),
                Err(e) => {
                    eprintln!("Error: Failed to terminate process {} ({}): {}", pid, name, e);
                    failed = true;
                }
            }
        }
        if failed {
            std::process::exit(1);
        }
    }
}
