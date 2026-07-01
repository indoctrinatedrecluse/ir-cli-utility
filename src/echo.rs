use crate::EchoOptions;
use std::fs::OpenOptions;
use std::io::Write;

fn parse_escapes(input: &str) -> String {
    let mut output = String::new();
    let mut chars = input.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.peek() {
                Some(&'n') => {
                    chars.next();
                    output.push('\n');
                }
                Some(&'t') => {
                    chars.next();
                    output.push('\t');
                }
                Some(&'r') => {
                    chars.next();
                    output.push('\r');
                }
                Some(&'\\') => {
                    chars.next();
                    output.push('\\');
                }
                Some(&'x') => {
                    chars.next(); // Consume 'x'
                    let mut hex_str = String::new();
                    if let Some(&h1) = chars.peek() {
                        if h1.is_ascii_hexdigit() {
                            hex_str.push(h1);
                            chars.next();
                            if let Some(&h2) = chars.peek() {
                                if h2.is_ascii_hexdigit() {
                                    hex_str.push(h2);
                                    chars.next();
                                }
                            }
                        }
                    }
                    if hex_str.len() == 2 {
                        if let Ok(val) = u8::from_str_radix(&hex_str, 16) {
                            output.push(val as char);
                        } else {
                            output.push_str("\\x");
                            output.push_str(&hex_str);
                        }
                    } else {
                        output.push_str("\\x");
                        output.push_str(&hex_str);
                    }
                }
                _ => {
                    output.push('\\');
                }
            }
        } else {
            output.push(c);
        }
    }
    output
}

pub fn run_echo(args: Vec<String>, options: EchoOptions) {
    let mut content_args = Vec::new();
    let mut output_file: Option<String> = None;
    let mut append_mode = false;
    
    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        if arg == ">" {
            match iter.next() {
                Some(file) => {
                    output_file = Some(file);
                    append_mode = false;
                    break; // Ignore anything after redirection
                }
                None => {
                    eprintln!("Error: Redirection '>' requires a destination file path.");
                    std::process::exit(1);
                }
            }
        } else if arg == ">>" {
            match iter.next() {
                Some(file) => {
                    output_file = Some(file);
                    append_mode = true;
                    break;
                }
                None => {
                    eprintln!("Error: Redirection '>>' requires a destination file path.");
                    std::process::exit(1);
                }
            }
        } else {
            content_args.push(arg);
        }
    }

    let joined = content_args.join(" ");
    let final_message = if options.escapes {
        parse_escapes(&joined)
    } else {
        joined
    };

    if let Some(ref path) = output_file {
        let mut file = match OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(!append_mode)
            .append(append_mode)
            .open(path)
        {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Error: Failed to open or create file '{}': {}", path, e);
                std::process::exit(1);
            }
        };

        if let Err(e) = file.write_all(final_message.as_bytes()) {
            eprintln!("Error: Failed to write to file '{}': {}", path, e);
            std::process::exit(1);
        }
        
        if !options.no_newline {
            let newline = if cfg!(windows) { "\r\n" } else { "\n" };
            if let Err(e) = file.write_all(newline.as_bytes()) {
                eprintln!("Error: Failed to write newline to file '{}': {}", path, e);
                std::process::exit(1);
            }
        }
    } else {
        let mut stdout = std::io::stdout();
        if let Err(e) = stdout.write_all(final_message.as_bytes()) {
            eprintln!("Error: Failed writing to stdout: {}", e);
            std::process::exit(1);
        }
        if !options.no_newline {
            println!();
        }
    }
}
