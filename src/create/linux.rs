use crate::CreateOptions;
use std::ffi::CString;
use std::io::Error;
use std::path::Path;
use libc::{open, O_CREAT, O_WRONLY, S_IRUSR, S_IWUSR, S_IRGRP, S_IROTH, mkdir};

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
        let c_path = match CString::new(path_str) {
            Ok(s) => s,
            Err(_) => { eprintln!("Error: Path contains invalid characters."); return; }
        };
        let mode = S_IRUSR | S_IWUSR | S_IRGRP | S_IROTH; // 644 permissions
        let fd = unsafe { open(c_path.as_ptr(), O_CREAT | O_WRONLY, mode) };

        if fd < 0 {
            eprintln!("Error creating file '{}': {}", path.display(), Error::last_os_error());
        } else {
            unsafe { libc::close(fd) };
            println!("Created file: {}", path.display());
        }
    } else {
        let c_path = match CString::new(path_str) {
            Ok(s) => s,
            Err(_) => { eprintln!("Error: Path contains invalid characters."); return; }
        };
        let mode = S_IRUSR | S_IWUSR | libc::S_IXUSR | S_IRGRP | libc::S_IXGRP | S_IROTH | libc::S_IXOTH; // 755
        if unsafe { mkdir(c_path.as_ptr(), mode) } != 0 {
            eprintln!("Error creating directory '{}': {}", path.display(), Error::last_os_error());
        } else {
            println!("Created directory: {}", path.display());
        }
    }
}
