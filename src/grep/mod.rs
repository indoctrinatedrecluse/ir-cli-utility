use crate::GrepOptions;
use std::io::{self, BufRead};

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "linux")]
fn read_file(path: &str) -> Result<String, String> {
    linux::read_file(path)
}

#[cfg(target_os = "windows")]
fn read_file(path: &str) -> Result<String, String> {
    windows::read_file(path)
}

pub fn grep(pattern: &str, paths: Vec<String>, options: GrepOptions) {
    if paths.is_empty() {
        grep_stdin(pattern, &options);
    } else {
        for path in paths {
            match read_file(&path) {
                Ok(content) => {
                    grep_content(&content, pattern, &options, Some(&path));
                }
                Err(e) => {
                    eprintln!("Error reading '{}': {}", path, e);
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// stdin path — context not supported (streaming)
// ---------------------------------------------------------------------------

fn grep_stdin(pattern: &str, options: &GrepOptions) {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let regex = match compile_pattern(pattern, options) {
        Ok(re) => re,
        Err(e) => { eprintln!("Error: {}", e); return; }
    };

    let mut line_number = 0;
    let mut match_count = 0;
    let use_context = options.before_context > 0 || options.after_context > 0;

    // For stdin context we buffer all lines first then reuse grep_lines_with_context
    if use_context && !options.count && !options.list {
        let all_lines: Vec<String> = reader.lines().map_while(Result::ok).collect();
        let content = all_lines.join("\n");
        grep_content(&content, pattern, options, None);
        return;
    }

    for line in reader.lines() {
        line_number += 1;
        if let Ok(line) = line {
            if should_match(&line, &regex, options) {
                match_count += 1;
                if !options.count {
                    print_match(&line, line_number, None, options, '-');
                }
            }
        }
    }

    if options.count {
        println!("{}", match_count);
    }
}

// ---------------------------------------------------------------------------
// file content path — supports context
// ---------------------------------------------------------------------------

fn grep_content(content: &str, pattern: &str, options: &GrepOptions, file_path: Option<&str>) {
    let regex = match compile_pattern(pattern, options) {
        Ok(re) => re,
        Err(e) => { eprintln!("Error: {}", e); return; }
    };

    let lines: Vec<&str> = content.lines().collect();
    let n = lines.len();
    let use_context = options.before_context > 0 || options.after_context > 0;

    // Gather match indices
    let match_indices: Vec<usize> = (0..n)
        .filter(|&i| should_match(lines[i], &regex, options))
        .collect();

    if options.list {
        if !match_indices.is_empty() {
            println!("{}", file_path.unwrap_or("(stdin)"));
        }
        return;
    }

    if options.count {
        let count = match_indices.len();
        if let Some(path) = file_path {
            println!("{}:{}", path, count);
        } else {
            println!("{}", count);
        }
        return;
    }

    if !use_context {
        // Simple path — no context
        for (i, line) in lines.iter().enumerate() {
            if should_match(line, &regex, options) {
                print_match(line, i + 1, file_path, options, ':');
            }
        }
        return;
    }

    // Context path — compute intervals to print
    // Each match expands to [match - before_context, match + after_context]
    // Merge overlapping intervals, separate non-adjacent groups with "--"
    let before = options.before_context;
    let after  = options.after_context;

    // Sorted list of (start, end) in line indices (0-based, inclusive)
    let mut intervals: Vec<(usize, usize)> = match_indices.iter().map(|&mi| {
        let s = mi.saturating_sub(before);
        let e = (mi + after).min(n.saturating_sub(1));
        (s, e)
    }).collect();

    if intervals.is_empty() {
        return;
    }

    // Merge overlapping/adjacent intervals
    let mut merged: Vec<(usize, usize)> = Vec::new();
    for (s, e) in intervals.drain(..) {
        if let Some(last) = merged.last_mut() {
            if s <= last.1 + 1 {
                last.1 = last.1.max(e);
                continue;
            }
        }
        merged.push((s, e));
    }

    // Build a set of matching line indices for separator choice
    let match_set: std::collections::HashSet<usize> = match_indices.into_iter().collect();

    let mut first_group = true;
    for (start, end) in merged {
        if !first_group {
            println!("--");
        }
        first_group = false;
        for i in start..=end {
            let sep = if match_set.contains(&i) { ':' } else { '-' };
            print_match(lines[i], i + 1, file_path, options, sep);
        }
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn compile_pattern(pattern: &str, options: &GrepOptions) -> Result<Regex, String> {
    if options.fixed_string {
        Ok(Regex::Literal(pattern.to_string()))
    } else if options.extended_regex {
        Regex::Extended(pattern.to_string()).validate()
    } else {
        Regex::Basic(pattern.to_string()).validate()
    }
}

fn should_match(line: &str, regex: &Regex, options: &GrepOptions) -> bool {
    let is_match = match regex {
        Regex::Literal(literal) => {
            if options.case_insensitive {
                line.to_lowercase().contains(&literal.to_lowercase())
            } else {
                line.contains(literal)
            }
        }
        Regex::Basic(pat) | Regex::Extended(pat) => {
            if options.case_insensitive {
                simple_regex_match(&line.to_lowercase(), &pat.to_lowercase(), options.entire_line)
            } else {
                simple_regex_match(line, pat, options.entire_line)
            }
        }
    };

    if options.invert_match { !is_match } else { is_match }
}

fn simple_regex_match(line: &str, pattern: &str, entire_line: bool) -> bool {
    if entire_line {
        line == pattern || line == format!("^{}$", pattern)
    } else if pattern.contains('*') {
        simple_glob_match(line, pattern)
    } else if pattern.starts_with('^') {
        line.starts_with(&pattern[1..])
    } else if pattern.ends_with('$') {
        line.ends_with(&pattern[..pattern.len() - 1])
    } else {
        line.contains(pattern)
    }
}

fn simple_glob_match(text: &str, pattern: &str) -> bool {
    let pattern_chars: Vec<char> = pattern.chars().collect();
    let text_chars: Vec<char> = text.chars().collect();

    let mut t = 0;
    let mut p = 0;
    let mut star_idx = None;
    let mut match_idx = 0;

    while t < text_chars.len() {
        if p < pattern_chars.len() && (pattern_chars[p] == text_chars[t] || pattern_chars[p] == '.') {
            t += 1; p += 1;
        } else if p < pattern_chars.len() && pattern_chars[p] == '*' {
            star_idx = Some(p); match_idx = t; p += 1;
        } else if let Some(idx) = star_idx {
            p = idx + 1; match_idx += 1; t = match_idx;
        } else {
            return false;
        }
    }

    while p < pattern_chars.len() && pattern_chars[p] == '*' { p += 1; }
    p == pattern_chars.len()
}

enum Regex {
    Literal(String),
    Basic(String),
    Extended(String),
}

impl Regex {
    fn validate(self) -> Result<Regex, String> { Ok(self) }
}

/// Print a single matching (or context) line.
/// `sep` is ':' for match lines, '-' for context lines (grep convention).
fn print_match(line: &str, line_number: usize, file_path: Option<&str>, options: &GrepOptions, sep: char) {
    let file_prefix = match file_path {
        Some(path) => format!("{}{}", path, sep),
        None       => String::new(),
    };

    if options.line_numbers {
        println!("{}{}{}{}", file_prefix, line_number, sep, line);
    } else {
        println!("{}{}", file_prefix, line);
    }
}
