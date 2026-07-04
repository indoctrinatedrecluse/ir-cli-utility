use crate::EditOptions;
use std::fs;
use std::io::{stdout, Write};
use std::path::Path;

// ─── Platform key polling ──────────────────────────────────────────────────────

#[cfg(target_os = "windows")]
extern "C" {
    fn _kbhit() -> std::os::raw::c_int;
    fn _getch() -> std::os::raw::c_int;
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Key {
    Char(char),
    Up, Down, Left, Right,
    Home, End,
    PageUp, PageDown,
    Backspace, Delete,
    Enter,
    CtrlS, CtrlQ,
    Escape,
    CtrlHome, CtrlEnd,
    Unknown,
}

#[cfg(target_os = "windows")]
fn read_key() -> Key {
    loop {
        unsafe {
            if _kbhit() != 0 {
                let ch = _getch();
                if ch == 0 || ch == 224 {
                    let sub = _getch();
                    return match sub {
                        72  => Key::Up,
                        80  => Key::Down,
                        75  => Key::Left,
                        77  => Key::Right,
                        71  => Key::Home,
                        79  => Key::End,
                        73  => Key::PageUp,
                        81  => Key::PageDown,
                        83  => Key::Delete,
                        119 => Key::CtrlHome,
                        117 => Key::CtrlEnd,
                        _   => Key::Unknown,
                    };
                }
                return match ch {
                    13  => Key::Enter,
                    8   => Key::Backspace,
                    27  => Key::Escape,
                    19  => Key::CtrlS,
                    17  => Key::CtrlQ,
                    c if c >= 32 && c < 127 => Key::Char(c as u8 as char),
                    _   => Key::Unknown,
                };
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}

#[cfg(target_os = "linux")]
struct RawModeGuard { orig: libc::termios }

#[cfg(target_os = "linux")]
impl Drop for RawModeGuard {
    fn drop(&mut self) {
        unsafe { libc::tcsetattr(0, libc::TCSAFLUSH, &self.orig); }
    }
}

#[cfg(target_os = "linux")]
fn set_raw_mode() -> Option<RawModeGuard> {
    unsafe {
        let mut orig = std::mem::zeroed();
        if libc::tcgetattr(0, &mut orig) != 0 { return None; }
        let mut raw = orig;
        raw.c_lflag &= !(libc::ECHO | libc::ICANON | libc::ISIG);
        raw.c_cc[libc::VMIN]  = 1;
        raw.c_cc[libc::VTIME] = 0;
        libc::tcsetattr(0, libc::TCSAFLUSH, &raw);
        Some(RawModeGuard { orig })
    }
}

#[cfg(target_os = "linux")]
fn read_key() -> Key {
    let mut buf = [0u8; 8];
    loop {
        let n = unsafe { libc::read(0, buf.as_mut_ptr() as *mut libc::c_void, 8) };
        if n <= 0 { continue; }
        if buf[0] == 27 {
            if n == 1 { return Key::Escape; }
            if n >= 3 && buf[1] == b'[' {
                return match buf[2] {
                    b'A' => Key::Up,
                    b'B' => Key::Down,
                    b'C' => Key::Right,
                    b'D' => Key::Left,
                    b'H' => Key::Home,
                    b'F' => Key::End,
                    b'5' => Key::PageUp,
                    b'6' => Key::PageDown,
                    b'3' => Key::Delete,
                    b'1' if n >= 6 && buf[3] == b';' && buf[4] == b'5' => match buf[5] {
                        b'H' => Key::CtrlHome,
                        b'F' => Key::CtrlEnd,
                        _    => Key::Unknown,
                    },
                    _ => Key::Unknown,
                };
            }
        }
        return match buf[0] {
            b'\r' | b'\n' => Key::Enter,
            127 | 8       => Key::Backspace,
            19            => Key::CtrlS,
            17            => Key::CtrlQ,
            c if c >= 32 && c < 127 => Key::Char(c as char),
            _             => Key::Unknown,
        };
    }
}

// ─── Terminal size ──────────────────────────────────────────────────────────────

fn terminal_size() -> (u16, u16) {
    #[cfg(target_os = "windows")]
    unsafe {
        use windows_sys::Win32::System::Console::{GetConsoleScreenBufferInfo, GetStdHandle, STD_OUTPUT_HANDLE};
        let h = GetStdHandle(STD_OUTPUT_HANDLE);
        let mut info = std::mem::zeroed();
        if GetConsoleScreenBufferInfo(h, &mut info) != 0 {
            let cols = (info.srWindow.Right  - info.srWindow.Left + 1) as u16;
            let rows = (info.srWindow.Bottom - info.srWindow.Top  + 1) as u16;
            return (cols, rows);
        }
        (80, 24)
    }
    #[cfg(target_os = "linux")]
    unsafe {
        let mut ws: libc::winsize = std::mem::zeroed();
        if libc::ioctl(1, libc::TIOCGWINSZ, &mut ws) == 0 && ws.ws_col > 0 {
            return (ws.ws_col, ws.ws_row);
        }
        (80, 24)
    }
}

// ─── Byte index helpers (avoid borrow conflicts) ───────────────────────────────

/// Return the byte offset of the `n`-th character in `s`, or `s.len()` if past end.
fn char_byte_idx(s: &str, n: usize) -> usize {
    s.char_indices().nth(n).map(|(i, _)| i).unwrap_or(s.len())
}

// ─── Editor State ──────────────────────────────────────────────────────────────

struct Editor {
    lines:    Vec<String>,
    cx:       usize,   // cursor column  (char index)
    cy:       usize,   // cursor row     (line index)
    row_off:  usize,
    col_off:  usize,
    filename: String,
    modified: bool,
    status_msg: String,
}

impl Editor {
    fn new(filename: &str) -> Self {
        let lines = if Path::new(filename).exists() {
            match fs::read_to_string(filename) {
                Ok(s) => {
                    let mut v: Vec<String> = s.lines().map(|l| l.to_string()).collect();
                    if v.is_empty() { v.push(String::new()); }
                    v
                }
                Err(e) => {
                    // Error will be shown after rendering starts
                    vec![format!("(error reading file: {})", e)]
                }
            }
        } else {
            // New file — start blank
            vec![String::new()]
        };
        Self {
            lines,
            cx: 0, cy: 0,
            row_off: 0, col_off: 0,
            filename: filename.to_string(),
            modified: false,
            status_msg: "Ctrl+S save  |  Ctrl+Q / Esc quit".to_string(),
        }
    }

    fn save(&mut self) {
        // Check if the parent directory exists
        let path = Path::new(&self.filename);
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() && !parent.exists() {
                self.status_msg = format!("Save failed: directory '{}' does not exist.", parent.display());
                return;
            }
        }
        let content = self.lines.join("\n") + "\n";
        match fs::write(&self.filename, &content) {
            Ok(_) => {
                self.modified = false;
                self.status_msg = format!("Saved: {} ({} bytes)", self.filename, content.len());
            }
            Err(e) => {
                // Covers: permission denied, disk full, file locked (Windows), etc.
                self.status_msg = format!("Save failed: {}", e);
            }
        }
    }

    fn render(&self, cols: u16, rows: u16) {
        let edit_rows = rows.saturating_sub(2) as usize;
        let cols_us   = cols as usize;

        let mut buf = String::with_capacity(8192);
        buf.push_str("\x1b[?25l\x1b[H"); // hide cursor, go home

        // ── Top bar ──────────────────────────────────────────────────────────
        let mod_flag = if self.modified { "* " } else { "  " };
        let title = format!("  ir edit  |  {}{}", mod_flag, self.filename);
        let pos   = format!("Ln {}, Col {}  ", self.cy + 1, self.cx + 1);
        let pad_len = cols_us.saturating_sub(title.chars().count() + pos.len());
        buf.push_str(&format!("\x1b[1;44;97m{}{}{}\x1b[0m\x1b[K\r\n",
            title, " ".repeat(pad_len), pos));

        // ── Edit area ────────────────────────────────────────────────────────
        for row in 0..edit_rows {
            let file_row = row + self.row_off;
            if file_row < self.lines.len() {
                let visible: String = self.lines[file_row]
                    .chars()
                    .skip(self.col_off)
                    .take(cols_us)
                    .collect();
                buf.push_str(&visible);
            } else {
                buf.push_str("\x1b[34m~\x1b[0m");
            }
            buf.push_str("\x1b[K\r\n");
        }

        // ── Status bar ───────────────────────────────────────────────────────
        let left  = format!("  {}", self.status_msg);
        let right = format!("  {} lines  ", self.lines.len());
        let pad_len = cols_us.saturating_sub(left.chars().count() + right.len());
        buf.push_str(&format!("\x1b[7m{}{}{}\x1b[0m\x1b[K",
            left, " ".repeat(pad_len), right));

        // ── Reposition cursor ─────────────────────────────────────────────────
        let screen_row = (self.cy - self.row_off) + 2; // +2 for top bar
        let screen_col = self.cx.saturating_sub(self.col_off) + 1;
        buf.push_str(&format!("\x1b[{};{}H\x1b[?25h", screen_row, screen_col));

        let _ = stdout().write_all(buf.as_bytes());
        let _ = stdout().flush();
    }

    fn scroll(&mut self, cols: u16, rows: u16) {
        let edit_rows = rows.saturating_sub(2) as usize;
        let cols_us   = cols as usize;

        if self.cy >= self.lines.len() {
            self.cy = self.lines.len().saturating_sub(1);
        }
        let line_len = self.lines[self.cy].chars().count();
        if self.cx > line_len { self.cx = line_len; }

        if self.cy < self.row_off {
            self.row_off = self.cy;
        } else if self.cy >= self.row_off + edit_rows {
            self.row_off = self.cy - edit_rows + 1;
        }
        if self.cx < self.col_off {
            self.col_off = self.cx;
        } else if self.cx >= self.col_off + cols_us {
            self.col_off = self.cx - cols_us + 1;
        }
    }

    /// Returns true if the editor should quit.
    fn process_key(&mut self, key: Key, cols: u16, rows: u16) -> bool {
        let edit_rows = rows.saturating_sub(2) as usize;

        match key {
            Key::CtrlS => { self.save(); }

            Key::CtrlQ | Key::Escape => { return true; }

            Key::Enter => {
                // Compute the byte split point BEFORE any borrow of self.lines
                let split_byte = char_byte_idx(&self.lines[self.cy], self.cx);
                let rest = self.lines[self.cy][split_byte..].to_string();
                self.lines[self.cy].truncate(split_byte);
                self.cy += 1;
                self.lines.insert(self.cy, rest);
                self.cx = 0;
                self.modified = true;
            }

            Key::Backspace => {
                if self.cx > 0 {
                    // Pre-compute both byte indices before mutating
                    let start = char_byte_idx(&self.lines[self.cy], self.cx - 1);
                    let end   = char_byte_idx(&self.lines[self.cy], self.cx);
                    self.lines[self.cy].drain(start..end);
                    self.cx -= 1;
                    self.modified = true;
                } else if self.cy > 0 {
                    let rest = self.lines.remove(self.cy);
                    self.cy -= 1;
                    self.cx = self.lines[self.cy].chars().count();
                    self.lines[self.cy].push_str(&rest);
                    self.modified = true;
                }
            }

            Key::Delete => {
                let line_len = self.lines[self.cy].chars().count();
                if self.cx < line_len {
                    let start = char_byte_idx(&self.lines[self.cy], self.cx);
                    let end   = char_byte_idx(&self.lines[self.cy], self.cx + 1);
                    self.lines[self.cy].drain(start..end);
                    self.modified = true;
                } else if self.cy + 1 < self.lines.len() {
                    let next = self.lines.remove(self.cy + 1);
                    self.lines[self.cy].push_str(&next);
                    self.modified = true;
                }
            }

            Key::Char(c) => {
                let byte_idx = char_byte_idx(&self.lines[self.cy], self.cx);
                self.lines[self.cy].insert(byte_idx, c);
                self.cx += 1;
                self.modified = true;
                // Clear hint message once user starts typing
                if self.status_msg.starts_with("Ctrl+S") {
                    self.status_msg = "Ctrl+S save  |  Ctrl+Q / Esc quit".to_string();
                }
            }

            Key::Left => {
                if self.cx > 0 {
                    self.cx -= 1;
                } else if self.cy > 0 {
                    self.cy -= 1;
                    self.cx = self.lines[self.cy].chars().count();
                }
            }
            Key::Right => {
                let line_len = self.lines[self.cy].chars().count();
                if self.cx < line_len {
                    self.cx += 1;
                } else if self.cy + 1 < self.lines.len() {
                    self.cy += 1;
                    self.cx = 0;
                }
            }
            Key::Up => {
                if self.cy > 0 {
                    self.cy -= 1;
                    let ll = self.lines[self.cy].chars().count();
                    if self.cx > ll { self.cx = ll; }
                }
            }
            Key::Down => {
                if self.cy + 1 < self.lines.len() {
                    self.cy += 1;
                    let ll = self.lines[self.cy].chars().count();
                    if self.cx > ll { self.cx = ll; }
                }
            }
            Key::Home     => { self.cx = 0; }
            Key::End      => { self.cx = self.lines[self.cy].chars().count(); }
            Key::PageUp   => { self.cy = self.cy.saturating_sub(edit_rows); }
            Key::PageDown => { self.cy = (self.cy + edit_rows).min(self.lines.len().saturating_sub(1)); }
            Key::CtrlHome => { self.cy = 0; self.cx = 0; }
            Key::CtrlEnd  => {
                self.cy = self.lines.len().saturating_sub(1);
                self.cx = self.lines[self.cy].chars().count();
            }
            Key::Unknown  => {}
        }

        self.scroll(cols, rows);
        false
    }
}

// ─── Entry point ───────────────────────────────────────────────────────────────

pub fn edit(filename: &str, _options: EditOptions) {
    let path = Path::new(filename);

    // ── Pre-flight validation ────────────────────────────────────────────────
    if path.is_dir() {
        eprintln!("error: '{}' is a directory, not a file.", filename);
        std::process::exit(1);
    }

    // Warn if existing file looks binary (check first 512 bytes)
    let mut warn_binary = false;
    if path.exists() {
        if let Ok(bytes) = fs::read(filename) {
            let sample = &bytes[..bytes.len().min(512)];
            let has_null = sample.contains(&0u8);
            let non_text = sample.iter().filter(|&&b| b < 9 || (b > 13 && b < 32)).count();
            warn_binary = has_null || non_text > sample.len() / 10;
        }
    }

    #[cfg(target_os = "linux")]
    let _raw = set_raw_mode();

    let mut editor = Editor::new(filename);

    if warn_binary {
        editor.status_msg = "WARNING: binary file — editing may corrupt it. Ctrl+S to save anyway.".to_string();
    }

    // Clear screen
    print!("\x1b[2J\x1b[H");
    let _ = stdout().flush();

    loop {
        let (cols, rows) = terminal_size();
        editor.scroll(cols, rows);
        editor.render(cols, rows);

        let key = read_key();
        let want_quit = editor.process_key(key, cols, rows);

        if want_quit {
            if editor.modified {
                // Unsaved changes: show warning and wait for a second decision
                editor.status_msg =
                    "Unsaved changes! Ctrl+S = save & quit  |  Ctrl+Q / Esc = discard".to_string();
                let (cols, rows) = terminal_size();
                editor.scroll(cols, rows);
                editor.render(cols, rows);
                let k2 = read_key();
                match k2 {
                    Key::CtrlQ | Key::Escape => break,
                    Key::CtrlS => { editor.save(); break; }
                    _ => {
                        // Resume editing
                        editor.status_msg = "Ctrl+S save  |  Ctrl+Q / Esc quit".to_string();
                    }
                }
            } else {
                break;
            }
        }
    }

    // Restore terminal
    print!("\x1b[2J\x1b[H\x1b[?25h");
    let _ = stdout().flush();
}
