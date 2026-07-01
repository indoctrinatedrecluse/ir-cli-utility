use crate::ClipOptions;
use std::io::{stdin, Read, IsTerminal};

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "windows")]
mod windows;

pub fn get_clipboard() -> Result<String, String> {
    #[cfg(target_os = "windows")]
    {
        windows::get_clipboard()
    }
    #[cfg(target_os = "linux")]
    {
        linux::get_clipboard()
    }
}

pub fn set_clipboard(text: &str) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        windows::set_clipboard(text)
    }
    #[cfg(target_os = "linux")]
    {
        linux::set_clipboard(text)
    }
}

pub fn clear_clipboard() -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        windows::clear_clipboard()
    }
    #[cfg(target_os = "linux")]
    {
        linux::clear_clipboard()
    }
}

pub fn run_clip(options: ClipOptions) {
    if options.clear {
        if let Err(e) = clear_clipboard() {
            eprintln!("Error: Failed to clear clipboard: {}", e);
            std::process::exit(1);
        }
        println!("Clipboard cleared.");
        return;
    }

    // Determine if input is being piped
    let is_piped = !stdin().is_terminal();

    if is_piped {
        // Read from stdin and copy to clipboard
        let mut buffer = String::new();
        if let Err(e) = stdin().read_to_string(&mut buffer) {
            eprintln!("Error: Failed to read from stdin: {}", e);
            std::process::exit(1);
        }
        
        if let Err(e) = set_clipboard(&buffer) {
            eprintln!("Error: Failed to copy to clipboard: {}", e);
            std::process::exit(1);
        }
    } else {
        // Read from clipboard and print to stdout
        match get_clipboard() {
            Ok(text) => {
                print!("{}", text);
            }
            Err(e) => {
                eprintln!("Error: Failed to read from clipboard: {}", e);
                std::process::exit(1);
            }
        }
    }
}
