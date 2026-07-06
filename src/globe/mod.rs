use crate::GlobeOptions;
use chrono::{Local, Datelike, Timelike};
use std::fs::File;
use std::io::{self, BufRead, IsTerminal, Write};
use std::time::{Duration, Instant};

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
    Tab,
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
                    9 => Key::Tab,
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
                9 => Key::Tab,
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

// ─── Built-in High Quality World Map (24x72 ASCII resolution) ───────────────────
const MAP_ROWS: usize = 24;
const MAP_COLS: usize = 72;
const WORLD_MAP: [&str; 24] = [
    "                                                                                ",
    "                      ██████                                                    ",
    "   ▄████████           ███████        ▄▄████████████████████████████████▄       ",
    "  ███████████▄           ████       ▄█████████████████████████████████████▄     ",
    "  ██████████████                    ███████████████████████████████████████     ",
    "   █████████████                    ██████████████████████████████████████▀     ",
    "    ▀█████████▀    ▄              ▄██████████████████████████████████████▀      ",
    "       ████       ▀              ███████████████████████████████████████▀       ",
    "        ██                       ████████████████████████████████████▀          ",
    "       ███▄                      ██████████████████████████████████▀     ▄▄     ",
    "      ██████                     ▀████████████████████████████████▀    ▄████    ",
    "     ████████                     ████████████████████████████▀▀      ███████   ",
    "     ████████                     ██████████████████████████▀          ▀████▀   ",
    "     ████████                     ▀██████████████████████▀                      ",
    "     ███████▀                      █████████████████████             ▄██████▄   ",
    "      ██████                       ██████████████████▀              ██████████  ",
    "      ████▀                         ███████████████▀                ██████████  ",
    "       ██▀                           ████████████▀                   ▀██████▀   ",
    "       ▀                              ██████████▀                      ▀██▀     ",
    "                                       ████████▀                                ",
    "                                        ██████                                  ",
    "  ████████████████████████████████████████████████████████████████████████████  ",
    "  ████████████████████████████████████████████████████████████████████████████  ",
    "                                                                                ",
];

struct Marker {
    lat: f64,
    lon: f64,
}

// Convert screen character index to land/sea state
fn get_map_char(lat: f64, lon: f64) -> char {
    let lat_rad = lat.to_radians();
    let lon_rad = lon.to_radians();
    
    // Map bounds mapping
    let r_val = (((std::f64::consts::FRAC_PI_2 - lat_rad) / std::f64::consts::PI) * (MAP_ROWS as f64 - 1.0)).round() as isize;
    let c_val = ((((lon_rad + std::f64::consts::PI) / (2.0 * std::f64::consts::PI)) * (MAP_COLS as f64 - 1.0)).round()) as isize;
    
    if r_val >= 0 && r_val < MAP_ROWS as isize && c_val >= 0 && c_val < MAP_COLS as isize {
        let ch = WORLD_MAP[r_val as usize].chars().nth(c_val as usize).unwrap_or(' ');
        if ch != ' ' { '█' } else { ' ' }
    } else {
        ' '
    }
}

// Sub-stellar solar point approximation
fn get_solar_position() -> (f64, f64) {
    let now = Local::now();
    let day_of_year = now.ordinal() as f64;
    // Declination varies between -23.44 and +23.44 degrees
    let declination = 23.44 * (2.0 * std::f64::consts::PI * (day_of_year - 80.0) / 365.25).sin();
    
    // Solar longitude corresponds to UTC time
    let utc = now.naive_utc();
    let utc_hours = utc.hour() as f64 + utc.minute() as f64 / 60.0 + utc.second() as f64 / 3600.0;
    // Sun moves 15 degrees per hour, 12:00 UTC is at 0 degrees longitude
    let solar_lon = -(utc_hours - 12.0) * 15.0;
    
    (declination, solar_lon)
}

pub fn run_globe(input_path: Option<&str>, options: GlobeOptions) {
    // Parse markers from log files if provided
    let mut markers = Vec::new();
    if let Some(path) = input_path {
        let file = match File::open(path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Error: Failed to open coordinate file '{}': {}", path, e);
                std::process::exit(1);
            }
        };
        let reader = io::BufReader::new(file);
        for (line_idx, line_res) in reader.lines().enumerate() {
            if let Ok(line) = line_res {
                let trimmed = line.trim();
                if trimmed.is_empty() || trimmed.starts_with('#') { continue; }
                let parts: Vec<&str> = trimmed.split(',').collect();
                if parts.len() >= 2 {
                    if let (Ok(lat), Ok(lon)) = (parts[0].trim().parse::<f64>(), parts[1].trim().parse::<f64>()) {
                        if lat >= -90.0 && lat <= 90.0 && lon >= -180.0 && lon <= 180.0 {
                            markers.push(Marker { lat, lon });
                        } else {
                            eprintln!("Warning: Skipping invalid coordinate coordinates at line {}: lat={}, lon={}", line_idx + 1, lat, lon);
                        }
                    }
                }
            }
        }
    }

    // Set initial center coordinates
    let mut center_lat = 0.0f64;
    let mut center_lon = 0.0f64;
    if let Some(ref c) = options.center {
        let parts: Vec<&str> = c.split(',').collect();
        if parts.len() == 2 {
            if let (Ok(lat), Ok(lon)) = (parts[0].trim().parse::<f64>(), parts[1].trim().parse::<f64>()) {
                if lat >= -90.0 && lat <= 90.0 && lon >= -180.0 && lon <= 180.0 {
                    center_lat = lat;
                    center_lon = lon;
                } else {
                    eprintln!("Error: Coordinates must be in range -90..90 latitude and -180..180 longitude.");
                    std::process::exit(1);
                }
            } else {
                eprintln!("Error: Center option must be formatted as 'lat,lon' (e.g. 40.7,-74.0).");
                std::process::exit(1);
            }
        } else {
            eprintln!("Error: Center option must be formatted as 'lat,lon'.");
            std::process::exit(1);
        }
    } else if !markers.is_empty() {
        // Center at the average of all coordinates
        let sum_lat: f64 = markers.iter().map(|m| m.lat).sum();
        let sum_lon: f64 = markers.iter().map(|m| m.lon).sum();
        center_lat = sum_lat / markers.len() as f64;
        center_lon = sum_lon / markers.len() as f64;
    }

    // Determine initial projection mode
    let mut flat_map_mode = false;
    if let Some(ref m) = options.mode {
        if m.to_lowercase() == "map" {
            flat_map_mode = true;
        }
    }

    // Interactive rendering state
    let mut zoom = 1.0f64;
    let mut day_night_shading = options.day_night;

    #[cfg(not(target_os = "windows"))]
    let _raw_guard = set_raw_mode();

    print!("\x1B[2J\x1B[?25l"); // clear screen and hide cursor
    let _ = io::stdout().flush();

    let mut last_size = terminal_size();
    let frame_time = Instant::now();

    loop {
        // Keyboard event polling
        let key = poll_key();
        match key {
            Key::Esc | Key::Char('q') | Key::Char('Q') => break,
            Key::Tab | Key::Char('m') | Key::Char('M') => {
                flat_map_mode = !flat_map_mode;
            }
            Key::Char('d') | Key::Char('D') => {
                day_night_shading = !day_night_shading;
            }
            Key::Char('+') | Key::Char('=') => {
                zoom += 0.1;
                if zoom > 3.0 { zoom = 3.0; }
            }
            Key::Char('-') | Key::Char('_') => {
                zoom -= 0.1;
                if zoom < 0.5 { zoom = 0.5; }
            }
            Key::Up => {
                center_lat += 5.0;
                if center_lat > 90.0 { center_lat = 90.0; }
            }
            Key::Down => {
                center_lat -= 5.0;
                if center_lat < -90.0 { center_lat = -90.0; }
            }
            Key::Left => {
                center_lon -= 10.0;
                if center_lon < -180.0 { center_lon += 360.0; }
            }
            Key::Right => {
                center_lon += 10.0;
                if center_lon > 180.0 { center_lon -= 360.0; }
            }
            _ => {}
        }

        let size = terminal_size();
        let (cols, rows) = size;
        let mut out = String::new();
        out.push_str("\x1B[H"); // Reset cursor home

        if size != last_size {
            out.push_str("\x1B[2J");
            last_size = size;
        }

        // Color definitions
        let use_color = io::stdout().is_terminal();
        let color_reset = if use_color { "\x1B[0m" } else { "" };
        let color_ocean = if use_color { "\x1B[34m" } else { "" }; // Blue
        let color_land = if use_color { "\x1B[32m" } else { "" };  // Green
        let color_night_ocean = if use_color { "\x1B[90m" } else { "" }; // Dark Grey
        let color_night_land = if use_color { "\x1B[2;32m" } else { "" }; // Dimmed Green
        let color_marker = if use_color { "\x1B[1;5;31m" } else { "" }; // Bold blinking Red
        let color_ui = if use_color { "\x1B[90m" } else { "" }; // Grey

        let (solar_dec, solar_lon) = get_solar_position();

        let render_rows = rows.saturating_sub(3) as usize;
        let render_cols = cols as usize;

        // Render buffer
        let mut screen_chars = vec![vec![' '; render_cols]; render_rows];
        let mut screen_colors = vec![vec![color_reset; render_cols]; render_rows];

        if flat_map_mode {
            // --- 2D EQUITECTANGULAR MAP PROJECTION ---
            for r in 0..render_rows {
                let lat = 90.0 - (r as f64 / (render_rows - 1) as f64) * 180.0;
                for c in 0..render_cols {
                    let lon = -180.0 + (c as f64 / (render_cols - 1) as f64) * 360.0;
                    
                    let map_char = get_map_char(lat, lon);
                    let mut char_to_draw = if map_char == '█' { '█' } else { ' ' };
                    let mut color_to_draw = if map_char == '█' { color_land } else { color_ocean };

                    // Apply Day/Night shading
                    if day_night_shading {
                        let lat_r = lat.to_radians();
                        let lon_r = lon.to_radians();
                        let sol_lat_r = solar_dec.to_radians();
                        let sol_lon_r = solar_lon.to_radians();
                        
                        let cos_c = sol_lat_r.sin() * lat_r.sin() + sol_lat_r.cos() * lat_r.cos() * (lon_r - sol_lon_r).cos();
                        if cos_c < 0.0 {
                            if char_to_draw == '█' {
                                color_to_draw = color_night_land;
                            } else {
                                char_to_draw = '░';
                                color_to_draw = color_night_ocean;
                            }
                        }
                    }

                    screen_chars[r][c] = char_to_draw;
                    screen_colors[r][c] = color_to_draw;
                }
            }

            // Plot markers
            let show_blink = (frame_time.elapsed().as_millis() / 500) % 2 == 0;
            if show_blink {
                for marker in &markers {
                    let c = (((marker.lon + 180.0) / 360.0) * (render_cols - 1) as f64).round() as usize;
                    let r = (((90.0 - marker.lat) / 180.0) * (render_rows - 1) as f64).round() as usize;
                    if r < render_rows && c < render_cols {
                        screen_chars[r][c] = '●';
                        screen_colors[r][c] = color_marker;
                    }
                }
            }
        } else {
            // --- 3D SPHERICAL GLOBE PROJECTION ---
            let cx = render_cols as f64 / 2.0;
            let cy = render_rows as f64 / 2.0;
            let radius = ((render_rows as f64 / 2.0) - 1.0) * zoom;
            
            // Console char cell aspect correction ratio (width is narrower than height is tall)
            let aspect = 2.0;

            for r in 0..render_rows {
                let dy = cy - r as f64;
                for c in 0..render_cols {
                    let dx = (c as f64 - cx) / aspect;
                    let dist_sq = dx * dx + dy * dy;

                    if dist_sq <= radius * radius {
                        let dz = (radius * radius - dist_sq).sqrt();
                        
                        // Form unit vector (x, y, z) on sphere
                        let x = dx / radius;
                        let y = dy / radius;
                        let z = dz / radius;

                        // 3D rotations based on center coordinates
                        let lat_0 = center_lat.to_radians();
                        let lon_0 = center_lon.to_radians();

                        // X-axis rotation by latitude center
                        let x1 = x;
                        let y1 = y * lat_0.cos() - z * lat_0.sin();
                        let z1 = y * lat_0.sin() + z * lat_0.cos();

                        // Y-axis rotation by longitude center
                        let x_rot = x1 * lon_0.cos() + z1 * lon_0.sin();
                        let y_rot = y1;
                        let z_rot = -x1 * lon_0.sin() + z1 * lon_0.cos();

                        // Convert rotated vector back to spherical coordinates
                        let lat = y_rot.asin().to_degrees();
                        let lon = x_rot.atan2(z_rot).to_degrees();

                        let map_char = get_map_char(lat, lon);
                        let mut char_to_draw = if map_char == '█' { '█' } else { ' ' };
                        let mut color_to_draw = if map_char == '█' { color_land } else { color_ocean };

                        // Apply Day/Night shading
                        if day_night_shading {
                            let lat_r = lat.to_radians();
                            let lon_r = lon.to_radians();
                            let sol_lat_r = solar_dec.to_radians();
                            let sol_lon_r = solar_lon.to_radians();
                            
                            let cos_c = sol_lat_r.sin() * lat_r.sin() + sol_lat_r.cos() * lat_r.cos() * (lon_r - sol_lon_r).cos();
                            if cos_c < 0.0 {
                                if char_to_draw == '█' {
                                    color_to_draw = color_night_land;
                                } else {
                                    char_to_draw = '░';
                                    color_to_draw = color_night_ocean;
                                }
                            }
                        }

                        screen_chars[r][c] = char_to_draw;
                        screen_colors[r][c] = color_to_draw;
                    }
                }
            }

            // Plot markers (only show if on front side of the globe)
            let show_blink = (frame_time.elapsed().as_millis() / 500) % 2 == 0;
            if show_blink {
                let lat_0 = center_lat.to_radians();
                let lon_0 = center_lon.to_radians();
                for marker in &markers {
                    let lat_r = marker.lat.to_radians();
                    let lon_r = marker.lon.to_radians();
                    
                    let cos_c = lat_0.sin() * lat_r.sin() + lat_0.cos() * lat_r.cos() * (lon_r - lon_0).cos();
                    if cos_c >= 0.0 {
                        // Forward orthographic projection
                        let x = lat_r.cos() * (lon_r - lon_0).sin();
                        let y = lat_0.cos() * lat_r.sin() - lat_0.sin() * lat_r.cos() * (lon_r - lon_0).cos();
                        
                        let c = (cx + x * radius * aspect).round() as usize;
                        let r = (cy - y * radius).round() as usize;
                        if r < render_rows && c < render_cols {
                            screen_chars[r][c] = '●';
                            screen_colors[r][c] = color_marker;
                        }
                    }
                }
            }
        }

        // Print render buffer
        for r in 0..render_rows {
            let mut line = String::new();
            for c in 0..render_cols {
                let col = screen_colors[r][c];
                let ch = screen_chars[r][c];
                if col != color_reset {
                    line.push_str(col);
                    line.push_str(&ch.to_string());
                    line.push_str(color_reset);
                } else {
                    line.push(ch);
                }
            }
            out.push_str(&format!("{}\n", line));
        }

        // Bottom Dashboard UI
        let center_str = format!("Center: {:.1} Lat, {:.1} Lon | Zoom: {:.1}x", center_lat, center_lon, zoom);
        let shortcuts_str = "[Arrows] Pan/Rotate  [Tab/M] Globe/Map  [D] Day/Night  [+/-] Zoom  [Q/Esc] Quit";
        let center_pad = ((cols as usize).saturating_sub(center_str.len()) / 2).max(0);
        let shortcuts_pad = ((cols as usize).saturating_sub(shortcuts_str.len()) / 2).max(0);

        out.push_str(&format!("{}{}{}{}\n", " ".repeat(center_pad), color_ui, center_str, color_reset));
        out.push_str(&format!("{}{}{}{}", " ".repeat(shortcuts_pad), color_ui, shortcuts_str, color_reset));

        print!("{}", out);
        let _ = io::stdout().flush();

        // 50ms refresh sleep
        std::thread::sleep(Duration::from_millis(50));
    }

    print!("\x1B[?25h\x1B[2J\x1B[H"); // show cursor and clear screen
    let _ = io::stdout().flush();
}
