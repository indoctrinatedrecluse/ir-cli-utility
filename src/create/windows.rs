use crate::CreateOptions;
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use std::path::Path;
use std::ptr::null_mut;
use windows_sys::Win32::Foundation::{GetLastError, GENERIC_WRITE, CloseHandle, INVALID_HANDLE_VALUE};
use windows_sys::Win32::System::Diagnostics::Debug::{FormatMessageW, FORMAT_MESSAGE_FROM_SYSTEM, FORMAT_MESSAGE_IGNORE_INSERTS};
use windows_sys::Win32::Storage::FileSystem::{
    CreateFileW, CreateDirectoryW, FILE_ATTRIBUTE_NORMAL, FILE_SHARE_READ, CREATE_NEW
};

fn get_last_error_message() -> String {
    let error_code = unsafe { GetLastError() };
    if error_code == 0 { return String::new(); }
    let mut buffer: Vec<u16> = vec![0; 256];
    let length = unsafe {
        FormatMessageW(FORMAT_MESSAGE_FROM_SYSTEM | FORMAT_MESSAGE_IGNORE_INSERTS, null_mut(), error_code, 0, buffer.as_mut_ptr(), buffer.len() as u32, null_mut())
    };
    if length == 0 { return format!("Unknown error (code: {})", error_code); }
    std::ffi::OsString::from_wide(&buffer[..length as usize]).to_string_lossy().trim().to_string()
}

pub fn create(path_str: &str, options: CreateOptions) {
    let path = Path::new(path_str);

    if path.file_name().is_none() {
        eprintln!("Error: Invalid path '{}'.", path_str);
        return;
    }

    let create_as_file = options.create_file || path.extension().is_some();

    if options.force_subdirs {
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                if let Err(e) = std::fs::create_dir_all(parent) {
                    eprintln!("Error creating parent directories for '{}': {}", path.display(), e);
                    return;
                }
            }
        }
    }

    if create_as_file {
        let path_wide: Vec<u16> = path.as_os_str().encode_wide().chain(Some(0)).collect();
        let handle = unsafe {
            CreateFileW(
                path_wide.as_ptr(), GENERIC_WRITE, FILE_SHARE_READ, null_mut(),
                CREATE_NEW, FILE_ATTRIBUTE_NORMAL, 0, // Use 0 for the handle argument
            )
        };

        if handle == INVALID_HANDLE_VALUE {
            eprintln!("Error creating file '{}': {}", path.display(), get_last_error_message());
        } else {
            unsafe { CloseHandle(handle) };
            println!("Created file: {}", path.display());
        }
    } else {
        let path_wide: Vec<u16> = path.as_os_str().encode_wide().chain(Some(0)).collect();
        if unsafe { CreateDirectoryW(path_wide.as_ptr(), null_mut()) } == 0 {
            eprintln!("Error creating directory '{}': {}", path.display(), get_last_error_message());
        } else {
            println!("Created directory: {}", path.display());
        }
    }
}
