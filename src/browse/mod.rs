use crate::BrowseOptions;
use std::fs::{self, File};
use std::io::{stdout, Read, Write};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

#[cfg(target_os = "windows")]
extern "C" {
    fn _kbhit() -> std::os::raw::c_int;
    fn _getch() -> std::os::raw::c_int;
}

#[cfg(target_os = "windows")]
fn poll_key() -> Option<char> {
    unsafe {
        if _kbhit() != 0 {
            let ch = _getch();
            if ch == 224 {
                let sub = _getch();
                match sub {
                    72 => Some('\x01'), // Up
                    80 => Some('\x02'), // Down
                    75 => Some('\x03'), // Left
                    77 => Some('\x04'), // Right
                    _ => None
                }
            } else {
                Some(ch as u8 as char)
            }
        } else {
            None
        }
    }
}

#[cfg(target_os = "linux")]
struct RawModeGuard {
    orig: libc::termios,
}

#[cfg(target_os = "linux")]
impl Drop for RawModeGuard {
    fn drop(&mut self) {
        unsafe {
            libc::tcsetattr(0, libc::TCSAFLUSH, &self.orig);
        }
    }
}

#[cfg(target_os = "linux")]
fn set_raw_mode() -> Option<RawModeGuard> {
    unsafe {
        let mut orig = std::mem::zeroed();
        if libc::tcgetattr(0, &mut orig) == 0 {
            let mut raw = orig;
            raw.c_lflag &= !(libc::ECHO | libc::ICANON);
            raw.c_cc[libc::VMIN] = 0;
            raw.c_cc[libc::VTIME] = 0;
            if libc::tcsetattr(0, libc::TCSAFLUSH, &raw) == 0 {
                return Some(RawModeGuard { orig });
            }
        }
        None
    }
}

#[cfg(target_os = "linux")]
fn poll_key() -> Option<char> {
    let mut buf = [0u8; 4];
    unsafe {
        let n = libc::read(0, buf.as_mut_ptr() as *mut libc::c_void, 4);
        if n > 0 {
            if buf[0] == 27 && n >= 3 && buf[1] == 91 {
                match buf[2] {
                    65 => Some('\x01'), // Up
                    66 => Some('\x02'), // Down
                    68 => Some('\x03'), // Left
                    67 => Some('\x04'), // Right
                    _ => None
                }
            } else {
                Some(buf[0] as char)
            }
        } else {
            None
        }
    }
}

struct BrowserItem {
    name: String,
    path: PathBuf,
    is_dir: bool,
    size_bytes: u64,
}

fn get_items(path: &Path) -> Vec<BrowserItem> {
    let mut items = Vec::new();
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.filter_map(|e| e.ok()) {
            let p = entry.path();
            let is_dir = p.is_dir();
            let name = p.file_name().unwrap_or(p.as_os_str()).to_string_lossy().to_string();
            let size_bytes = p.metadata().map(|m| m.len()).unwrap_or(0);
            items.push(BrowserItem { name, path: p, is_dir, size_bytes });
        }
    }
    // Sort: directories first, then files
    items.sort_by(|a, b| {
        if a.is_dir != b.is_dir {
            b.is_dir.cmp(&a.is_dir)
        } else {
            a.name.to_lowercase().cmp(&b.name.to_lowercase())
        }
    });
    items
}

fn format_size(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{} B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else if bytes < 1024 * 1024 * 1024 {
        format!("{:.1} MB", bytes as f64 / 1024.0 / 1024.0)
    } else {
        format!("{:.1} GB", bytes as f64 / 1024.0 / 1024.0 / 1024.0)
    }
}

fn get_preview(item: &BrowserItem) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!("\x1b[1;36m=== Properties ===\x1b[0m"));
    lines.push(format!("Name: {}", item.name));
    lines.push(format!("Type: {}", if item.is_dir { "Directory" } else { "File" }));
    if !item.is_dir {
        lines.push(format!("Size: {}", format_size(item.size_bytes)));
        
        // Try reading content preview
        if let Ok(mut f) = File::open(&item.path) {
            let mut buf = [0u8; 1024];
            if let Ok(n) = f.read(&mut buf) {
                let is_text = buf[..n].iter().all(|&b| b == 9 || b == 10 || b == 13 || (b >= 32 && b <= 126));
                if is_text {
                    lines.push(String::new());
                    lines.push(format!("\x1b[1;33m--- Preview ---\x1b[0m"));
                    let content = String::from_utf8_lossy(&buf[..n]);
                    for line in content.lines().take(12) {
                        // Truncate line for clean pane boundary
                        let clean_line: String = line.chars().take(35).collect();
                        lines.push(clean_line);
                    }
                } else {
                    lines.push(String::new());
                    lines.push(format!("\x1b[1;30m[Binary File]\x1b[0m"));
                }
            }
        }
    } else {
        // List directory items
        if let Ok(entries) = fs::read_dir(&item.path) {
            lines.push(String::new());
            lines.push(format!("\x1b[1;33m--- Contents ---\x1b[0m"));
            for entry in entries.filter_map(|e| e.ok()).take(12) {
                let name = entry.file_name().to_string_lossy().to_string();
                let is_sub_dir = entry.path().is_dir();
                let disp = if is_sub_dir {
                    format!("\x1b[1;34m{}/\x1b[0m", name)
                } else {
                    name
                };
                let clean_disp: String = disp.chars().take(45).collect();
                lines.push(clean_disp);
            }
        }
    }
    lines
}

pub fn browse(path_str: &str, _options: BrowseOptions) {
    let mut current_dir = fs::canonicalize(Path::new(path_str)).unwrap_or_else(|_| PathBuf::from("."));
    
    #[cfg(target_os = "linux")]
    let _raw_guard = set_raw_mode();

    let mut selected_idx = 0usize;
    let mut status_message = String::new();
    let mut status_timer = Instant::now();

    // Clear screen first
    print!("\x1b[2J\x1b[H");
    let _ = stdout().flush();

    loop {
        let items = get_items(&current_dir);
        if selected_idx >= items.len() && !items.is_empty() {
            selected_idx = items.len() - 1;
        }

        // Render TUI
        print!("\x1b[H");
        println!("\x1b[1;34m=== Terminal File Browser (browse) ===\x1b[0m\x1b[K");
        println!("Path: \x1b[33m{}\x1b[0m\x1b[K", current_dir.display());
        println!("Keys: \x1b[36m↑/↓\x1b[0m Navigate | \x1b[36mEnter\x1b[0m Enter dir | \x1b[36mBackspace\x1b[0m Up | \x1b[36m'c'/'m'/'r'/'d'\x1b[0m Actions | \x1b[36m'q'\x1b[0m Quit\x1b[K");
        println!("\x1b[34m--------------------------------------------------------------------------------\x1b[0m\x1b[K");

        // Dual pane rendering
        let pane_height = 20usize;
        let preview_lines = if !items.is_empty() {
            get_preview(&items[selected_idx])
        } else {
            vec![format!("\x1b[1;30m[Directory is Empty]\x1b[0m")]
        };

        for row in 0..pane_height {
            // Left Pane (File list)
            let left_str = if row < items.len() {
                let item = &items[row];
                let prefix = if row == selected_idx { "-> " } else { "   " };
                let display_name = if item.is_dir {
                    format!("{}/", item.name)
                } else {
                    item.name.clone()
                };
                
                // Truncate name
                let truncated: String = display_name.chars().take(32).collect();
                let padding = " ".repeat(32_usize.saturating_sub(truncated.chars().count()));

                if row == selected_idx {
                    format!("\x1b[7m{}{}{}\x1b[0m", prefix, truncated, padding)
                } else {
                    let color = if item.is_dir { "\x1b[1;34m" } else { "\x1b[0m" };
                    format!("{}{}{}{}\x1b[0m", color, prefix, truncated, padding)
                }
            } else {
                " ".repeat(35)
            };

            // Right Pane (Preview)
            let mut right_str = String::new();
            if row < preview_lines.len() {
                right_str = preview_lines[row].clone();
            }

            println!("{} \x1b[34m|\x1b[0m {}\x1b[K", left_str, right_str);
        }

        // Status bar
        println!("\x1b[K");
        if !status_message.is_empty() && Instant::now().duration_since(status_timer).as_secs() < 3 {
            println!("\x1b[1;33m{}\x1b[0m\x1b[K", status_message);
        } else {
            println!("\x1b[K");
        }

        print!("\x1b[J");
        let _ = stdout().flush();

        // Keyboard polling
        let mut key_pressed = None;
        for _ in 0..10 {
            if let Some(key) = poll_key() {
                key_pressed = Some(key);
                break;
            }
            std::thread::sleep(Duration::from_millis(20));
        }

        if let Some(key) = key_pressed {
            match key {
                'q' | 'Q' => {
                    print!("\x1b[2J\x1b[H");
                    let _ = stdout().flush();
                    return;
                }
                '\x01' | 'k' => { // Up
                    if selected_idx > 0 {
                        selected_idx -= 1;
                    }
                }
                '\x02' | 'j' => { // Down
                    if !items.is_empty() && selected_idx < items.len() - 1 {
                        selected_idx += 1;
                    }
                }
                '\r' | '\n' | '\x04' | 'l' => { // Enter / Right
                    if !items.is_empty() && items[selected_idx].is_dir {
                        current_dir = items[selected_idx].path.clone();
                        selected_idx = 0;
                    }
                }
                '\u{8}' | '\u{7f}' | '\x03' | 'h' => { // Backspace / Left
                    if let Some(parent) = current_dir.parent() {
                        current_dir = parent.to_path_buf();
                        selected_idx = 0;
                    }
                }
                'c' | 'C' => { // Copy
                    if !items.is_empty() {
                        let target = &items[selected_idx];
                        print!("\x1b[H\x1b[23;1H\x1b[KCopy '{}' to path: ", target.name);
                        let _ = stdout().flush();
                        
                        // Disable raw mode temporarily on Unix to read standard line input
                        // (Wait, we can just read from stdin or write a small line input, but since we are running cross-platform, reading lines from console can be done by capturing characters).
                        // Let's do a simple inline character line reader!
                        let mut dest_input = String::new();
                        loop {
                            let mut k = None;
                            while k.is_none() {
                                k = poll_key();
                                std::thread::sleep(Duration::from_millis(10));
                            }
                            let k = k.unwrap();
                            if k == '\r' || k == '\n' {
                                break;
                            } else if k == '\u{8}' || k == '\u{7f}' {
                                dest_input.pop();
                            } else if k.is_ascii() && k >= ' ' {
                                dest_input.push(k);
                            }
                            print!("\x1b[H\x1b[23;1H\x1b[KCopy '{}' to path: {}", target.name, dest_input);
                            let _ = stdout().flush();
                        }

                        if !dest_input.is_empty() {
                            let dest_path = PathBuf::from(dest_input.trim());
                            let copy_res = if target.is_dir {
                                // Simple recursive directory copy helper
                                fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
                                    fs::create_dir_all(dst)?;
                                    for entry in fs::read_dir(src)? {
                                        let entry = entry?;
                                        let file_type = entry.file_type()?;
                                        if file_type.is_dir() {
                                            copy_dir_all(&entry.path(), &dst.join(entry.file_name()))?;
                                        } else {
                                            fs::copy(&entry.path(), &dst.join(entry.file_name()))?;
                                        }
                                    }
                                    Ok(())
                                }
                                copy_dir_all(&target.path, &dest_path.join(&target.name))
                            } else {
                                fs::copy(&target.path, &dest_path)
                                    .map(|_| ())
                            };

                            match copy_res {
                                Ok(_) => {
                                    status_message = format!("Copied successfully: {}", target.name);
                                    status_timer = Instant::now();
                                }
                                Err(e) => {
                                    status_message = format!("Copy failed: {}", e);
                                    status_timer = Instant::now();
                                }
                            }
                        }
                    }
                }
                'm' | 'M' => { // Move
                    if !items.is_empty() {
                        let target = &items[selected_idx];
                        print!("\x1b[H\x1b[23;1H\x1b[KMove '{}' to path: ", target.name);
                        let _ = stdout().flush();

                        let mut dest_input = String::new();
                        loop {
                            let mut k = None;
                            while k.is_none() {
                                k = poll_key();
                                std::thread::sleep(Duration::from_millis(10));
                            }
                            let k = k.unwrap();
                            if k == '\r' || k == '\n' {
                                break;
                            } else if k == '\u{8}' || k == '\u{7f}' {
                                dest_input.pop();
                            } else if k.is_ascii() && k >= ' ' {
                                dest_input.push(k);
                            }
                            print!("\x1b[H\x1b[23;1H\x1b[KMove '{}' to path: {}", target.name, dest_input);
                            let _ = stdout().flush();
                        }

                        if !dest_input.is_empty() {
                            let dest_path = PathBuf::from(dest_input.trim());
                            match fs::rename(&target.path, &dest_path) {
                                Ok(_) => {
                                    status_message = format!("Moved: {}", target.name);
                                    status_timer = Instant::now();
                                }
                                Err(e) => {
                                    status_message = format!("Move failed: {}", e);
                                    status_timer = Instant::now();
                                }
                            }
                        }
                    }
                }
                'r' | 'R' => { // Rename
                    if !items.is_empty() {
                        let target = &items[selected_idx];
                        print!("\x1b[H\x1b[23;1H\x1b[KRename '{}' to: ", target.name);
                        let _ = stdout().flush();

                        let mut dest_input = String::new();
                        loop {
                            let mut k = None;
                            while k.is_none() {
                                k = poll_key();
                                std::thread::sleep(Duration::from_millis(10));
                            }
                            let k = k.unwrap();
                            if k == '\r' || k == '\n' {
                                break;
                            } else if k == '\u{8}' || k == '\u{7f}' {
                                dest_input.pop();
                            } else if k.is_ascii() && k >= ' ' {
                                dest_input.push(k);
                            }
                            print!("\x1b[H\x1b[23;1H\x1b[KRename '{}' to: {}", target.name, dest_input);
                            let _ = stdout().flush();
                        }

                        if !dest_input.is_empty() {
                            let new_path = target.path.parent().unwrap().join(dest_input.trim());
                            match fs::rename(&target.path, &new_path) {
                                Ok(_) => {
                                    status_message = format!("Renamed: {} to {}", target.name, dest_input);
                                    status_timer = Instant::now();
                                }
                                Err(e) => {
                                    status_message = format!("Rename failed: {}", e);
                                    status_timer = Instant::now();
                                }
                            }
                        }
                    }
                }
                'd' | 'D' => { // Delete
                    if !items.is_empty() {
                        let target = &items[selected_idx];
                        let prompt_msg = format!("Delete '{}'? Press 'y' to confirm, any other key to cancel.", target.name);
                        
                        // Redraw prompt
                        print!("\x1b[H");
                        println!("\x1b[22;1H\x1b[1;31m{}\x1b[0m\x1b[K", prompt_msg);
                        let _ = stdout().flush();

                        // Wait for confirmation
                        let confirm_key = loop {
                            if let Some(k) = poll_key() {
                                break k;
                            }
                            std::thread::sleep(Duration::from_millis(20));
                        };

                        if confirm_key == 'y' || confirm_key == 'Y' {
                            let path_to_remove = target.path.clone();
                            let is_dir_to_remove = target.is_dir;

                            let remove_result = if is_dir_to_remove {
                                fs::remove_dir_all(&path_to_remove)
                            } else {
                                fs::remove_file(&path_to_remove)
                            };

                            match remove_result {
                                Ok(_) => {
                                    status_message = format!("Deleted: {}", target.name);
                                    status_timer = Instant::now();
                                    
                                    if selected_idx >= items.len() - 1 && selected_idx > 0 {
                                        selected_idx = items.len() - 2;
                                    }
                                }
                                Err(e) => {
                                    status_message = format!("Delete failed: {}", e);
                                    status_timer = Instant::now();
                                }
                            }
                        } else {
                            status_message = "Deletion cancelled.".to_string();
                            status_timer = Instant::now();
                        }
                    }
                }
                _ => {}
            }
        }
    }
}
