use crate::SortOptions;
use std::io::{self, BufRead, IsTerminal};

pub fn run_sort(paths: Vec<String>, options: SortOptions) {
    let lines = if paths.is_empty() {
        read_stdin()
    } else {
        read_files(&paths)
    };

    let lines = match lines {
        Ok(l) => l,
        Err(e) => { eprintln!("Error: {}", e); return; }
    };

    if options.check {
        check_sorted(&lines, &options);
        return;
    }

    let mut sorted = lines;
    sort_lines(&mut sorted, &options);

    if options.unique {
        sorted.dedup_by(|a, b| lines_equal(a, b, &options));
    }

    for line in &sorted {
        println!("{}", line);
    }
}

// ---------------------------------------------------------------------------
// Reading
// ---------------------------------------------------------------------------

fn read_stdin() -> Result<Vec<String>, String> {
    let stdin = io::stdin();
    if stdin.is_terminal() {
        return Err("No input file specified and stdin is a terminal.".to_string());
    }
    let mut lines: Vec<String> = stdin.lock().lines()
        .map(|l| l.map_err(|e| e.to_string()))
        .collect::<Result<Vec<String>, String>>()?;
    if let Some(first) = lines.first_mut() {
        if first.starts_with('\u{feff}') {
            first.remove(0);
        }
    }
    Ok(lines)
}

fn read_files(paths: &[String]) -> Result<Vec<String>, String> {
    let mut all = Vec::new();
    for path in paths {
        let mut content = std::fs::read_to_string(path)
            .map_err(|e| format!("Cannot read '{}': {}", path, e))?;
        if content.starts_with('\u{feff}') {
            content.remove(0);
        }
        for line in content.lines() {
            all.push(line.to_string());
        }
    }
    Ok(all)
}

// ---------------------------------------------------------------------------
// Sorting
// ---------------------------------------------------------------------------

fn sort_lines(lines: &mut Vec<String>, options: &SortOptions) {
    lines.sort_by(|a, b| compare_lines(a, b, options));
    if options.reverse {
        lines.reverse();
    }
}

fn get_sort_key<'a>(line: &'a str, options: &SortOptions) -> &'a str {
    if options.field == 0 {
        return line;
    }
    let sep = options.separator;
    let tokens: Vec<&str> = if let Some(ch) = sep {
        line.split(ch).collect()
    } else {
        line.split_whitespace().collect()
    };
    // field is 1-indexed
    tokens.get(options.field - 1).copied().unwrap_or("")
}

fn compare_lines(a: &str, b: &str, options: &SortOptions) -> std::cmp::Ordering {
    let ka = get_sort_key(a, options);
    let kb = get_sort_key(b, options);

    if options.numeric {
        let na = ka.trim().parse::<f64>().unwrap_or(f64::NAN);
        let nb = kb.trim().parse::<f64>().unwrap_or(f64::NAN);
        // NaN sorts last
        match (na.is_nan(), nb.is_nan()) {
            (true, true)  => std::cmp::Ordering::Equal,
            (true, false) => std::cmp::Ordering::Greater,
            (false, true) => std::cmp::Ordering::Less,
            _ => na.partial_cmp(&nb).unwrap_or(std::cmp::Ordering::Equal),
        }
    } else if options.ignore_case {
        ka.to_lowercase().cmp(&kb.to_lowercase())
    } else {
        ka.cmp(kb)
    }
}

fn lines_equal(a: &str, b: &str, options: &SortOptions) -> bool {
    matches!(compare_lines(a, b, options), std::cmp::Ordering::Equal)
}

// ---------------------------------------------------------------------------
// Check mode
// ---------------------------------------------------------------------------

fn check_sorted(lines: &[String], options: &SortOptions) {
    for i in 1..lines.len() {
        let ord = compare_lines(&lines[i - 1], &lines[i], options);
        let out_of_order = if options.reverse {
            matches!(ord, std::cmp::Ordering::Less)
        } else {
            matches!(ord, std::cmp::Ordering::Greater)
        };
        if out_of_order {
            eprintln!("sort: input is not sorted (line {}): {}", i + 1, lines[i]);
            std::process::exit(1);
        }
    }
    // Exits 0 implicitly
}
