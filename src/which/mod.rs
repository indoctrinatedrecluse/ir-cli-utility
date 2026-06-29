use crate::WhichOptions;
use std::collections::HashSet;
use std::env;
use std::path::{Path, PathBuf};

pub fn which(command: &str, options: WhichOptions) {
    if command.contains('/') || command.contains('\\') {
        if is_executable(Path::new(command)) {
            println!("{}", Path::new(command).display());
        } else {
            eprintln!("Error: '{}' not found or not executable.", command);
        }
        return;
    }

    let path_var = match env::var_os("PATH") {
        Some(value) => value,
        None => {
            eprintln!("Error: PATH is not set.");
            return;
        }
    };

    let mut printed = HashSet::new();
    let mut found = false;

    for dir in env::split_paths(&path_var) {
        for candidate in candidates(&dir, command) {
            if is_executable(&candidate) {
                let display = candidate.display().to_string();
                if printed.insert(display.clone()) {
                    println!("{}", display);
                    found = true;
                    if !options.all {
                        return;
                    }
                }
            }
        }
    }

    if !found {
        eprintln!("Error: '{}' not found in PATH.", command);
    }
}

#[cfg(windows)]
fn candidates(dir: &Path, command: &str) -> Vec<PathBuf> {
    let command_path = Path::new(command);
    if command_path.extension().is_some() {
        return vec![dir.join(command)];
    }

    let pathext = env::var("PATHEXT").unwrap_or_else(|_| ".COM;.EXE;.BAT;.CMD".to_string());
    pathext
        .split(';')
        .filter(|ext| !ext.trim().is_empty())
        .map(|ext| {
            let ext = ext.trim();
            if ext.starts_with('.') {
                dir.join(format!("{}{}", command, ext))
            } else {
                dir.join(format!("{}.{}", command, ext))
            }
        })
        .collect()
}

#[cfg(not(windows))]
fn candidates(dir: &Path, command: &str) -> Vec<PathBuf> {
    vec![dir.join(command)]
}

#[cfg(windows)]
fn is_executable(path: &Path) -> bool {
    path.is_file()
}

#[cfg(unix)]
fn is_executable(path: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;

    match path.metadata() {
        Ok(metadata) if metadata.is_file() => metadata.permissions().mode() & 0o111 != 0,
        _ => false,
    }
}

#[cfg(all(not(windows), not(unix)))]
fn is_executable(path: &Path) -> bool {
    path.is_file()
}
