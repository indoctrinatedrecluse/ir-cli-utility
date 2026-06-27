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
    // If no paths provided, read from stdin
    if paths.is_empty() {
        grep_stdin(pattern, &options);
    } else {
        // Process each file
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

fn grep_stdin(pattern: &str, options: &GrepOptions) {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let regex = match compile_pattern(pattern, options) {
        Ok(re) => re,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    let mut line_number = 0;
    let mut match_count = 0;

    for line in reader.lines() {
        line_number += 1;
        if let Ok(line) = line {
            if should_match(&line, &regex, options) {
                match_count += 1;
                if !options.count {
                    print_match(&line, line_number, None, options);
                }
            }
        }
    }

    if options.count {
        println!("{}", match_count);
    }
}

fn grep_content(content: &str, pattern: &str, options: &GrepOptions, file_path: Option<&str>) {
    let regex = match compile_pattern(pattern, options) {
        Ok(re) => re,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    let mut line_number = 0;
    let mut match_count = 0;
    let mut has_match = false;

    for line in content.lines() {
        line_number += 1;
        if should_match(line, &regex, options) {
            has_match = true;
            match_count += 1;
            if !options.count && !options.list {
                print_match(line, line_number, file_path, options);
            }
        }
    }

    if options.list && has_match {
        println!("{}", file_path.unwrap_or("(stdin)"));
    } else if options.count {
        if let Some(path) = file_path {
            println!("{}:{}", path, match_count);
        } else {
            println!("{}", match_count);
        }
    }
}

fn compile_pattern(pattern: &str, options: &GrepOptions) -> Result<Regex, String> {
    if options.fixed_string {
        // Escape special regex characters for literal matching
        Ok(Regex::Literal(regex_escape(pattern)))
    } else if options.extended_regex {
        // Use the pattern as-is for extended regex
        Regex::Extended(pattern.to_string()).validate()
    } else {
        // Basic regex support
        Regex::Basic(pattern.to_string()).validate()
    }
}

fn regex_escape(s: &str) -> String {
    s.chars()
        .flat_map(|c| {
            match c {
                '.' | '*' | '+' | '?' | '^' | '$' | '(' | ')' | '[' | ']' | '{' | '}' | '|'
                | '\\' => {
                    vec!['\\', c]
                }
                _ => vec![c],
            }
        })
        .collect()
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

    if options.invert_match {
        !is_match
    } else {
        is_match
    }
}

fn simple_regex_match(line: &str, pattern: &str, entire_line: bool) -> bool {
    if entire_line {
        // Match entire line exactly
        line == pattern || line == format!("^{}$", pattern)
    } else {
        // Simple substring and basic regex support
        if pattern.contains('*') {
            // Very basic * support (0 or more of previous char)
            simple_glob_match(line, pattern)
        } else if pattern.contains('^') && pattern.starts_with('^') {
            line.starts_with(&pattern[1..])
        } else if pattern.contains('$') && pattern.ends_with('$') {
            line.ends_with(&pattern[..pattern.len() - 1])
        } else {
            line.contains(pattern)
        }
    }
}

fn simple_glob_match(text: &str, pattern: &str) -> bool {
    // Basic glob-like matching for simple patterns
    let pattern_chars: Vec<char> = pattern.chars().collect();
    let text_chars: Vec<char> = text.chars().collect();

    let mut t = 0;
    let mut p = 0;
    let mut star_idx = None;
    let mut match_idx = 0;

    while t < text_chars.len() {
        if p < pattern_chars.len() && (pattern_chars[p] == text_chars[t] || pattern_chars[p] == '.') {
            t += 1;
            p += 1;
        } else if p < pattern_chars.len() && pattern_chars[p] == '*' {
            star_idx = Some(p);
            match_idx = t;
            p += 1;
        } else if let Some(idx) = star_idx {
            p = idx + 1;
            match_idx += 1;
            t = match_idx;
        } else {
            return false;
        }
    }

    while p < pattern_chars.len() && pattern_chars[p] == '*' {
        p += 1;
    }

    p == pattern_chars.len()
}

enum Regex {
    Literal(String),
    Basic(String),
    Extended(String),
}

impl Regex {
    fn validate(self) -> Result<Regex, String> {
        Ok(self)
    }
}

fn print_match(line: &str, line_number: usize, file_path: Option<&str>, options: &GrepOptions) {
    let prefix = if let Some(path) = file_path {
        format!("{}:", path)
    } else {
        String::new()
    };

    if options.line_numbers {
        if prefix.is_empty() {
            println!("{}:{}", line_number, line);
        } else {
            println!("{}{}:{}", prefix, line_number, line);
        }
    } else {
        if prefix.is_empty() {
            println!("{}", line);
        } else {
            println!("{}{}", prefix, line);
        }
    }
}

