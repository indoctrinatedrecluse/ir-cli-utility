use crate::MatrixOptions;
use std::io::{self, Write, IsTerminal};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

// ─── Keyboard FFI ─────────────────────────────────────────────────────────────
#[cfg(target_os = "windows")]
extern "C" {
    fn _kbhit() -> std::os::raw::c_int;
    fn _getch() -> std::os::raw::c_int;
}

#[cfg(not(target_os = "windows"))]
struct RawModeGuard {
    orig: libc::termios,
}

#[cfg(not(target_os = "windows"))]
impl Drop for RawModeGuard {
    fn drop(&mut self) {
        unsafe {
            libc::tcsetattr(0, libc::TCSAFLUSH, &self.orig);
        }
    }
}

#[cfg(not(target_os = "windows"))]
fn set_raw_mode() -> Option<RawModeGuard> {
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
enum Key {
    Up,
    Down,
    Char(char),
    Esc,
    None,
}

fn poll_key() -> Key {
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
                        _ => Key::None,
                    };
                }
                match ch {
                    27 => Key::Esc,
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
                        _ => Key::None,
                    };
                }
            }
            match buf[0] {
                27 => Key::Esc,
                c if c >= 32 && c < 127 => Key::Char(c as char),
                _ => Key::None,
            }
        } else {
            Key::None
        }
    }
}

fn terminal_size() -> (u16, u16) {
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

// Simple LCG randomizer
struct Lcg {
    state: u64,
}

impl Lcg {
    fn new() -> Self {
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(1337);
        Self { state: seed }
    }

    fn next(&mut self) -> u64 {
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        self.state
    }

    fn range(&mut self, min: u32, max: u32) -> u32 {
        if min >= max { return min; }
        let diff = max - min;
        (self.next() as u32 % diff) + min
    }
}

// Matrix rain drop structure
struct RainDrop {
    col: usize,
    y: f32,
    speed: f32,
    length: usize,
}

pub fn run_matrix(options: MatrixOptions) {
    if !io::stdout().is_terminal() {
        eprintln!("Error: ir matrix must be run in an interactive terminal.");
        std::process::exit(1);
    }

    let mut fps = options.fps;
    let is_fire = options.mode == "fire";

    #[cfg(not(target_os = "windows"))]
    let _raw_guard = set_raw_mode();

    // Clear screen, hide cursor, enter alternate buffer
    print!("\x1B[?1049h\x1B[2J\x1B[H\x1B[?25l");
    let _ = io::stdout().flush();

    let mut rng = Lcg::new();
    let (mut cols, mut rows) = terminal_size();

    // Color schemes: 0 = Green, 1 = Cyan, 2 = Red, 3 = Magenta, 4 = Blue, 5 = Rainbow
    let mut color_scheme = 0;
    let mut density = 35u32; // Drop probability (percentage)
    let mut fire_wind = 1i32;  // Left/right wind factor

    // Matrix drops list
    let mut drops: Vec<RainDrop> = Vec::new();
    
    // Doom fire heat buffer
    let mut fire_buffer = vec![0u8; (cols as usize) * (rows as usize)];

    let mut last_frame = Instant::now();
    let mut paused = false;

    loop {
        // Handle keyboard input
        match poll_key() {
            Key::Esc | Key::Char('q') | Key::Char('Q') => {
                break;
            }
            Key::Char(' ') => {
                paused = !paused;
            }
            Key::Char('+') | Key::Char('=') => {
                fps = (fps + 2).min(60);
            }
            Key::Char('-') | Key::Char('_') => {
                fps = (fps.saturating_sub(2)).max(1);
            }
            Key::Char('c') | Key::Char('C') => {
                color_scheme = (color_scheme + 1) % 6;
            }
            Key::Char('d') | Key::Char('D') => {
                if is_fire {
                    fire_wind = if fire_wind == 1 { -1 } else if fire_wind == -1 { 0 } else { 1 };
                } else {
                    density = if density >= 80 { 15 } else { density + 15 };
                }
            }
            _ => {}
        }

        // Handle resize
        let (new_cols, new_rows) = terminal_size();
        if new_cols != cols || new_rows != rows {
            cols = new_cols;
            rows = new_rows;
            // Clear screen completely
            print!("\x1B[2J\x1B[H");
            let _ = io::stdout().flush();
            // Re-allocate buffers
            drops.clear();
            fire_buffer = vec![0u8; (cols as usize) * (rows as usize)];
        }

        if !paused {
            if is_fire {
                // Doom fire rendering loop
                let width = cols as usize;
                let height = rows as usize;
                
                // Initialize bottom row with max heat (36)
                for x in 0..width {
                    let idx = (height - 1) * width + x;
                    if idx < fire_buffer.len() {
                        fire_buffer[idx] = 36;
                    }
                }

                // Propagate fire heat upwards
                for y in 1..height {
                    for x in 0..width {
                        let src_idx = y * width + x;
                        if src_idx >= fire_buffer.len() { continue; }
                        let heat = fire_buffer[src_idx];

                        if heat == 0 {
                            let dest_idx = (y - 1) * width + x;
                            if dest_idx < fire_buffer.len() {
                                fire_buffer[dest_idx] = 0;
                            }
                        } else {
                            // Procedural random wind & decay
                            let decay = rng.range(0, 3) as u8;
                            let wind = (rng.range(0, 3) as i32 - 1) + fire_wind;
                            let dest_x = (x as i32 + wind).rem_euclid(width as i32) as usize;
                            let dest_y = y - 1;
                            let dest_idx = dest_y * width + dest_x;

                            if dest_idx < fire_buffer.len() {
                                fire_buffer[dest_idx] = heat.saturating_sub(decay);
                            }
                        }
                    }
                }

                // Draw fire to console
                let mut out = String::new();
                out.push_str("\x1B[H"); // Cursor to home

                for y in 0..height {
                    for x in 0..width {
                        let idx = y * width + x;
                        let heat = fire_buffer[idx];
                        
                        // Map heat to color gradient: black -> red -> orange -> yellow -> white
                        let color_ansi = match heat {
                            0..=1 => "\x1B[0m ",
                            2..=5 => "\x1B[38;5;52m.",
                            6..=9 => "\x1B[38;5;88m*",
                            10..=13 => "\x1B[38;5;124m#",
                            14..=17 => "\x1B[38;5;160m#",
                            18..=21 => "\x1B[38;5;202mo",
                            22..=25 => "\x1B[38;5;208mo",
                            26..=29 => "\x1B[38;5;214ma",
                            30..=33 => "\x1B[38;5;220ma",
                            34..=35 => "\x1B[38;5;226m@",
                            _ => "\x1B[1;37m@",
                        };
                        out.push_str(color_ansi);
                    }
                    if y < height - 1 {
                        out.push('\n');
                    }
                }
                print!("{}", out);
                let _ = io::stdout().flush();
            } else {
                // Matrix digital rain loop
                let width = cols as usize;
                let height = rows as usize;

                // Spawn new drops based on density limit
                if drops.len() < width / 2 {
                    for x in 0..width {
                        if drops.iter().all(|d| d.col != x) {
                            if rng.range(0, 100) < density {
                                drops.push(RainDrop {
                                    col: x,
                                    y: -(rng.range(0, height as u32) as f32),
                                    speed: (rng.range(3, 9) as f32) / 10.0,
                                    length: rng.range(8, 25) as usize,
                                });
                            }
                        }
                    }
                }

                // Grid update buffers
                let mut screen = vec![vec![' '; width]; height];
                let mut colors = vec![vec!["\x1B[0m"; width]; height];

                // Render active drops into the buffers
                for drop in &mut drops {
                    drop.y += drop.speed;
                    let lead_y = drop.y as usize;

                    for i in 0..drop.length {
                        let segment_y = lead_y.saturating_sub(i);
                        if segment_y < height {
                            // Populate random glyph
                            let glyph = match rng.range(0, 3) {
                                0 => (rng.range(33, 126) as u8) as char,
                                // Half-width katakana approximation
                                1 => std::char::from_u32(rng.range(65377, 65439)).unwrap_or('?'),
                                _ => (rng.range(48, 57) as u8) as char,
                            };
                            screen[segment_y][drop.col] = glyph;

                            // Apply color gradient scheme based on distance from head
                            let scheme_color = if i == 0 {
                                "\x1B[1;37m" // Always white leading head
                            } else {
                                // Main body colors
                                match color_scheme {
                                    1 => {
                                        // Cyberpunk Cyan
                                        if i < 4 { "\x1B[1;36m" } else { "\x1B[0;36m" }
                                    }
                                    2 => {
                                        // Classic Red
                                        if i < 4 { "\x1B[1;31m" } else { "\x1B[0;31m" }
                                    }
                                    3 => {
                                        // Neon Magenta
                                        if i < 4 { "\x1B[1;35m" } else { "\x1B[0;35m" }
                                    }
                                    4 => {
                                        // Icy Blue
                                        if i < 4 { "\x1B[1;34m" } else { "\x1B[0;34m" }
                                    }
                                    5 => {
                                        // Rainbow loop
                                        let offset_color = (drop.col + segment_y) % 5;
                                        match offset_color {
                                            0 => "\x1B[1;31m",
                                            1 => "\x1B[1;32m",
                                            2 => "\x1B[1;33m",
                                            3 => "\x1B[1;34m",
                                            _ => "\x1B[1;35m",
                                        }
                                    }
                                    _ => {
                                        // Matrix Green (Default)
                                        if i < 4 { "\x1B[1;32m" } else { "\x1B[0;32m" }
                                    }
                                }
                            };
                            colors[segment_y][drop.col] = scheme_color;
                        }
                    }
                }

                // Remove drops that have exited the screen bottom
                drops.retain(|d| (d.y as usize - d.length) < height);

                // Write the full frame string to stdout
                let mut out = String::new();
                out.push_str("\x1B[H"); // move cursor to home
                for y in 0..height {
                    let mut last_color = "";
                    for x in 0..width {
                        let color = colors[y][x];
                        if color != last_color {
                            out.push_str(color);
                            last_color = color;
                        }
                        out.push(screen[y][x]);
                    }
                    if y < height - 1 {
                        out.push('\n');
                    }
                }
                print!("{}", out);
                let _ = io::stdout().flush();
            }
        }

        // Sleep to regulate target frame rate
        let elapsed = last_frame.elapsed();
        let target_dur = Duration::from_millis(1000 / fps as u64);
        if elapsed < target_dur {
            std::thread::sleep(target_dur - elapsed);
        }
        last_frame = Instant::now();
    }

    // Clean exit: restore cursor, return to main buffer, show cursor
    print!("\x1B[?1049l\x1B[?25h\x1B[0m");
    let _ = io::stdout().flush();
}
