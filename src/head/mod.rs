use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::collections::VecDeque;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HeadCount {
    Lines(usize),
    Bytes(usize),
    LinesAllButLast(usize),
    BytesAllButLast(usize),
}

pub struct HeadOptions {
    pub files: Vec<String>,
    pub count: HeadCount,
    pub quiet: bool,
    pub verbose: bool,
}

pub fn run_head(options: HeadOptions) -> io::Result<()> {
    let show_headers = if options.quiet {
        false
    } else if options.verbose {
        true
    } else {
        options.files.len() > 1
    };

    let mut stdout = io::BufWriter::new(io::stdout());

    if options.files.is_empty() {
        head_stream(io::stdin(), options.count, &mut stdout)?;
    } else {
        for (idx, file_path) in options.files.iter().enumerate() {
            if show_headers {
                if idx > 0 {
                    writeln!(stdout)?;
                }
                if file_path == "-" {
                    writeln!(stdout, "==> standard input <==")?;
                } else {
                    writeln!(stdout, "==> {} <==", file_path)?;
                }
                stdout.flush()?;
            }

            if file_path == "-" {
                head_stream(io::stdin(), options.count, &mut stdout)?;
            } else {
                let file = match File::open(file_path) {
                    Ok(f) => f,
                    Err(e) => {
                        eprintln!("ir head: cannot open '{}' for reading: {}", file_path, e);
                        continue;
                    }
                };
                head_stream(file, options.count, &mut stdout)?;
            }
        }
    }
    stdout.flush()?;
    Ok(())
}

fn head_stream<R: Read, W: Write>(stream: R, count: HeadCount, stdout: &mut W) -> io::Result<()> {
    match count {
        HeadCount::Lines(n) => {
            let mut reader = BufReader::new(stream);
            let mut count = 0;
            let mut line = String::new();
            while count < n {
                line.clear();
                match reader.read_line(&mut line) {
                    Ok(0) => break, // EOF
                    Ok(_) => {
                        write!(stdout, "{}", line)?;
                        count += 1;
                    }
                    Err(e) => return Err(e),
                }
            }
        }
        HeadCount::LinesAllButLast(n) => {
            let mut reader = BufReader::new(stream);
            let mut deque = VecDeque::with_capacity(n + 1);
            let mut line = String::new();
            loop {
                line.clear();
                match reader.read_line(&mut line) {
                    Ok(0) => break,
                    Ok(_) => {
                        deque.push_back(line.clone());
                        if deque.len() > n {
                            if let Some(front) = deque.pop_front() {
                                write!(stdout, "{}", front)?;
                            }
                        }
                    }
                    Err(e) => return Err(e),
                }
            }
        }
        HeadCount::Bytes(n) => {
            let mut reader = stream;
            let mut buffer = [0u8; 8192];
            let mut remaining = n;
            while remaining > 0 {
                let limit = remaining.min(buffer.len());
                match reader.read(&mut buffer[..limit]) {
                    Ok(0) => break,
                    Ok(bytes_read) => {
                        stdout.write_all(&buffer[..bytes_read])?;
                        remaining -= bytes_read;
                    }
                    Err(e) => return Err(e),
                }
            }
        }
        HeadCount::BytesAllButLast(n) => {
            let mut reader = stream;
            let mut buffer = [0u8; 8192];
            let mut ring = VecDeque::with_capacity(n + 1);
            loop {
                match reader.read(&mut buffer) {
                    Ok(0) => break,
                    Ok(bytes_read) => {
                        for &b in &buffer[..bytes_read] {
                            ring.push_back(b);
                            if ring.len() > n {
                                if let Some(front) = ring.pop_front() {
                                    stdout.write_all(&[front])?;
                                }
                            }
                        }
                    }
                    Err(e) => return Err(e),
                }
            }
        }
    }
    Ok(())
}
