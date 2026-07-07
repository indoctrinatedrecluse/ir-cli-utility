use crate::LifeOptions;
use std::io::{self, Write, IsTerminal};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

struct Lcg {
    state: u64,
}

impl Lcg {
    fn new() -> Self {
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(42);
        Self { state: seed }
    }

    fn next_f64(&mut self) -> f64 {
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        (self.state as f64) / (u64::MAX as f64)
    }
}

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
    Left,
    Right,
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
                        75 => Key::Left,
                        77 => Key::Right,
                        _ => Key::None,
                    };
                }
                match ch {
                    27 => Key::Esc,
                    13 => Key::Char('\n'),
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
        let mut buf = [0u8; 16];
        let n = unsafe { libc::read(0, buf.as_mut_ptr() as *mut libc::c_void, 16) };
        if n > 0 {
            if buf[0] == 27 {
                if n == 1 { return Key::Esc; }
                if n >= 3 && buf[1] == b'[' {
                    return match buf[2] {
                        b'A' => Key::Up,
                        b'B' => Key::Down,
                        b'C' => Key::Right,
                        b'D' => Key::Left,
                        _ => Key::None,
                    };
                }
            }
            match buf[0] {
                10 | 13 => Key::Char('\n'),
                27 => Key::Esc,
                c if c >= 32 && c < 127 => Key::Char(c as char),
                _ => Key::None,
            }
        } else {
            Key::None
        }
    }
}

// ─── Terminal Size ──────────────────────────────────────────────────────────────
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

fn spawn_preset(grid: &mut Vec<Vec<u32>>, cy: usize, cx: usize, preset_idx: usize) {
    let rows = grid.len();
    if rows == 0 { return; }
    let cols = grid[0].len();

    let offsets = match preset_idx {
        1 => {
            // Glider
            vec![(0, -1), (1, 0), (-1, 1), (0, 1), (1, 1)]
        }
        2 => {
            // Pulsar (symmetry parts)
            vec![
                (0, -2), (0, -3), (0, -4), (0, 2), (0, 3), (0, 4),
                (-2, 0), (-3, 0), (-4, 0), (2, 0), (3, 0), (4, 0),
                (-2, -2), (-2, -3), (-2, -4), (2, -2), (2, -3), (2, -4),
                (-2, 2), (-2, 3), (-2, 4), (2, 2), (2, 3), (2, 4),
            ]
        }
        3 => {
            // Gosper Glider Gun
            vec![
                (0, 4), (0, 5), (1, 4), (1, 5), // Left block
                (0, -6), (1, -6), (2, -6), (-1, -5), (3, -5), // First gun bubble
                (-2, -4), (-2, -3), (4, -4), (4, -3),
                (1, -2), (-1, -1), (3, -1), (0, 0), (1, 0), (2, 0), (1, 1),
                (-2, 2), (-1, 2), (0, 2), (-2, 3), (-1, 3), (0, 3), (1, 4), (-3, 4), (-3, 6), (-4, 6), (1, 6), (2, 6),
                (-2, 14), (-1, 14), (-2, 15), (-1, 15) // Right block
            ]
        }
        4 => {
            // Toad
            vec![(-1, 0), (0, 0), (1, 0), (-2, 1), (-1, 1), (0, 1)]
        }
        _ => vec![],
    };

    for (dy, dx) in offsets {
        let ny = (cy as isize + dy).rem_euclid(rows as isize) as usize;
        let nx = (cx as isize + dx).rem_euclid(cols as isize) as usize;
        grid[ny][nx] = 1;
    }
}

pub fn run_life(options: LifeOptions) {
    let mut fps = options.fps.unwrap_or(10).max(1).min(30);
    let preset = options.preset.clone().unwrap_or_else(|| "random".to_string());

    #[cfg(not(target_os = "windows"))]
    let _raw_guard = set_raw_mode();

    print!("\x1B[2J\x1B[?25l"); // clear screen and hide cursor
    let _ = io::stdout().flush();

    let (mut cols, mut rows) = terminal_size();
    let mut grid_rows = rows.saturating_sub(3) as usize;
    let mut grid_cols = cols as usize;

    let mut grid = vec![vec![0u32; grid_cols]; grid_rows];

    let mut rng = Lcg::new();

    // Seed preset / randomization
    match preset.to_lowercase().as_str() {
        "glider-gun" => {
            spawn_preset(&mut grid, grid_rows / 2, grid_cols / 2, 3);
        }
        "pulsar" => {
            spawn_preset(&mut grid, grid_rows / 2, grid_cols / 2, 2);
        }
        _ => {
            // Random
            for r in 0..grid_rows {
                for c in 0..grid_cols {
                    if rng.next_f64() < 0.2 {
                        grid[r][c] = 1;
                    }
                }
            }
        }
    }

    let mut paused = false;
    let mut cursor_y = grid_rows / 2;
    let mut cursor_x = grid_cols / 2;
    let mut generation = 0u64;

    let mut last_update = Instant::now();

    loop {
        // Keyboard event polling
        let key = poll_key();
        match key {
            Key::Esc | Key::Char('q') | Key::Char('Q') => break,
            Key::Char(' ') => paused = !paused,
            Key::Char('\n') => {
                if cursor_y < grid_rows && cursor_x < grid_cols {
                    grid[cursor_y][cursor_x] = if grid[cursor_y][cursor_x] > 0 { 0 } else { 1 };
                }
            }
            Key::Char('r') | Key::Char('R') => {
                generation = 0;
                for r in 0..grid_rows {
                    for c in 0..grid_cols {
                        grid[r][c] = if rng.next_f64() < 0.2 { 1 } else { 0 };
                    }
                }
            }
            Key::Char('c') | Key::Char('C') => {
                generation = 0;
                for r in 0..grid_rows {
                    for c in 0..grid_cols {
                        grid[r][c] = 0;
                    }
                }
            }
            Key::Char('1') => spawn_preset(&mut grid, cursor_y, cursor_x, 1),
            Key::Char('2') => spawn_preset(&mut grid, cursor_y, cursor_x, 2),
            Key::Char('3') => spawn_preset(&mut grid, cursor_y, cursor_x, 3),
            Key::Char('4') => spawn_preset(&mut grid, cursor_y, cursor_x, 4),
            Key::Char('[') => {
                if fps > 1 { fps -= 1; }
            }
            Key::Char(']') => {
                if fps < 30 { fps += 1; }
            }
            Key::Up => {
                if cursor_y > 0 { cursor_y -= 1; }
            }
            Key::Down => {
                if cursor_y + 1 < grid_rows { cursor_y += 1; }
            }
            Key::Left => {
                if cursor_x > 0 { cursor_x -= 1; }
            }
            Key::Right => {
                if cursor_x + 1 < grid_cols { cursor_x += 1; }
            }
            _ => {}
        }

        // Handle terminal resizing
        let (new_cols, new_rows) = terminal_size();
        if new_cols != cols || new_rows != rows {
            cols = new_cols;
            rows = new_rows;
            let new_grid_rows = rows.saturating_sub(3) as usize;
            let new_grid_cols = cols as usize;

            let mut new_grid = vec![vec![0u32; new_grid_cols]; new_grid_rows];
            for r in 0..new_grid_rows.min(grid_rows) {
                for c in 0..new_grid_cols.min(grid_cols) {
                    new_grid[r][c] = grid[r][c];
                }
            }
            grid = new_grid;
            grid_rows = new_grid_rows;
            grid_cols = new_grid_cols;
            cursor_y = cursor_y.min(grid_rows - 1);
            cursor_x = cursor_x.min(grid_cols - 1);
            print!("\x1B[2J");
        }

        // Simulation update (FPS controlled)
        let delta = Duration::from_millis(1000 / fps as u64);
        if !paused && last_update.elapsed() >= delta {
            let mut next_grid = vec![vec![0u32; grid_cols]; grid_rows];
            for r in 0..grid_rows {
                for c in 0..grid_cols {
                    // Count neighbors (with wrap-around borders)
                    let mut neighbors = 0;
                    for dr in -1..=1 {
                        for dc in -1..=1 {
                            if dr == 0 && dc == 0 { continue; }
                            let nr = (r as isize + dr).rem_euclid(grid_rows as isize) as usize;
                            let nc = (c as isize + dc).rem_euclid(grid_cols as isize) as usize;
                            if grid[nr][nc] > 0 {
                                neighbors += 1;
                            }
                        }
                    }

                    if grid[r][c] > 0 {
                        // Survival
                        if neighbors == 2 || neighbors == 3 {
                            next_grid[r][c] = grid[r][c] + 1; // age increments
                        } else {
                            next_grid[r][c] = 0; // dies
                        }
                    } else {
                        // Birth
                        if neighbors == 3 {
                            next_grid[r][c] = 1; // born
                        } else {
                            next_grid[r][c] = 0;
                        }
                    }
                }
            }
            grid = next_grid;
            generation += 1;
            last_update = Instant::now();
        }

        // Rendering buffer compilation
        let mut out = String::new();
        out.push_str("\x1B[H"); // Reset cursor home

        let use_color = io::stdout().is_terminal();
        let color_reset = if use_color { "\x1B[0m" } else { "" };
        let color_cursor = if use_color { "\x1B[7m" } else { "" }; // Inverse color
        let color_ui = if use_color { "\x1B[90m" } else { "" };

        // Colors for cell age
        let color_age_new = if use_color { "\x1B[32m" } else { "" };     // Green
        let color_age_med = if use_color { "\x1B[36m" } else { "" };     // Cyan
        let color_age_old = if use_color { "\x1B[34m" } else { "" };     // Blue
        let color_age_ancient = if use_color { "\x1B[35m" } else { "" }; // Magenta

        for r in 0..grid_rows {
            let mut line = String::new();
            for c in 0..grid_cols {
                let is_cursor = r == cursor_y && c == cursor_x;
                let age = grid[r][c];

                if is_cursor {
                    let cell_char = if age > 0 { "█" } else { " " };
                    line.push_str(color_cursor);
                    line.push_str(cell_char);
                    line.push_str(color_reset);
                } else if age > 0 {
                    let color = if age == 1 {
                        color_age_new
                    } else if age <= 3 {
                        color_age_med
                    } else if age <= 8 {
                        color_age_old
                    } else {
                        color_age_ancient
                    };
                    line.push_str(color);
                    line.push_str("█");
                    line.push_str(color_reset);
                } else {
                    line.push(' ');
                }
            }
            out.push_str(&format!("{}\n", line));
        }

        // Stats dashboard
        let live_cells: usize = grid.iter().map(|row| row.iter().filter(|&&age| age > 0).count()).sum();
        let state_str = if paused { "PAUSED" } else { "RUNNING" };
        let stats_str = format!(
            "Gen: {} | Live Cells: {} | Speed: {} FPS | State: {}",
            generation, live_cells, fps, state_str
        );
        let shortcuts_str = "  [Space] Play/Pause [Enter] Draw [R] Random [C] Clear [1-4] Presets [[/]] Speed  ";
        let stats_pad = ((cols as usize).saturating_sub(stats_str.len()) / 2).max(0);
        let shortcuts_pad = ((cols as usize).saturating_sub(shortcuts_str.len()) / 2).max(0);

        out.push_str(&format!("{}{}{}{}\n", " ".repeat(stats_pad), color_ui, stats_str, color_reset));
        out.push_str(&format!("{}{}{}{}", " ".repeat(shortcuts_pad), color_ui, shortcuts_str, color_reset));

        print!("{}", out);
        let _ = io::stdout().flush();

        // 30ms sleep (event processing loop rate)
        std::thread::sleep(Duration::from_millis(30));
    }

    print!("\x1B[?25h\x1B[2J\x1B[H"); // show cursor and clear screen
    let _ = io::stdout().flush();
}
