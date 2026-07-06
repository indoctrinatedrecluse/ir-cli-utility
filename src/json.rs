use crate::JsonOptions;
use serde::Serialize;
use std::fs::File;
use std::io::{Read, Write};

pub fn run_json(input_path: Option<&str>, options: JsonOptions) {
    let input_bytes = read_input(input_path);

    // Parse JSON
    let mut value: serde_json::Value = match serde_json::from_slice(&input_bytes) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: JSON parse error at line {}, column {}: {}", e.line(), e.column(), e);
            std::process::exit(1);
        }
    };

    // Apply query if provided
    if let Some(ref q) = options.query {
        match query_json(&value, q) {
            Ok(v) => value = v,
            Err(e) => {
                eprintln!("Error: JSON query failed: {}", e);
                std::process::exit(1);
            }
        }
    }

    // Format output
    let output_bytes = if options.minify {
        match serde_json::to_string(&value) {
            Ok(s) => s.into_bytes(),
            Err(e) => {
                eprintln!("Error: Failed to serialize JSON: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        // Pretty print with custom indent
        let mut writer = Vec::new();
        let indent_bytes = vec![b' '; options.indent];
        let formatter = serde_json::ser::PrettyFormatter::with_indent(&indent_bytes);
        let mut serializer = serde_json::Serializer::with_formatter(&mut writer, formatter);
        if let Err(e) = value.serialize(&mut serializer) {
            eprintln!("Error: Failed to serialize JSON: {}", e);
            std::process::exit(1);
        }
        writer
    };

    // Write output
    write_output(&output_bytes, options.output.as_deref(), !options.minify);
}

fn read_input(input_path: Option<&str>) -> Vec<u8> {
    if let Some(path) = input_path {
        let mut file = match File::open(path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Error: Failed to open input file '{}': {}", path, e);
                std::process::exit(1);
            }
        };
        let mut buf = Vec::new();
        if let Err(e) = file.read_to_end(&mut buf) {
            eprintln!("Error: Failed to read input file: {}", e);
            std::process::exit(1);
        }
        buf
    } else {
        let mut buf = Vec::new();
        if let Err(e) = std::io::stdin().read_to_end(&mut buf) {
            eprintln!("Error: Failed to read from stdin: {}", e);
            std::process::exit(1);
        }
        buf
    }
}

fn write_output(bytes: &[u8], output_path: Option<&str>, add_newline: bool) {
    if let Some(path) = output_path {
        let mut file = match File::create(path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Error: Failed to create output file '{}': {}", path, e);
                std::process::exit(1);
            }
        };
        if let Err(e) = file.write_all(bytes) {
            eprintln!("Error: Failed to write to output file: {}", e);
            std::process::exit(1);
        }
        if add_newline {
            if let Err(e) = file.write_all(b"\n") {
                eprintln!("Error: Failed to write to output file: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        let mut stdout = std::io::stdout();
        if let Err(e) = stdout.write_all(bytes) {
            eprintln!("Error: Failed to write to stdout: {}", e);
            std::process::exit(1);
        }
        if add_newline {
            println!();
        }
    }
}

fn query_json(value: &serde_json::Value, query: &str) -> Result<serde_json::Value, String> {
    let mut q = query.trim();
    if q.is_empty() || q == "." {
        return Ok(value.clone());
    }

    // Strip leading dot if present
    if q.starts_with('.') {
        q = &q[1..];
    }

    let mut current = value;
    let mut parts = Vec::new();
    let mut temp = String::new();
    let chars: Vec<char> = q.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        match chars[i] {
            '.' => {
                if !temp.is_empty() {
                    parts.push(temp.clone());
                    temp.clear();
                }
                i += 1;
            }
            '[' => {
                if !temp.is_empty() {
                    parts.push(temp.clone());
                    temp.clear();
                }
                i += 1;
                let mut idx_str = String::new();
                while i < chars.len() && chars[i] != ']' {
                    idx_str.push(chars[i]);
                    i += 1;
                }
                if i < chars.len() && chars[i] == ']' {
                    parts.push(format!("[{}]", idx_str.trim()));
                } else {
                    return Err("Unclosed index bracket '['".to_string());
                }
                i += 1;
            }
            c => {
                temp.push(c);
                i += 1;
            }
        }
    }
    if !temp.is_empty() {
        parts.push(temp);
    }

    for part in parts {
        if part.starts_with('[') && part.ends_with(']') {
            let idx_str = &part[1..part.len() - 1];
            if let Ok(idx) = idx_str.parse::<usize>() {
                if let Some(arr) = current.as_array() {
                    if idx < arr.len() {
                        current = &arr[idx];
                    } else {
                        return Err(format!("Index {} out of bounds (array length {})", idx, arr.len()));
                    }
                } else {
                    return Err(format!("Cannot index non-array value with [{}]", idx));
                }
            } else {
                // Object key index with quotes, e.g. ["some key"] or ['some key']
                let key = if (idx_str.starts_with('"') && idx_str.ends_with('"'))
                    || (idx_str.starts_with('\'') && idx_str.ends_with('\''))
                {
                    &idx_str[1..idx_str.len() - 1]
                } else {
                    idx_str
                };
                if let Some(obj) = current.as_object() {
                    if let Some(val) = obj.get(key) {
                        current = val;
                    } else {
                        return Err(format!("Key '{}' not found in object", key));
                    }
                } else {
                    return Err(format!("Cannot look up key '{}' in non-object value", key));
                }
            }
        } else {
            if let Some(obj) = current.as_object() {
                if let Some(val) = obj.get(&part) {
                    current = val;
                } else {
                    return Err(format!("Key '{}' not found in object", part));
                }
            } else {
                return Err(format!("Cannot look up key '{}' in non-object value", part));
            }
        }
    }

    Ok(current.clone())
}
