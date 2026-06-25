use crate::CopyOptions;
use std::ffi::CString;
use std::io::Error;
use std::path::{Path, PathBuf};
use libc::{lstat, S_IFDIR, S_IFLNK, open, read, write, close, O_RDONLY, O_WRONLY, O_CREAT, S_IRUSR, S_IWUSR, S_IRGRP, S_IROTH, mkdir, opendir, readdir, closedir};
use std::mem::MaybeUninit;

fn copy_item(source: &Path, dest: &Path, options: &CopyOptions) -> Result<(), String> {
    let c_source = CString::new(source.as_os_str().to_str().unwrap()).unwrap();
    let mut stat_buf = MaybeUninit::uninit();
    if unsafe { lstat(c_source.as_ptr(), stat_buf.as_mut_ptr()) } != 0 {
        return Err(format!("Failed to stat '{}': {}", source.display(), Error::last_os_error()));
    }
    let stat_info = unsafe { stat_buf.assume_init() };
    let is_dir = (stat_info.st_mode & libc::S_IFMT) == S_IFDIR;

    if is_dir {
        if options.files_only { return Ok(()); }

        if !dest.exists() {
            let c_dest = CString::new(dest.as_os_str().to_str().unwrap()).unwrap();
            if unsafe { mkdir(c_dest.as_ptr(), stat_info.st_mode) } != 0 {
                return Err(format!("Failed to create directory '{}': {}", dest.display(), Error::last_os_error()));
            }
        }

        if options.recursive {
            let dir = unsafe { opendir(c_source.as_ptr()) };
            if dir.is_null() {
                return Err(format!("Failed to open source directory '{}': {}", source.display(), Error::last_os_error()));
            }
            loop {
                let entry = unsafe { readdir(dir) };
                if entry.is_null() { break; }
                let name = unsafe { std::ffi::CStr::from_ptr((*entry).d_name.as_ptr()) }.to_str().unwrap();
                if name == "." || name == ".." { continue; }

                let new_source = source.join(name);
                let new_dest = dest.join(name);
                copy_item(&new_source, &new_dest, options)?;
            }
            unsafe { closedir(dir) };
        }
    } else { // It's a file
        if options.folders_only { return Ok(()); }

        let c_dest = CString::new(dest.as_os_str().to_str().unwrap()).unwrap();
        let mut open_flags = O_WRONLY | O_CREAT;
        if !options.force {
            open_flags |= libc::O_EXCL;
        }

        let source_fd = unsafe { open(c_source.as_ptr(), O_RDONLY) };
        if source_fd < 0 { return Err(format!("Failed to open source file '{}': {}", source.display(), Error::last_os_error())); }

        let dest_fd = unsafe { open(c_dest.as_ptr(), open_flags, stat_info.st_mode) };
        if dest_fd < 0 {
            unsafe { close(source_fd) };
            return Err(format!("Failed to open destination file '{}': {}", dest.display(), Error::last_os_error()));
        }

        let mut buffer = [0u8; 4096];
        loop {
            let bytes_read = unsafe { read(source_fd, buffer.as_mut_ptr() as *mut _, buffer.len()) };
            if bytes_read == 0 { break; }
            if bytes_read < 0 {
                let err = format!("Error reading from '{}': {}", source.display(), Error::last_os_error());
                unsafe { close(source_fd); close(dest_fd); }
                return Err(err);
            }
            let bytes_written = unsafe { write(dest_fd, buffer.as_ptr() as *const _, bytes_read as usize) };
            if bytes_written < 0 {
                let err = format!("Error writing to '{}': {}", dest.display(), Error::last_os_error());
                unsafe { close(source_fd); close(dest_fd); }
                return Err(err);
            }
        }
        unsafe { close(source_fd); close(dest_fd); }
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
