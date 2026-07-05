use crate::{FindItemType, FindOptions};
use std::ffi::OsStr;
use std::fs;
use std::io::{self, BufRead, IsTerminal};
use std::path::Path;
use std::time::SystemTime;
use walkdir::{DirEntry, WalkDir};

pub fn find(paths: Vec<String>, options: FindOptions) {
    // Pre-resolve reference timestamps for --newer / --older
    let newer_time = options.newer.as_deref().map(|p| resolve_mtime(p));
    let older_time = options.older.as_deref().map(|p| resolve_mtime(p));

    let roots = if paths.is_empty() {
        stdin_roots().unwrap_or_else(|| vec![".".to_string()])
    } else {
        paths
    };

    for root in roots {
        find_root(&root, &options, newer_time, older_time);
    }
}

fn resolve_mtime(path: &str) -> SystemTime {
    fs::metadata(path)
        .and_then(|m| m.modified())
        .unwrap_or_else(|_| {
            eprintln!("Warning: cannot read mtime of '{}'; filter ignored.", path);
            SystemTime::UNIX_EPOCH
        })
}

fn stdin_roots() -> Option<Vec<String>> {
    let stdin = io::stdin();
    if stdin.is_terminal() {
        return None;
    }

    let roots: Vec<String> = stdin
        .lock()
        .lines()
        .map_while(Result::ok)
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect();

    if roots.is_empty() { None } else { Some(roots) }
}

fn find_root(root: &str, options: &FindOptions, newer_time: Option<SystemTime>, older_time: Option<SystemTime>) {
    let mut walker = WalkDir::new(root)
        .follow_links(false)
        .min_depth(options.min_depth);

    if let Some(max_depth) = options.max_depth {
        walker = walker.max_depth(max_depth);
    }

    for entry in walker {
        match entry {
            Ok(entry) => {
                if matches_entry(&entry, options, newer_time, older_time) {
                    println!("{}", entry.path().display());
                }
            }
            Err(error) => eprintln!("Error: {}", error),
        }
    }
}

fn matches_entry(
    entry: &DirEntry,
    options: &FindOptions,
    newer_time: Option<SystemTime>,
    older_time: Option<SystemTime>,
) -> bool {
    // Type filter
    if let Some(item_type) = options.item_type {
        match item_type {
            FindItemType::File      if !entry.file_type().is_file() => return false,
            FindItemType::Directory if !entry.file_type().is_dir()  => return false,
            _ => {}
        }
    }

    // Name filter
    if let Some(pattern) = &options.name {
        if !matches_name(entry.file_name(), pattern, false) { return false; }
    }
    if let Some(pattern) = &options.case_insensitive_name {
        if !matches_name(entry.file_name(), pattern, true) { return false; }
    }

    // Empty filter
    if options.empty && !is_empty(entry.path(), entry.file_type().is_dir()) {
        return false;
    }

    // Size filters (files only; directories are skipped for size checks)
    if entry.file_type().is_file() {
        if let Ok(meta) = entry.metadata() {
            let size = meta.len();
            if let Some(min) = options.min_size {
                if size < min { return false; }
            }
            if let Some(max) = options.max_size {
                if size > max { return false; }
            }

            // Time filters
            if let Ok(mtime) = meta.modified() {
                if let Some(ref_time) = newer_time {
                    if mtime <= ref_time { return false; }
                }
                if let Some(ref_time) = older_time {
                    if mtime >= ref_time { return false; }
                }
            }
        }
    }

    true
}

fn matches_name(name: &OsStr, pattern: &str, case_insensitive: bool) -> bool {
    let name = name.to_string_lossy();
    if case_insensitive {
        glob_matches(&name.to_lowercase(), &pattern.to_lowercase())
    } else {
        glob_matches(&name, pattern)
    }
}

fn glob_matches(text: &str, pattern: &str) -> bool {
    let text: Vec<char> = text.chars().collect();
    let pattern: Vec<char> = pattern.chars().collect();
    let mut text_index = 0;
    let mut pattern_index = 0;
    let mut star_index = None;
    let mut star_text_index = 0;

    while text_index < text.len() {
        if pattern_index < pattern.len()
            && (pattern[pattern_index] == '?' || pattern[pattern_index] == text[text_index])
        {
            text_index += 1;
            pattern_index += 1;
        } else if pattern_index < pattern.len() && pattern[pattern_index] == '*' {
            star_index = Some(pattern_index);
            star_text_index = text_index;
            pattern_index += 1;
        } else if let Some(star) = star_index {
            pattern_index = star + 1;
            star_text_index += 1;
            text_index = star_text_index;
        } else {
            return false;
        }
    }

    while pattern_index < pattern.len() && pattern[pattern_index] == '*' {
        pattern_index += 1;
    }

    pattern_index == pattern.len()
}

fn is_empty(path: &Path, is_dir: bool) -> bool {
    if is_dir {
        match fs::read_dir(path) {
            Ok(mut entries) => entries.next().is_none(),
            Err(_) => false,
        }
    } else {
        fs::metadata(path)
            .map(|metadata| metadata.len() == 0)
            .unwrap_or(false)
    }
}

/// Parse a human-friendly size string into bytes.
/// Accepts plain integers or values with K/M/G suffixes (case-insensitive).
pub fn parse_size(s: &str) -> Option<u64> {
    let s = s.trim();
    if let Some(n) = s.strip_suffix(['K', 'k']) {
        n.trim().parse::<u64>().ok().map(|v| v * 1024)
    } else if let Some(n) = s.strip_suffix(['M', 'm']) {
        n.trim().parse::<u64>().ok().map(|v| v * 1024 * 1024)
    } else if let Some(n) = s.strip_suffix(['G', 'g']) {
        n.trim().parse::<u64>().ok().map(|v| v * 1024 * 1024 * 1024)
    } else {
        s.parse::<u64>().ok()
    }
}
