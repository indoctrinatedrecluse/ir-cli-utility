use crate::TreeOptions;
use std::path::Path;
use std::os::unix::fs::PermissionsExt;

fn format_size(bytes: u64) -> String {
    if bytes == 0 { return "0 B".to_string(); }
    const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];
    let digit_groups = (bytes as f64).log10() / (1024.0f64).log10();
    let unit_index = (digit_groups.floor() as i32).min(UNITS.len() as i32 - 1);
    let size = bytes as f64 / 1024.0f64.powi(unit_index);
    format!("{:.2} {}", size, UNITS[unit_index as usize])
}

fn get_permissions(metadata: &std::fs::Metadata) -> String {
    let mode = metadata.permissions().mode();
    let type_char = if metadata.is_dir() {
        'd'
    } else if metadata.file_type().is_symlink() {
        'l'
    } else {
        '-'
    };
    let user = format!(
        "{}{}{}",
        if mode & 0o400 != 0 { 'r' } else { '-' },
        if mode & 0o200 != 0 { 'w' } else { '-' },
        if mode & 0o100 != 0 { 'x' } else { '-' }
    );
    let group = format!(
        "{}{}{}",
        if mode & 0o040 != 0 { 'r' } else { '-' },
        if mode & 0o020 != 0 { 'w' } else { '-' },
        if mode & 0o010 != 0 { 'x' } else { '-' }
    );
    let other = format!(
        "{}{}{}",
        if mode & 0o004 != 0 { 'r' } else { '-' },
        if mode & 0o002 != 0 { 'w' } else { '-' },
        if mode & 0o001 != 0 { 'x' } else { '-' }
    );
    format!("{}{}{}{}", type_char, user, group, other)
}

fn traverse_dir(
    dir_path: &Path,
    current_depth: usize,
    prefix: &str,
    options: &TreeOptions,
    dir_count: &mut usize,
    file_count: &mut usize,
) {
    if let Some(max_depth) = options.max_depth {
        if current_depth >= max_depth {
            return;
        }
    }

    let read_entries = match std::fs::read_dir(dir_path) {
        Ok(entries) => entries,
        Err(err) => {
            eprintln!("Error reading directory '{}': {}", dir_path.display(), err);
            return;
        }
    };

    let mut entries = Vec::new();
    for entry_result in read_entries {
        if let Ok(entry) = entry_result {
            let file_name = entry.file_name().to_string_lossy().into_owned();
            if file_name == "." || file_name == ".." {
                continue;
            }
            if !options.show_all && file_name.starts_with('.') {
                continue;
            }
            let is_dir = entry.file_type().map(|t| t.is_dir()).unwrap_or(false);
            if options.dirs_only && !is_dir {
                continue;
            }
            entries.push(entry);
        }
    }

    // Sort alphabetically
    entries.sort_by(|a, b| a.file_name().cmp(&b.file_name()));

    let len = entries.len();
    for (i, entry) in entries.iter().enumerate() {
        let is_last = i == len - 1;
        let is_dir = entry.file_type().map(|t| t.is_dir()).unwrap_or(false);

        // Update counts
        if is_dir {
            *dir_count += 1;
        } else {
            *file_count += 1;
        }

        // Print prefix / drawing
        let mut line_prefix = String::new();
        if !options.no_indent {
            line_prefix.push_str(prefix);
            if is_last {
                line_prefix.push_str("└── ");
            } else {
                line_prefix.push_str("├── ");
            }
        }

        // Print metadata (perms, size)
        let mut meta_str = String::new();
        let show_size = options.show_size || options.human_readable;
        if options.show_perms || show_size {
            if let Ok(metadata) = entry.metadata() {
                meta_str.push('[');
                let mut parts = Vec::new();
                if options.show_perms {
                    parts.push(get_permissions(&metadata));
                }
                if show_size {
                    let size = metadata.len();
                    if options.human_readable {
                        parts.push(format_size(size));
                    } else {
                        parts.push(format!("{}", size));
                    }
                }
                meta_str.push_str(&parts.join(" "));
                meta_str.push_str("]  ");
            }
        }

        // Print name or full path
        let display_name = if options.full_path {
            entry.path().display().to_string()
        } else {
            entry.file_name().to_string_lossy().into_owned()
        };

        println!("{}{}{}", line_prefix, meta_str, display_name);

        // Recurse into directories
        if is_dir {
            let next_prefix = if !options.no_indent {
                let suffix = if is_last { "    " } else { "│   " };
                format!("{}{}", prefix, suffix)
            } else {
                String::new()
            };
            traverse_dir(&entry.path(), current_depth + 1, &next_prefix, options, dir_count, file_count);
        }
    }
}

pub fn tree(path: &str, options: TreeOptions) {
    let root_path = Path::new(path);
    if !root_path.exists() {
        eprintln!("Error: Path '{}' does not exist.", path);
        return;
    }

    // Print root path first
    println!("{}", path);

    let mut dir_count = 0;
    let mut file_count = 0;

    traverse_dir(root_path, 0, "", &options, &mut dir_count, &mut file_count);

    if !options.no_report {
        println!("\n{} directories, {} files", dir_count, file_count);
    }
}
