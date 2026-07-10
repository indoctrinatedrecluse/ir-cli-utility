use std::fs;
use std::io::{self, Write};
use std::path::Path;
use crate::tui_util::{self, Key};

pub struct HexViewConfig {
    pub file_path: String,
}

pub fn run_hexview(config: HexViewConfig) -> io::Result<()> {
    let path = Path::new(&config.file_path);
    if !path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("File not found: {}", config.file_path),
        ));
    }

    let mut bytes = fs::read(path)?;
    if bytes.is_empty() {
        bytes.push(0); // Ensure at least 1 byte to view/edit
    }

    // 1. Setup TUI Raw Mode
    let _raw_mode = tui_util::set_raw_mode();

    // Enter alternate screen buffer
    print!("\x1B[?1049h\x1B[?25l\x1B[2J\x1B[H");
    let _ = io::stdout().flush();

    let result = hexview_loop(&config.file_path, &mut bytes);

    // Exit alternate screen buffer
    print!("\x1B[?1049l\x1B[?25h\x1B[0m");
    let _ = io::stdout().flush();

    result
}

fn hexview_loop(file_path: &str, bytes: &mut Vec<u8>) -> io::Result<()> {
    let mut cursor_byte_idx = 0;
    let mut is_hex_pane = true;
    let mut is_edit_mode = false;
    let mut cursor_nibble = 0; // 0: high nibble, 1: low nibble (used in hex pane editing)

    let mut search_active = false;
    let mut search_query = String::new();
    let mut search_matches = Vec::new();

    let mut jump_active = false;
    let mut jump_query = String::new();

    let mut has_changes = false;

    let (mut cols, mut rows) = tui_util::terminal_size();

    loop {
        let (new_cols, new_rows) = tui_util::terminal_size();
        if new_cols != cols || new_rows != rows {
            cols = new_cols;
            rows = new_rows;
            print!("\x1B[2J\x1B[H");
            let _ = io::stdout().flush();
        }

        // Render Title Bar
        let mut frame = String::new();
        frame.push_str("\x1B[H");
        frame.push_str("\x1B[1;30;46m ir hexview TUI \x1B[0m  ");
        frame.push_str(&format!(
            "\x1B[1mFile:\x1B[0m {}  |  \x1B[1mSize:\x1B[0m {} bytes{}",
            file_path,
            bytes.len(),
            if has_changes { " \x1B[1;31m[MODIFIED]\x1B[0m" } else { "" }
        ));
        frame.push_str("\x1B[K\n");
        frame.push_str(&"━".repeat(cols as usize));
        frame.push_str("\x1B[K\n");

        let body_height = rows.saturating_sub(6) as usize; // Title(2) + Offset Headers(2) + Footer(2)

        // Math for offsets
        let total_rows = (bytes.len() + 15) / 16;
        let cursor_row = cursor_byte_idx / 16;
        let _cursor_col = cursor_byte_idx % 16;

        let start_row = if cursor_row >= body_height {
            cursor_row - body_height + 1
        } else {
            0
        };

        // Render Hex Header
        frame.push_str("  \x1B[1;37mOFFSET    00 01 02 03 04 05 06 07  08 09 0A 0B 0C 0D 0E 0F  | ASCII           \x1B[0m\x1B[K\n");
        frame.push_str(&format!("  {}\x1B[K\n", "━".repeat(74)));

        // Render Body
        for y in 0..body_height {
            let row_idx = start_row + y;
            if row_idx < total_rows {
                let offset = row_idx * 16;
                frame.push_str(&format!("  \x1B[1;33m{:08X}\x1B[0m  ", offset));

                // 1. Render Hex Pane
                for x in 0..16 {
                    let byte_idx = offset + x;
                    if byte_idx < bytes.len() {
                        let val = bytes[byte_idx];
                        let is_current = byte_idx == cursor_byte_idx;
                        let is_search_match = search_matches.contains(&byte_idx);

                        let color_esc = if is_current && is_hex_pane {
                            if is_edit_mode { "\x1B[1;30;43m" } else { "\x1B[1;30;42m" }
                        } else if is_current {
                            "\x1B[1;32m" // highlighted in inactive pane
                        } else if is_search_match {
                            "\x1B[1;31m" // red search match
                        } else if val == 0 {
                            "\x1B[90m" // grey out zeros
                        } else {
                            ""
                        };

                        frame.push_str(color_esc);
                        frame.push_str(&format!("{:02X}", val));
                        if !color_esc.is_empty() {
                            frame.push_str("\x1B[0m");
                        }
                    } else {
                        frame.push_str("  ");
                    }
                    if x == 7 {
                        frame.push_str("  ");
                    } else {
                        frame.push_str(" ");
                    }
                }

                frame.push_str(" | ");

                // 2. Render ASCII Pane
                for x in 0..16 {
                    let byte_idx = offset + x;
                    if byte_idx < bytes.len() {
                        let val = bytes[byte_idx];
                        let is_current = byte_idx == cursor_byte_idx;
                        let is_search_match = search_matches.contains(&byte_idx);

                        let c = if val >= 32 && val < 127 { val as char } else { '.' };

                        let color_esc = if is_current && !is_hex_pane {
                            if is_edit_mode { "\x1B[1;30;43m" } else { "\x1B[1;30;42m" }
                        } else if is_current {
                            "\x1B[1;32m"
                        } else if is_search_match {
                            "\x1B[1;31m"
                        } else if val == 0 {
                            "\x1B[90m"
                        } else {
                            ""
                        };

                        frame.push_str(color_esc);
                        frame.push(c);
                        if !color_esc.is_empty() {
                            frame.push_str("\x1B[0m");
                        }
                    } else {
                        frame.push(' ');
                    }
                }
                frame.push_str("\x1B[K\n");
            } else {
                frame.push_str("\x1B[K\n");
            }
        }

        // Render Footer Status Bar
        frame.push_str(&"━".repeat(cols as usize));
        frame.push_str("\x1B[K\n");

        if search_active {
            frame.push_str(&format!(" \x1B[1;33mSearch Bytes/Text:\x1B[0m {}_ \x1B[90m(Esc to exit search)\x1B[0m\x1B[K", search_query));
        } else if jump_active {
            frame.push_str(&format!(" \x1B[1;35mGo to Hex Address:\x1B[0m 0x{}_ \x1B[90m(Enter to jump)\x1B[0m\x1B[K", jump_query));
        } else {
            let status_msg = format!(
                " Mode: {}  |  Byte: 0x{:X} (Dec: {})  |  Ctrl+S: Save  |  Tab: Swap pane  |  e: Edit  |  g: Jump  |  /: Search",
                if is_edit_mode { "\x1B[1;33mEDIT\x1B[0m" } else { "\x1B[1;32mVIEW\x1B[0m" },
                cursor_byte_idx,
                cursor_byte_idx
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
                    update_search_matches(&search_query, bytes, &mut search_matches);
                }
                Key::Char(c) => {
                    search_query.push(c);
                    update_search_matches(&search_query, bytes, &mut search_matches);
                }
                _ => {}
            }
            continue;
        }

        if jump_active {
            match key {
                Key::Esc => {
                    jump_active = false;
                }
                Key::Enter => {
                    if let Ok(offset) = usize::from_str_radix(&jump_query, 16) {
                        if offset < bytes.len() {
                            cursor_byte_idx = offset;
                        }
                    }
                    jump_active = false;
                    jump_query = String::new();
                }
                Key::Backspace => {
                    jump_query.pop();
                }
                Key::Char(c) if c.is_ascii_hexdigit() => {
                    jump_query.push(c);
                }
                _ => {}
            }
            continue;
        }

        match key {
            Key::Esc | Key::Char('q') | Key::Char('Q') => {
                if is_edit_mode {
                    is_edit_mode = false;
                    cursor_nibble = 0;
                } else {
                    break;
                }
            }
            Key::Tab => {
                is_hex_pane = !is_hex_pane;
                cursor_nibble = 0;
            }
            Key::Char('e') | Key::Char('E') => {
                is_edit_mode = !is_edit_mode;
                cursor_nibble = 0;
            }
            Key::Char('g') | Key::Char('G') => {
                if !is_edit_mode {
                    jump_active = true;
                }
            }
            Key::Char('/') => {
                if !is_edit_mode {
                    search_active = true;
                    search_query = String::new();
                    search_matches.clear();
                }
            }
            Key::Char(c) if is_edit_mode => {
                // Type edits
                if is_hex_pane {
                    if c.is_ascii_hexdigit() {
                        let val = c.to_digit(16).unwrap() as u8;
                        let current_byte = bytes[cursor_byte_idx];
                        if cursor_nibble == 0 {
                            bytes[cursor_byte_idx] = (current_byte & 0x0F) | (val << 4);
                            cursor_nibble = 1;
                        } else {
                            bytes[cursor_byte_idx] = (current_byte & 0xF0) | val;
                            cursor_nibble = 0;
                            // Advance cursor
                            if cursor_byte_idx + 1 < bytes.len() {
                                cursor_byte_idx += 1;
                            }
                        }
                        has_changes = true;
                    }
                } else {
                    // ASCII Pane edits
                    bytes[cursor_byte_idx] = c as u8;
                    has_changes = true;
                    if cursor_byte_idx + 1 < bytes.len() {
                        cursor_byte_idx += 1;
                    }
                }
            }
            Key::Char('s') | Key::Char('S') => {
                if !is_edit_mode {
                    fs::write(file_path, bytes.as_slice())?;
                    has_changes = false;
                }
            }
            Key::Char('\x13') => {
                // Raw Ctrl+S
                fs::write(file_path, bytes.as_slice())?;
                has_changes = false;
            }
            Key::Up => {
                if cursor_byte_idx >= 16 {
                    cursor_byte_idx -= 16;
                }
            }
            Key::Down => {
                if cursor_byte_idx + 16 < bytes.len() {
                    cursor_byte_idx += 16;
                }
            }
            Key::Left => {
                if cursor_byte_idx > 0 {
                    cursor_byte_idx -= 1;
                }
            }
            Key::Right => {
                if cursor_byte_idx + 1 < bytes.len() {
                    cursor_byte_idx += 1;
                }
            }
            Key::PageUp => {
                cursor_byte_idx = cursor_byte_idx.saturating_sub(body_height * 16);
            }
            Key::PageDown => {
                cursor_byte_idx = (cursor_byte_idx + body_height * 16).min(bytes.len() - 1);
            }
            _ => {}
        }

        std::thread::sleep(std::time::Duration::from_millis(30));
    }

    Ok(())
}

fn update_search_matches(query: &str, bytes: &[u8], matches: &mut Vec<usize>) {
    matches.clear();
    if query.is_empty() {
        return;
    }

    // Try parsing query as hex string (e.g. "41 42" or "4142")
    let clean_hex = query.replace(' ', "");
    let hex_bytes = if clean_hex.len() % 2 == 0 && clean_hex.chars().all(|c| c.is_ascii_hexdigit()) {
        let mut temp = Vec::new();
        for i in (0..clean_hex.len()).step_by(2) {
            if let Ok(b) = u8::from_str_radix(&clean_hex[i..i+2], 16) {
                temp.push(b);
            }
        }
        Some(temp)
    } else {
        None
    };

    let search_pattern = hex_bytes.unwrap_or_else(|| query.as_bytes().to_vec());
    if search_pattern.is_empty() {
        return;
    }

    // Match search pattern against buffer bytes
    let pat_len = search_pattern.len();
    if pat_len > bytes.len() {
        return;
    }

    for i in 0..=(bytes.len() - pat_len) {
        if &bytes[i..i+pat_len] == search_pattern.as_slice() {
            for offset in 0..pat_len {
                matches.push(i + offset);
            }
        }
    }
}
