use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use std::time::Duration;
use rusqlite::Connection;
use crate::tui_util::{self, Key};

pub struct DbViewConfig {
    pub file_path: String,
}

struct TableData {
    name: String,
    columns: Vec<String>,
    rows: Vec<Vec<String>>,
}

pub fn run_dbview(config: DbViewConfig) -> io::Result<()> {
    let path = Path::new(&config.file_path);
    if !path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("File not found: {}", config.file_path),
        ));
    }

    // 1. Load data
    let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("").to_lowercase();
    let is_sqlite = ext == "db" || ext == "sqlite" || ext == "sqlite3";

    let mut tables: Vec<TableData> = Vec::new();
    if is_sqlite {
        match load_sqlite_tables(path) {
            Ok(t) => tables = t,
            Err(e) => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!("Failed to load SQLite database: {}", e),
                ));
            }
        }
    } else {
        // Fallback or treat as CSV/TSV
        let delimiter = if ext == "tsv" { '\t' } else { ',' };
        match load_csv_table(path, delimiter) {
            Ok(t) => tables.push(t),
            Err(e) => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!("Failed to parse CSV/TSV: {}", e),
                ));
            }
        }
    }

    if tables.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "No tables or data loaded.",
        ));
    }

    // 2. Setup TUI Raw Mode
    let _raw_mode = tui_util::set_raw_mode();

    // Enter alternate screen buffer and hide cursor
    print!("\x1B[?1049h\x1B[?25l\x1B[2J\x1B[H");
    let _ = io::stdout().flush();

    let result = dbview_loop(&mut tables, is_sqlite);

    // Exit alternate screen buffer and show cursor
    print!("\x1B[?1049l\x1B[?25h\x1B[0m");
    let _ = io::stdout().flush();

    result
}

fn load_csv_table(path: &Path, delimiter: char) -> io::Result<TableData> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let mut columns = Vec::new();
    if let Some(Ok(header)) = lines.next() {
        columns = parse_csv_line(&header, delimiter);
    }

    if columns.is_empty() {
        columns.push("Column1".to_string());
    }

    let mut rows = Vec::new();
    for line in lines {
        if let Ok(l) = line {
            if !l.trim().is_empty() {
                rows.push(parse_csv_line(&l, delimiter));
            }
        }
    }

    Ok(TableData {
        name: path.file_name().and_then(|s| s.to_str()).unwrap_or("Table").to_string(),
        columns,
        rows,
    })
}

fn parse_csv_line(line: &str, delimiter: char) -> Vec<String> {
    let mut fields = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut chars = line.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '"' {
            in_quotes = !in_quotes;
        } else if c == delimiter && !in_quotes {
            fields.push(current.trim().to_string());
            current = String::new();
        } else {
            current.push(c);
        }
    }
    fields.push(current.trim().to_string());
    fields
}

fn load_sqlite_tables(path: &Path) -> Result<Vec<TableData>, rusqlite::Error> {
    let conn = Connection::open(path)?;
    let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='table' OR type='view' ORDER BY name")?;
    let table_names: Vec<String> = stmt.query_map([], |row| row.get(0))?.collect::<Result<_, _>>()?;

    let mut tables = Vec::new();
    for name in table_names {
        if name.starts_with("sqlite_") {
            continue;
        }
        // Get column names
        let mut col_stmt = conn.prepare(&format!("PRAGMA table_info(\"{}\")", name))?;
        let columns: Vec<String> = col_stmt.query_map([], |row| row.get(1))?.collect::<Result<_, _>>()?;

        // Get row data (limit to 5000 for TUI performance)
        let mut row_stmt = conn.prepare(&format!("SELECT * FROM \"{}\" LIMIT 5000", name))?;
        let col_count = columns.len();
        let mut rows = Vec::new();
        let mut query_rows = row_stmt.query([])?;
        while let Some(row) = query_rows.next()? {
            let mut fields = Vec::new();
            for i in 0..col_count {
                let val: rusqlite::types::Value = row.get(i)?;
                let val_str = match val {
                    rusqlite::types::Value::Null => "NULL".to_string(),
                    rusqlite::types::Value::Integer(i) => i.to_string(),
                    rusqlite::types::Value::Real(r) => r.to_string(),
                    rusqlite::types::Value::Text(t) => t,
                    rusqlite::types::Value::Blob(b) => format!("[BLOB: {} B]", b.len()),
                };
                fields.push(val_str);
            }
            rows.push(fields);
        }

        tables.push(TableData { name, columns, rows });
    }
    Ok(tables)
}

fn dbview_loop(tables: &mut [TableData], is_sqlite: bool) -> io::Result<()> {
    let mut active_table_idx = 0;
    let mut row_select = 0;
    let mut col_select = 0;
    let mut h_scroll = 0;
    let mut sorting_col: Option<usize> = None;
    let mut sorting_desc = false;

    // Search filter state
    let mut search_active = false;
    let mut search_query = String::new();

    // Row detail popup state
    let mut detail_popup = false;

    let (mut cols, mut rows) = tui_util::terminal_size();

    loop {
        let (new_cols, new_rows) = tui_util::terminal_size();
        if new_cols != cols || new_rows != rows {
            cols = new_cols;
            rows = new_rows;
            print!("\x1B[2J\x1B[H");
            let _ = io::stdout().flush();
        }

        let table = &tables[active_table_idx];

        // Apply search filter
        let filtered_row_indices: Vec<usize> = if search_query.is_empty() {
            (0..table.rows.len()).collect()
        } else {
            let query = search_query.to_lowercase();
            table.rows.iter().enumerate()
                .filter(|(_, r)| {
                    r.iter().any(|f| f.to_lowercase().contains(&query))
                })
                .map(|(i, _)| i)
                .collect()
        };

        // Render Title Bar
        let mut frame = String::new();
        frame.push_str("\x1B[H");
        frame.push_str("\x1B[1;30;42m");
        let header_title = format!(" ir dbview TUI  |  File: {}  ", table.name);
        frame.push_str(&header_title);
        frame.push_str("\x1B[0m");
        if is_sqlite {
            frame.push_str("  \x1B[1;36mTables:\x1B[0m ");
            for (idx, t) in tables.iter().enumerate() {
                if idx == active_table_idx {
                    frame.push_str(&format!("\x1B[1;32m[{}]\x1B[0m ", t.name));
                } else {
                    frame.push_str(&format!("{} ", t.name));
                }
            }
        }
        frame.push_str("\x1B[K\n");
        frame.push_str(&"━".repeat(cols as usize));
        frame.push_str("\x1B[K\n");

        let body_height = rows.saturating_sub(6) as usize; // Title(2) + Column Header(2) + Footer(2)

        // Ensure row selection is valid
        if filtered_row_indices.is_empty() {
            row_select = 0;
        } else if row_select >= filtered_row_indices.len() {
            row_select = filtered_row_indices.len() - 1;
        }

        // Sort data if sorting column is set
        let mut display_row_indices = filtered_row_indices.clone();
        if let Some(col_idx) = sorting_col {
            display_row_indices.sort_by(|&a, &b| {
                let val_a = &table.rows[a].get(col_idx).cloned().unwrap_or_default();
                let val_b = &table.rows[b].get(col_idx).cloned().unwrap_or_default();
                // Try numeric sort
                let num_a = val_a.parse::<f64>();
                let num_b = val_b.parse::<f64>();
                let ord = match (num_a, num_b) {
                    (Ok(na), Ok(nb)) => na.partial_cmp(&nb).unwrap_or(std::cmp::Ordering::Equal),
                    _ => val_a.cmp(val_b),
                };
                if sorting_desc {
                    ord.reverse()
                } else {
                    ord
                }
            });
        }

        // Column Auto-fit widths calculation
        let mut col_widths = vec![8; table.columns.len()];
        for (i, col) in table.columns.iter().enumerate() {
            col_widths[i] = col_widths[i].max(col.len()).max(4);
            // Scan sample of rows to fit widths
            for &r_idx in display_row_indices.iter().take(100) {
                if let Some(val) = table.rows[r_idx].get(i) {
                    col_widths[i] = col_widths[i].max(val.chars().count());
                }
            }
            col_widths[i] = col_widths[i].min(40); // Cap width to 40 columns
        }

        // Horizontal scrolling view bounds
        let view_width = cols.saturating_sub(4) as usize;
        let mut visible_cols = Vec::new();
        let mut total_w = 0;
        for i in h_scroll..table.columns.len() {
            if total_w + col_widths[i] + 3 <= view_width {
                visible_cols.push(i);
                total_w += col_widths[i] + 3;
            } else {
                break;
            }
        }

        // Ensure selected column is visible
        if col_select < h_scroll {
            h_scroll = col_select;
        } else if !visible_cols.contains(&col_select) && col_select < table.columns.len() {
            h_scroll = col_select;
        }

        // Render Column Headers
        frame.push_str("  ");
        for &col_idx in &visible_cols {
            let col_name = &table.columns[col_idx];
            let cell_width = col_widths[col_idx];
            let mut label = if col_name.chars().count() > cell_width {
                col_name.chars().take(cell_width - 3).collect::<String>() + "..."
            } else {
                col_name.clone()
            };
            if Some(col_idx) == sorting_col {
                label.push(if sorting_desc { '↓' } else { '↑' });
            }
            if col_idx == col_select {
                frame.push_str(&format!(" \x1B[1;36;40m{:<width$}\x1B[0m │", label, width = cell_width));
            } else {
                frame.push_str(&format!(" \x1B[1;37m{:<width$}\x1B[0m │", label, width = cell_width));
            }
        }
        frame.push_str("\x1B[K\n");
        frame.push_str("  ");
        for &col_idx in &visible_cols {
            frame.push_str(&"━".repeat(col_widths[col_idx] + 2));
            frame.push_str("╪");
        }
        frame.push_str("\x1B[K\n");

        // Vertical scroll boundaries
        let start_row = if row_select >= body_height {
            row_select - body_height + 1
        } else {
            0
        };

        // Render Table Body
        for y in 0..body_height {
            let row_idx_in_view = start_row + y;
            if row_idx_in_view < display_row_indices.len() {
                let actual_row_idx = display_row_indices[row_idx_in_view];
                let is_selected_row = row_idx_in_view == row_select;
                let select_marker = if is_selected_row { "\x1B[1;32m>\x1B[0m " } else { "  " };
                frame.push_str(select_marker);

                for &col_idx in &visible_cols {
                    let cell_width = col_widths[col_idx];
                    let cell_val = table.rows[actual_row_idx].get(col_idx).cloned().unwrap_or_default();
                    let truncated_val = if cell_val.chars().count() > cell_width {
                        cell_val.chars().take(cell_width - 3).collect::<String>() + "..."
                    } else {
                        cell_val
                    };

                    if is_selected_row && col_idx == col_select {
                        frame.push_str(&format!(" \x1B[7m{:<width$}\x1B[0m │", truncated_val, width = cell_width));
                    } else if is_selected_row {
                        frame.push_str(&format!(" \x1B[1;32m{:<width$}\x1B[0m │", truncated_val, width = cell_width));
                    } else {
                        frame.push_str(&format!(" {:<width$} │", truncated_val, width = cell_width));
                    }
                }
                frame.push_str("\x1B[K\n");
            } else {
                frame.push_str("\x1B[K\n");
            }
        }

        // Render Row Detail Sidebar Popup
        if detail_popup && !display_row_indices.is_empty() {
            let actual_row_idx = display_row_indices[row_select];
            let popup_w = 40.min(cols as usize - 5);
            // Move cursor to draw popup
            for y in 0..body_height {
                let _row_idx_in_view = start_row + y;
                let cursor_y = 5 + y;
                let field_idx = y;
                let text = if field_idx < table.columns.len() {
                    let col_name = &table.columns[field_idx];
                    let cell_val = table.rows[actual_row_idx].get(field_idx).cloned().unwrap_or_default();
                    let full_field = format!("{}: {}", col_name, cell_val);
                    if full_field.chars().count() > popup_w - 4 {
                        full_field.chars().take(popup_w - 7).collect::<String>() + "..."
                    } else {
                        full_field
                    }
                } else {
                    "".to_string()
                };
                let border_char = if y == 0 { "┌" } else if y == body_height - 1 { "└" } else { "│" };
                let pad_w = popup_w.saturating_sub(text.chars().count() + 4);
                frame.push_str(&format!(
                    "\x1B[{};{}H\x1B[1;33;40m{}  {}{}\x1B[0m",
                    cursor_y, cols as usize - popup_w, border_char, text, " ".repeat(pad_w)
                ));
            }
        }

        // Render Footer Status Bar
        frame.push_str(&"━".repeat(cols as usize));
        frame.push_str("\x1B[K\n");
        if search_active {
            frame.push_str(&format!(
                " \x1B[1;33mSearch Filter:\x1B[0m {}_ \x1B[90m(Esc to exit search)\x1B[0m\x1B[K",
                search_query
            ));
        } else {
            let total_rows = display_row_indices.len();
            let status_msg = format!(
                " Rows: {}/{}  |  Sort: col {}  |  Esc/Q: Quit  |  Enter: Details  |  s: Sort  |  /: Search",
                if total_rows > 0 { row_select + 1 } else { 0 },
                total_rows,
                sorting_col.map(|c| c.to_string()).unwrap_or_else(|| "none".to_string())
            );
            frame.push_str(&format!(" \x1B[1;30;47m{}\x1B[0m\x1B[K", status_msg));
        }

        print!("{}", frame);
        let _ = io::stdout().flush();

        // Handle keys
        let key = tui_util::poll_key();
        if search_active {
            match key {
                Key::Esc | Key::Enter => {
                    search_active = false;
                }
                Key::Backspace => {
                    search_query.pop();
                }
                Key::Char(c) => {
                    search_query.push(c);
                }
                _ => {}
            }
            continue;
        }

        match key {
            Key::Esc | Key::Char('q') | Key::Char('Q') => {
                break;
            }
            Key::Up => {
                row_select = row_select.saturating_sub(1);
            }
            Key::Down => {
                if !display_row_indices.is_empty() && row_select + 1 < display_row_indices.len() {
                    row_select += 1;
                }
            }
            Key::Left => {
                if detail_popup {
                    detail_popup = false;
                }
                if col_select > 0 {
                    col_select -= 1;
                } else if is_sqlite && active_table_idx > 0 {
                    active_table_idx -= 1;
                    row_select = 0;
                    col_select = 0;
                    h_scroll = 0;
                    sorting_col = None;
                    print!("\x1B[2J\x1B[H");
                }
            }
            Key::Right => {
                if detail_popup {
                    detail_popup = false;
                }
                if col_select + 1 < table.columns.len() {
                    col_select += 1;
                } else if is_sqlite && active_table_idx + 1 < tables.len() {
                    active_table_idx += 1;
                    row_select = 0;
                    col_select = 0;
                    h_scroll = 0;
                    sorting_col = None;
                    print!("\x1B[2J\x1B[H");
                }
            }
            Key::PageUp => {
                row_select = row_select.saturating_sub(body_height);
            }
            Key::PageDown => {
                if !display_row_indices.is_empty() {
                    row_select = (row_select + body_height).min(display_row_indices.len() - 1);
                }
            }
            Key::Enter => {
                detail_popup = !detail_popup;
                print!("\x1B[2J\x1B[H");
            }
            Key::Char('s') | Key::Char('S') => {
                if sorting_col == Some(col_select) {
                    if sorting_desc {
                        sorting_col = None; // Reset sorting
                    } else {
                        sorting_desc = true;
                    }
                } else {
                    sorting_col = Some(col_select);
                    sorting_desc = false;
                }
            }
            Key::Char('/') => {
                search_active = true;
                detail_popup = false;
            }
            _ => {}
        }

        std::thread::sleep(Duration::from_millis(30));
    }

    Ok(())
}
