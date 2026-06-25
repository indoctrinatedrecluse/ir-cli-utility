use std::ffi::OsString;
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use std::path::Path;
use std::ptr::null_mut;
use windows_sys::Win32::Foundation::{
    CloseHandle, GetLastError, GENERIC_READ, INVALID_HANDLE_VALUE,
};
use windows_sys::Win32::Storage::FileSystem::{
    CreateFileW, ReadFile, FILE_ATTRIBUTE_NORMAL, FILE_SHARE_DELETE, FILE_SHARE_READ,
    FILE_SHARE_WRITE, OPEN_EXISTING,
};
use windows_sys::Win32::System::Diagnostics::Debug::{
    FormatMessageW, FORMAT_MESSAGE_FROM_SYSTEM, FORMAT_MESSAGE_IGNORE_INSERTS,
};

pub fn read_file(path: &str) -> Result<Vec<u8>, String> {
    let path_wide: Vec<u16> = Path::new(path)
        .as_os_str()
        .encode_wide()
        .chain(Some(0))
        .collect();

    let handle = unsafe {
        CreateFileW(
            path_wide.as_ptr(),
            GENERIC_READ,
            FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE,
            null_mut(),
            OPEN_EXISTING,
            FILE_ATTRIBUTE_NORMAL,
            0,
        )
    };

    if handle == INVALID_HANDLE_VALUE {
        return Err(get_last_error_message());
    }

    let mut bytes = Vec::new();
    let mut buffer = [0u8; 8192];

    loop {
        let mut read_count = 0u32;
        let result = unsafe {
            ReadFile(
                handle,
                buffer.as_mut_ptr() as *mut _,
                buffer.len() as u32,
                &mut read_count,
                null_mut(),
            )
        };

        if result == 0 {
            let message = get_last_error_message();
            unsafe { CloseHandle(handle) };
            return Err(message);
        }

        if read_count == 0 {
            break;
        }

        bytes.extend_from_slice(&buffer[..read_count as usize]);
    }

    unsafe { CloseHandle(handle) };
    Ok(bytes)
}

fn get_last_error_message() -> String {
    let error_code = unsafe { GetLastError() };
    if error_code == 0 {
        return String::new();
    }

    let mut buffer: Vec<u16> = vec![0; 256];
    let length = unsafe {
        FormatMessageW(
            FORMAT_MESSAGE_FROM_SYSTEM | FORMAT_MESSAGE_IGNORE_INSERTS,
            null_mut(),
            error_code,
            0,
            buffer.as_mut_ptr(),
            buffer.len() as u32,
            null_mut(),
        )
    };

    if length == 0 {
        return format!("Unknown error (code: {})", error_code);
    }

    OsString::from_wide(&buffer[..length as usize])
        .to_string_lossy()
        .trim()
        .to_string()
}
