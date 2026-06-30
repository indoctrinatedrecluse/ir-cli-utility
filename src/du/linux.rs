use crate::DuOptions;
use std::path::Path;

fn format_size_human(bytes: u64) -> String {
    if bytes == 0 { return "0B".to_string(); }
    const UNITS: [&str; 5] = ["B", "K", "M", "G", "T"];
    let bytes_f = bytes as f64;
    let digit_groups = (bytes_f.log10() / 1024.0f64.log10()).floor() as i32;
    let unit_index = digit_groups.min(UNITS.len() as i32 - 1).max(0);
    
    if unit_index == 0 {
        return format!("{}B", bytes);
    }
    
    let size = bytes_f / 1024.0f64.powi(unit_index);
    if size >= 10.0 {
        format!("{:.0}{}", size.round(), UNITS[unit_index as usize])
    } else {
        format!("{:.1}{}", size, UNITS[unit_index as usize])
    }
}

fn print_item(path: &str, bytes: u64, options: &DuOptions) {
    let size_str = if options.human_readable {
        format_size_human(bytes)
    } else if options.megabytes {
        let mb = (bytes + 1048575) / 1048576;
        format!("{}", mb)
    } else {
        // Default / kilobytes (rounded up to 1024-byte blocks)
        let kb = (bytes + 1023) / 1024;
        format!("{}", kb)
    };

    println!("{}\t{}", size_str, path);
}

fn du_traverse(
    path: &Path,
    current_depth: usize,
    max_depth: Option<usize>,
    options: &DuOptions,
) -> Result<u64, std::io::Error> {
    let mut total_size = 0;

    let read_entries = std::fs::read_dir(path)?;
    let mut entries = Vec::new();
    for entry_result in read_entries {
        if let Ok(entry) = entry_result {
            entries.push(entry);
        }
    }

    // Sort alphabetically to maintain consistent ordering
    entries.sort_by(|a, b| a.file_name().cmp(&b.file_name()));

    for entry in entries {
        let is_dir = entry.file_type().map(|t| t.is_dir()).unwrap_or(false);
        let entry_path = entry.path();
        
        if is_dir {
            let dir_size = du_traverse(&entry_path, current_depth + 1, max_depth, options)?;
            total_size += dir_size;
        } else {
            let file_size = entry.metadata().map(|m| m.len()).unwrap_or(0);
            total_size += file_size;
            
            if options.show_all {
                let should_print_file = match max_depth {
                    Some(max) => current_depth + 1 <= max,
                    None => true,
                };
                if should_print_file {
                    print_item(&entry_path.to_string_lossy(), file_size, options);
                }
            }
        }
    }

    let should_print_dir = match max_depth {
        Some(max) => current_depth <= max,
        None => true,
    };
    if should_print_dir {
        print_item(&path.to_string_lossy(), total_size, options);
    }

    Ok(total_size)
}

pub fn du(paths: Vec<String>, options: DuOptions) {
    let roots = if paths.is_empty() {
        vec![".".to_string()]
    } else {
        paths
    };

    let mut grand_total = 0;

    let max_depth = if options.summarize {
        Some(0)
    } else {
        options.max_depth
    };

    for root in &roots {
        let path = Path::new(root);
        if !path.exists() {
            eprintln!("Error: Path '{}' does not exist.", root);
            continue;
        }

        let size = if path.is_file() {
            let file_size = path.metadata().map(|m| m.len()).unwrap_or(0);
            print_item(root, file_size, &options);
            file_size
        } else {
            match du_traverse(path, 0, max_depth, &options) {
                Ok(size) => size,
                Err(err) => {
                    eprintln!("Error traversing '{}': {}", root, err);
                    0
                }
            }
        };

        grand_total += size;
    }

    if options.total {
        print_item("total", grand_total, &options);
    }
}
