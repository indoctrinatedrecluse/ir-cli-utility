use std::fs::{OpenOptions, File};
use std::io::{Write, BufReader, BufRead};
use std::path::{Path, PathBuf};

fn get_shell_profile_paths() -> Vec<PathBuf> {
    let mut files = Vec::new();
    if let Some(home) = std::env::var_os("HOME") {
        let home_path = Path::new(&home);
        files.push(home_path.join(".bashrc"));
        files.push(home_path.join(".zshrc"));
        files.push(home_path.join(".profile"));
    }
    files
}

pub fn list_path() {
    println!("Active Process PATH:");
    if let Ok(process_path) = std::env::var("PATH") {
        for entry in process_path.split(':') {
            let trimmed = entry.trim();
            if !trimmed.is_empty() {
                println!("  {}", trimmed);
            }
        }
    } else {
        println!("  <None>");
    }
}

pub fn add_path(dir: &str) {
    let normalized = dir.replace("\\", "/");
    let export_line = format!("export PATH=\"{}:$PATH\"", normalized);

    // Locate home profiles
    let profiles = get_shell_profile_paths();
    let mut target_profile = None;

    for profile in &profiles {
        if profile.exists() {
            target_profile = Some(profile.clone());
            break;
        }
    }

    // Default to ~/.bashrc if none exist
    let target = target_profile.unwrap_or_else(|| {
        profiles.first().cloned().unwrap_or_else(|| PathBuf::from(".bashrc"))
    });

    // Check if export line already exists in target
    let mut already_exists = false;
    if target.exists() {
        if let Ok(file) = File::open(&target) {
            let reader = BufReader::new(file);
            for line in reader.lines().map_while(Result::ok) {
                if line.trim() == export_line {
                    already_exists = true;
                    break;
                }
            }
        }
    }

    if already_exists {
        println!("Info: PATH export for '{}' already exists in '{}'.", normalized, target.display());
        return;
    }

    // Append to file
    let mut file = match OpenOptions::new()
        .create(true)
        .append(true)
        .open(&target)
    {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error: Failed to open profile '{}' for writing: {}", target.display(), e);
            std::process::exit(1);
        }
    };

    if let Err(e) = writeln!(file, "\n{}", export_line) {
        eprintln!("Error: Failed to write to profile '{}': {}", target.display(), e);
        std::process::exit(1);
    }

    println!("Success: Appended PATH export to '{}'.", target.display());
    println!("Please restart your shell or run 'source {}' to apply changes.", target.display());
}

pub fn remove_path(dir: &str) {
    let normalized = dir.replace("\\", "/");
    let export_line = format!("export PATH=\"{}:$PATH\"", normalized);

    let profiles = get_shell_profile_paths();
    let mut removed_count = 0;

    for profile in profiles {
        if !profile.exists() {
            continue;
        }

        let file = match File::open(&profile) {
            Ok(f) => f,
            Err(_) => continue,
        };

        let reader = BufReader::new(file);
        let mut new_lines = Vec::new();
        let mut file_changed = false;

        for line in reader.lines().map_while(Result::ok) {
            if line.trim() == export_line {
                file_changed = true;
                removed_count += 1;
            } else {
                new_lines.push(line);
            }
        }

        if file_changed {
            let mut write_file = match OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(&profile)
            {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("Error: Failed to open '{}' for writing: {}", profile.display(), e);
                    std::process::exit(1);
                }
            };

            for line in new_lines {
                if let Err(e) = writeln!(write_file, "{}", line) {
                    eprintln!("Error: Failed to write back to '{}': {}", profile.display(), e);
                    std::process::exit(1);
                }
            }
        }
    }

    if removed_count > 0 {
        println!("Success: Removed PATH export for '{}' from profile files.", normalized);
    } else {
        println!("Info: No PATH export found for '{}' in profile files.", normalized);
    }
}
