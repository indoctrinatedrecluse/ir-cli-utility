use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Read, Seek, SeekFrom, Write};
use std::collections::VecDeque;
use std::time::Duration;
use std::thread;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TailCount {
    LastLines(usize),
    LastBytes(usize),
    FromKthLine(usize),
    FromKthByte(usize),
}

pub struct TailOptions {
    pub files: Vec<String>,
    pub count: TailCount,
    pub follow: bool,
    pub sleep_interval_ms: u64,
    pub quiet: bool,
    pub verbose: bool,
}

pub fn run_tail(options: TailOptions) -> io::Result<()> {
    let show_headers = if options.quiet {
        false
    } else if options.verbose {
        true
    } else {
        options.files.len() > 1
    };

    let mut stdout = io::BufWriter::new(io::stdout());

    if options.files.is_empty() {
        if options.follow {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "cannot follow standard input",
            ));
        }
        tail_stream(io::stdin(), options.count, &mut stdout)?;
    } else if options.files.len() == 1 {
        let file_path = &options.files[0];
        if file_path == "-" {
            if options.follow {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "cannot follow standard input",
                ));
            }
            tail_stream(io::stdin(), options.count, &mut stdout)?;
        } else {
            let mut file = File::open(file_path)?;
            tail_stream(&mut file, options.count, &mut stdout)?;
            stdout.flush()?;
            if options.follow {
                follow_file(file_path, &mut file, options.sleep_interval_ms, &mut stdout)?;
            }
        }
    } else {
        // Multiple files
        for (idx, file_path) in options.files.iter().enumerate() {
            if show_headers {
                if idx > 0 {
                    writeln!(stdout)?;
                }
                writeln!(stdout, "==> {} <==", file_path)?;
                stdout.flush()?;
            }

            if file_path == "-" {
                tail_stream(io::stdin(), options.count, &mut stdout)?;
            } else {
                let mut file = match File::open(file_path) {
                    Ok(f) => f,
                    Err(e) => {
                        eprintln!("ir tail: cannot open '{}' for reading: {}", file_path, e);
                        continue;
                    }
                };
                tail_stream(&mut file, options.count, &mut stdout)?;
                stdout.flush()?;
            }
        }

        if options.follow {
            // Following multiple files is not standard with simple loops,
            // but we can implement it by round-robin polling each file!
            follow_multiple_files(&options.files, options.sleep_interval_ms, &mut stdout)?;
        }
    }

    stdout.flush()?;
    Ok(())
}

fn tail_stream<R: Read, W: Write>(stream: R, count: TailCount, stdout: &mut W) -> io::Result<()> {
    match count {
        TailCount::LastLines(n) => {
            let mut reader = BufReader::new(stream);
            let mut deque = VecDeque::with_capacity(n);
            let mut line = String::new();
            loop {
                line.clear();
                match reader.read_line(&mut line) {
                    Ok(0) => break,
                    Ok(_) => {
                        if deque.len() == n && n > 0 {
                            deque.pop_front();
                        }
                        if n > 0 {
                            deque.push_back(line.clone());
                        }
                    }
                    Err(e) => return Err(e),
                }
            }
            for l in deque {
                write!(stdout, "{}", l)?;
            }
        }
        TailCount::FromKthLine(k) => {
            let mut reader = BufReader::new(stream);
            let mut line = String::new();
            let mut line_num = 1;
            loop {
                line.clear();
                match reader.read_line(&mut line) {
                    Ok(0) => break,
                    Ok(_) => {
                        if line_num >= k {
                            write!(stdout, "{}", line)?;
                        }
                        line_num += 1;
                    }
                    Err(e) => return Err(e),
                }
            }
        }
        TailCount::LastBytes(n) => {
            let mut reader = stream;
            let mut deque = VecDeque::with_capacity(n);
            let mut buffer = [0u8; 8192];
            loop {
                match reader.read(&mut buffer) {
                    Ok(0) => break,
                    Ok(bytes_read) => {
                        for &b in &buffer[..bytes_read] {
                            if deque.len() == n && n > 0 {
                                deque.pop_front();
                            }
                            if n > 0 {
                                deque.push_back(b);
                            }
                        }
                    }
                    Err(e) => return Err(e),
                }
            }
            // Write collected bytes
            let (slice1, slice2) = deque.as_slices();
            stdout.write_all(slice1)?;
            stdout.write_all(slice2)?;
        }
        TailCount::FromKthByte(k) => {
            let mut reader = stream;
            let mut byte_num = 1;
            let mut buffer = [0u8; 8192];
            loop {
                match reader.read(&mut buffer) {
                    Ok(0) => break,
                    Ok(bytes_read) => {
                        for (idx, _) in buffer[..bytes_read].iter().enumerate() {
                            let current_pos = byte_num + idx;
                            if current_pos >= k {
                                stdout.write_all(&buffer[idx..bytes_read])?;
                                byte_num += bytes_read;
                                break;
                            }
                        }
                        if byte_num < k {
                            byte_num += bytes_read;
                        }
                    }
                    Err(e) => return Err(e),
                }
            }
        }
    }
    Ok(())
}

fn follow_file<W: Write>(
    file_path: &str,
    file: &mut File,
    sleep_ms: u64,
    stdout: &mut W,
) -> io::Result<()> {
    let mut pos = file.metadata()?.len();
    let sleep_dur = Duration::from_millis(sleep_ms);
    let mut buffer = [0u8; 8192];

    loop {
        // Check if file shrank (e.g. truncated)
        if let Ok(meta) = fs::metadata(file_path) {
            let new_len = meta.len();
            if new_len < pos {
                let _ = file.seek(SeekFrom::Start(0));
                pos = 0;
            }
        }

        match file.read(&mut buffer) {
            Ok(0) => {
                thread::sleep(sleep_dur);
            }
            Ok(n) => {
                stdout.write_all(&buffer[..n])?;
                stdout.flush()?;
                pos += n as u64;
            }
            Err(ref e) if e.kind() == io::ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        }
    }
}

struct FollowState {
    file_path: String,
    file: File,
    pos: u64,
}

fn follow_multiple_files<W: Write>(
    file_paths: &[String],
    sleep_ms: u64,
    stdout: &mut W,
) -> io::Result<()> {
    let mut states = Vec::new();
    for path in file_paths {
        if path == "-" {
            continue;
        }
        if let Ok(mut f) = File::open(path) {
            let len = f.metadata()?.len();
            let _ = f.seek(SeekFrom::Start(len));
            states.push(FollowState {
                file_path: path.clone(),
                file: f,
                pos: len,
            });
        }
    }

    let sleep_dur = Duration::from_millis(sleep_ms);
    let mut buffer = [0u8; 8192];
    let mut last_announced_path = String::new();

    loop {
        let mut read_any = false;

        for state in &mut states {
            if let Ok(meta) = fs::metadata(&state.file_path) {
                let new_len = meta.len();
                if new_len < state.pos {
                    let _ = state.file.seek(SeekFrom::Start(0));
                    state.pos = 0;
                }
            }

            match state.file.read(&mut buffer) {
                Ok(0) => {}
                Ok(n) => {
                    read_any = true;
                    if last_announced_path != state.file_path {
                        writeln!(stdout, "\n==> {} <==", state.file_path)?;
                        last_announced_path = state.file_path.clone();
                    }
                    stdout.write_all(&buffer[..n])?;
                    stdout.flush()?;
                    state.pos += n as u64;
                }
                Err(ref e) if e.kind() == io::ErrorKind::Interrupted => continue,
                Err(e) => return Err(e),
            }
        }

        if !read_any {
            thread::sleep(sleep_dur);
        }
    }
}
