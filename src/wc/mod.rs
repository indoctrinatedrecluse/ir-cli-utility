use crate::WcOptions;
use std::fs::File;
use std::io::{self, BufReader, Read};
use std::path::Path;

pub fn wc(paths: Vec<String>, options: WcOptions) {
    let mut opts = options;
    if !opts.lines && !opts.words && !opts.bytes && !opts.chars {
        opts.lines = true;
        opts.words = true;
        opts.bytes = true;
    }

    let mut total_lines = 0;
    let mut total_words = 0;
    let mut total_chars = 0;
    let mut total_bytes = 0;

    let show_total = paths.len() > 1;

    if paths.is_empty() {
        match count_reader(io::stdin(), &opts) {
            Ok((l, w, c, b)) => {
                print_counts(l, w, c, b, "", &opts);
            }
            Err(e) => {
                eprintln!("Error reading stdin: {}", e);
            }
        }
        return;
    }

    for path_str in &paths {
        if path_str == "-" {
            match count_reader(io::stdin(), &opts) {
                Ok((l, w, c, b)) => {
                    print_counts(l, w, c, b, "-", &opts);
                    total_lines += l;
                    total_words += w;
                    total_chars += c;
                    total_bytes += b;
                }
                Err(e) => {
                    eprintln!("Error reading stdin: {}", e);
                }
            }
        } else {
            let path = Path::new(path_str);
            match File::open(path) {
                Ok(file) => {
                    match count_reader(BufReader::new(file), &opts) {
                        Ok((l, w, c, b)) => {
                            print_counts(l, w, c, b, path_str, &opts);
                            total_lines += l;
                            total_words += w;
                            total_chars += c;
                            total_bytes += b;
                        }
                        Err(e) => {
                            eprintln!("Error reading {}: {}", path_str, e);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error: Cannot open {}: {}", path_str, e);
                }
            }
        }
    }

    if show_total {
        print_counts(total_lines, total_words, total_chars, total_bytes, "total", &opts);
    }
}

fn count_reader<R: Read>(mut reader: R, opts: &WcOptions) -> io::Result<(usize, usize, usize, usize)> {
    let mut buffer = [0u8; 16384];
    let mut lines = 0;
    let mut words = 0;
    let mut chars = 0;
    let mut bytes = 0;

    let mut in_word = false;

    loop {
        let n = reader.read(&mut buffer)?;
        if n == 0 {
            break;
        }

        bytes += n;

        for &b in &buffer[..n] {
            if b == b'\n' {
                lines += 1;
            }

            if opts.chars && (b & 0xC0) != 0x80 {
                chars += 1;
            }

            let is_space = b == b' ' || b == b'\t' || b == b'\n' || b == b'\r' || b == b'\x0b' || b == b'\x0c';
            if is_space {
                in_word = false;
            } else if !in_word {
                in_word = true;
                words += 1;
            }
        }
    }

    Ok((lines, words, chars, bytes))
}

fn print_counts(lines: usize, words: usize, chars: usize, bytes: usize, label: &str, opts: &WcOptions) {
    let mut parts = Vec::new();
    if opts.lines {
        parts.push(format!("{:>7}", lines));
    }
    if opts.words {
        parts.push(format!("{:>7}", words));
    }
    if opts.chars {
        parts.push(format!("{:>7}", chars));
    }
    if opts.bytes {
        parts.push(format!("{:>7}", bytes));
    }

    if label.is_empty() {
        println!("{}", parts.join(" "));
    } else {
        println!("{} {}", parts.join(" "), label);
    }
}
