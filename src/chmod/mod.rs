use crate::ChmodOptions;
use std::path::Path;
use walkdir::WalkDir;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
use linux::chmod_single;

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
use windows::chmod_single;

pub fn chmod(mode: &str, paths: Vec<String>, options: ChmodOptions) {
    let val = match u32::from_str_radix(mode, 8) {
        Ok(v) => v,
        Err(_) => {
            eprintln!("Error: Invalid octal mode '{}'. Mode must be a valid octal number (e.g., 644, 755).", mode);
            return;
        }
    };

    if val > 0o7777 {
        eprintln!("Error: Octal mode '{}' is out of range.", mode);
        return;
    }

    if paths.is_empty() {
        eprintln!("Error: Missing file operand.");
        return;
    }

    for path_str in &paths {
        let path = Path::new(path_str);
        if !path.exists() {
            eprintln!("Error: Cannot access '{}': No such file or directory.", path_str);
            continue;
        }

        if options.recursive && path.is_dir() {
            for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
                if let Err(e) = chmod_single(entry.path(), val) {
                    eprintln!("Error: Failed to change permissions of '{}': {}", entry.path().display(), e);
                }
            }
        } else {
            if let Err(e) = chmod_single(path, val) {
                eprintln!("Error: Failed to change permissions of '{}': {}", path.display(), e);
            }
        }
    }
}
