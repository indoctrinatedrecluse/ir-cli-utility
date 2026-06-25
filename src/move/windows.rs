use crate::MoveOptions;
use std::ffi::OsString;
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use std::path::Path;
use std::ptr::null_mut;
use windows_sys::Win32::Foundation::GetLastError;
use windows_sys::Win32::System::Diagnostics::Debug::{FormatMessageW, FORMAT_MESSAGE_FROM_SYSTEM, FORMAT_MESSAGE_IGNORE_INSERTS};
use windows_sys::Win32::Storage::FileSystem::{MoveFileExW, GetFileAttributesW, INVALID_FILE_ATTRIBUTES, MOVEFILE_REPLACE_EXISTING};

fn get_last_error_message() -> String {
    let error_code = unsafe { GetLastError() };
    if error_code == 0 { return String::new(); }
    let mut buffer: Vec<u16> = vec![0; 256];
    let length = unsafe {
        FormatMessageW(FORMAT_MESSAGE_FROM_SYSTEM | FORMAT_MESSAGE_IGNORE_INSERTS, null_mut(), error_code, 0, buffer.as_mut_ptr(), buffer.len() as u32, null_mut())
    };
    if length == 0 { return format!("Unknown error (code: {})", error_code); }
    OsString::from_wide(&buffer[..length as usize]).to_string_lossy().trim().to_string()
}

pub fn move_item(source: &str, destination: &str, options: MoveOptions) {
    let source_path = Path::new(source);
    let dest_path = Path::new(destination);

    if unsafe { GetFileAttributesW(source_path.as_os_str().encode_wide().chain(Some(0)).collect::<Vec<_>>().as_ptr()) } == INVALID_FILE_ATTRIBUTES {
        eprintln!("Error: Source '{}' not found. ({})", source, get_last_error_message());
        return;
    }

    let final_dest_path = if let Some(new_name) = &options.rename {
        if !source_path.is_file() {
            eprintln!("Error: --rename can only be used with files.");
            return;
        }
        if !dest_path.is_dir() {
            eprintln!("Error: Destination '{}' must be a directory when using --rename.", destination);
            return;
        }
        dest_path.join(new_name)
    } else {
        if dest_path.is_dir() {
            dest_path.join(source_path.file_name().unwrap_or_default())
        } else {
            dest_path.to_path_buf()
        }
    };

    let source_wide: Vec<u16> = source_path.as_os_str().encode_wide().chain(Some(0)).collect();
    let dest_wide: Vec<u16> = final_dest_path.as_os_str().encode_wide().chain(Some(0)).collect();

    let mut flags = 0;
    if options.force {
        flags |= MOVEFILE_REPLACE_EXISTING;
    }

    let result = unsafe { MoveFileExW(source_wide.as_ptr(), dest_wide.as_ptr(), flags) };

    if result == 0 {
        eprintln!("Error moving '{}' to '{}': {}", source, final_dest_path.display(), get_last_error_message());
    } else {
        println!("Successfully moved '{}' to '{}'.", source, final_dest_path.display());
    }
}
