use crate::PlotOptions;
use std::fs::File;
use std::io::{self, IsTerminal, Read};

pub fn run_plot(input_path: Option<&str>, options: PlotOptions) {
    let input_bytes = read_input(input_path);
    let raw_data = parse_data(&input_bytes, &options);

    if raw_data.is_empty() {
        eprintln!("Error: No numerical data points found to plot.");
        std::process::exit(1);
    }

    // Determine dimensions
    let cols = options.width.unwrap_or(60);
    let rows = options.height.unwrap_or(15);

    if cols == 0 || rows == 0 {
        eprintln!("Error: Width and height must be greater than zero.");
        std::process::exit(1);
    }

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

    // Find min and max values
    let mut min_val = x_data[0];
    let mut max_val = x_data[0];
    for &val in &x_data {
        if val < min_val {
            min_val = val;
        }
        if val > max_val {
            max_val = val;
        }
    }

    // Handle flat lines
    let range = if (max_val - min_val).abs() < 1e-9 {
        1.0
    } else {
        max_val - min_val
    };

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

    // Render logic
    match options.chart_type.as_str() {
        "bar" => {
            // Render vertical bar chart
            // For each row (from top row down to 0)
            for r in (0..rows).rev() {
                // Print left Y-axis labels/ticks
                if r == rows - 1 {
                    print!("{}{:>10.2} ┤{}", color_label, max_val, color_axis);
                } else if r == 0 {
                    print!("{}{:>10.2} ┴{}", color_label, min_val, color_axis);
                } else {
                    print!("           │");
                }

                // Print bars
                for c in 0..x_data.len() {
                    let val = x_data[c];
                    let scaled = (val - min_val) / range * (rows as f64);
                    // Determine block character
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
            // Scatter and Line plots use a 2D character grid
            let mut grid = vec![vec![' '; x_data.len()]; rows];
            let mut y_coords = Vec::new();

            // Calculate target row coordinate for each column point
            for c in 0..x_data.len() {
                let val = x_data[c];
                let scaled = (val - min_val) / range * ((rows - 1) as f64);
                let r = scaled.round() as usize;
                y_coords.push(r.min(rows - 1));
            }

            // Populate grid
            if options.chart_type == "scatter" {
                for c in 0..x_data.len() {
                    let r = y_coords[c];
                    grid[r][c] = '*';
                }
            } else {
                // Line plot - connect points
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

            // Render grid
            for r in (0..rows).rev() {
                // Print left Y-axis labels/ticks
                if r == rows - 1 {
                    print!("{}{:>10.2} ┤{}", color_label, max_val, color_axis);
                } else if r == 0 {
                    print!("{}{:>10.2} ┴{}", color_label, min_val, color_axis);
                } else {
                    print!("           │");
                }

                // Print grid row characters
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

    // Print bottom X-axis line
    print!("           ");
    for _ in 0..x_data.len() {
        print!("─");
    }
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
    let text = String::from_utf8_lossy(bytes);

    match options.source_format.as_str() {
        "csv" => {
            let col_idx = options.csv_col.unwrap_or(0);
            let mut lines = text.lines();
            if options.csv_headers {
                lines.next(); // Skip header row
            }
            for line in lines {
                if line.trim().is_empty() {
                    continue;
                }
                // Basic CSV splitter
                let parts: Vec<&str> = line.split(',').collect();
                if col_idx < parts.len() {
                    let val_str = parts[col_idx].trim().trim_matches('"').trim_matches('\'').trim();
                    if let Ok(num) = val_str.parse::<f64>() {
                        data.push(num);
                    }
                }
            }
        }
        "json" => {
            let v: serde_json::Value = match serde_json::from_slice(bytes) {
                Ok(json_val) => json_val,
                Err(e) => {
                    eprintln!("Error: Failed to parse JSON source data: {}", e);
                    std::process::exit(1);
                }
            };
            if let Some(arr) = v.as_array() {
                for item in arr {
                    if let Some(num) = item.as_f64() {
                        data.push(num);
                    } else if let Some(num_str) = item.as_str() {
                        if let Ok(num) = num_str.parse::<f64>() {
                            data.push(num);
                        }
                    }
                }
            } else {
                eprintln!("Error: JSON source format expects a flat array of numbers.");
                std::process::exit(1);
            }
        }
        "txt" | _ => {
            // Parse whitespace, comma, or newline separated floats
            // Replace commas with spaces to allow comma separation, then split by whitespace
            let sanitized = text.replace(',', " ");
            for token in sanitized.split_whitespace() {
                if let Ok(num) = token.parse::<f64>() {
                    data.push(num);
                }
            }
        }
    }

    data
}
