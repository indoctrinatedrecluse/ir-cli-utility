use crate::WatchOptions;
use std::io::{stdout, Write};
use std::process::Command;
use std::time::{Duration, Instant};

#[cfg(target_os = "windows")]
extern "C" {
    fn _kbhit() -> std::os::raw::c_int;
    fn _getch() -> std::os::raw::c_int;
}

#[cfg(target_os = "windows")]
fn check_quit() -> bool {
    unsafe {
        if _kbhit() != 0 {
            let ch = _getch();
            ch == b'q' as i32 || ch == b'Q' as i32 || ch == 3 || ch == 27
        } else {
            false
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
fn check_quit() -> bool {
    let mut buf = [0u8; 1];
    unsafe {
        let n = libc::read(0, buf.as_mut_ptr() as *mut libc::c_void, 1);
        if n > 0 {
            let ch = buf[0];
            ch == b'q' || ch == b'Q' || ch == 3 || ch == 27
        } else {
            false
        }
    }
}

fn print_diff(last: &str, current: &str) {
    let last_chars: Vec<char> = last.chars().collect();
    let current_chars: Vec<char> = current.chars().collect();
    
    let mut out = String::new();
    let mut in_highlight = false;

    for i in 0..current_chars.len() {
        let is_diff = if i < last_chars.len() {
            current_chars[i] != last_chars[i]
        } else {
            true
        };

        if is_diff {
            if !in_highlight {
                // Inverted or background color for changes
                out.push_str("\x1b[7m"); 
                in_highlight = true;
            }
            out.push(current_chars[i]);
        } else {
            if in_highlight {
                out.push_str("\x1b[0m");
                in_highlight = false;
            }
            out.push(current_chars[i]);
        }
    }
    if in_highlight {
        out.push_str("\x1b[0m");
    }
    print!("{}", out);
}

pub fn watch(command: &str, options: WatchOptions) {
    #[cfg(target_os = "linux")]
    let _raw_guard = set_raw_mode();

    let mut last_output = String::new();
    
    // Clear screen first
    print!("\x1b[2J\x1b[H");
    let _ = stdout().flush();

    let mut last_update = Instant::now() - Duration::from_millis(options.interval_ms);

    loop {
        // Non-blocking keyboard check to quit
        if check_quit() {
            print!("\x1b[2J\x1b[H");
            let _ = stdout().flush();
            return;
        }

        let now = Instant::now();
        if now.duration_since(last_update).as_millis() >= options.interval_ms as u128 {
            last_update = now;

            // Execute command
            let output = if cfg!(target_os = "windows") {
                Command::new("cmd")
                    .args(["/c", command])
                    .output()
            } else {
                Command::new("sh")
                    .args(["-c", command])
                    .output()
            };

            let stdout_str = match output {
                Ok(out) => {
                    let combined = format!(
                        "{}{}",
                        String::from_utf8_lossy(&out.stdout),
                        String::from_utf8_lossy(&out.stderr)
                    );
                    combined
                }
                Err(e) => format!("Error executing command: {}", e),
            };

            // Reposition cursor to top left
            print!("\x1b[H");

            // Header line (bright blue)
            let secs = options.interval_ms as f64 / 1000.0;
            println!(
                "\x1b[1;34mEvery {:.1}s: {}\x1b[0m\x1b[K",
                secs, command
            );
            println!("\x1b[34m--------------------------------------------------------------------------------\x1b[0m\x1b[K");

            // Output block
            if options.diff && !last_output.is_empty() {
                print_diff(&last_output, &stdout_str);
            } else {
                print!("{}", stdout_str);
            }
            
            // Clear rest of screen
            print!("\x1b[J");
            let _ = stdout().flush();

            last_output = stdout_str;
        }

        // Poll delay to avoid CPU pinning
        std::thread::sleep(Duration::from_millis(20));
    }
}
