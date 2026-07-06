use crate::ClockOptions;
use chrono::{Local, Datelike, Timelike};
use std::io::{self, Write};
use std::time::{Duration, Instant};

// ─── Key enum ──────────────────────────────────────────────────────────────────
#[derive(Debug, PartialEq, Clone, Copy)]
enum Key {
    Char(char),
    Tab,
    Enter,
    Space,
    Esc,
    None,
}

// ─── Windows FFI ───────────────────────────────────────────────────────────────
#[cfg(target_os = "windows")]
extern "C" {
    fn _kbhit() -> std::os::raw::c_int;
    fn _getch() -> std::os::raw::c_int;
}

// ─── Linux Raw Mode Guard ───────────────────────────────────────────────────────
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

// ─── Cross-platform Keyboard Polling ───────────────────────────────────────────
fn poll_key() -> Key {
    #[cfg(target_os = "windows")]
    {
        unsafe {
            if _kbhit() != 0 {
                let ch = _getch();
                match ch {
                    9 => Key::Tab,
                    13 => Key::Enter,
                    32 => Key::Space,
                    27 => Key::Esc,
                    c if c >= 32 && c < 127 => Key::Char(c as u8 as char),
                    _ => Key::None,
                }
            } else {
                Key::None
            }
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        let mut buf = [0u8; 1];
        let n = unsafe { libc::read(0, buf.as_mut_ptr() as *mut libc::c_void, 1) };
        if n > 0 {
            match buf[0] {
                9 => Key::Tab,
                10 | 13 => Key::Enter,
                32 => Key::Space,
                27 => Key::Esc,
                c if c >= 32 && c < 127 => Key::Char(c as char),
                _ => Key::None,
            }
        } else {
            Key::None
        }
    }
}

// ─── Terminal Dimensions ────────────────────────────────────────────────────────
fn terminal_size() -> (u16, u16) {
    #[cfg(target_os = "windows")]
    unsafe {
        use windows_sys::Win32::System::Console::{
            GetConsoleScreenBufferInfo, GetStdHandle, STD_OUTPUT_HANDLE,
        };
        let h = GetStdHandle(STD_OUTPUT_HANDLE);
        let mut info = std::mem::zeroed();
        if GetConsoleScreenBufferInfo(h, &mut info) != 0 {
            let cols = (info.srWindow.Right  - info.srWindow.Left + 1) as u16;
            let rows = (info.srWindow.Bottom - info.srWindow.Top  + 1) as u16;
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

// ─── Giant Block Digits (5 lines high) ──────────────────────────────────────────
const DIGITS: [[&str; 5]; 12] = [
    // 0
    ["██████▄ ", "██  ▀██ ", "██   ██ ", "██   ██ ", "▀█████▀ "],
    // 1
    ["   ██   ", "   ██   ", "   ██   ", "   ██   ", "   ██   "],
    // 2
    ["███████▄", "      ██", "▄██████▀", "██      ", "████████"],
    // 3
    ["███████▄", "      ██", " ▀█████▄", "      ██", "███████▀"],
    // 4
    ["██    ██", "██    ██", "████████", "      ██", "      ██"],
    // 5
    ["████████", "██      ", "███████▄", "      ██", "███████▀"],
    // 6
    ["████████", "██      ", "███████▄", "██    ██", "███████▀"],
    // 7
    ["████████", "      ██", "    ██▀ ", "   ██   ", "  ██    "],
    // 8
    ["▄██████▄", "██▀  ▀██", "▀██████▀", "██▄  ▄██", "▀██████▀"],
    // 9
    ["▄██████▄", "██▄  ▄██", "▀███████", "      ██", "▀██████▀"],
    // : (Index 10)
    ["   ", " ▄ ", "   ", " ▄ ", "   "],
    // . (Index 11)
    ["   ", "   ", "   ", "   ", " ▄ "],
];

// Helper to calculate total display width of a giant block string representation
fn get_rendered_width(s: &str) -> usize {
    let mut w: usize = 0;
    for ch in s.chars() {
        match ch {
            ':' | '.' => w += 3 + 1,
            '0'..='9' => w += 8 + 1,
            _ => w += 8 + 1,
        }
    }
    w.saturating_sub(1)
}

// Parse duration string into standard Duration (e.g. 5m30s -> 330 seconds)
fn parse_duration(s: &str) -> Option<Duration> {
    let mut total_secs = 0u64;
    let mut temp = String::new();
    for ch in s.chars() {
        if ch.is_ascii_digit() {
            temp.push(ch);
        } else {
            if temp.is_empty() { return None; }
            let val = temp.parse::<u64>().ok()?;
            temp.clear();
            match ch.to_ascii_lowercase() {
                'h' => total_secs += val * 3600,
                'm' => total_secs += val * 60,
                's' => total_secs += val,
                _ => return None,
            }
        }
    }
    if !temp.is_empty() {
        let val = temp.parse::<u64>().ok()?;
        total_secs += val;
    }
    Some(Duration::from_secs(total_secs))
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum AppMode {
    Clock,
    Stopwatch,
    Timer,
}

pub fn run_clock(options: ClockOptions) {
    // Setup Unix Raw mode guard if on Unix
    #[cfg(not(target_os = "windows"))]
    let _raw_guard = set_raw_mode();

    // Configure initial mode
    let mut mode = AppMode::Clock;
    if let Some(ref m) = options.mode {
        match m.as_str() {
            "stopwatch" => mode = AppMode::Stopwatch,
            "timer" => mode = AppMode::Timer,
            _ => {}
        }
    }

    // Timer setup
    let mut timer_duration = Duration::from_secs(300); // 5 minutes default
    if let Some(ref d) = options.timer_duration {
        if let Some(parsed) = parse_duration(d) {
            timer_duration = parsed;
            mode = AppMode::Timer; // Auto-switch to timer mode if duration is set
        } else {
            eprintln!("Error: Invalid timer duration format '{}'. Use e.g. 5m30s, 10m, or 300.", d);
            std::process::exit(1);
        }
    }

    let mut timer_remaining = timer_duration;
    let mut timer_running = false;
    let mut timer_last_tick = Instant::now();

    // Stopwatch setup
    let mut stopwatch_elapsed = Duration::ZERO;
    let mut stopwatch_running = false;
    let mut stopwatch_last_tick = Instant::now();
    let mut stopwatch_laps: Vec<Duration> = Vec::new();

    // Keyboard and UI loop
    let mut last_size = terminal_size();
    
    // Clear screen and hide cursor
    print!("\x1B[2J\x1B[?25l");
    let _ = io::stdout().flush();

    let mut alarm_active = false;
    let mut alarm_flash = false;
    let mut alarm_start = Instant::now();

    loop {
        // Handle input events
        let key = poll_key();
        match key {
            Key::Esc | Key::Char('q') | Key::Char('Q') => break,
            Key::Tab | Key::Char('c') | Key::Char('C') => {
                alarm_active = false;
                mode = match mode {
                    AppMode::Clock => AppMode::Stopwatch,
                    AppMode::Stopwatch => AppMode::Timer,
                    AppMode::Timer => AppMode::Clock,
                };
            }
            Key::Char('1') => { alarm_active = false; mode = AppMode::Clock; }
            Key::Char('2') => { alarm_active = false; mode = AppMode::Stopwatch; }
            Key::Char('3') => { alarm_active = false; mode = AppMode::Timer; }
            Key::Space => {
                if mode == AppMode::Stopwatch {
                    if stopwatch_running {
                        stopwatch_elapsed += stopwatch_last_tick.elapsed();
                        stopwatch_running = false;
                    } else {
                        stopwatch_last_tick = Instant::now();
                        stopwatch_running = true;
                    }
                } else if mode == AppMode::Timer {
                    alarm_active = false;
                    if timer_running {
                        timer_remaining = timer_remaining.saturating_sub(timer_last_tick.elapsed());
                        timer_running = false;
                    } else {
                        if timer_remaining == Duration::ZERO {
                            timer_remaining = timer_duration; // Auto-reset on run
                        }
                        timer_last_tick = Instant::now();
                        timer_running = true;
                    }
                }
            }
            Key::Enter | Key::Char('l') | Key::Char('L') => {
                if mode == AppMode::Stopwatch {
                    if stopwatch_running {
                        let cur_lap = stopwatch_elapsed + stopwatch_last_tick.elapsed();
                        if stopwatch_laps.len() < 10 {
                            stopwatch_laps.push(cur_lap);
                        } else {
                            stopwatch_laps.remove(0);
                            stopwatch_laps.push(cur_lap);
                        }
                    }
                }
            }
            Key::Char('r') | Key::Char('R') => {
                if mode == AppMode::Stopwatch {
                    stopwatch_elapsed = Duration::ZERO;
                    stopwatch_running = false;
                    stopwatch_laps.clear();
                } else if mode == AppMode::Timer {
                    alarm_active = false;
                    timer_remaining = timer_duration;
                    timer_running = false;
                }
            }
            _ => {}
        }

        // Time updates
        if stopwatch_running {
            // Keep dynamic updating but don't commit to stopwatch_elapsed until pause/lap
        }

        if timer_running {
            let elapsed = timer_last_tick.elapsed();
            if elapsed >= timer_remaining {
                timer_remaining = Duration::ZERO;
                timer_running = false;
                alarm_active = true;
                alarm_start = Instant::now();
            }
        }

        // Handle alarm flashing
        if alarm_active {
            if alarm_start.elapsed() > Duration::from_secs(10) {
                alarm_active = false; // Turn off alarm after 10 seconds
            } else {
                alarm_flash = (alarm_start.elapsed().as_millis() / 300) % 2 == 0;
            }
        }

        // Fetch size and render
        let size = terminal_size();
        let (cols, rows) = size;

        // Visual render
        let mut out = String::new();
        // Move to top-left corner
        out.push_str("\x1B[H");

        // Clear screens on resize to prevent visual ghosting
        if size != last_size {
            out.push_str("\x1B[2J");
            last_size = size;
        }

        // Setup colors based on alarm or mode
        let color_cyan = "\x1B[1;36m";
        let color_green = "\x1B[1;32m";
        let color_yellow = "\x1B[1;33m";
        let color_red = "\x1B[1;31m";
        let color_grey = "\x1B[90m";
        let color_reset = "\x1B[0m";

        let current_color = if alarm_active {
            if alarm_flash { "\x1B[1;37;41m" } else { "\x1B[1;31;40m" }
        } else {
            match mode {
                AppMode::Clock => color_cyan,
                AppMode::Stopwatch => color_green,
                AppMode::Timer => if timer_running { color_yellow } else { color_red },
            }
        };

        // Render main content
        let time_str;
        let sub_text;
        let mut bar_content = String::new();
        let mut bar_width = 0;

        match mode {
            AppMode::Clock => {
                let now = Local::now();
                time_str = format!("{:02}:{:02}:{:02}", now.hour(), now.minute(), now.second());
                sub_text = format!(
                    "{} {}, {} — Week {}",
                    now.format("%A"),
                    now.format("%B %d"),
                    now.year(),
                    now.iso_week().week()
                );
            }
            AppMode::Stopwatch => {
                let cur = if stopwatch_running {
                    stopwatch_elapsed + stopwatch_last_tick.elapsed()
                } else {
                    stopwatch_elapsed
                };
                let mins = cur.as_secs() / 60;
                let secs = cur.as_secs() % 60;
                let centis = (cur.as_millis() % 1000) / 10;
                time_str = format!("{:02}:{:02}.{:02}", mins, secs, centis);
                
                let state_str = if stopwatch_running { "RUNNING" } else { "PAUSED" };
                sub_text = format!("[Stopwatch: {}]", state_str);
            }
            AppMode::Timer => {
                let cur = if timer_running {
                    timer_remaining.saturating_sub(timer_last_tick.elapsed())
                } else {
                    timer_remaining
                };
                let hrs = cur.as_secs() / 3600;
                let mins = (cur.as_secs() % 3600) / 60;
                let secs = cur.as_secs() % 60;
                time_str = if hrs > 0 {
                    format!("{:02}:{:02}:{:02}", hrs, mins, secs)
                } else {
                    format!("{:02}:{:02}", mins, secs)
                };

                let state_str = if alarm_active {
                    "ALARM! TIME IS UP"
                } else if timer_running {
                    "COUNTDOWN RUNNING"
                } else {
                    "TIMER PAUSED"
                };
                sub_text = format!("[Timer: {}]", state_str);

                bar_width = (cols as usize).saturating_sub(16).max(10).min(60);
                let percent = if timer_duration.as_secs_f64() > 0.0 {
                    cur.as_secs_f64() / timer_duration.as_secs_f64()
                } else {
                    0.0
                };
                let filled = (percent * bar_width as f64).round() as usize;
                let empty = bar_width.saturating_sub(filled);

                bar_content = format!(
                    "{}{}{}{}{}",
                    color_grey,
                    "░".repeat(empty),
                    current_color,
                    "█".repeat(filled),
                    color_reset
                );
            }
        }

        // Center calculation
        let rendered_w = get_rendered_width(&time_str);
        let left_pad = ((cols as usize).saturating_sub(rendered_w) / 2).max(0);
        let pad_str = " ".repeat(left_pad);

        let sub_left_pad = ((cols as usize).saturating_sub(sub_text.chars().count()) / 2).max(0);
        let sub_pad_str = " ".repeat(sub_left_pad);

        // Giant digit drawing
        let mut giant_rows = Vec::new();
        for r in 0..5 {
            let mut line = String::new();
            line.push_str(&pad_str);
            for ch in time_str.chars() {
                let idx = match ch {
                    ':' => 10,
                    '.' => 11,
                    '0'..='9' => ch as usize - '0' as usize,
                    _ => 0,
                };
                line.push_str(DIGITS[idx][r]);
                line.push_str(" ");
            }
            giant_rows.push(line);
        }

        // Vertical padding
        let total_content_height = 5 + 2 + if !bar_content.is_empty() { 2 } else { 0 } + if mode == AppMode::Stopwatch && !stopwatch_laps.is_empty() { stopwatch_laps.len() + 1 } else { 0 };
        let top_pad = ((rows as usize).saturating_sub(total_content_height) / 2).saturating_sub(2).max(0);

        for _ in 0..top_pad {
            out.push_str("\n\x1B[K");
        }

        // Draw time giant digits
        for grow in giant_rows {
            out.push_str(&format!("{}{}{}{}\n", current_color, grow, color_reset, "\x1B[K"));
        }
        out.push_str("\n\x1B[K");

        // Draw sub-text (date/details)
        out.push_str(&format!("{}{}{}{}\n", sub_pad_str, current_color, sub_text, color_reset));
        out.push_str("\x1B[K\n");

        // Draw Progress Bar (Timer Mode)
        if !bar_content.is_empty() {
            let bar_left_pad = ((cols as usize).saturating_sub(bar_width) / 2).max(0);
            let bar_pad_str = " ".repeat(bar_left_pad);
            out.push_str(&format!("{}{}\n", bar_pad_str, bar_content));
            out.push_str("\x1B[K\n");
        }

        // Draw Recorded Laps (Stopwatch Mode)
        if mode == AppMode::Stopwatch && !stopwatch_laps.is_empty() {
            let title_str = "--- Laps ---";
            let lap_title_pad = ((cols as usize).saturating_sub(title_str.len()) / 2).max(0);
            out.push_str(&format!("{}{}{}\n", " ".repeat(lap_title_pad), color_grey, title_str));
            for (idx, lap) in stopwatch_laps.iter().enumerate() {
                let lmins = lap.as_secs() / 60;
                let lsecs = lap.as_secs() % 60;
                let lcentis = (lap.as_millis() % 1000) / 10;
                let lap_str = format!("Lap {}: {:02}:{:02}.{:02}", idx + 1, lmins, lsecs, lcentis);
                let lap_pad = ((cols as usize).saturating_sub(lap_str.len()) / 2).max(0);
                out.push_str(&format!("{}{}{}{}\n", " ".repeat(lap_pad), color_grey, lap_str, color_reset));
            }
            out.push_str("\x1B[K\n");
        }

        // Fill remaining height to clear leftover chars
        let current_printed_rows = top_pad + total_content_height + 2;
        let bottom_pad = (rows as usize).saturating_sub(current_printed_rows).saturating_sub(3).max(0);
        for _ in 0..bottom_pad {
            out.push_str("\n\x1B[K");
        }

        // Draw Help Guide at the very bottom
        let shortcuts_str = "[Tab/C] Mode   [Space] Play/Pause   [Enter/L] Lap   [R] Reset   [1/2/3] QuickMode   [Q] Quit";
        let shortcuts_pad = ((cols as usize).saturating_sub(shortcuts_str.len()) / 2).max(0);
        out.push_str(&format!("\n{}{}{}{}\n", " ".repeat(shortcuts_pad), color_grey, shortcuts_str, color_reset));

        // Output to screen
        print!("{}", out);
        let _ = io::stdout().flush();

        // Trigger alarm beep (bell sound)
        if alarm_active && alarm_flash {
            print!("\x07");
            let _ = io::stdout().flush();
        }

        // Frame interval sleep
        std::thread::sleep(Duration::from_millis(50));
    }

    // Cleanup: Show cursor and clear screen
    print!("\x1B[?25h\x1B[2J\x1B[H");
    let _ = io::stdout().flush();
}
