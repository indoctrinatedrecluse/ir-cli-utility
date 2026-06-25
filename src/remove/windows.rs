use crate::RemoveOptions;
use std::ffi::OsString;
use std::io::{self, Write};
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use std::path::Path;
use std::ptr::null_mut;
use windows_sys::Win32::Foundation::{GetLastError, FALSE};
use windows_sys::Win32::System::Diagnostics::Debug::{FormatMessageW, FORMAT_MESSAGE_FROM_SYSTEM, FORMAT_MESSAGE_IGNORE_INSERTS};
use windows_sys::Win32::Storage::FileSystem::{
    GetFileAttributesW, SetFileAttributesW, DeleteFileW, RemoveDirectoryW, FindFirstFileW, FindNextFileW, FindClose,
    WIN32_FIND_DATAW, INVALID_FILE_ATTRIBUTES, FILE_ATTRIBUTE_DIRECTORY, FILE_ATTRIBUTE_READONLY
};
use windows_sys::Win32::UI::Shell::{SHFileOperationW, SHFILEOPSTRUCTW, FO_DELETE, FOF_ALLOWUNDO, FOF_NOCONFIRMATION, FOF_SILENT};

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

fn remove_readonly(path: &Path) -> Result<(), String> {
    let path_wide: Vec<u16> = path.as_os_str().encode_wide().chain(Some(0)).collect();
    let attrs = unsafe { GetFileAttributesW(path_wide.as_ptr()) };
    if attrs & FILE_ATTRIBUTE_READONLY != 0 {
        if unsafe { SetFileAttributesW(path_wide.as_ptr(), attrs & !FILE_ATTRIBUTE_READONLY) } == 0 {
            return Err(format!("Failed to remove read-only attribute from '{}'", path.display()));
        }
    }
    Ok(())
}

fn delete_directory_recursive(path: &Path, options: &RemoveOptions) -> Result<(), String> {
    let search_path: Vec<u16> = path.join("*").as_os_str().encode_wide().chain(Some(0)).collect();
    let mut find_data: WIN32_FIND_DATAW = unsafe { std::mem::zeroed() };
    let find_handle = unsafe { FindFirstFileW(search_path.as_ptr(), &mut find_data) };

    if find_handle == INVALID_FILE_ATTRIBUTES as isize {
        return Err(format!("Failed to list contents of '{}'", path.display()));
    }

    loop {
        let name_slice = &find_data.cFileName[..find_data.cFileName.iter().position(|&c| c == 0).unwrap_or(0)];
        let os_name = OsString::from_wide(name_slice);
        let name = os_name.to_string_lossy();

        if name == "." || name == ".." {
            if unsafe { FindNextFileW(find_handle, &mut find_data) } == 0 { break; }
            continue;
        }

        let full_path = path.join(name.as_ref());
        let attrs = unsafe { GetFileAttributesW(full_path.as_os_str().encode_wide().chain(Some(0)).collect::<Vec<_>>().as_ptr()) };
        if attrs & FILE_ATTRIBUTE_DIRECTORY != 0 {
            delete_directory_recursive(&full_path, options)?;
        } else {
            if options.verbose { println!("Removing file: {}", full_path.display()); }
            remove_readonly(&full_path)?;
            if unsafe { DeleteFileW(full_path.as_os_str().encode_wide().chain(Some(0)).collect::<Vec<_>>().as_ptr()) } == 0 {
                return Err(format!("Failed to delete file '{}': {}", full_path.display(), get_last_error_message()));
            }
        }

        if unsafe { FindNextFileW(find_handle, &mut find_data) } == 0 {
            break;
        }
    }
    unsafe { FindClose(find_handle) };

    if options.verbose { println!("Removing directory: {}", path.display()); }
    remove_readonly(path)?;
    if unsafe { RemoveDirectoryW(path.as_os_str().encode_wide().chain(Some(0)).collect::<Vec<_>>().as_ptr()) } == 0 {
        return Err(format!("Failed to delete directory '{}': {}", path.display(), get_last_error_message()));
    }

    Ok(())
}

fn to_trash(path: &Path, options: &RemoveOptions) -> Result<(), String> {
    let path_wide: Vec<u16> = path.as_os_str().encode_wide().chain(Some(0)).collect();
    let mut op = SHFILEOPSTRUCTW {
        hwnd: 0, // Use 0 for a console application
        wFunc: FO_DELETE,
        pFrom: path_wide.as_ptr(),
        pTo: null_mut(),
        fFlags: FOF_ALLOWUNDO as u16, // Cast to u16
        fAnyOperationsAborted: FALSE,
        hNameMappings: null_mut(),
        lpszProgressTitle: null_mut(),
    };
    if !options.interactive {
        op.fFlags |= FOF_NOCONFIRMATION as u16;
    }
    if !options.verbose {
        op.fFlags |= FOF_SILENT as u16;
    }

    let result = unsafe { SHFileOperationW(&mut op) };
    if result != 0 {
        Err(format!("Failed to move '{}' to Recycle Bin.", path.display()))
    } else {
        if options.verbose { println!("Moved '{}' to Recycle Bin.", path.display()); }
        Ok(())
    }
}

pub fn remove(path_str: &str, options: &RemoveOptions) {
    let path = Path::new(path_str);
    if !path.exists() {
        eprintln!("Error: '{}' not found.", path.display());
        return;
    }

    if options.trash {
        if let Err(e) = to_trash(path, options) { eprintln!("{}", e); }
        return;
    }

    let is_dir = path.is_dir();

    if options.interactive {
        print!("Permanently remove '{}'? [y/N] ", path.display());
        io::stdout().flush().unwrap();
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() || input.trim().to_lowercase() != "y" {
            println!("Operation cancelled.");
            return;
        }
    }

    if is_dir {
        let is_empty = path.read_dir().map_or(false, |mut i| i.next().is_none());
        if !is_empty && !options.yes && !options.force {
             print!("'{}' is not empty. Remove it and all its contents? [y/N] ", path.display());
             io::stdout().flush().unwrap();
             let mut input = String::new();
             if io::stdin().read_line(&mut input).is_err() || input.trim().to_lowercase() != "y" {
                 println!("Operation cancelled.");
                 return;
             }
        }
        if let Err(e) = delete_directory_recursive(path, options) { eprintln!("{}", e); }
    } else {
        if options.verbose { println!("Removing file: {}", path.display()); }
        if let Err(e) = remove_readonly(path) { eprintln!("{}", e); return; }
        if unsafe { DeleteFileW(path.as_os_str().encode_wide().chain(Some(0)).collect::<Vec<_>>().as_ptr()) } == 0 {
            eprintln!("Failed to delete file '{}': {}", path.display(), get_last_error_message());
        }
    }
}
