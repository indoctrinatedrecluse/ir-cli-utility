use crate::DuaOptions;
use std::io::{stdout, Write};
use std::path::{Path, PathBuf};
use std::time::Duration;

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
            if ch == 224 { // Arrow prefix
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

#[derive(Clone)]
struct DiskNode {
    name: String,
    path: PathBuf,
    is_dir: bool,
    size_bytes: u64,
    children: Vec<DiskNode>,
}

fn scan_dir(path: &Path, show_progress: bool, counter: &mut u64) -> Option<DiskNode> {
    if show_progress && *counter % 100 == 0 {
        print!("\rIndexing... {} items scanned", *counter);
        let _ = stdout().flush();
    }
    *counter += 1;

    let metadata = path.metadata().ok()?;
    let is_dir = metadata.is_dir();
    let mut children = Vec::new();
    let mut size_bytes = 0;

    if is_dir {
        if let Ok(entries) = std::fs::read_dir(path) {
            for entry in entries.filter_map(|e| e.ok()) {
                if let Some(child) = scan_dir(&entry.path(), show_progress, counter) {
                    size_bytes += child.size_bytes;
                    children.push(child);
                }
            }
        }
        children.sort_by(|a, b| b.size_bytes.cmp(&a.size_bytes));
    } else {
        size_bytes = metadata.len();
    }

    let name = path.file_name().unwrap_or(path.as_os_str()).to_string_lossy().to_string();
    Some(DiskNode {
        name,
        path: path.to_path_buf(),
        is_dir,
        size_bytes,
        children,
    })
}

fn format_size(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{} B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.2} KB", bytes as f64 / 1024.0)
    } else if bytes < 1024 * 1024 * 1024 {
        format!("{:.2} MB", bytes as f64 / 1024.0 / 1024.0)
    } else {
        format!("{:.2} GB", bytes as f64 / 1024.0 / 1024.0 / 1024.0)
    }
}

pub fn dua(path_str: &str, _options: DuaOptions) {
    let target_path = Path::new(path_str);
    if !target_path.exists() {
        eprintln!("Error: Path '{}' does not exist.", path_str);
        return;
    }

    println!("Scanning directory '{}'...", target_path.display());
    let mut counter = 0;
    let root = match scan_dir(target_path, true, &mut counter) {
        Some(node) => node,
        None => {
            eprintln!("Error: Failed to scan directory.");
            return;
        }
    };
    println!("\rScan complete. Total items indexed: {}", counter);

    #[cfg(target_os = "linux")]
    let _raw_guard = set_raw_mode();

    let mut current_node = root;
    let mut history: Vec<DiskNode> = Vec::new();
    let mut selected_idx = 0usize;
    let mut status_message = String::new();
    let mut status_timer = Instant::now();

    // Clear screen
    print!("\x1b[2J\x1b[H");
    let _ = stdout().flush();

    use std::time::Instant;

    loop {
        // Handle TUI display
        print!("\x1b[H");
        println!("\x1b[1;34m=== Disk Usage Analyzer (dua) ===\x1b[0m\x1b[K");
        println!("Current Path: \x1b[33m{}\x1b[0m\x1b[K", current_node.path.display());
        println!("Total Size:   \x1b[32m{}\x1b[0m\x1b[K", format_size(current_node.size_bytes));
        println!("Controls:     \x1b[36m↑/↓\x1b[0m navigate | \x1b[36mEnter\x1b[0m open | \x1b[36mBackspace\x1b[0m up | \x1b[36m'd'\x1b[0m delete | \x1b[36m'q'\x1b[0m quit\x1b[K");
        println!("\x1b[34m--------------------------------------------------------------------------------\x1b[0m\x1b[K");

        let items = &current_node.children;
        if items.is_empty() {
            println!("  \x1b[33m[Directory is empty]\x1b[0m\x1b[K");
        } else {
            for (idx, item) in items.iter().enumerate() {
                let ratio = if current_node.size_bytes > 0 {
                    item.size_bytes as f64 / current_node.size_bytes as f64
                } else {
                    0.0
                };
                let filled = (ratio * 10.0).round().clamp(0.0, 10.0) as usize;
                let bar = format!("[{}{}]", "#".repeat(filled), " ".repeat(10 - filled));
                let pct = ratio * 100.0;

                let size_str = format!("{:<10}", format_size(item.size_bytes));
                let bar_str = format!("{} {:>5.1}%", bar, pct);

                let display_name = if item.is_dir {
                    format!("\x1b[1;34m{}/\x1b[0m", item.name)
                } else {
                    item.name.clone()
                };

                if idx == selected_idx {
                    println!("-> \x1b[7m{}  {}  {}\x1b[0m\x1b[K", size_str, bar_str, display_name);
                } else {
                    println!("   {}  {}  {}\x1b[K", size_str, bar_str, display_name);
                }
            }
        }

        // Draw status messages
        println!("\x1b[K");
        if !status_message.is_empty() && Instant::now().duration_since(status_timer).as_secs() < 3 {
            println!("\x1b[1;33m{}\x1b[0m\x1b[K", status_message);
        } else {
            println!("\x1b[K");
        }

        print!("\x1b[J");
        let _ = stdout().flush();

        // Keyboard polling loop
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
                        let entered_node = items[selected_idx].clone();
                        history.push(current_node.clone());
                        current_node = entered_node;
                        selected_idx = 0;
                    }
                }
                '\u{8}' | '\u{7f}' | '\x03' | 'h' => { // Backspace / Left
                    if let Some(parent) = history.pop() {
                        current_node = parent;
                        selected_idx = 0;
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
                            let size_to_remove = target.size_bytes;

                            let remove_result = if is_dir_to_remove {
                                std::fs::remove_dir_all(&path_to_remove)
                            } else {
                                std::fs::remove_file(&path_to_remove)
                            };

                            match remove_result {
                                Ok(_) => {
                                    status_message = format!("Deleted: {}", target.name);
                                    status_timer = Instant::now();
                                    
                                    // Remove from current node children list
                                    current_node.children.remove(selected_idx);
                                    current_node.size_bytes = current_node.size_bytes.saturating_sub(size_to_remove);
                                    
                                    // Adjust sizes of parents in history stack
                                    for parent in &mut history {
                                        parent.size_bytes = parent.size_bytes.saturating_sub(size_to_remove);
                                        // Update size in parent's children list as well
                                        for child in &mut parent.children {
                                            if child.path == current_node.path {
                                                child.size_bytes = current_node.size_bytes;
                                            }
                                        }
                                    }
                                    
                                    if selected_idx >= current_node.children.len() && selected_idx > 0 {
                                        selected_idx = current_node.children.len() - 1;
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
