use crate::RenameOptions;
use std::ffi::{OsStr, OsString};
use std::io::{self, Write};
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use std::path::Path;
use std::ptr::null_mut;
use windows_sys::Win32::Foundation::GetLastError;
use windows_sys::Win32::System::Diagnostics::Debug::{FormatMessageW, FORMAT_MESSAGE_FROM_SYSTEM, FORMAT_MESSAGE_IGNORE_INSERTS};
use windows_sys::Win32::Storage::FileSystem::{MoveFileExW, GetFileAttributesW, INVALID_FILE_ATTRIBUTES, FILE_ATTRIBUTE_DIRECTORY, FILE_ATTRIBUTE_REPARSE_POINT, MOVEFILE_REPLACE_EXISTING};

fn get_last_error_message() -> String {
    // ... (this function remains the same)
    let error_code = unsafe { GetLastError() };
    if error_code == 0 { return String::new(); }
    let mut buffer: Vec<u16> = vec![0; 256];
    let length = unsafe {
        FormatMessageW(
            FORMAT_MESSAGE_FROM_SYSTEM | FORMAT_MESSAGE_IGNORE_INSERTS,
            null_mut(), error_code, 0, buffer.as_mut_ptr(), buffer.len() as u32, null_mut(),
        )
    };
    if length == 0 { return format!("Unknown error (code: {})", error_code); }
    OsString::from_wide(&buffer[..length as usize]).to_string_lossy().trim().to_string()
}

pub fn rename(source: &str, destination: &str, options: RenameOptions) {
    let dest_path = Path::new(destination);

    // 1. Check if source exists
    let source_attrs = unsafe { GetFileAttributesW(OsStr::new(source).encode_wide().chain(Some(0)).collect::<Vec<_>>().as_ptr()) };
    if source_attrs == INVALID_FILE_ATTRIBUTES {
        eprintln!("Error: Source '{}' not found. ({})", source, get_last_error_message());
        return;
    }
    let source_is_dir = (source_attrs & FILE_ATTRIBUTE_DIRECTORY) != 0;
    let source_is_link = (source_attrs & FILE_ATTRIBUTE_REPARSE_POINT) != 0;

    // 2. Handle symbolic links
    if source_is_link && !options.force_links {
        eprintln!("Error: Source '{}' is a symbolic link. Use --force-links to rename it.", source);
        return;
    }

    // 3. Prevent renaming a folder to a file
    if source_is_dir && dest_path.extension().is_some() {
        eprintln!("Error: Cannot rename a folder ('{}') to a file with an extension ('{}').", source, destination);
        return;
    }

    // 4. Interactive prompt
    if options.interactive {
        print!("Rename '{}' to '{}'? [y/N] ", source, destination);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() || input.trim().to_lowercase() != "y" {
            println!("Operation cancelled.");
            return;
        }
    }

    // 5. Perform the rename
    let source_wide: Vec<u16> = OsStr::new(source).encode_wide().chain(Some(0)).collect();
    let dest_wide: Vec<u16> = OsStr::new(destination).encode_wide().chain(Some(0)).collect();

    let mut flags = 0;
    if options.force {
        flags |= MOVEFILE_REPLACE_EXISTING;
    }

    let result = unsafe { MoveFileExW(source_wide.as_ptr(), dest_wide.as_ptr(), flags) };

    if result == 0 {
        eprintln!("Error renaming '{}' to '{}': {}", source, destination, get_last_error_message());
    } else {
        println!("Successfully renamed '{}' to '{}'.", source, destination);
    }
}
