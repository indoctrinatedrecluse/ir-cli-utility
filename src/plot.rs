use crate::PlotOptions;
use std::fs::File;
use std::io::{self, IsTerminal, Read};

// Helper function to query a nested JSON key (reused from json.rs traversal logic)
fn query_json(value: &serde_json::Value, query: &str) -> Result<serde_json::Value, String> {
    let mut q = query.trim();
    if q.is_empty() || q == "." {
        return Ok(value.clone());
    }
    if q.starts_with('.') {
        q = &q[1..];
    }
    let mut current = value;
    let mut parts = Vec::new();
    let mut temp = String::new();
    let chars: Vec<char> = q.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        match chars[i] {
            '.' => {
                if !temp.is_empty() {
                    parts.push(temp.clone());
                    temp.clear();
                }
                i += 1;
            }
            '[' => {
                if !temp.is_empty() {
                    parts.push(temp.clone());
                    temp.clear();
                }
                i += 1;
                let mut idx_str = String::new();
                while i < chars.len() && chars[i] != ']' {
                    idx_str.push(chars[i]);
                    i += 1;
                }
                if i < chars.len() && chars[i] == ']' {
                    parts.push(format!("[{}]", idx_str.trim()));
                } else {
                    return Err("Unclosed index bracket '['".to_string());
                }
                i += 1;
            }
            c => {
                temp.push(c);
                i += 1;
            }
        }
    }
    if !temp.is_empty() {
        parts.push(temp);
    }
    for part in parts {
        if part.starts_with('[') && part.ends_with(']') {
            let idx_str = &part[1..part.len() - 1];
            if let Ok(idx) = idx_str.parse::<usize>() {
                if let Some(arr) = current.as_array() {
                    if idx < arr.len() {
                        current = &arr[idx];
                    } else {
                        return Err(format!("Index {} out of bounds (array length {})", idx, arr.len()));
                    }
                } else {
                    return Err(format!("Cannot index non-array value with [{}]", idx));
                }
            } else {
                let key = if (idx_str.starts_with('"') && idx_str.ends_with('"'))
                    || (idx_str.starts_with('\'') && idx_str.ends_with('\''))
                {
                    &idx_str[1..idx_str.len() - 1]
                } else {
                    idx_str
                };
                if let Some(obj) = current.as_object() {
                    if let Some(val) = obj.get(key) {
                        current = val;
                    } else {
                        return Err(format!("Key '{}' not found in object", key));
                    }
                } else {
                    return Err(format!("Cannot look up key '{}' in non-object value", key));
                }
            }
        } else {
            if let Some(obj) = current.as_object() {
                if let Some(val) = obj.get(&part) {
                    current = val;
                } else {
                    return Err(format!("Key '{}' not found in object", part));
                }
            } else {
                return Err(format!("Cannot look up key '{}' in non-object value", part));
            }
        }
    }
    Ok(current.clone())
}

// Bresenham's line algorithm on boolean grid
fn draw_line(x0: isize, y0: isize, x1: isize, y1: isize, grid: &mut Vec<Vec<bool>>) {
    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;
    let mut x = x0;
    let mut y = y0;
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;
    loop {
        if x >= 0 && x < cols && y >= 0 && y < rows {
            grid[y as usize][x as usize] = true;
        }
        if x == x1 && y == y1 { break; }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x += sx;
        }
        if e2 <= dx {
            err += dx;
            y += sy;
        }
    }
}

// ─── Terminal size ──────────────────────────────────────────────────────────────
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
    #[cfg(target_os = "linux")]
    unsafe {
        let mut ws: libc::winsize = std::mem::zeroed();
        if libc::ioctl(1, libc::TIOCGWINSZ, &mut ws) == 0 && ws.ws_col > 0 {
            return (ws.ws_col, ws.ws_row);
        }
        (80, 24)
    }
    #[cfg(not(any(target_os = "windows", target_os = "linux")))]
    {
        (80, 24)
    }
}

pub fn run_plot(input_path: Option<&str>, options: PlotOptions) {
    let input_bytes = read_input(input_path);
    let mut raw_data = parse_data(&input_bytes, &options);

    if raw_data.is_empty() {
        eprintln!("Error: No numerical data points found to plot.");
        std::process::exit(1);
    }

    // Determine dimensions
    let (term_cols, term_rows) = terminal_size();
    let cols = options.width.unwrap_or_else(|| {
        (term_cols as usize).saturating_sub(15).max(20).min(100)
    });
    let rows = options.height.unwrap_or_else(|| {
        ((term_rows as usize) / 3).max(5).min(25)
    });

    if cols == 0 || rows == 0 {
        eprintln!("Error: Width and height must be greater than zero.");
        std::process::exit(1);
    }

    // Apply Logarithmic scale if requested
    let mut orig_min = raw_data[0];
    let mut orig_max = raw_data[0];
    for &val in &raw_data {
        if val < orig_min { orig_min = val; }
        if val > orig_max { orig_max = val; }
    }

    if options.log_scale {
        for &val in &raw_data {
            if val <= 0.0 {
                eprintln!("Error: Logarithmic scale requires all data points to be positive (greater than zero).");
                std::process::exit(1);
            }
        }
        raw_data = raw_data.iter().map(|&x| x.log10()).collect();
    }

    // Setup terminal color switches
    let use_color = io::stdout().is_terminal();
    let color_reset = if use_color { "\x1B[0m" } else { "" };
    let color_title = if use_color { "\x1B[1;33m" } else { "" }; // Bold Yellow
    let color_axis = if use_color { "\x1B[37m" } else { "" };  // Light Grey
    let color_label = if use_color { "\x1B[35m" } else { "" }; // Magenta
    let color_plot = if use_color {
        match options.chart_type.as_str() {
            "bar" => "\x1B[32m",    // Green
            "scatter" => "\x1B[33m",// Yellow
            _ => "\x1B[36m",        // Cyan
        }
    } else {
        ""
    };

    // Print Title if provided
    if let Some(ref title) = options.title {
        println!("{}{}{}", color_title, title, color_reset);
    }

    // --- HORIZONTAL BAR CHART ---
    if options.horizontal {
        // Downsample/aggregate data to match row count
        let y_data = if raw_data.len() <= rows {
            raw_data.clone()
        } else {
            let mut downsampled = Vec::new();
            for i in 0..rows {
                let start = (i * raw_data.len()) / rows;
                let end = ((i + 1) * raw_data.len()) / rows;
                let chunk = &raw_data[start..end];
                if !chunk.is_empty() {
                    let sum: f64 = chunk.iter().sum();
                    downsampled.push(sum / chunk.len() as f64);
                }
            }
            downsampled
        };

        let mut min_val = y_data[0];
        let mut max_val = y_data[0];
        for &val in &y_data {
            if val < min_val { min_val = val; }
            if val > max_val { max_val = val; }
        }
        let range = if (max_val - min_val).abs() < 1e-9 { 1.0 } else { max_val - min_val };

        for r in 0..y_data.len() {
            let val = y_data[r];
            let label_val = if options.log_scale { 10.0f64.powf(val) } else { val };
            print!("{}{:>10.2} ┼{}", color_label, label_val, color_axis);

            let scaled = (val - min_val) / range * (cols as f64);
            let full_blocks = scaled.floor() as usize;
            let fraction = scaled - (full_blocks as f64);

            print!("{}", color_plot);
            for _ in 0..full_blocks {
                print!("█");
            }
            if full_blocks < cols {
                let sub_char = match (fraction * 8.0).round() as usize {
                    0 => ' ',
                    1 => '▏',
                    2 => '▎',
                    3 => '▍',
                    4 => '▌',
                    5 => '▋',
                    6 => '▊',
                    7 => '▉',
                    _ => '█',
                };
                if sub_char != ' ' {
                    print!("{}", sub_char);
                }
            }
            println!("{}", color_reset);
        }

        // Print bottom X axis line
        print!("           +");
        for _ in 0..cols { print!("─"); }
        println!();
        let display_min = if options.log_scale { 10.0f64.powf(min_val) } else { min_val };
        let display_max = if options.log_scale { 10.0f64.powf(max_val) } else { max_val };
        println!("           {:<.2}{:>width$.2}", display_min, display_max, width = cols.saturating_sub(10));
        return;
    }

    // --- VERTICAL CHARTS (LINE, BAR, SCATTER) ---
    // Downsample/aggregate data to match width columns
    let x_data = if raw_data.len() <= cols {
        raw_data.clone()
    } else {
        let mut downsampled = Vec::new();
        for i in 0..cols {
            let start = (i * raw_data.len()) / cols;
            let end = ((i + 1) * raw_data.len()) / cols;
            let chunk = &raw_data[start..end];
            if !chunk.is_empty() {
                let sum: f64 = chunk.iter().sum();
                downsampled.push(sum / chunk.len() as f64);
            }
        }
        downsampled
    };

    let mut min_val = x_data[0];
    let mut max_val = x_data[0];
    for &val in &x_data {
        if val < min_val { min_val = val; }
        if val > max_val { max_val = val; }
    }
    let range = if (max_val - min_val).abs() < 1e-9 { 1.0 } else { max_val - min_val };

    let display_min = if options.log_scale { 10.0f64.powf(min_val) } else { min_val };
    let display_max = if options.log_scale { 10.0f64.powf(max_val) } else { max_val };

    // --- SMOOTH UNICODE BRAILLE PLOTTING ---
    if options.smooth && options.chart_type != "bar" {
        let hires_cols = cols * 2;
        let hires_rows = rows * 4;

        // Downsample to hires columns
        let x_data_hires = if raw_data.len() <= hires_cols {
            raw_data.clone()
        } else {
            let mut downsampled = Vec::new();
            for i in 0..hires_cols {
                let start = (i * raw_data.len()) / hires_cols;
                let end = ((i + 1) * raw_data.len()) / hires_cols;
                let chunk = &raw_data[start..end];
                if !chunk.is_empty() {
                    let sum: f64 = chunk.iter().sum();
                    downsampled.push(sum / chunk.len() as f64);
                }
            }
            downsampled
        };

        let mut hires_grid = vec![vec![false; hires_cols]; hires_rows];
        let mut y_coords_hires = Vec::new();
        for c in 0..x_data_hires.len() {
            let val = x_data_hires[c];
            let scaled = (val - min_val) / range * ((hires_rows - 1) as f64);
            y_coords_hires.push((scaled.round() as usize).min(hires_rows - 1));
        }

        // Populate high-resolution grid
        if options.chart_type == "scatter" {
            for c in 0..x_data_hires.len() {
                hires_grid[y_coords_hires[c]][c] = true;
            }
        } else {
            for c in 0..x_data_hires.len() {
                if c > 0 {
                    draw_line(
                        (c - 1) as isize,
                        y_coords_hires[c - 1] as isize,
                        c as isize,
                        y_coords_hires[c] as isize,
                        &mut hires_grid,
                    );
                } else {
                    hires_grid[y_coords_hires[0]][0] = true;
                }
            }
        }

        // Render Braille cells
        for r in (0..rows).rev() {
            // Print left Y-axis labels/ticks
            if r == rows - 1 {
                print!("{}{:>10.2} ┤{}", color_label, display_max, color_axis);
            } else if r == 0 {
                print!("{}{:>10.2} ┴{}", color_label, display_min, color_axis);
            } else {
                print!("           │");
            }

            for c in 0..cols {
                let mut offset = 0;
                let h_x = c * 2;
                let h_y = r * 4;

                // Braille dot mapping layout
                if h_y + 3 < hires_rows && h_x < hires_cols     && hires_grid[h_y + 3][h_x]     { offset |= 0x01; }
                if h_y + 2 < hires_rows && h_x < hires_cols     && hires_grid[h_y + 2][h_x]     { offset |= 0x02; }
                if h_y + 1 < hires_rows && h_x < hires_cols     && hires_grid[h_y + 1][h_x]     { offset |= 0x04; }
                if h_y + 3 < hires_rows && h_x + 1 < hires_cols && hires_grid[h_y + 3][h_x + 1] { offset |= 0x08; }
                if h_y + 2 < hires_rows && h_x + 1 < hires_cols && hires_grid[h_y + 2][h_x + 1] { offset |= 0x10; }
                if h_y + 1 < hires_rows && h_x + 1 < hires_cols && hires_grid[h_y + 1][h_x + 1] { offset |= 0x20; }
                if h_y < hires_rows     && h_x < hires_cols     && hires_grid[h_y][h_x]         { offset |= 0x40; }
                if h_y < hires_rows     && h_x + 1 < hires_cols && hires_grid[h_y][h_x + 1]     { offset |= 0x80; }

                if offset > 0 {
                    let braille_char = std::char::from_u32(0x2800 + offset).unwrap_or(' ');
                    print!("{}{}{}", color_plot, braille_char, color_reset);
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    } else {
        // --- STANDARD ASCII RENDERER ---
        match options.chart_type.as_str() {
            "bar" => {
                for r in (0..rows).rev() {
                    if r == rows - 1 {
                        print!("{}{:>10.2} ┤{}", color_label, display_max, color_axis);
                    } else if r == 0 {
                        print!("{}{:>10.2} ┴{}", color_label, display_min, color_axis);
                    } else {
                        print!("           │");
                    }

                    for c in 0..x_data.len() {
                        let val = x_data[c];
                        let scaled = (val - min_val) / range * (rows as f64);
                        if scaled >= (r + 1) as f64 {
                            print!("{}█{}", color_plot, color_reset);
                        } else if scaled <= r as f64 {
                            print!(" ");
                        } else {
                            let fraction = scaled - (r as f64);
                            let block_idx = (fraction * 8.0).round() as usize;
                            let block_char = match block_idx {
                                0 => ' ',
                                1 => ' ',
                                2 => '▂',
                                3 => '▃',
                                4 => '▄',
                                5 => '▅',
                                6 => '▆',
                                7 => '▇',
                                _ => '█',
                            };
                            print!("{}{}{}", color_plot, block_char, color_reset);
                        }
                    }
                    println!();
                }
            }
            "scatter" | "line" | _ => {
                let mut grid = vec![vec![' '; x_data.len()]; rows];
                let mut y_coords = Vec::new();

                for c in 0..x_data.len() {
                    let val = x_data[c];
                    let scaled = (val - min_val) / range * ((rows - 1) as f64);
                    let r = scaled.round() as usize;
                    y_coords.push(r.min(rows - 1));
                }

                if options.chart_type == "scatter" {
                    for c in 0..x_data.len() {
                        let r = y_coords[c];
                        grid[r][c] = '*';
                    }
                } else {
                    for c in 0..x_data.len() {
                        let r_curr = y_coords[c];
                        grid[r_curr][c] = '*';
                        if c > 0 {
                            let r_prev = y_coords[c - 1];
                            if r_curr > r_prev {
                                for r_between in (r_prev + 1)..r_curr {
                                    grid[r_between][c] = '/';
                                }
                            } else if r_curr < r_prev {
                                for r_between in (r_curr + 1)..r_prev {
                                    grid[r_between][c] = '\\';
                                }
                            }
                        }
                    }
                }

                for r in (0..rows).rev() {
                    if r == rows - 1 {
                        print!("{}{:>10.2} ┤{}", color_label, display_max, color_axis);
                    } else if r == 0 {
                        print!("{}{:>10.2} ┴{}", color_label, display_min, color_axis);
                    } else {
                        print!("           │");
                    }

                    for c in 0..x_data.len() {
                        let ch = grid[r][c];
                        if ch != ' ' {
                            print!("{}{}{}", color_plot, ch, color_reset);
                        } else {
                            print!(" ");
                        }
                    }
                    println!();
                }
            }
        }
    }

    // Print bottom X-axis line
    print!("           ");
    for _ in 0..x_data.len() { print!("─"); }
    println!("{}", color_reset);

    // Print X-axis indices at start and end
    println!("           0{:>width$}", raw_data.len() - 1, width = x_data.len() - 1);
}

fn read_input(input_path: Option<&str>) -> Vec<u8> {
    if let Some(path) = input_path {
        let mut file = match File::open(path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Error: Failed to open input file '{}': {}", path, e);
                std::process::exit(1);
            }
        };
        let mut buf = Vec::new();
        if let Err(e) = file.read_to_end(&mut buf) {
            eprintln!("Error: Failed to read input file: {}", e);
            std::process::exit(1);
        }
        buf
    } else {
        let mut buf = Vec::new();
        if let Err(e) = io::stdin().read_to_end(&mut buf) {
            eprintln!("Error: Failed to read from stdin: {}", e);
            std::process::exit(1);
        }
        buf
    }
}

fn parse_data(bytes: &[u8], options: &PlotOptions) -> Vec<f64> {
    let mut data = Vec::new();
    let bytes_to_parse = if bytes.starts_with(&[0xEF, 0xBB, 0xBF]) {
        &bytes[3..]
    } else {
        bytes
    };
    let mut text = String::from_utf8_lossy(bytes_to_parse).into_owned();
    if text.starts_with('\u{FEFF}') {
        text.remove(0);
    }

    match options.source_format.as_str() {
        "csv" => {
            let col_idx = options.csv_col.unwrap_or(0);
            let mut lines = text.lines();
            if options.csv_headers {
                lines.next();
            }
            for line in lines {
                if line.trim().is_empty() { continue; }
                let parts: Vec<&str> = line.split(',').collect();
                if col_idx < parts.len() {
                    let val_str = parts[col_idx].trim().trim_matches('"').trim_matches('\'').trim();
                    if let Ok(num) = val_str.parse::<f64>() {
                        data.push(num);
                    } else {
                        eprintln!("Error: Failed to parse CSV column value '{}' as a number.", val_str);
                        std::process::exit(1);
                    }
                } else {
                    eprintln!("Error: CSV parsing failed: column index {} is out of bounds (row only has {} columns).", col_idx, parts.len());
                    std::process::exit(1);
                }
            }
        }
        "json" => {
            let v: serde_json::Value = match serde_json::from_slice(bytes_to_parse) {
                Ok(json_val) => json_val,
                Err(e) => {
                    eprintln!("Error: Failed to parse JSON source data: {}", e);
                    std::process::exit(1);
                }
            };
            if let Some(arr) = v.as_array() {
                for (item_idx, item) in arr.iter().enumerate() {
                    let target_val = if let Some(ref key) = options.json_key {
                        match query_json(item, key) {
                            Ok(nested) => nested,
                            Err(e) => {
                                eprintln!("Error: JSON key extraction failed at index {}: {}", item_idx, e);
                                std::process::exit(1);
                            }
                        }
                    } else {
                        item.clone()
                    };

                    if let Some(num) = target_val.as_f64() {
                        data.push(num);
                    } else if let Some(num_str) = target_val.as_str() {
                        if let Ok(num) = num_str.parse::<f64>() {
                            data.push(num);
                        } else {
                            eprintln!("Error: JSON element value '{}' could not be parsed as number.", num_str);
                            std::process::exit(1);
                        }
                    } else {
                        eprintln!("Error: JSON element is not a numerical value (and no valid string representation found).");
                        std::process::exit(1);
                    }
                }
            } else {
                eprintln!("Error: JSON source format expects a flat array of numbers (or objects queried via --json-key).");
                std::process::exit(1);
            }
        }
        "txt" | _ => {
            let sanitized = text.replace(',', " ");
            for token in sanitized.split_whitespace() {
                if let Ok(num) = token.parse::<f64>() {
                    data.push(num);
                } else {
                    eprintln!("Error: Failed to parse text token '{}' as a number.", token);
                    std::process::exit(1);
                }
            }
        }
    }
    data
}
