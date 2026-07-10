use crate::ChmodOptions;
use std::path::Path;
use walkdir::WalkDir;

#[cfg(unix)]
mod unix;
#[cfg(unix)]
use unix::{get_mode, set_mode};

#[cfg(windows)]
mod windows;
#[cfg(windows)]
use windows::{get_mode, set_mode};

pub fn chmod(mode_str: &str, paths: Vec<String>, options: ChmodOptions) {
    if paths.is_empty() {
        eprintln!("Error: Missing file operand.");
        return;
    }

    // Try parsing as octal mode first
    let mut is_octal = false;
    let mut octal_val = 0;
    if mode_str.chars().all(|c| c.is_digit(8)) {
        if let Ok(v) = u32::from_str_radix(mode_str, 8) {
            if v <= 0o7777 {
                is_octal = true;
                octal_val = v;
            }
        }
    }

    for path_str in &paths {
        let path = Path::new(path_str);
        if !path.exists() {
            eprintln!("Error: Cannot access '{}': No such file or directory.", path_str);
            continue;
        }

        if options.recursive && path.is_dir() {
            for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
                if let Err(e) = chmod_single(entry.path(), mode_str, is_octal, octal_val, &options) {
                    eprintln!("Error: Failed to change permissions of '{}': {}", entry.path().display(), e);
                }
            }
        } else {
            if let Err(e) = chmod_single(path, mode_str, is_octal, octal_val, &options) {
                eprintln!("Error: Failed to change permissions of '{}': {}", path.display(), e);
            }
        }
    }
}

fn chmod_single(
    path: &Path,
    mode_str: &str,
    is_octal: bool,
    octal_val: u32,
    options: &ChmodOptions,
) -> std::io::Result<()> {
    let current_mode = get_mode(path)?;
    let target_mode = if is_octal {
        octal_val
    } else {
        match parse_symbolic_mode(current_mode, mode_str) {
            Ok(m) => m,
            Err(e) => {
                return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, e));
            }
        }
    };

    if current_mode != target_mode {
        set_mode(path, target_mode)?;
        if options.verbose || options.changes {
            println!(
                "mode of '{}' changed from {:04o} ({}) to {:04o} ({})",
                path.display(),
                current_mode,
                format_mode(current_mode),
                target_mode,
                format_mode(target_mode)
            );
        }
    } else {
        if options.verbose {
            println!(
                "mode of '{}' retained as {:04o} ({})",
                path.display(),
                current_mode,
                format_mode(current_mode)
            );
        }
    }
    Ok(())
}

fn format_mode(mode: u32) -> String {
    let mut s = String::new();
    s.push(if mode & 0o400 != 0 { 'r' } else { '-' });
    s.push(if mode & 0o200 != 0 { 'w' } else { '-' });
    s.push(if mode & 0o100 != 0 { 'x' } else { '-' });
    s.push(if mode & 0o040 != 0 { 'r' } else { '-' });
    s.push(if mode & 0o020 != 0 { 'w' } else { '-' });
    s.push(if mode & 0o010 != 0 { 'x' } else { '-' });
    s.push(if mode & 0o004 != 0 { 'r' } else { '-' });
    s.push(if mode & 0o002 != 0 { 'w' } else { '-' });
    s.push(if mode & 0o001 != 0 { 'x' } else { '-' });
    s
}

fn parse_symbolic_mode(current_mode: u32, symbol: &str) -> Result<u32, String> {
    let mut new_mode = current_mode;
    for clause in symbol.split(',') {
        let clause = clause.trim();
        if clause.is_empty() {
            continue;
        }

        let op_pos = clause.find(|c| c == '+' || c == '-' || c == '=');
        let (classes_str, op_char, perms_str) = match op_pos {
            Some(pos) => {
                let op = clause.chars().nth(pos).unwrap();
                (&clause[..pos], op, &clause[pos + 1..])
            }
            None => return Err(format!("Invalid symbolic mode clause: '{}'", clause)),
        };

        let mut target_user = false;
        let mut target_group = false;
        let mut target_other = false;
        if classes_str.is_empty() {
            target_user = true;
            target_group = true;
            target_other = true;
        } else {
            for c in classes_str.chars() {
                match c {
                    'u' => target_user = true,
                    'g' => target_group = true,
                    'o' => target_other = true,
                    'a' => {
                        target_user = true;
                        target_group = true;
                        target_other = true;
                    }
                    _ => return Err(format!("Invalid class character: '{}'", c)),
                }
            }
        }

        let mut r = false;
        let mut w = false;
        let mut x = false;
        for c in perms_str.chars() {
            match c {
                'r' => r = true,
                'w' => w = true,
                'x' => x = true,
                _ => return Err(format!("Invalid permission character: '{}'", c)),
            }
        }

        let mut mask = 0;
        let mut val = 0;

        if target_user {
            mask |= 0o700;
            if r { val |= 0o400; }
            if w { val |= 0o200; }
            if x { val |= 0o100; }
        }
        if target_group {
            mask |= 0o070;
            if r { val |= 0o040; }
            if w { val |= 0o020; }
            if x { val |= 0o010; }
        }
        if target_other {
            mask |= 0o007;
            if r { val |= 0o004; }
            if w { val |= 0o002; }
            if x { val |= 0o001; }
        }

        match op_char {
            '+' => {
                new_mode |= val;
            }
            '-' => {
                new_mode &= !val;
            }
            '=' => {
                new_mode = (new_mode & !mask) | val;
            }
            _ => unreachable!(),
        }
    }
    Ok(new_mode & 0o7777)
}
