use crate::SearchOptions;
use std::ffi::OsStr;
use std::fs;
use std::io::{self, BufRead, IsTerminal};
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

const SKIPPED_EXTENSIONS: &[&str] = &[
    "7z", "a", "ar", "bin", "bz2", "class", "dll", "dmg", "doc", "docx", "exe", "gz",
    "iso", "jar", "jpg", "jpeg", "lib", "o", "obj", "pdf", "png", "pyc", "rar", "so",
    "tar", "tgz", "wasm", "webp", "xls", "xlsx", "zip",
];

pub fn search(phrase: &str, paths: Vec<String>, options: SearchOptions) {
    let roots = if paths.is_empty() {
        stdin_roots().unwrap_or_else(|| vec![".".to_string()])
    } else {
        paths
    };

    for root in roots {
        search_root(phrase, &root, &options);
    }
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

    if roots.is_empty() {
        None
    } else {
        Some(roots)
    }
}

fn search_root(phrase: &str, root: &str, options: &SearchOptions) {
    let mut walker = WalkDir::new(root)
        .follow_links(false)
        .min_depth(options.min_depth);

    if let Some(max_depth) = options.max_depth {
        walker = walker.max_depth(max_depth);
    }

    for entry in walker {
        match entry {
            Ok(entry) => {
                if entry.file_type().is_file() && should_search(&entry, options) {
                    search_file(phrase, entry.path(), options);
                }
            }
            Err(error) => eprintln!("Error: {}", error),
        }
    }
}

fn should_search(entry: &DirEntry, options: &SearchOptions) -> bool {
    if let Some(pattern) = &options.name {
        if !matches_name(entry.file_name(), pattern, false) {
            return false;
        }
    }

    if let Some(pattern) = &options.case_insensitive_name {
        if !matches_name(entry.file_name(), pattern, true) {
            return false;
        }
    }

    let ext = entry
        .path()
        .extension()
        .and_then(OsStr::to_str)
        .map(|ext| ext.to_ascii_lowercase());

    if !options.include_extensions.is_empty() {
        return ext
            .as_ref()
            .map(|ext| options.include_extensions.iter().any(|allowed| allowed == ext))
            .unwrap_or(false);
    }

    if ext
        .as_ref()
        .map(|ext| options.exclude_extensions.iter().any(|blocked| blocked == ext))
        .unwrap_or(false)
    {
        return false;
    }

    if !options.include_skipped
        && ext
            .as_ref()
            .map(|ext| SKIPPED_EXTENSIONS.contains(&ext.as_str()))
            .unwrap_or(false)
    {
        return false;
    }

    true
}

fn search_file(phrase: &str, path: &Path, options: &SearchOptions) {
    let bytes = match fs::read(path) {
        Ok(bytes) => bytes,
        Err(error) => {
            eprintln!("Error reading '{}': {}", path.display(), error);
            return;
        }
    };

    if !options.include_skipped && looks_binary(&bytes) {
        return;
    }

    let content = match String::from_utf8(bytes) {
        Ok(content) => content,
        Err(_) => return,
    };

    let phrase_cmp = if options.case_insensitive {
        phrase.to_lowercase()
    } else {
        phrase.to_string()
    };
    let mut match_count = 0;

    for (line_index, line) in content.lines().enumerate() {
        let line_cmp = if options.case_insensitive {
            line.to_lowercase()
        } else {
            line.to_string()
        };

        if line_cmp.contains(&phrase_cmp) {
            match_count += 1;

            if options.count || options.files_with_matches {
                continue;
            }

            if options.line_numbers {
                println!("{}:{}:{}", path.display(), line_index + 1, line);
            } else {
                println!("{}:{}", path.display(), line);
            }
        }
    }

    if options.files_with_matches && match_count > 0 {
        println!("{}", path.display());
    } else if options.count && match_count > 0 {
        println!("{}:{}", path.display(), match_count);
    }
}

fn looks_binary(bytes: &[u8]) -> bool {
    bytes.iter().take(1024).any(|byte| *byte == 0)
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
