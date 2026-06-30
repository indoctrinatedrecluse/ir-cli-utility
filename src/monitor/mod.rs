use std::process::Command;
#[cfg(target_os = "linux")]
use std::path::Path;

#[cfg(target_os = "windows")]
pub fn monitor() {
    let exe_name = "term-sys-monitor-windows.exe";
    let current_exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|d| d.to_path_buf()));

    let local_path = current_exe_dir.map(|d| d.join(exe_name));
    
    let exe_to_run = if let Some(ref path) = local_path {
        if path.exists() {
            path.to_string_lossy().to_string()
        } else {
            exe_name.to_string()
        }
    } else {
        exe_name.to_string()
    };

    println!("Launching system monitor in a new window...");
    let status = Command::new("cmd")
        .args(["/c", "start", "", &exe_to_run])
        .status();

    if status.is_err() || !status.unwrap().success() {
        eprintln!("Error: Failed to launch system monitor ({}). Make sure it is installed and in your PATH.", exe_name);
    }
}

#[cfg(target_os = "linux")]
pub fn monitor() {
    let exe_name = "term-sys-monitor-linux";
    let current_exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|d| d.to_path_buf()));

    let local_path = current_exe_dir.map(|d| d.join(exe_name));

    let exe_to_run = if let Some(ref path) = local_path {
        if path.exists() {
            path.to_path_buf()
        } else {
            Path::new(exe_name).to_path_buf()
        }
    } else {
        Path::new(exe_name).to_path_buf()
    };

    println!("Launching system monitor...");
    let status = Command::new(&exe_to_run)
        .status();

    if status.is_err() || !status.unwrap().success() {
        eprintln!("Error: Failed to launch system monitor ({}). Make sure it is installed and in your PATH.", exe_name);
    }
}
