use crate::KillOptions;
use std::fs;

fn get_pids_by_name(name_filter: &str) -> Vec<(u32, String)> {
    let mut results = Vec::new();
    let proc_dir = match fs::read_dir("/proc") {
        Ok(dir) => dir,
        Err(_) => return results,
    };

    for entry_result in proc_dir {
        let entry = match entry_result {
            Ok(e) => e,
            Err(_) => continue,
        };
        let file_name = entry.file_name();
        let name_str = file_name.to_string_lossy();
        
        if let Ok(pid) = name_str.parse::<u32>() {
            let stat_path = format!("/proc/{}/stat", name_str);
            if let Ok(content) = fs::read_to_string(stat_path) {
                // Find process name between parentheses
                if let Some(first_paren) = content.find('(') {
                    if let Some(last_paren) = content.rfind(')') {
                        let proc_name = content[first_paren + 1..last_paren].to_string();
                        if proc_name.eq_ignore_ascii_case(name_filter) {
                            results.push((pid, proc_name));
                        }
                    }
                }
            }
        }
    }
    results
}

fn kill_pid(pid: u32, force: bool) -> Result<(), String> {
    let sig = if force { libc::SIGKILL } else { libc::SIGTERM };
    unsafe {
        if libc::kill(pid as i32, sig) == 0 {
            Ok(())
        } else {
            let err = std::io::Error::last_os_error();
            Err(format!("{}", err))
        }
    }
}

pub fn kill(target: &str, options: KillOptions) {
    let sig_name = if options.force { "SIGKILL" } else { "SIGTERM" };

    if let Ok(pid) = target.parse::<u32>() {
        // Target is a PID
        match kill_pid(pid, options.force) {
            Ok(_) => println!("Successfully sent {} to process {}", sig_name, pid),
            Err(e) => {
                eprintln!("Error: Failed to kill process {}: {}", pid, e);
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
            eprintln!("Use '-a' / '--all' to kill all matching processes, or specify the PID directly.");
            std::process::exit(1);
        }

        // Perform termination
        let mut failed = false;
        for (pid, name) in matching {
            match kill_pid(pid, options.force) {
                Ok(_) => println!("Successfully sent {} to process {} ({})", sig_name, pid, name),
                Err(e) => {
                    eprintln!("Error: Failed to kill process {} ({}): {}", pid, name, e);
                    failed = true;
                }
            }
        }
        if failed {
            std::process::exit(1);
        }
    }
}
