use crate::TextOptions;
use std::fs::File;
use std::io::{self, IsTerminal, Read, Write};

fn split_words(s: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut current = String::new();
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        if c == '_' || c == '-' || c.is_whitespace() {
            if !current.is_empty() {
                words.push(current.clone());
                current.clear();
            }
        } else if c.is_uppercase() {
            let prev_is_lowercase = i > 0 && chars[i - 1].is_lowercase();
            let next_is_lowercase = i + 1 < chars.len() && chars[i + 1].is_lowercase();
            if (prev_is_lowercase || next_is_lowercase) && !current.is_empty() {
                words.push(current.clone());
                current.clear();
            }
            current.push(c);
        } else {
            current.push(c);
        }
        i += 1;
    }
    if !current.is_empty() {
        words.push(current);
    }
    words.into_iter().filter(|w| !w.is_empty()).collect()
}

fn convert_case(s: &str, case_type: &str) -> String {
    let words = split_words(s);
    if words.is_empty() {
        return s.to_string();
    }
    match case_type.to_lowercase().as_str() {
        "camel" => {
            let mut out = words[0].to_lowercase();
            for word in words.iter().skip(1) {
                let mut chars = word.chars();
                if let Some(first) = chars.next() {
                    out.push(first.to_ascii_uppercase());
                    out.push_str(&chars.collect::<String>().to_lowercase());
                }
            }
            out
        }
        "snake" => {
            words.iter().map(|w| w.to_lowercase()).collect::<Vec<String>>().join("_")
        }
        "pascal" => {
            let mut out = String::new();
            for word in &words {
                let mut chars = word.chars();
                if let Some(first) = chars.next() {
                    out.push(first.to_ascii_uppercase());
                    out.push_str(&chars.collect::<String>().to_lowercase());
                }
            }
            out
        }
        "kebab" => {
            words.iter().map(|w| w.to_lowercase()).collect::<Vec<String>>().join("-")
        }
        "upper" => {
            words.iter().map(|w| w.to_uppercase()).collect::<Vec<String>>().join("_")
        }
        "lower" => {
            s.to_lowercase()
        }
        "title" => {
            words.iter().map(|w| {
                let mut chars = w.chars();
                if let Some(first) = chars.next() {
                    format!("{}{}", first.to_ascii_uppercase(), chars.collect::<String>().to_lowercase())
                } else {
                    String::new()
                }
            }).collect::<Vec<String>>().join(" ")
        }
        "sentence" => {
            let mut out = String::new();
            let joined = words.iter().map(|w| w.to_lowercase()).collect::<Vec<String>>().join(" ");
            let mut chars = joined.chars();
            if let Some(first) = chars.next() {
                out.push(first.to_ascii_uppercase());
                out.push_str(&chars.collect::<String>());
            }
            out
        }
        "slug" => {
            let sanitized: Vec<String> = words.iter().map(|w| {
                w.chars().filter(|c| c.is_alphanumeric()).collect::<String>().to_lowercase()
            }).filter(|s| !s.is_empty()).collect();
            sanitized.join("-")
        }
        _ => s.to_string(),
    }
}

fn strip_ansi(s: &str) -> String {
    let mut out = String::new();
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\x1B' {
            if chars.peek() == Some(&'[') {
                chars.next();
                while let Some(&nc) = chars.peek() {
                    chars.next();
                    if nc.is_ascii_alphabetic() {
                        break;
                    }
                }
            }
        } else {
            out.push(c);
        }
    }
    out
}

fn strip_non_alphanumeric(s: &str) -> String {
    s.chars().filter(|c| c.is_alphanumeric() || c.is_whitespace()).collect()
}

pub fn run_text(input_path: Option<&str>, options: TextOptions) {
    let mut input_bytes = Vec::new();
    if let Some(path) = input_path {
        let mut file = match File::open(path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Error: Failed to open file '{}': {}", path, e);
                std::process::exit(1);
            }
        };
        if let Err(e) = file.read_to_end(&mut input_bytes) {
            eprintln!("Error: Failed to read file '{}': {}", path, e);
            std::process::exit(1);
        }
    } else {
        // Read from stdin
        let mut stdin = io::stdin();
        if stdin.is_terminal() {
            eprintln!("Error: No input file specified and stdin is a terminal.");
            std::process::exit(1);
        }
        if let Err(e) = stdin.read_to_end(&mut input_bytes) {
            eprintln!("Error: Failed to read standard input: {}", e);
            std::process::exit(1);
        }
    }

    // Strip BOM if present
    let bytes_to_parse = if input_bytes.starts_with(&[0xEF, 0xBB, 0xBF]) {
        &input_bytes[3..]
    } else {
        &input_bytes
    };

    let text_content = String::from_utf8_lossy(bytes_to_parse);
    let mut output_lines = Vec::new();

    let width = options.width.unwrap_or(80);
    let ellipsis = options.ellipsis.clone().unwrap_or_else(|| "...".to_string());

    for line in text_content.lines() {
        let mut processed = line.to_string();

        if options.strip_ansi {
            processed = strip_ansi(&processed);
        }

        if options.strip_non_alphanumeric {
            processed = strip_non_alphanumeric(&processed);
        }

        if let Some(ref case) = options.case {
            processed = convert_case(&processed, case);
        }

        if options.truncate {
            let count = processed.chars().count();
            if count > width {
                let sub_width = width.saturating_sub(ellipsis.chars().count());
                processed = processed.chars().take(sub_width).collect::<String>() + &ellipsis;
            }
        }

        if let Some(ref align) = options.align {
            let count = processed.chars().count();
            if count < width {
                let pad = width - count;
                processed = match align.to_lowercase().as_str() {
                    "left" => {
                        format!("{}{}", processed, " ".repeat(pad))
                    }
                    "right" => {
                        format!("{}{}", " ".repeat(pad), processed)
                    }
                    "center" => {
                        let left = pad / 2;
                        let right = pad - left;
                        format!("{}{}{}", " ".repeat(left), processed, " ".repeat(right))
                    }
                    _ => {
                        eprintln!("Error: Invalid align option '{}'. Supported: left, right, center", align);
                        std::process::exit(1);
                    }
                };
            }
        }

        output_lines.push(processed);
    }

    let final_output = output_lines.join("\n");

    if let Some(ref out_path) = options.output {
        let mut file = match File::create(out_path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Error: Failed to create output file '{}': {}", out_path, e);
                std::process::exit(1);
            }
        };
        if let Err(e) = file.write_all(final_output.as_bytes()) {
            eprintln!("Error: Failed to write to output file '{}': {}", out_path, e);
            std::process::exit(1);
        }
    } else {
        println!("{}", final_output);
    }
}
