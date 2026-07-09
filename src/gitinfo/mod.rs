use std::collections::{HashMap, HashSet};
use std::fs::{self, File};
use std::io::{self, Read, Write, BufRead, BufReader, IsTerminal};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant, SystemTime};
use crate::GitInfoOptions;

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
    PageUp,
    PageDown,
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
                        73 => Key::PageUp,
                        81 => Key::PageDown,
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
                        b'5' => Key::PageUp, // usually Esc[5~
                        b'6' => Key::PageDown, // usually Esc[6~
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

// ─── Git Structures ───────────────────────────────────────────────────────────
#[derive(Debug, Clone)]
struct GitCommit {
    sha: String,
    _parent_sha: String,
    author: String,
    email: String,
    timestamp: u64,
    tz: String,
    message: String,
}

#[derive(Debug, Clone)]
struct IndexEntry {
    path: String,
    size: u32,
    mtime_sec: u32,
    _sha1: String,
}

#[derive(Debug, Clone, PartialEq)]
enum FileStatus {
    Modified,
    Untracked,
    Deleted,
}

#[derive(Debug, Clone)]
struct StatusItem {
    path: String,
    status: FileStatus,
    size: u64,
    mtime: String,
}

#[derive(Debug, Clone)]
struct GitRef {
    name: String,
    sha: String,
    is_tag: bool,
    is_remote: bool,
}

// Locate the .git folder by walking up from the start path
fn find_git_repo(start_path: &str) -> Option<(PathBuf, PathBuf)> {
    let mut curr = PathBuf::from(start_path);
    loop {
        let git_dir = curr.join(".git");
        if git_dir.exists() && git_dir.is_dir() {
            if let Ok(canon_root) = curr.canonicalize() {
                if let Ok(canon_git) = git_dir.canonicalize() {
                    return Some((canon_root, canon_git));
                }
            }
        }
        if !curr.pop() {
            break;
        }
    }
    None
}

// Read plain text refs file or packed-refs
fn parse_refs(git_dir: &Path) -> Vec<GitRef> {
    let mut refs = Vec::new();
    
    // 1. Scan heads (local branches)
    let heads_dir = git_dir.join("refs").join("heads");
    if heads_dir.exists() {
        scan_refs_dir(&heads_dir, &heads_dir, false, false, &mut refs);
    }

    // 2. Scan tags
    let tags_dir = git_dir.join("refs").join("tags");
    if tags_dir.exists() {
        scan_refs_dir(&tags_dir, &tags_dir, true, false, &mut refs);
    }

    // 3. Scan remotes
    let remotes_dir = git_dir.join("refs").join("remotes");
    if remotes_dir.exists() {
        scan_refs_dir(&remotes_dir, &remotes_dir, false, true, &mut refs);
    }

    // 4. Scan packed-refs
    let packed_file = git_dir.join("packed-refs");
    if packed_file.exists() {
        if let Ok(file) = File::open(&packed_file) {
            let reader = BufReader::new(file);
            for line in reader.lines().map_while(Result::ok) {
                let line = line.trim();
                if line.starts_with('#') || line.starts_with('^') || line.is_empty() {
                    continue;
                }
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    let sha = parts[0].to_string();
                    let ref_path = parts[1];
                    let is_tag = ref_path.contains("refs/tags/");
                    let is_remote = ref_path.contains("refs/remotes/");
                    let name = ref_path
                        .trim_start_matches("refs/heads/")
                        .trim_start_matches("refs/tags/")
                        .trim_start_matches("refs/remotes/")
                        .to_string();
                    
                    // Deduplicate ref overrides (local unpacked overrides packed-refs)
                    if !refs.iter().any(|r: &GitRef| r.name == name && r.is_tag == is_tag && r.is_remote == is_remote) {
                        refs.push(GitRef { name, sha, is_tag, is_remote });
                    }
                }
            }
        }
    }

    refs
}

fn scan_refs_dir(root: &Path, curr: &Path, is_tag: bool, is_remote: bool, refs: &mut Vec<GitRef>) {
    if let Ok(entries) = fs::read_dir(curr) {
        for entry in entries.map_while(Result::ok) {
            let path = entry.path();
            if path.is_dir() {
                scan_refs_dir(root, &path, is_tag, is_remote, refs);
            } else if path.is_file() {
                if let Ok(sha_raw) = fs::read_to_string(&path) {
                    let sha = sha_raw.trim().to_string();
                    if sha.len() == 40 {
                        if let Ok(rel) = path.strip_prefix(root) {
                            let name = rel.to_string_lossy().replace('\\', "/");
                            refs.push(GitRef { name, sha, is_tag, is_remote });
                        }
                    }
                }
            }
        }
    }
}

// Parse binary .git/index
fn parse_git_index(git_dir: &Path) -> Result<Vec<IndexEntry>, String> {
    let index_file = git_dir.join("index");
    if !index_file.exists() {
        return Ok(Vec::new());
    }

    let mut file = File::open(&index_file).map_err(|e| format!("Failed to open index: {}", e))?;
    let mut data = Vec::new();
    file.read_to_end(&mut data).map_err(|e| format!("Failed to read index: {}", e))?;

    if data.len() < 12 {
        return Err("Index file too small".to_string());
    }

    // Header validation: DIRC signature
    if &data[0..4] != b"DIRC" {
        return Err("Invalid index signature (not DIRC)".to_string());
    }

    let _version = u32::from_be_bytes([data[4], data[5], data[6], data[7]]);
    let entry_count = u32::from_be_bytes([data[8], data[9], data[10], data[11]]) as usize;

    let mut pos = 12;
    let mut entries = Vec::with_capacity(entry_count);

    for _ in 0..entry_count {
        if pos + 62 > data.len() {
            break;
        }

        // metadata offsets:
        // ctime_sec (4B), ctime_nsec (4B), mtime_sec (4B), mtime_nsec (4B) -> start pos+0..pos+16
        // dev (4B), ino (4B), mode (4B), uid (4B), gid (4B), file_size (4B) -> pos+16..pos+40
        let mtime_sec = u32::from_be_bytes([data[pos + 8], data[pos + 9], data[pos + 10], data[pos + 11]]);
        let file_size = u32::from_be_bytes([data[pos + 36], data[pos + 37], data[pos + 38], data[pos + 39]]);
        
        // SHA-1 (20B) -> pos+40..pos+60
        let mut sha_bytes = [0u8; 20];
        sha_bytes.copy_from_slice(&data[pos + 40..pos + 60]);
        let sha1 = sha_bytes.iter().map(|b| format!("{:02x}", b)).collect::<String>();

        // flags (2B) -> pos+60..pos+62
        let flags = u16::from_be_bytes([data[pos + 60], data[pos + 61]]);
        let path_len = (flags & 0x0FFF) as usize;

        pos += 62;

        let path = if path_len < 0x0FFF {
            if pos + path_len > data.len() {
                break;
            }
            let p = String::from_utf8_lossy(&data[pos..pos + path_len]).into_owned();
            pos += path_len;
            p
        } else {
            // scan for null terminator
            let mut len = 0;
            while pos + len < data.len() && data[pos + len] != 0 {
                len += 1;
            }
            let p = String::from_utf8_lossy(&data[pos..pos + len]).into_owned();
            pos += len;
            p
        };

        // Padding: entries are aligned to 8-byte boundaries relative to index start (pos 12, but basically offsets match entry index start alignment)
        // Pad length is calculated relative to 62 metadata/flags + path length
        let align_len = ((62 + path.len() + 8) / 8) * 8;
        let padding = align_len - (62 + path.len());
        pos += padding;
        
        entries.push(IndexEntry { path, size: file_size, mtime_sec, _sha1: sha1 });
    }

    Ok(entries)
}

// Parse .git/logs/HEAD reflog to extract chronological commits list
fn parse_commit_history(git_dir: &Path) -> Vec<GitCommit> {
    let mut list = Vec::new();
    let reflog_file = git_dir.join("logs").join("HEAD");
    if !reflog_file.exists() {
        return list;
    }

    if let Ok(file) = File::open(&reflog_file) {
        let reader = BufReader::new(file);
        for line in reader.lines().map_while(Result::ok) {
            if line.is_empty() { continue; }
            let parts: Vec<&str> = line.splitn(3, ' ').collect();
            if parts.len() < 3 { continue; }
            let parent_sha = parts[0].to_string();
            let sha = parts[1].to_string();
            
            // Remainder format: Author <email> Timestamp TZ \t Message
            let remainder = parts[2];
            let tab_split: Vec<&str> = remainder.splitn(2, '\t').collect();
            let author_meta = tab_split[0];
            let message = if tab_split.len() == 2 { tab_split[1].to_string() } else { "".to_string() };

            // Parse author metadata
            let email_start = author_meta.find('<');
            let email_end = author_meta.find('>');
            
            let mut author = "Unknown".to_string();
            let mut email = "unknown@unknown.com".to_string();
            let mut timestamp = 0;
            let mut tz = "+0000".to_string();

            if let (Some(es), Some(ee)) = (email_start, email_end) {
                author = author_meta[..es].trim().to_string();
                email = author_meta[es+1..ee].to_string();
                
                let time_meta = author_meta[ee+1..].trim();
                let time_parts: Vec<&str> = time_meta.split_whitespace().collect();
                if !time_parts.is_empty() {
                    timestamp = time_parts[0].parse::<u64>().unwrap_or(0);
                    if time_parts.len() > 1 {
                        tz = time_parts[1].to_string();
                    }
                }
            }

            list.push(GitCommit { sha, _parent_sha: parent_sha, author, email, timestamp, tz, message });
        }
    }

    // Deduplicate and reverse (to get latest first)
    let mut unique_commits = Vec::new();
    let mut seen = HashSet::new();
    for commit in list.into_iter().rev() {
        if seen.insert(commit.sha.clone()) {
            unique_commits.push(commit);
        }
    }
    unique_commits
}

// Traverse directories to find untracked files
fn get_untracked_files(repo_root: &Path, index_paths: &HashSet<String>) -> Vec<PathBuf> {
    let mut untracked = Vec::new();
    let mut stack = vec![repo_root.to_path_buf()];

    while let Some(dir) = stack.pop() {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.map_while(Result::ok) {
                let path = entry.path();
                let name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
                if name == ".git" || name == "target" || name == ".idea" || name == ".vscode" || name == "node_modules" {
                    continue;
                }
                
                if path.is_dir() {
                    stack.push(path);
                } else if path.is_file() {
                    if let Ok(rel) = path.strip_prefix(repo_root) {
                        let rel_str = rel.to_string_lossy().replace('\\', "/");
                        if !index_paths.contains(&rel_str) {
                            untracked.push(path);
                        }
                    }
                }
            }
        }
    }
    untracked
}

// Compute contribution metrics
fn calculate_contributions(commits: &[GitCommit]) -> Vec<(String, usize)> {
    let mut map = HashMap::new();
    for commit in commits {
        let entry = map.entry(commit.author.clone()).or_insert(0);
        *entry += 1;
    }
    let mut sorted: Vec<(String, usize)> = map.into_iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));
    sorted
}

fn get_dir_size(dir: &Path) -> u64 {
    let mut size = 0;
    let mut stack = vec![dir.to_path_buf()];
    while let Some(p) = stack.pop() {
        if p.is_file() {
            size += p.metadata().map(|m| m.len()).unwrap_or(0);
        } else if p.is_dir() {
            if let Ok(entries) = fs::read_dir(p) {
                for entry in entries.map_while(Result::ok) {
                    stack.push(entry.path());
                }
            }
        }
    }
    size
}

pub fn run_gitinfo(options: GitInfoOptions) {
    let (repo_root, git_dir) = match find_git_repo(&options.source) {
        Some(paths) => paths,
        None => {
            eprintln!("Error: Not a git repository (or any of the parent directories): .git");
            std::process::exit(1);
        }
    };

    // Parse git refs
    let refs = parse_refs(&git_dir);
    
    // Resolve active branch
    let head_file = git_dir.join("HEAD");
    let mut active_ref = "HEAD (Detached)".to_string();
    let mut active_sha = "".to_string();
    if head_file.exists() {
        if let Ok(content) = fs::read_to_string(&head_file) {
            let content = content.trim();
            if content.starts_with("ref:") {
                let full_ref = content["ref:".len()..].trim();
                active_ref = full_ref.trim_start_matches("refs/heads/").to_string();
                
                // Read SHA of active branch
                let ref_path = git_dir.join(full_ref);
                if ref_path.exists() {
                    active_sha = fs::read_to_string(&ref_path).unwrap_or_default().trim().to_string();
                } else {
                    // Try to resolve in packed-refs
                    if let Some(r) = refs.iter().find(|r| r.name == active_ref && !r.is_tag && !r.is_remote) {
                        active_sha = r.sha.clone();
                    }
                }
            } else {
                active_sha = content.to_string();
            }
        }
    }

    // Load commit history logs
    let commits = parse_commit_history(&git_dir);

    // Parse Git Index
    let index_entries = match parse_git_index(&git_dir) {
        Ok(entries) => entries,
        Err(e) => {
            eprintln!("Warning: Failed to parse Git index: {}", e);
            Vec::new()
        }
    };

    // Calculate Status items
    let mut status_items = Vec::new();
    let mut index_paths = HashSet::new();
    
    for entry in &index_entries {
        index_paths.insert(entry.path.clone());
        let disk_path = repo_root.join(&entry.path);
        
        if !disk_path.exists() {
            status_items.push(StatusItem {
                path: entry.path.clone(),
                status: FileStatus::Deleted,
                size: 0,
                mtime: "-".to_string(),
            });
        } else if let Ok(meta) = disk_path.metadata() {
            let disk_size = meta.len() as u32;
            let disk_mtime = meta.modified()
                .map(|t| t.duration_since(SystemTime::UNIX_EPOCH).map(|d| d.as_secs() as u32).unwrap_or(0))
                .unwrap_or(0);
            
            if disk_size != entry.size || (disk_mtime != entry.mtime_sec && entry.mtime_sec != 0) {
                let mod_time_str = meta.modified().ok()
                    .map(|t| {
                        let dt: chrono::DateTime<chrono::Local> = t.into();
                        dt.format("%Y-%m-%d %H:%M:%S").to_string()
                    })
                    .unwrap_or_else(|| "-".to_string());
                
                status_items.push(StatusItem {
                    path: entry.path.clone(),
                    status: FileStatus::Modified,
                    size: meta.len(),
                    mtime: mod_time_str,
                });
            }
        }
    }

    // Traverse and find untracked files
    let untracked_paths = get_untracked_files(&repo_root, &index_paths);
    for path in untracked_paths {
        if let Ok(rel) = path.strip_prefix(&repo_root) {
            let rel_str = rel.to_string_lossy().replace('\\', "/");
            if let Ok(meta) = path.metadata() {
                let mod_time_str = meta.modified().ok()
                    .map(|t| {
                        let dt: chrono::DateTime<chrono::Local> = t.into();
                        dt.format("%Y-%m-%d %H:%M:%S").to_string()
                    })
                    .unwrap_or_else(|| "-".to_string());
                status_items.push(StatusItem {
                    path: rel_str,
                    status: FileStatus::Untracked,
                    size: meta.len(),
                    mtime: mod_time_str,
                });
            }
        }
    }

    // Sort status items by path name
    status_items.sort_by(|a, b| a.path.cmp(&b.path));

    // Calculate Summary stats
    let total_tracked_files = index_entries.len();
    let total_commits = commits.len();
    let top_contributors = calculate_contributions(&commits);
    let total_tracked_size = index_entries.iter().map(|e| e.size as u64).sum::<u64>();
    let git_dir_size = get_dir_size(&git_dir);

    // Initialize TUI
    if !io::stdout().is_terminal() {
        // Fallback simple print summary
        println!("Active Branch: {}", active_ref);
        println!("Head Commit:   {}", active_sha);
        println!("Total Commits: {}", total_commits);
        println!("Index Files:   {} tracked files ({})", total_tracked_files, format_size(total_tracked_size));
        println!("Dirty Changes: {} files modified/untracked/deleted", status_items.len());
        return;
    }

    #[cfg(not(target_os = "windows"))]
    let _raw_guard = set_raw_mode();

    // Alternate buffer, hide cursor
    print!("\x1B[?1049h\x1B[2J\x1B[H\x1B[?25l");
    let _ = io::stdout().flush();

    let mut current_tab = 0; // 0: History, 1: Status, 2: Refs, 3: Summary
    let mut history_scroll = 0usize;
    let mut status_scroll = 0usize;
    let mut refs_scroll = 0usize;
    let mut summary_scroll = 0usize;

    let (mut cols, mut rows) = terminal_size();
    let mut last_frame = Instant::now();

    loop {
        // Handle input
        match poll_key() {
            Key::Esc | Key::Char('q') | Key::Char('Q') => {
                break;
            }
            Key::Char('1') => { if current_tab != 0 { current_tab = 0; print!("\x1B[2J"); } }
            Key::Char('2') => { if current_tab != 1 { current_tab = 1; print!("\x1B[2J"); } }
            Key::Char('3') => { if current_tab != 2 { current_tab = 2; print!("\x1B[2J"); } }
            Key::Char('4') => { if current_tab != 3 { current_tab = 3; print!("\x1B[2J"); } }
            
            Key::Up => {
                match current_tab {
                    0 => { history_scroll = history_scroll.saturating_sub(1); }
                    1 => { status_scroll = status_scroll.saturating_sub(1); }
                    2 => { refs_scroll = refs_scroll.saturating_sub(1); }
                    3 => { summary_scroll = summary_scroll.saturating_sub(1); }
                    _ => {}
                }
            }
            Key::Down => {
                match current_tab {
                    0 => {
                        if history_scroll + 1 < commits.len() {
                            history_scroll += 1;
                        }
                    }
                    1 => {
                        if status_scroll + 1 < status_items.len() {
                            status_scroll += 1;
                        }
                    }
                    2 => {
                        if refs_scroll + 1 < refs.len() {
                            refs_scroll += 1;
                        }
                    }
                    3 => {
                        if summary_scroll + 1 < top_contributors.len() {
                            summary_scroll += 1;
                        }
                    }
                    _ => {}
                }
            }
            Key::PageUp => {
                let step = rows.saturating_sub(6) as usize;
                match current_tab {
                    0 => { history_scroll = history_scroll.saturating_sub(step); }
                    1 => { status_scroll = status_scroll.saturating_sub(step); }
                    2 => { refs_scroll = refs_scroll.saturating_sub(step); }
                    3 => { summary_scroll = summary_scroll.saturating_sub(step); }
                    _ => {}
                }
            }
            Key::PageDown => {
                let step = rows.saturating_sub(6) as usize;
                match current_tab {
                    0 => { history_scroll = (history_scroll + step).min(commits.len().saturating_sub(1)); }
                    1 => { status_scroll = (status_scroll + step).min(status_items.len().saturating_sub(1)); }
                    2 => { refs_scroll = (refs_scroll + step).min(refs.len().saturating_sub(1)); }
                    3 => { summary_scroll = (summary_scroll + step).min(top_contributors.len().saturating_sub(1)); }
                    _ => {}
                }
            }
            _ => {}
        }

        // Resize detection
        let (new_cols, new_rows) = terminal_size();
        if new_cols != cols || new_rows != rows {
            cols = new_cols;
            rows = new_rows;
            print!("\x1B[2J\x1B[H");
            let _ = io::stdout().flush();
        }

        // Build frame buffer
        let width = cols as usize;
        let height = rows as usize;
        let mut frame = String::new();
        frame.push_str("\x1B[H"); // Cursor to home

        // 1. Header (Menu tabs)
        frame.push_str("\x1B[1;30;42m ");
        frame.push_str("ir gitinfo TUI ");
        frame.push_str("\x1B[0m");
        frame.push_str("  ");

        let tab_labels = &["[1] History & Graph", "[2] Changes Status", "[3] Refs & Branches", "[4] Repository Stats"];
        for (i, label) in tab_labels.iter().enumerate() {
            if i == current_tab {
                frame.push_str(&format!("\x1B[1;32m> {} < \x1B[0m ", label));
            } else {
                frame.push_str(&format!("\x1B[37m  {}   \x1B[0m ", label));
            }
        }
        frame.push_str("\x1B[K\n");
        frame.push_str(&"━".repeat(width));
        frame.push_str("\x1B[K\n");

        // 2. Body based on selected Tab
        let content_height = height.saturating_sub(5); // header(3) + footer(2)

        match current_tab {
            0 => {
                // TAB 1: HISTORY & GRAPH
                let details_width = if width > 90 { 50 } else { (width * 45) / 100 };
                let commits_list_width = width.saturating_sub(details_width).saturating_sub(3);

                let selected_commit = if !commits.is_empty() && history_scroll < commits.len() {
                    Some(&commits[history_scroll])
                } else {
                    None
                };

                let message_lines = if let Some(c) = &selected_commit {
                    let line_limit = details_width.saturating_sub(2);
                    wrap_text(&c.message, line_limit)
                } else {
                    Vec::new()
                };

                for row_y in 0..content_height {
                    let commit_idx = row_y + history_scroll;
                    
                    // Render left: commit item list
                    let mut left_str = String::new();
                    if commit_idx < commits.len() {
                        let commit = &commits[commit_idx];
                        let short_sha = if commit.sha.len() > 7 { &commit.sha[..7] } else { &commit.sha };
                        
                        // Check if this commit has branch/tag annotations
                        let mut ref_names = Vec::new();
                        for r in &refs {
                            if r.sha == commit.sha {
                                ref_names.push(r);
                            }
                        }
                        
                        let mut ref_deco = String::new();
                        let mut current_deco_len = 0;
                        let max_deco_len = commits_list_width.saturating_sub(25);
                        
                        for r in &ref_names {
                            let item_display_len = r.name.chars().count() + 3;
                            if current_deco_len + item_display_len <= max_deco_len {
                                let color = if r.is_tag {
                                    "\x1B[1;33m" // Yellow tags
                                } else if r.is_remote {
                                    "\x1B[1;31m" // Red remote refs
                                } else {
                                    "\x1B[1;32m" // Green local branch
                                };
                                ref_deco.push_str(&format!("{}({}) \x1B[0m", color, r.name));
                                current_deco_len += item_display_len;
                            } else {
                                if current_deco_len == 0 {
                                    let allowed = max_deco_len.saturating_sub(5);
                                    if allowed > 0 {
                                        let truncated_name: String = r.name.chars().take(allowed).collect();
                                        let color = if r.is_tag { "\x1B[1;33m" } else if r.is_remote { "\x1B[1;31m" } else { "\x1B[1;32m" };
                                        ref_deco.push_str(&format!("{}({}...) \x1B[0m", color, truncated_name));
                                        current_deco_len += allowed + 6;
                                    }
                                }
                                break;
                            }
                        }

                        let is_selected = commit_idx == history_scroll;
                        let select_marker = if is_selected { "\x1B[1;32m>\x1B[0m " } else { "  " };

                        let mut graph_line = "* ".to_string();
                        // Draw vertical connections if not last commit
                        if commit_idx + 1 < commits.len() {
                            graph_line = "*".to_string();
                        }
                        
                        let prefix_len = 13 + current_deco_len;
                        let msg_limit = commits_list_width.saturating_sub(prefix_len);
                        let clean_msg = commit.message.trim();
                        let char_count = clean_msg.chars().count();
                        let truncated_msg = if char_count > msg_limit {
                            if msg_limit > 3 {
                                let mut tr: String = clean_msg.chars().take(msg_limit - 3).collect();
                                tr.push_str("...");
                                tr
                            } else {
                                "".to_string()
                            }
                        } else {
                            clean_msg.to_string()
                        };

                        left_str = format!(
                            "{}\x1B[33m{}\x1B[0m \x1B[32m{}\x1B[0m {}{}",
                            select_marker, short_sha, graph_line, ref_deco, truncated_msg
                        );
                    }

                    // Render right: details panel (split line + commit details)
                    let right_str = match &selected_commit {
                        Some(c) if row_y == 0 => {
                            let sha_limit = details_width.saturating_sub(9);
                            let display_sha = if c.sha.chars().count() > sha_limit {
                                c.sha.chars().take(sha_limit).collect::<String>()
                            } else {
                                c.sha.clone()
                            };
                            format!("\x1B[1;36mCommit:\x1B[0m  {}", display_sha)
                        }
                        Some(c) if row_y == 1 => {
                            let author_str = format!("{} <{}>", c.author, c.email);
                            let author_limit = details_width.saturating_sub(9);
                            let display_author = if author_str.chars().count() > author_limit {
                                if author_limit > 3 {
                                    let mut tr: String = author_str.chars().take(author_limit - 3).collect();
                                    tr.push_str("...");
                                    tr
                                } else {
                                    "".to_string()
                                }
                            } else {
                                author_str
                            };
                            format!("\x1B[1;36mAuthor:\x1B[0m  {}", display_author)
                        }
                        Some(c) if row_y == 2 => {
                            let dt = chrono::DateTime::from_timestamp(c.timestamp as i64, 0)
                                .map(|d| d.with_timezone(&chrono::Local).format("%Y-%m-%d %H:%M:%S").to_string())
                                .unwrap_or_else(|| "-".to_string());
                            let date_str = format!("{} ({})", dt, c.tz);
                            let date_limit = details_width.saturating_sub(9);
                            let display_date = if date_str.chars().count() > date_limit {
                                if date_limit > 3 {
                                    let mut tr: String = date_str.chars().take(date_limit - 3).collect();
                                    tr.push_str("...");
                                    tr
                                } else {
                                    "".to_string()
                                }
                            } else {
                                date_str
                            };
                            format!("\x1B[1;36mDate:\x1B[0m    {}", display_date)
                        }
                        Some(_) if row_y == 4 => {
                            let label = "Message:";
                            if label.chars().count() > details_width {
                                "".to_string()
                            } else {
                                "\x1B[1;37mMessage:\x1B[0m".to_string()
                            }
                        }
                        Some(_) if row_y >= 5 && (row_y - 5) < message_lines.len() => {
                            let line = &message_lines[row_y - 5];
                            format!("  {}", line)
                        }
                        _ => "".to_string(),
                    };

                    // Format columns
                    let left_len = strip_ansi_len(&left_str);
                    let pad = commits_list_width.saturating_sub(left_len);
                    frame.push_str(&left_str);
                    frame.push_str(&" ".repeat(pad));
                    frame.push_str(" \x1B[90m│\x1B[0m ");
                    frame.push_str(&right_str);
                    frame.push_str("\x1B[K\n");
                }
            }
            1 => {
                // TAB 2: CHANGES STATUS
                frame.push_str(&format!(
                    " \x1B[1mTracked Files:\x1B[0m {}  |  \x1B[1mChanges:\x1B[0m {} uncommitted\x1B[K\n\x1B[K\n",
                    total_tracked_files, status_items.len()
                ));
                let adjusted_height = content_height.saturating_sub(2);

                if status_items.is_empty() {
                    frame.push_str("  \x1B[32mNo uncommitted changes. Working directory is clean.\x1B[0m\x1B[K\n");
                    for _ in 1..adjusted_height {
                        frame.push_str("\x1B[K\n");
                    }
                } else {
                    frame.push_str(&format!(
                        "  {:<3} {:<60} {:<15} {}\x1B[K\n",
                        "ST", "FILE PATH", "SIZE", "MODIFIED TIME"
                    ));
                    frame.push_str(&format!("  {}\x1B[K\n", "━".repeat(width.saturating_sub(4))));

                    for row_y in 2..adjusted_height {
                        let item_idx = row_y - 2 + status_scroll;
                        if item_idx < status_items.len() {
                            let item = &status_items[item_idx];
                            let status_lbl = match item.status {
                                FileStatus::Modified => "\x1B[1;33m[M]\x1B[0m",
                                FileStatus::Untracked => "\x1B[1;32m[?]\x1B[0m",
                                FileStatus::Deleted => "\x1B[1;31m[D]\x1B[0m",
                            };

                            let path_limit = (width * 55) / 100;
                            let truncated_path = if item.path.len() > path_limit {
                                format!("...{}", &item.path[item.path.len() - path_limit + 3..])
                            } else {
                                item.path.clone()
                            };

                            let select_marker = if item_idx == status_scroll { "\x1B[1;32m>\x1B[0m" } else { " " };
                            frame.push_str(&format!(
                                "{} {} {:<60} {:<15} {}\x1B[K\n",
                                select_marker, status_lbl, truncated_path, format_size(item.size), item.mtime
                            ));
                        } else {
                            frame.push_str("\x1B[K\n");
                        }
                    }
                }
            }
            2 => {
                // TAB 3: REFERENCES & BRANCHES
                frame.push_str(&format!("  {:<25} {:<42} {}\x1B[K\n", "REF LABEL", "COMMIT HASH", "REF TYPE"));
                frame.push_str(&format!("  {}\x1B[K\n", "━".repeat(width.saturating_sub(4))));
                let adjusted_height = content_height.saturating_sub(2);

                for row_y in 2..adjusted_height {
                    let ref_idx = row_y - 2 + refs_scroll;
                    if ref_idx < refs.len() {
                        let r = &refs[ref_idx];
                        let ref_type = if r.is_tag {
                            "\x1B[1;33mTag\x1B[0m"
                        } else if r.is_remote {
                            "\x1B[1;31mRemote Branch\x1B[0m"
                        } else {
                            "\x1B[1;32mLocal Branch\x1B[0m"
                        };

                        let is_active = r.name == active_ref && !r.is_tag && !r.is_remote;
                        let active_marker = if is_active { " \x1B[1;32m*\x1B[0m" } else { "  " };
                        let select_marker = if ref_idx == refs_scroll { "\x1B[1;32m>\x1B[0m" } else { " " };

                        frame.push_str(&format!(
                            "{}{} {:<25} {:<42} {}\x1B[K\n",
                            select_marker, active_marker, r.name, r.sha, ref_type
                        ));
                    } else {
                        frame.push_str("\x1B[K\n");
                    }
                }
            }
            _ => {
                // TAB 4: REPOSITORY STATS SUMMARY
                let stats_lines = vec![
                    format!("  \x1B[1;36mActive Branch:\x1B[0m      {}", active_ref),
                    format!("  \x1B[1;36mLatest Commit SHA:\x1B[0m  {}", active_sha),
                    format!("  \x1B[1;36mTotal Commits:\x1B[0m      {}", total_commits),
                    format!("  \x1B[1;36mTracked Files:\x1B[0m      {}", total_tracked_files),
                    format!("  \x1B[1;36mTracked Files Size:\x1B[0m {}", format_size(total_tracked_size)),
                    format!("  \x1B[1;36m.git Storage Size:\x1B[0m  {}", format_size(git_dir_size)),
                    "".to_string(),
                    "  \x1B[1;37mTop Contributors (Commit Count):\x1B[0m".to_string(),
                    "  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".to_string(),
                ];

                for row_y in 0..content_height {
                    if row_y < stats_lines.len() {
                        frame.push_str(&stats_lines[row_y]);
                        frame.push_str("\x1B[K\n");
                    } else {
                        let contrib_idx = row_y - stats_lines.len() + summary_scroll;
                        if contrib_idx < top_contributors.len() {
                            let (author, count) = &top_contributors[contrib_idx];
                            frame.push_str(&format!(
                                "    - {:<25} : {} commits",
                                author, count
                            ));
                        }
                        frame.push_str("\x1B[K\n");
                    }
                }
            }
        }

        // 3. Footer
        frame.push_str(&"━".repeat(width));
        frame.push_str("\n");
        frame.push_str(" \x1B[1;30;47m Q / Esc \x1B[0m Quit  | \x1B[1;30;47m 1-4 \x1B[0m Switch tabs  | \x1B[1;30;47m ↑/↓ / PgUp/PgDn \x1B[0m Navigate scroll");
        print!("{}", frame);
        let _ = io::stdout().flush();

        // Target 15 FPS
        let elapsed = last_frame.elapsed();
        let target_dur = Duration::from_millis(66);
        if elapsed < target_dur {
            std::thread::sleep(target_dur - elapsed);
        }
        last_frame = Instant::now();
    }

    // Clean exit: return to normal screen and restore cursor
    print!("\x1B[?1049l\x1B[?25h\x1B[0m");
    let _ = io::stdout().flush();
}

fn strip_ansi_len(s: &str) -> usize {
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

fn format_size(bytes: u64) -> String {
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

fn wrap_text(text: &str, max_width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    for paragraph in text.lines() {
        if paragraph.trim().is_empty() {
            lines.push("".to_string());
            continue;
        }
        let mut current_line = String::new();
        for word in paragraph.split_whitespace() {
            if word.chars().count() > max_width {
                if !current_line.is_empty() {
                    lines.push(current_line);
                    current_line = String::new();
                }
                let mut word_chars = word.chars().peekable();
                while word_chars.peek().is_some() {
                    let chunk: String = word_chars.by_ref().take(max_width).collect();
                    lines.push(chunk);
                }
            } else {
                let space_needed = if current_line.is_empty() { 0 } else { 1 };
                if current_line.chars().count() + space_needed + word.chars().count() > max_width {
                    lines.push(current_line);
                    current_line = word.to_string();
                } else {
                    if !current_line.is_empty() {
                        current_line.push(' ');
                    }
                    current_line.push_str(word);
                }
            }
        }
        if !current_line.is_empty() {
            lines.push(current_line);
        }
    }
    lines
}
