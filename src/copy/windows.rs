use crate::CopyOptions;
use std::ffi::OsString;
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use std::path::Path;
use std::ptr::null_mut;
use windows_sys::Win32::Foundation::{GetLastError, FALSE, TRUE};
use windows_sys::Win32::System::Diagnostics::Debug::{FormatMessageW, FORMAT_MESSAGE_FROM_SYSTEM, FORMAT_MESSAGE_IGNORE_INSERTS};
use windows_sys::Win32::Storage::FileSystem::{
    CopyFileW, CreateDirectoryW, GetFileAttributesW, FindFirstFileW, FindNextFileW, FindClose,
    WIN32_FIND_DATAW, INVALID_FILE_ATTRIBUTES, FILE_ATTRIBUTE_DIRECTORY
};

fn get_last_error_message() -> String {
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

fn copy_item(source: &Path, dest: &Path, options: &CopyOptions) -> Result<(), String> {
    let source_attrs = unsafe { GetFileAttributesW(source.as_os_str().encode_wide().chain(Some(0)).collect::<Vec<_>>().as_ptr()) };
    let is_dir = (source_attrs & FILE_ATTRIBUTE_DIRECTORY) != 0;

    if is_dir {
        if options.files_only { return Ok(()); }

        if !dest.exists() {
            let dest_wide: Vec<u16> = dest.as_os_str().encode_wide().chain(Some(0)).collect();
            if unsafe { CreateDirectoryW(dest_wide.as_ptr(), null_mut()) } == 0 {
                return Err(format!("Failed to create directory '{}': {}", dest.display(), get_last_error_message()));
            }
        }

        if options.recursive {
            let search_path: Vec<u16> = source.join("*").as_os_str().encode_wide().chain(Some(0)).collect();
            let mut find_data: WIN32_FIND_DATAW = unsafe { std::mem::zeroed() };
            let find_handle = unsafe { FindFirstFileW(search_path.as_ptr(), &mut find_data) };

            if find_handle == INVALID_FILE_ATTRIBUTES as isize {
                return Err(format!("Failed to list contents of '{}'", source.display()));
            }

            loop {
                let name_slice = {
                    let len = find_data.cFileName.iter().position(|&c| c == 0).unwrap_or(0);
                    &find_data.cFileName[..len]
                };
                // **FIX**: Create a longer-lived OsString
                let os_name = OsString::from_wide(name_slice);
                let name = os_name.to_string_lossy();

                if name == "." || name == ".." {
                    if unsafe { FindNextFileW(find_handle, &mut find_data) } == 0 { break; }
                    continue;
                }

                let new_source = source.join(name.as_ref());
                let new_dest = dest.join(name.as_ref());
                copy_item(&new_source, &new_dest, options)?;

                if unsafe { FindNextFileW(find_handle, &mut find_data) } == 0 {
                    break;
                }
            }
            unsafe { FindClose(find_handle) };
        }
    } else { // It's a file
        if options.folders_only { return Ok(()); }

        let fail_if_exists = if options.force { FALSE } else { TRUE };
        let source_wide: Vec<u16> = source.as_os_str().encode_wide().chain(Some(0)).collect();
        let dest_wide: Vec<u16> = dest.as_os_str().encode_wide().chain(Some(0)).collect();

        if unsafe { CopyFileW(source_wide.as_ptr(), dest_wide.as_ptr(), fail_if_exists) } == 0 {
            return Err(format!("Failed to copy file to '{}': {}", dest.display(), get_last_error_message()));
        }
    }
    Ok(())
}

pub fn copy(source: &str, destination: &str, options: CopyOptions) {
    let source_path = Path::new(source);
    let dest_path = Path::new(destination);

    if !source_path.exists() {
        eprintln!("Error: Source path '{}' does not exist.", source);
        return;
    }
    if !dest_path.is_dir() {
        eprintln!("Error: Destination path '{}' is not a valid directory.", destination);
        return;
    }

    if let Some(new_name) = &options.rename {
        if source_path.is_dir() {
            eprintln!("Error: The --rename switch can only be used with files, not directories.");
            return;
        }
        let final_dest = dest_path.join(new_name);
        if let Err(e) = copy_item(source_path, &final_dest, &options) {
            eprintln!("{}", e);
        } else {
            println!("Copied '{}' to '{}'.", source, final_dest.display());
        }
        return;
    }

    let final_dest = dest_path.join(source_path.file_name().unwrap_or_default());
    if let Err(e) = copy_item(source_path, &final_dest, &options) {
        eprintln!("{}", e);
    } else {
        println!("Copied '{}' to '{}'.", source, final_dest.display());
    }
}
