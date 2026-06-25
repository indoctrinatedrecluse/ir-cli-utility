use crate::CatOptions;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "linux")]
fn read_file(path: &str) -> Result<Vec<u8>, String> {
    linux::read_file(path)
}

#[cfg(target_os = "windows")]
fn read_file(path: &str) -> Result<Vec<u8>, String> {
    windows::read_file(path)
}

pub fn cat(path: &str, options: CatOptions) {
    let bytes = match read_file(path) {
        Ok(bytes) => bytes,
        Err(message) => {
            eprintln!("Error reading '{}': {}", path, message);
            return;
        }
    };

    if options.binary {
        print_binary(&bytes);
        return;
    }

    let text = match decode_text(&bytes, options.encoding.as_deref()) {
        Ok(text) => text,
        Err(message) => {
            eprintln!("Error decoding '{}': {}", path, message);
            return;
        }
    };

    print_text(&text, &options);
}

fn decode_text(bytes: &[u8], encoding: Option<&str>) -> Result<String, String> {
    match encoding.unwrap_or("utf-8").to_ascii_lowercase().as_str() {
        "utf-8" | "utf8" => String::from_utf8(bytes.to_vec())
            .map_err(|_| "file is not valid UTF-8; use --binary or --encoding utf-16".to_string()),
        "ascii" => {
            if bytes.iter().any(|byte| *byte > 0x7f) {
                Err("file contains non-ASCII bytes; use --binary or another encoding".to_string())
            } else {
                Ok(bytes.iter().map(|byte| *byte as char).collect())
            }
        }
        "utf-16" | "utf16" => decode_utf16(bytes),
        other => Err(format!(
            "unsupported encoding '{}'; supported encodings: utf-8, utf-16, ascii",
            other
        )),
    }
}

fn decode_utf16(bytes: &[u8]) -> Result<String, String> {
    if bytes.len() % 2 != 0 {
        return Err("UTF-16 input has an odd number of bytes".to_string());
    }

    let (start, little_endian) = if bytes.starts_with(&[0xff, 0xfe]) {
        (2, true)
    } else if bytes.starts_with(&[0xfe, 0xff]) {
        (2, false)
    } else {
        (0, cfg!(target_endian = "little"))
    };

    let words = bytes[start..].chunks_exact(2).map(|chunk| {
        if little_endian {
            u16::from_le_bytes([chunk[0], chunk[1]])
        } else {
            u16::from_be_bytes([chunk[0], chunk[1]])
        }
    });

    String::from_utf16(&words.collect::<Vec<u16>>())
        .map_err(|_| "file is not valid UTF-16".to_string())
}

fn print_text(text: &str, options: &CatOptions) {
    if !options.line_numbers
        && options.head.is_none()
        && options.tail.is_none()
        && options.range.is_none()
    {
        print!("{}", text);
        return;
    }

    let lines: Vec<&str> = text.lines().collect();
    let total = lines.len();
    let mut start = 0usize;
    let mut end = total;

    if let Some(count) = options.head {
        end = end.min(count);
    }

    if let Some(count) = options.tail {
        start = start.max(total.saturating_sub(count));
    }

    if let Some((range_start, range_end)) = options.range {
        start = start.max(range_start.saturating_sub(1));
        end = end.min(range_end.min(total));
    }

    if start > end {
        return;
    }

    for (index, line) in lines.iter().enumerate().take(end).skip(start) {
        if options.line_numbers {
            println!("{:>6}\t{}", index + 1, line);
        } else {
            println!("{}", line);
        }
    }
}

fn print_binary(bytes: &[u8]) {
    for (offset, chunk) in bytes.chunks(16).enumerate() {
        print!("{:08x}  ", offset * 16);

        for index in 0..16 {
            if let Some(byte) = chunk.get(index) {
                print!("{:02x} ", byte);
            } else {
                print!("   ");
            }

            if index == 7 {
                print!(" ");
            }
        }

        print!(" |");
        for byte in chunk {
            let character = if byte.is_ascii_graphic() || *byte == b' ' {
                *byte as char
            } else {
                '.'
            };
            print!("{}", character);
        }
        println!("|");
    }
}
