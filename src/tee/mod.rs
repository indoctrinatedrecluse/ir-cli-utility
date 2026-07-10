use std::fs::OpenOptions;
use std::io::{self, Read, Write};

pub struct TeeOptions {
    pub files: Vec<String>,
    pub append: bool,
    pub ignore_interrupts: bool,
}

pub fn run_tee(options: TeeOptions) -> io::Result<()> {
    if options.ignore_interrupts {
        ignore_sigint();
    }

    // Open all file handles
    let mut file_handles = Vec::new();
    for file_path in &options.files {
        let file_res = OpenOptions::new()
            .write(true)
            .create(true)
            .append(options.append)
            .truncate(!options.append)
            .open(file_path);
        match file_res {
            Ok(f) => file_handles.push(f),
            Err(e) => {
                eprintln!("ir tee: {}: {}", file_path, e);
            }
        }
    }

    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut buffer = [0u8; 8192];

    loop {
        let bytes_read = match stdin.read(&mut buffer) {
            Ok(0) => break, // EOF
            Ok(n) => n,
            Err(ref e) if e.kind() == io::ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        };

        let data = &buffer[..bytes_read];

        // Write to stdout
        if let Err(e) = stdout.write_all(data) {
            return Err(e);
        }
        let _ = stdout.flush();

        // Write to all files
        for f in &mut file_handles {
            let _ = f.write_all(data);
            let _ = f.flush();
        }
    }

    Ok(())
}

#[cfg(not(target_os = "windows"))]
fn ignore_sigint() {
    unsafe {
        libc::signal(libc::SIGINT, libc::SIG_IGN);
    }
}

#[cfg(target_os = "windows")]
fn ignore_sigint() {
    unsafe {
        use windows_sys::Win32::System::Console::SetConsoleCtrlHandler;
        unsafe extern "system" fn handler(_: u32) -> i32 {
            1
        }
        let _ = SetConsoleCtrlHandler(Some(handler), 1);
    }
}
