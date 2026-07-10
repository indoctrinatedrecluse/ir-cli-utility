
// ─── Keyboard FFI ─────────────────────────────────────────────────────────────
#[cfg(target_os = "windows")]
extern "C" {
    fn _kbhit() -> std::os::raw::c_int;
    fn _getch() -> std::os::raw::c_int;
}

pub struct RawModeGuard {
    #[cfg(target_os = "windows")]
    orig: u32,
    #[cfg(not(target_os = "windows"))]
    orig: libc::termios,
}

impl Drop for RawModeGuard {
    fn drop(&mut self) {
        #[cfg(target_os = "windows")]
        unsafe {
            use windows_sys::Win32::System::Console::{GetStdHandle, SetConsoleMode, STD_INPUT_HANDLE};
            let h = GetStdHandle(STD_INPUT_HANDLE);
            let _ = SetConsoleMode(h, self.orig);
        }
        #[cfg(not(target_os = "windows"))]
        unsafe {
            libc::tcsetattr(0, libc::TCSAFLUSH, &self.orig);
        }
    }
}

pub fn set_raw_mode() -> Option<RawModeGuard> {
    #[cfg(target_os = "windows")]
    unsafe {
        use windows_sys::Win32::System::Console::{GetStdHandle, GetConsoleMode, SetConsoleMode, STD_INPUT_HANDLE, ENABLE_PROCESSED_INPUT};
        let h = GetStdHandle(STD_INPUT_HANDLE);
        let mut orig = 0;
        if GetConsoleMode(h, &mut orig) != 0 {
            if SetConsoleMode(h, orig & !ENABLE_PROCESSED_INPUT) != 0 {
                return Some(RawModeGuard { orig });
            }
        }
        None
    }
    #[cfg(not(target_os = "windows"))]
    unsafe {
        let mut orig = std::mem::zeroed();
        if libc::tcgetattr(0, &mut orig) == 0 {
            let mut raw = orig;
            raw.c_lflag &= !(libc::ECHO | libc::ICANON | libc::ISIG);
            raw.c_cc[libc::VMIN] = 0;
            raw.c_cc[libc::VTIME] = 0;
            if libc::tcsetattr(0, libc::TCSAFLUSH, &raw) == 0 {
                return Some(RawModeGuard { orig });
            }
        }
        None
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Key {
    Up,
    Down,
    Left,
    Right,
    PageUp,
    PageDown,
    Char(char),
    Tab,
    Enter,
    Backspace,
    Esc,
    None,
}

pub fn poll_key() -> Key {
    #[cfg(target_os = "windows")]
    {
        unsafe {
            if _kbhit() != 0 {
                let ch = _getch();
                if ch == 0 || ch == 224 {
                    let sub = _getch();
                    return match sub {
                        72 => Key::Up,
                        80 => Key::Down,
                        75 => Key::Left,
                        77 => Key::Right,
                        73 => Key::PageUp,
                        81 => Key::PageDown,
                        _ => Key::None,
                    };
                }
                match ch {
                    27 => Key::Esc,
                    9 => Key::Tab,
                    13 => Key::Enter,
                    8 => Key::Backspace,
                    c if c >= 32 && c < 127 => Key::Char(c as u8 as char),
                    _ => Key::None
                }
            } else {
                Key::None
            }
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        let mut buf = [0u8; 16];
        let n = unsafe { libc::read(0, buf.as_mut_ptr() as *mut libc::c_void, 16) };
        if n > 0 {
            if buf[0] == 27 {
                if n == 1 { return Key::Esc; }
                if n >= 3 && buf[1] == b'[' {
                    return match buf[2] {
                        b'A' => Key::Up,
                        b'B' => Key::Down,
                        b'D' => Key::Left,
                        b'C' => Key::Right,
                        b'5' => Key::PageUp,
                        b'6' => Key::PageDown,
                        _ => Key::None,
                    };
                }
            }
            match buf[0] {
                27 => Key::Esc,
                9 => Key::Tab,
                10 | 13 => Key::Enter,
                127 => Key::Backspace,
                c if c >= 32 && c < 127 => Key::Char(c as char),
                _ => Key::None,
            }
        } else {
            Key::None
        }
    }
}

pub fn terminal_size() -> (u16, u16) {
    #[cfg(target_os = "windows")]
    unsafe {
        use windows_sys::Win32::System::Console::{
            GetConsoleScreenBufferInfo, GetStdHandle, STD_OUTPUT_HANDLE,
        };
        let h = GetStdHandle(STD_OUTPUT_HANDLE);
        let mut info = std::mem::zeroed();
        if GetConsoleScreenBufferInfo(h, &mut info) != 0 {
            let cols = (info.srWindow.Right - info.srWindow.Left + 1) as u16;
            let rows = (info.srWindow.Bottom - info.srWindow.Top + 1) as u16;
            return (cols, rows);
        }
        (80, 24)
    }
    #[cfg(not(target_os = "windows"))]
    unsafe {
        let mut ws: libc::winsize = std::mem::zeroed();
        if libc::ioctl(1, libc::TIOCGWINSZ, &mut ws) == 0 && ws.ws_col > 0 {
            return (ws.ws_col, ws.ws_row);
        }
        (80, 24)
    }
}

pub fn strip_ansi_len(s: &str) -> usize {
    let mut len = 0;
    let mut in_esc = false;
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c == '\x1B' {
            in_esc = true;
        } else if in_esc {
            if c.is_ascii_alphabetic() {
                in_esc = false;
            }
        } else {
            len += 1;
        }
    }
    len
}

pub fn format_size(bytes: u64) -> String {
    if bytes >= 1024 * 1024 * 1024 {
        format!("{:.2} GB", bytes as f64 / (1024 * 1024 * 1024) as f64)
    } else if bytes >= 1024 * 1024 {
        format!("{:.2} MB", bytes as f64 / (1024 * 1024) as f64)
    } else if bytes >= 1024 {
        format!("{:.2} KB", bytes as f64 / 1024.0)
    } else {
        format!("{} B", bytes)
    }
}
