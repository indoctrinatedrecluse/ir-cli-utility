//! src/edit/mod.rs — Minimalist inline terminal text editor for `ir edit`.
use crate::EditOptions;
use std::fs;
use std::io::{stdout, Write};
use std::path::Path;

// ─── Key enum ──────────────────────────────────────────────────────────────────

#[cfg(target_os = "windows")]
extern "C" {
    fn _kbhit() -> std::os::raw::c_int;
    fn _getch() -> std::os::raw::c_int;
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Key {
    Char(char),
    Tab, BackTab,
    Up, Down, Left, Right,
    ShiftLeft, ShiftRight, ShiftUp, ShiftDown,
    CtrlLeft, CtrlRight,
    Home, End, CtrlHome, CtrlEnd,
    PageUp, PageDown,
    Backspace, Delete,
    CtrlBackspace, CtrlDelete,
    Enter,
    CtrlA, CtrlC, CtrlF, CtrlG,
    CtrlQ, CtrlS, CtrlV, CtrlX, CtrlY, CtrlZ,
    Escape,
    Unknown,
}

// ─── Windows key reader ─────────────────────────────────────────────────────────

#[cfg(target_os = "windows")]
fn read_key() -> Key {
    loop {
        unsafe {
            if _kbhit() != 0 {
                let ch = _getch();
                if ch == 0 || ch == 224 {
                    let sub = _getch();
                    return match (ch, sub) {
                        (224, 72)  => Key::Up,
                        (224, 80)  => Key::Down,
                        (224, 75)  => Key::Left,
                        (224, 77)  => Key::Right,
                        (224, 71)  => Key::Home,
                        (224, 79)  => Key::End,
                        (224, 73)  => Key::PageUp,
                        (224, 81)  => Key::PageDown,
                        (224, 83)  => Key::Delete,
                        (224, 115) => Key::CtrlLeft,
                        (224, 116) => Key::CtrlRight,
                        (224, 119) => Key::CtrlHome,
                        (224, 117) => Key::CtrlEnd,
                        (224, 147) => Key::CtrlDelete,
                        // Shift+Arrow via numpad codes (prefix 0)
                        (0, 72) | (0, 56) => Key::ShiftUp,
                        (0, 80) | (0, 50) => Key::ShiftDown,
                        (0, 75) | (0, 52) => Key::ShiftLeft,
                        (0, 77) | (0, 54) => Key::ShiftRight,
                        // Shift+Tab
                        (0, 15) => Key::BackTab,
                        _       => Key::Unknown,
                    };
                }
                return match ch {
                    9   => Key::Tab,
                    13  => Key::Enter,
                    8   => Key::Backspace,
                    27  => Key::Escape,
                    1   => Key::CtrlA,
                    3   => Key::CtrlC,
                    6   => Key::CtrlF,
                    7   => Key::CtrlG,
                    17  => Key::CtrlQ,
                    19  => Key::CtrlS,
                    22  => Key::CtrlV,
                    24  => Key::CtrlX,
                    25  => Key::CtrlY,
                    26  => Key::CtrlZ,
                    127 => Key::CtrlBackspace,
                    c if c >= 32 && c < 127 => Key::Char(c as u8 as char),
                    _   => Key::Unknown,
                };
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}

// ─── Linux raw mode ─────────────────────────────────────────────────────────────

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

// ─── Linux key reader ───────────────────────────────────────────────────────────

#[cfg(target_os = "linux")]
fn read_key() -> Key {
    let mut buf = [0u8; 16];
    loop {
        let n = unsafe { libc::read(0, buf.as_mut_ptr() as *mut libc::c_void, 16) };
        if n <= 0 { continue; }
        let n = n as usize;

        if buf[0] == 27 {
            if n == 1 { return Key::Escape; }
            if n >= 3 && buf[1] == b'[' {
                // Sequences with modifiers: ESC [ 1 ; <mod> <letter>
                if n >= 6 && buf[2] == b'1' && buf[3] == b';' {
                    let (modifier, letter) = (buf[4], buf[5]);
                    return match (modifier, letter) {
                        (b'2', b'A') => Key::ShiftUp,
                        (b'2', b'B') => Key::ShiftDown,
                        (b'2', b'C') => Key::ShiftRight,
                        (b'2', b'D') => Key::ShiftLeft,
                        (b'5', b'C') => Key::CtrlRight,
                        (b'5', b'D') => Key::CtrlLeft,
                        (b'5', b'H') => Key::CtrlHome,
                        (b'5', b'F') => Key::CtrlEnd,
                        _            => Key::Unknown,
                    };
                }
                // Ctrl+Delete: ESC [ 3 ; 5 ~
                if n >= 6 && buf[2] == b'3' && buf[3] == b';'
                    && buf[4] == b'5' && buf[5] == b'~'
                {
                    return Key::CtrlDelete;
                }
                return match buf[2] {
                    b'A' => Key::Up,
                    b'B' => Key::Down,
                    b'C' => Key::Right,
                    b'D' => Key::Left,
                    b'H' => Key::Home,
                    b'F' => Key::End,
                    b'5' => Key::PageUp,
                    b'6' => Key::PageDown,
                    b'3' => Key::Delete,   // ESC [ 3 ~
                    b'Z' => Key::BackTab,  // ESC [ Z  (Shift+Tab)
                    _    => Key::Unknown,
                };
            }
            if n >= 3 && buf[1] == b'O' {
                return match buf[2] {
                    b'H' => Key::Home,
                    b'F' => Key::End,
                    _    => Key::Unknown,
                };
            }
        }

        return match buf[0] {
            b'\r' | b'\n' => Key::Enter,
            127 => Key::Backspace,
            8   => Key::CtrlBackspace,
            9   => Key::Tab,
            1   => Key::CtrlA,
            3   => Key::CtrlC,
            6   => Key::CtrlF,
            7   => Key::CtrlG,
            17  => Key::CtrlQ,
            19  => Key::CtrlS,
            22  => Key::CtrlV,
            24  => Key::CtrlX,
            25  => Key::CtrlY,
            26  => Key::CtrlZ,
            c if c >= 32 && c < 127 => Key::Char(c as char),
            _   => Key::Unknown,
        };
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
}

// ─── Helpers ────────────────────────────────────────────────────────────────────

/// Byte offset of the n-th character in s (or s.len() if past end).
fn char_byte_idx(s: &str, n: usize) -> usize {
    s.char_indices().nth(n).map(|(i, _)| i).unwrap_or(s.len())
}

/// Previous word boundary (whitespace-delimited).
fn word_prev(chars: &[char], cx: usize) -> usize {
    if cx == 0 { return 0; }
    let mut i = cx;
    while i > 0 && chars[i - 1].is_whitespace() { i -= 1; }
    while i > 0 && !chars[i - 1].is_whitespace() { i -= 1; }
    i
}

/// Next word boundary (whitespace-delimited).
fn word_next(chars: &[char], cx: usize) -> usize {
    let len = chars.len();
    if cx >= len { return len; }
    let mut i = cx;
    while i < len && !chars[i].is_whitespace() { i += 1; }
    while i < len && chars[i].is_whitespace() { i += 1; }
    i
}

/// Leading whitespace of a line (for auto-indent).
fn leading_indent(line: &str) -> String {
    let end = line.find(|c: char| !c.is_whitespace()).unwrap_or(line.len());
    line[..end].to_string()
}

// ─── Editor mode ────────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Clone)]
enum EditorMode {
    Normal,
    Search,
    GoToLine,
}

// ─── Undo snapshot ──────────────────────────────────────────────────────────────

#[derive(Clone)]
struct EditSnapshot {
    lines: Vec<String>,
    cx:    usize,
    cy:    usize,
}

// ─── Editor state ───────────────────────────────────────────────────────────────

struct Editor {
    lines:    Vec<String>,
    cx:       usize,
    cy:       usize,
    row_off:  usize,
    col_off:  usize,
    filename: String,
    modified: bool,
    status_msg: String,
    tab_size: usize,
    // Undo / Redo
    undo_stack: Vec<EditSnapshot>,
    redo_stack: Vec<EditSnapshot>,
    // Selection: anchor = where Shift+move started; cursor is the other end
    sel_anchor: Option<(usize, usize)>,
    // Editor-internal clipboard
    clipboard: String,
    // Mode
    mode: EditorMode,
    // Search
    search_query:   String,
    search_matches: Vec<(usize, usize, usize)>, // (row, col_start, col_end)
    search_idx:     usize,
    search_start:   (usize, usize),
    // Go-to-line
    goto_input: String,
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
                Err(e) => vec![format!("(error reading file: {})", e)],
            }
        } else {
            vec![String::new()]
        };
        Self {
            lines,
            cx: 0, cy: 0,
            row_off: 0, col_off: 0,
            filename: filename.to_string(),
            modified: false,
            status_msg: String::new(),
            tab_size: 4,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            sel_anchor: None,
            clipboard: String::new(),
            mode: EditorMode::Normal,
            search_query: String::new(),
            search_matches: Vec::new(),
            search_idx: 0,
            search_start: (0, 0),
            goto_input: String::new(),
        }
    }

    fn gutter_width(&self) -> usize {
        self.lines.len().max(1).to_string().len() + 2
    }

    // ── Undo / Redo ────────────────────────────────────────────────────────────

    fn push_undo(&mut self) {
        if self.undo_stack.len() >= 100 {
            self.undo_stack.remove(0);
        }
        self.undo_stack.push(EditSnapshot {
            lines: self.lines.clone(),
            cx: self.cx, cy: self.cy,
        });
        self.redo_stack.clear();
    }

    fn undo(&mut self) {
        if let Some(snap) = self.undo_stack.pop() {
            self.redo_stack.push(EditSnapshot {
                lines: self.lines.clone(), cx: self.cx, cy: self.cy,
            });
            self.lines = snap.lines;
            self.cx = snap.cx; self.cy = snap.cy;
            self.modified = true; self.sel_anchor = None;
            self.status_msg = "Undone.".to_string();
        } else {
            self.status_msg = "Nothing to undo.".to_string();
        }
    }

    fn redo(&mut self) {
        if let Some(snap) = self.redo_stack.pop() {
            self.undo_stack.push(EditSnapshot {
                lines: self.lines.clone(), cx: self.cx, cy: self.cy,
            });
            self.lines = snap.lines;
            self.cx = snap.cx; self.cy = snap.cy;
            self.modified = true; self.sel_anchor = None;
            self.status_msg = "Redone.".to_string();
        } else {
            self.status_msg = "Nothing to redo.".to_string();
        }
    }

    // ── Save ───────────────────────────────────────────────────────────────────

    fn save(&mut self) {
        let path = Path::new(&self.filename);
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() && !parent.exists() {
                self.status_msg = format!(
                    "Save failed: directory '{}' does not exist.", parent.display());
                return;
            }
        }
        let content = self.lines.join("\n") + "\n";
        match fs::write(&self.filename, &content) {
            Ok(_) => {
                self.modified = false;
                self.status_msg = format!("Saved — {} bytes", content.len());
            }
            Err(e) => { self.status_msg = format!("Save failed: {}", e); }
        }
    }

    // ── Selection ──────────────────────────────────────────────────────────────

    /// Normalised bounds: ((start_row, start_col), (end_row, end_col)).
    fn selection_bounds(&self) -> Option<((usize, usize), (usize, usize))> {
        self.sel_anchor.map(|(ar, ac)| {
            let (cr, cc) = (self.cy, self.cx);
            if (ar, ac) <= (cr, cc) { ((ar, ac), (cr, cc)) }
            else                    { ((cr, cc), (ar, ac)) }
        })
    }

    /// Flat string of selected text, or None if no selection.
    fn selected_text(&self) -> Option<String> {
        let ((sr, sc), (er, ec)) = self.selection_bounds()?;
        if sr == er {
            let l = &self.lines[sr];
            return Some(l[char_byte_idx(l, sc)..char_byte_idx(l, ec)].to_string());
        }
        let mut out = String::new();
        let first = &self.lines[sr];
        out.push_str(&first[char_byte_idx(first, sc)..]);
        out.push('\n');
        for r in sr + 1..er {
            out.push_str(&self.lines[r]);
            out.push('\n');
        }
        let last = &self.lines[er];
        out.push_str(&last[..char_byte_idx(last, ec)]);
        Some(out)
    }

    /// Delete selected region. Returns true if a selection existed.
    fn delete_selection(&mut self) -> bool {
        let ((sr, sc), (er, ec)) = match self.selection_bounds() {
            Some(b) => b, None => return false,
        };
        self.push_undo();
        if sr == er {
            let a = char_byte_idx(&self.lines[sr], sc);
            let b = char_byte_idx(&self.lines[sr], ec);
            self.lines[sr].drain(a..b);
        } else {
            let first_keep = {
                let l = &self.lines[sr];
                l[..char_byte_idx(l, sc)].to_string()
            };
            let last_keep = {
                let l = &self.lines[er];
                l[char_byte_idx(l, ec)..].to_string()
            };
            self.lines.drain(sr..=er);
            self.lines.insert(sr, format!("{}{}", first_keep, last_keep));
        }
        self.cy = sr; self.cx = sc;
        self.sel_anchor = None; self.modified = true;
        true
    }

    /// Insert (possibly multi-line) text at current cursor.
    fn insert_text(&mut self, text: &str) {
        self.push_undo();
        let parts: Vec<&str> = text.split('\n').collect();
        if parts.len() == 1 {
            let bi = char_byte_idx(&self.lines[self.cy], self.cx);
            self.lines[self.cy].insert_str(bi, parts[0]);
            self.cx += parts[0].chars().count();
        } else {
            let split_byte = char_byte_idx(&self.lines[self.cy], self.cx);
            let rest = self.lines[self.cy][split_byte..].to_string();
            self.lines[self.cy].truncate(split_byte);
            self.lines[self.cy].push_str(parts[0]);
            let n = parts.len();
            for (i, part) in parts[1..n - 1].iter().enumerate() {
                self.lines.insert(self.cy + 1 + i, part.to_string());
            }
            let last_row = self.cy + n - 1;
            self.lines.insert(last_row, format!("{}{}", parts[n - 1], rest));
            self.cy = last_row;
            self.cx = parts[n - 1].chars().count();
        }
        self.modified = true;
    }

    // ── Indent helpers ─────────────────────────────────────────────────────────

    fn indent_lines(&mut self, sr: usize, er: usize) {
        self.push_undo();
        let spaces = " ".repeat(self.tab_size);
        for r in sr..=er { self.lines[r].insert_str(0, &spaces); }
        if self.cy >= sr && self.cy <= er { self.cx += self.tab_size; }
        self.modified = true;
    }

    fn dedent_lines(&mut self, sr: usize, er: usize) {
        self.push_undo();
        for r in sr..=er {
            let remove = self.lines[r].chars()
                .take(self.tab_size)
                .take_while(|c| *c == ' ')
                .count();
            if remove > 0 { self.lines[r].drain(..remove); }
        }
        if self.cy >= sr && self.cy <= er {
            self.cx = self.cx.saturating_sub(self.tab_size);
        }
        self.modified = true;
    }

    // ── Search ─────────────────────────────────────────────────────────────────

    fn update_search_matches(&mut self) {
        self.search_matches.clear();
        if self.search_query.is_empty() { return; }
        let qchars: Vec<char> = self.search_query.chars().collect();
        let qlen = qchars.len();
        for (ri, line) in self.lines.iter().enumerate() {
            let lchars: Vec<char> = line.chars().collect();
            let mut col = 0;
            while col + qlen <= lchars.len() {
                if lchars[col..col + qlen] == qchars[..] {
                    self.search_matches.push((ri, col, col + qlen));
                    col += qlen;
                } else { col += 1; }
            }
        }
        // Jump to nearest match at or after where search started
        if !self.search_matches.is_empty() {
            let pos = self.search_start;
            self.search_idx = self.search_matches.iter()
                .position(|(r, c, _)| (*r, *c) >= pos)
                .unwrap_or(0);
            let (r, c, _) = self.search_matches[self.search_idx];
            self.cy = r; self.cx = c;
        }
    }

    fn find_next(&mut self, forward: bool) {
        if self.search_matches.is_empty() { return; }
        self.search_idx = if forward {
            (self.search_idx + 1) % self.search_matches.len()
        } else if self.search_idx == 0 {
            self.search_matches.len() - 1
        } else {
            self.search_idx - 1
        };
        let (r, c, _) = self.search_matches[self.search_idx];
        self.cy = r; self.cx = c;
    }

    // ── Render ─────────────────────────────────────────────────────────────────

    fn render(&self, cols: u16, rows: u16) {
        let gw       = self.gutter_width();
        let cols_us  = cols as usize;
        let edit_cols = cols_us.saturating_sub(gw);
        // Layout: title(1) + edit(N) + status(1) + controls(1)
        let edit_rows = (rows as usize).saturating_sub(3);

        let mut buf = String::with_capacity(65536);
        buf.push_str("\x1b[?25l\x1b[H"); // hide cursor, go home

        // ── Title bar ─────────────────────────────────────────────────────────
        let mod_flag = if self.modified { "* " } else { "  " };
        let title = format!("  ir edit  │  {}{}", mod_flag, self.filename);
        let pos   = format!("Ln {}, Col {}  ", self.cy + 1, self.cx + 1);
        let pad   = cols_us.saturating_sub(title.chars().count() + pos.len());
        buf.push_str(&format!("\x1b[1;44;97m{}{}{}\x1b[0m\x1b[K\r\n",
            title, " ".repeat(pad), pos));

        // ── Edit area ─────────────────────────────────────────────────────────
        let sel = self.selection_bounds();

        for screen_row in 0..edit_rows {
            let file_row = screen_row + self.row_off;

            if file_row >= self.lines.len() {
                buf.push_str(&format!("\x1b[2;34m{:>w$}~\x1b[0m\x1b[K\r\n",
                    "", w = gw - 1));
                continue;
            }

            // Gutter: current line = bright yellow, others = dim grey
            let num_str = format!("{:>w$} ", file_row + 1, w = gw - 1);
            if file_row == self.cy {
                buf.push_str(&format!("\x1b[1;33m{}\x1b[0m", num_str));
            } else {
                buf.push_str(&format!("\x1b[2;37m{}\x1b[0m", num_str));
            }

            // Text with highlight ranges
            let search_ranges: Vec<(usize, usize)> = self.search_matches.iter()
                .filter(|(r, _, _)| *r == file_row)
                .map(|(_, cs, ce)| (*cs, *ce))
                .collect();

            render_line_content(&mut buf, &self.lines[file_row],
                self.col_off, edit_cols, file_row, sel, &search_ranges);
            buf.push_str("\x1b[K\r\n");
        }

        // ── Status bar ────────────────────────────────────────────────────────
        match &self.mode {
            EditorMode::Normal => {
                let sel_badge = if self.sel_anchor.is_some() {
                    "  \x1b[0;7;93mSEL\x1b[7m"
                } else { "" };
                let msg = if self.status_msg.is_empty() {
                    format!("  {} lines{}", self.lines.len(), sel_badge)
                } else {
                    format!("  {}{}", self.status_msg, sel_badge)
                };
                buf.push_str(&format!("\x1b[7m{}\x1b[0m\x1b[K\r\n", msg));
            }
            EditorMode::Search => {
                let match_badge = if self.search_matches.is_empty() {
                    " \x1b[0;7;91m[no matches]\x1b[7m".to_string()
                } else {
                    format!(" \x1b[0;7;92m[{}/{}]\x1b[7m",
                        self.search_idx + 1, self.search_matches.len())
                };
                buf.push_str(&format!("\x1b[7m  Search: {}{}\x1b[0m\x1b[K\r\n",
                    self.search_query, match_badge));
            }
            EditorMode::GoToLine => {
                buf.push_str(&format!("\x1b[7m  Go to line: {}_\x1b[0m\x1b[K\r\n",
                    self.goto_input));
            }
        }

        // ── Controls bar (nano-style, context-aware, always visible) ──────────
        let controls = match &self.mode {
            EditorMode::Normal =>
                "  \x1b[1m^S\x1b[0;2m Save  \x1b[1m^Q\x1b[0;2m Quit  \x1b[1m^Z\x1b[0;2m Undo  \
                 \x1b[1m^Y\x1b[0;2m Redo  \x1b[1m^F\x1b[0;2m Find  \x1b[1m^G\x1b[0;2m GoTo  \
                 \x1b[1m^A\x1b[0;2m All  \x1b[1m^C\x1b[0;2m Copy  \x1b[1m^X\x1b[0;2m Cut  \
                 \x1b[1m^V\x1b[0;2m Paste".to_string(),
            EditorMode::Search =>
                "  \x1b[2mType to search  \x1b[1m↑↓\x1b[0;2m Prev/Next  \
                 \x1b[1mEnter\x1b[0;2m Confirm  \x1b[1mEsc\x1b[0;2m Cancel".to_string(),
            EditorMode::GoToLine =>
                "  \x1b[2mType a line number  \x1b[1mEnter\x1b[0;2m Jump  \
                 \x1b[1mEsc\x1b[0;2m Cancel".to_string(),
        };
        buf.push_str(&format!("\x1b[0m{}\x1b[0m\x1b[K", controls));

        // ── Reposition hardware cursor ─────────────────────────────────────────
        let cur_row = (self.cy.saturating_sub(self.row_off)) + 2; // +2 for title bar row
        let cur_col = gw + self.cx.saturating_sub(self.col_off) + 1;
        buf.push_str(&format!("\x1b[{};{}H\x1b[?25h", cur_row, cur_col));

        let _ = stdout().write_all(buf.as_bytes());
        let _ = stdout().flush();
    }

    // ── Scroll ─────────────────────────────────────────────────────────────────

    fn scroll(&mut self, cols: u16, rows: u16) {
        let gw        = self.gutter_width();
        let edit_cols = (cols as usize).saturating_sub(gw);
        let edit_rows = (rows as usize).saturating_sub(3);

        if self.cy >= self.lines.len() {
            self.cy = self.lines.len().saturating_sub(1);
        }
        let ll = self.lines[self.cy].chars().count();
        if self.cx > ll { self.cx = ll; }

        if self.cy < self.row_off {
            self.row_off = self.cy;
        } else if self.cy >= self.row_off + edit_rows {
            self.row_off = self.cy - edit_rows + 1;
        }
        if self.cx < self.col_off {
            self.col_off = self.cx;
        } else if self.cx >= self.col_off + edit_cols {
            self.col_off = self.cx - edit_cols + 1;
        }
    }

    // ── Key dispatch ───────────────────────────────────────────────────────────

    /// Returns true if the editor should quit.
    fn process_key(&mut self, key: Key, cols: u16, rows: u16) -> bool {
        match self.mode.clone() {
            EditorMode::Normal   => self.process_normal(key, cols, rows),
            EditorMode::Search   => { self.process_search(key);  false }
            EditorMode::GoToLine => { self.process_goto(key);    false }
        }
    }

    fn process_normal(&mut self, key: Key, cols: u16, rows: u16) -> bool {
        let edit_rows = (rows as usize).saturating_sub(3);

        // Selection extension vs clearing
        let is_shift_move = matches!(key,
            Key::ShiftLeft | Key::ShiftRight | Key::ShiftUp | Key::ShiftDown);
        let is_plain_move = matches!(key,
            Key::Left | Key::Right | Key::Up | Key::Down
            | Key::Home | Key::End | Key::PageUp | Key::PageDown
            | Key::CtrlLeft | Key::CtrlRight | Key::CtrlHome | Key::CtrlEnd);

        if is_shift_move && self.sel_anchor.is_none() {
            self.sel_anchor = Some((self.cy, self.cx));
        }
        if is_plain_move { self.sel_anchor = None; }

        match key {
            // ── Quit / Save ──────────────────────────────────────────────────
            Key::CtrlQ | Key::Escape => return true,
            Key::CtrlS => self.save(),

            // ── Undo / Redo ──────────────────────────────────────────────────
            Key::CtrlZ => self.undo(),
            Key::CtrlY => self.redo(),

            // ── Find / GoToLine ──────────────────────────────────────────────
            Key::CtrlF => {
                self.search_start = (self.cy, self.cx);
                self.search_query.clear();
                self.search_matches.clear();
                self.mode = EditorMode::Search;
            }
            Key::CtrlG => {
                self.goto_input.clear();
                self.mode = EditorMode::GoToLine;
            }

            // ── Selection ────────────────────────────────────────────────────
            Key::CtrlA => {
                self.sel_anchor = Some((0, 0));
                self.cy = self.lines.len().saturating_sub(1);
                self.cx = self.lines[self.cy].chars().count();
            }
            Key::CtrlC => {
                if let Some(text) = self.selected_text() {
                    self.clipboard = text;
                    self.status_msg = "Copied.".to_string();
                } else {
                    self.clipboard = format!("{}\n", self.lines[self.cy]);
                    self.status_msg = "Line copied.".to_string();
                }
            }
            Key::CtrlX => {
                if let Some(text) = self.selected_text() {
                    self.clipboard = text;
                    self.delete_selection();
                    self.status_msg = "Cut.".to_string();
                } else {
                    self.clipboard = format!("{}\n", self.lines[self.cy]);
                    self.push_undo();
                    if self.lines.len() > 1 {
                        self.lines.remove(self.cy);
                        if self.cy >= self.lines.len() { self.cy = self.lines.len() - 1; }
                    } else {
                        self.lines[0].clear();
                    }
                    self.cx = 0; self.modified = true;
                    self.status_msg = "Line cut.".to_string();
                }
            }
            Key::CtrlV => {
                let text = self.clipboard.clone();
                if !text.is_empty() {
                    self.delete_selection();
                    self.insert_text(&text);
                    self.status_msg = "Pasted.".to_string();
                }
            }

            // ── Tab / Indent ─────────────────────────────────────────────────
            Key::Tab => {
                if let Some(((sr, _), (er, _))) = self.selection_bounds() {
                    self.indent_lines(sr, er);
                } else {
                    self.push_undo();
                    let spaces = " ".repeat(self.tab_size);
                    let bi = char_byte_idx(&self.lines[self.cy], self.cx);
                    self.lines[self.cy].insert_str(bi, &spaces);
                    self.cx += self.tab_size;
                    self.modified = true;
                }
            }
            Key::BackTab => {
                if let Some(((sr, _), (er, _))) = self.selection_bounds() {
                    self.dedent_lines(sr, er);
                } else {
                    self.dedent_lines(self.cy, self.cy);
                }
            }

            // ── Enter with auto-indent ───────────────────────────────────────
            Key::Enter => {
                self.delete_selection();
                self.push_undo();
                let indent     = leading_indent(&self.lines[self.cy]);
                let split_byte = char_byte_idx(&self.lines[self.cy], self.cx);
                let rest       = self.lines[self.cy][split_byte..].to_string();
                self.lines[self.cy].truncate(split_byte);
                self.cy += 1;
                self.lines.insert(self.cy, format!("{}{}", indent, rest));
                self.cx = indent.chars().count();
                self.modified = true;
            }

            // ── Backspace ────────────────────────────────────────────────────
            Key::Backspace => {
                if !self.delete_selection() {
                    if self.cx > 0 {
                        self.push_undo();
                        let start = char_byte_idx(&self.lines[self.cy], self.cx - 1);
                        let end   = char_byte_idx(&self.lines[self.cy], self.cx);
                        self.lines[self.cy].drain(start..end);
                        self.cx -= 1; self.modified = true;
                    } else if self.cy > 0 {
                        self.push_undo();
                        let rest = self.lines.remove(self.cy);
                        self.cy -= 1;
                        self.cx = self.lines[self.cy].chars().count();
                        self.lines[self.cy].push_str(&rest);
                        self.modified = true;
                    }
                }
            }

            // ── Ctrl+Backspace: delete word before cursor ────────────────────
            Key::CtrlBackspace => {
                if !self.delete_selection() {
                    let chars: Vec<char> = self.lines[self.cy].chars().collect();
                    let new_cx = word_prev(&chars, self.cx);
                    if new_cx < self.cx {
                        self.push_undo();
                        let s = char_byte_idx(&self.lines[self.cy], new_cx);
                        let e = char_byte_idx(&self.lines[self.cy], self.cx);
                        self.lines[self.cy].drain(s..e);
                        self.cx = new_cx; self.modified = true;
                    }
                }
            }

            // ── Delete ───────────────────────────────────────────────────────
            Key::Delete => {
                if !self.delete_selection() {
                    let ll = self.lines[self.cy].chars().count();
                    if self.cx < ll {
                        self.push_undo();
                        let s = char_byte_idx(&self.lines[self.cy], self.cx);
                        let e = char_byte_idx(&self.lines[self.cy], self.cx + 1);
                        self.lines[self.cy].drain(s..e);
                        self.modified = true;
                    } else if self.cy + 1 < self.lines.len() {
                        self.push_undo();
                        let next = self.lines.remove(self.cy + 1);
                        self.lines[self.cy].push_str(&next);
                        self.modified = true;
                    }
                }
            }

            // ── Ctrl+Delete: delete word after cursor ────────────────────────
            Key::CtrlDelete => {
                if !self.delete_selection() {
                    let chars: Vec<char> = self.lines[self.cy].chars().collect();
                    let new_cx = word_next(&chars, self.cx);
                    if new_cx > self.cx {
                        self.push_undo();
                        let s = char_byte_idx(&self.lines[self.cy], self.cx);
                        let e = char_byte_idx(&self.lines[self.cy], new_cx);
                        self.lines[self.cy].drain(s..e);
                        self.modified = true;
                    }
                }
            }

            // ── Char insert ──────────────────────────────────────────────────
            Key::Char(c) => {
                self.delete_selection();
                self.push_undo();
                let bi = char_byte_idx(&self.lines[self.cy], self.cx);
                self.lines[self.cy].insert(bi, c);
                self.cx += 1; self.modified = true;
                // Clear transient status after a character is typed
                if self.status_msg.ends_with('.') || self.status_msg.starts_with("Saved") {
                    self.status_msg.clear();
                }
            }

            // ── Cursor movement ───────────────────────────────────────────────
            Key::Left | Key::ShiftLeft => {
                if self.cx > 0 { self.cx -= 1; }
                else if self.cy > 0 {
                    self.cy -= 1; self.cx = self.lines[self.cy].chars().count();
                }
            }
            Key::Right | Key::ShiftRight => {
                let ll = self.lines[self.cy].chars().count();
                if self.cx < ll { self.cx += 1; }
                else if self.cy + 1 < self.lines.len() { self.cy += 1; self.cx = 0; }
            }
            Key::Up | Key::ShiftUp => {
                if self.cy > 0 {
                    self.cy -= 1;
                    let ll = self.lines[self.cy].chars().count();
                    if self.cx > ll { self.cx = ll; }
                }
            }
            Key::Down | Key::ShiftDown => {
                if self.cy + 1 < self.lines.len() {
                    self.cy += 1;
                    let ll = self.lines[self.cy].chars().count();
                    if self.cx > ll { self.cx = ll; }
                }
            }
            Key::CtrlLeft => {
                if self.cx > 0 {
                    let chars: Vec<char> = self.lines[self.cy].chars().collect();
                    self.cx = word_prev(&chars, self.cx);
                } else if self.cy > 0 {
                    self.cy -= 1; self.cx = self.lines[self.cy].chars().count();
                }
            }
            Key::CtrlRight => {
                let ll = self.lines[self.cy].chars().count();
                if self.cx < ll {
                    let chars: Vec<char> = self.lines[self.cy].chars().collect();
                    self.cx = word_next(&chars, self.cx);
                } else if self.cy + 1 < self.lines.len() {
                    self.cy += 1; self.cx = 0;
                }
            }
            Key::Home => {
                // Smart Home: first press → first non-whitespace, second → column 0
                let indent_col = self.lines[self.cy]
                    .find(|c: char| !c.is_whitespace())
                    .map(|byte_end| self.lines[self.cy][..byte_end].chars().count())
                    .unwrap_or(0);
                self.cx = if self.cx != indent_col { indent_col } else { 0 };
            }
            Key::End     => { self.cx = self.lines[self.cy].chars().count(); }
            Key::PageUp  => { self.cy = self.cy.saturating_sub(edit_rows); }
            Key::PageDown => {
                self.cy = (self.cy + edit_rows).min(self.lines.len().saturating_sub(1));
            }
            Key::CtrlHome => { self.cy = 0; self.cx = 0; }
            Key::CtrlEnd  => {
                self.cy = self.lines.len().saturating_sub(1);
                self.cx = self.lines[self.cy].chars().count();
            }
            Key::Unknown => {}
        }

        self.scroll(cols, rows);
        false
    }

    fn process_search(&mut self, key: Key) {
        match key {
            Key::Escape => {
                self.cy = self.search_start.0; self.cx = self.search_start.1;
                self.mode = EditorMode::Normal;
                self.search_query.clear(); self.search_matches.clear();
                self.status_msg = "Search cancelled.".to_string();
            }
            Key::Enter => {
                self.mode = EditorMode::Normal;
                self.status_msg = if self.search_matches.is_empty() {
                    format!("'{}' — not found.", self.search_query)
                } else {
                    format!("Match {}/{} found.",
                        self.search_idx + 1, self.search_matches.len())
                };
            }
            Key::Down => self.find_next(true),
            Key::Up   => self.find_next(false),
            Key::Backspace => { self.search_query.pop(); self.update_search_matches(); }
            Key::Char(c)   => { self.search_query.push(c); self.update_search_matches(); }
            _ => {}
        }
    }

    fn process_goto(&mut self, key: Key) {
        match key {
            Key::Escape => {
                self.mode = EditorMode::Normal;
                self.status_msg = "Go-to cancelled.".to_string();
            }
            Key::Enter => {
                if let Ok(n) = self.goto_input.trim().parse::<usize>() {
                    let target = n.saturating_sub(1).min(self.lines.len().saturating_sub(1));
                    self.cy = target; self.cx = 0;
                    self.status_msg = format!("Jumped to line {}.", n);
                } else {
                    self.status_msg = "Invalid line number.".to_string();
                }
                self.mode = EditorMode::Normal;
            }
            Key::Backspace         => { self.goto_input.pop(); }
            Key::Char(c) if c.is_ascii_digit() => { self.goto_input.push(c); }
            _ => {}
        }
    }
}

// ─── Line renderer (free function, avoids borrow conflicts) ─────────────────────

fn render_line_content(
    buf: &mut String,
    line: &str,
    col_off: usize,
    max_cols: usize,
    file_row: usize,
    sel: Option<((usize, usize), (usize, usize))>,
    search_ranges: &[(usize, usize)],
) {
    let chars: Vec<char> = line.chars().collect();
    let end = (col_off + max_cols).min(chars.len());
    let mut cur_style = 0u8; // 0=normal 1=selection 2=search
    buf.push_str("\x1b[0m");

    for c in col_off..end {
        let in_sel = sel.map_or(false, |((sr, sc), (er, ec))| {
            if file_row < sr || file_row > er { return false; }
            if sr == er { return c >= sc && c < ec; }
            if file_row == sr { return c >= sc; }
            if file_row == er { return c < ec; }
            true
        });
        let in_search = search_ranges.iter().any(|(cs, ce)| c >= *cs && c < *ce);

        let new_style: u8 = if in_search { 2 } else if in_sel { 1 } else { 0 };
        if new_style != cur_style {
            buf.push_str(match new_style {
                1 => "\x1b[0;44;97m",  // blue bg / white text  — selection
                2 => "\x1b[0;43;30m",  // amber bg / black text — search match
                _ => "\x1b[0m",
            });
            cur_style = new_style;
        }
        buf.push(chars[c]);
    }
    buf.push_str("\x1b[0m");
}

// ─── Entry point ─────────────────────────────────────────────────────────────────

pub fn edit(filename: &str, _options: EditOptions) {
    let path = Path::new(filename);

    // Reject directories immediately
    if path.is_dir() {
        eprintln!("error: '{}' is a directory, not a file.", filename);
        std::process::exit(1);
    }

    // Binary file detection (first 512 bytes)
    let warn_binary = if path.exists() {
        fs::read(filename).map_or(false, |bytes| {
            let sample = &bytes[..bytes.len().min(512)];
            let has_null  = sample.contains(&0u8);
            let non_text  = sample.iter()
                .filter(|&&b| b < 9 || (b > 13 && b < 32)).count();
            has_null || non_text > sample.len() / 10
        })
    } else { false };

    #[cfg(target_os = "linux")]
    let _raw = set_raw_mode();

    let mut editor = Editor::new(filename);
    if warn_binary {
        editor.status_msg =
            "WARNING: binary file — editing may corrupt it.".to_string();
    }

    print!("\x1b[2J\x1b[H");
    let _ = stdout().flush();

    loop {
        let (cols, rows) = terminal_size();
        editor.scroll(cols, rows);
        editor.render(cols, rows);

        let key       = read_key();
        let want_quit = editor.process_key(key, cols, rows);

        if want_quit {
            if editor.modified {
                editor.status_msg =
                    "Unsaved changes!  Ctrl+S = save & quit   Ctrl+Q / Esc = discard"
                        .to_string();
                let (cols, rows) = terminal_size();
                editor.scroll(cols, rows);
                editor.render(cols, rows);
                match read_key() {
                    Key::CtrlQ | Key::Escape => break,
                    Key::CtrlS => { editor.save(); break; }
                    _ => { editor.status_msg.clear(); }
                }
            } else {
                break;
            }
        }
    }

    print!("\x1b[2J\x1b[H\x1b[?25h");
    let _ = stdout().flush();
}
