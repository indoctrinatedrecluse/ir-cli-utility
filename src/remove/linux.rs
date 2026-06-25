use crate::RemoveOptions;
use std::env;
use std::ffi::CString;
use std::fs;
use std::io::{self, Write, Error};
use std::path::{Path, PathBuf};
use libc::{lstat, S_IFDIR, unlink, rmdir, opendir, readdir, closedir};
use std::mem::MaybeUninit;
use chrono::Local;

fn delete_recursive(path: &Path, options: &RemoveOptions) -> Result<(), String> {
    let c_path = CString::new(path.as_os_str().to_str().unwrap()).unwrap();
    let mut stat_buf = MaybeUninit::uninit();
    if unsafe { lstat(c_path.as_ptr(), stat_buf.as_mut_ptr()) } != 0 {
        return Err(format!("Failed to stat '{}': {}", path.display(), Error::last_os_error()));
    }
    let stat_info = unsafe { stat_buf.assume_init() };

    if (stat_info.st_mode & libc::S_IFMT) == S_IFDIR {
        if options.verbose { println!("Entering directory: {}", path.display()); }
        let dir = unsafe { opendir(c_path.as_ptr()) };
        if dir.is_null() { return Err(format!("Failed to open directory '{}': {}", path.display(), Error::last_os_error())); }

        loop {
            let entry = unsafe { readdir(dir) };
            if entry.is_null() { break; }
            let name = unsafe { std::ffi::CStr::from_ptr((*entry).d_name.as_ptr()) }.to_str().unwrap();
            if name == "." || name == ".." { continue; }
            delete_recursive(&path.join(name), options)?;
        }
        unsafe { closedir(dir) };

        if options.verbose { println!("Removing directory: {}", path.display()); }
        if unsafe { rmdir(c_path.as_ptr()) } != 0 {
            return Err(format!("Failed to remove directory '{}': {}", path.display(), Error::last_os_error()));
        }
    } else {
        if options.verbose { println!("Removing file: {}", path.display()); }
        if unsafe { unlink(c_path.as_ptr()) } != 0 {
            return Err(format!("Failed to remove file '{}': {}", path.display(), Error::last_os_error()));
        }
    }
    Ok(())
}

fn to_trash(path: &Path, options: &RemoveOptions) -> Result<(), String> {
    let home = env::var("HOME").map_err(|_| "Could not determine home directory.".to_string())?;
    let trash_dir = Path::new(&home).join(".local/share/Trash");
    let files_dir = trash_dir.join("files");
    let info_dir = trash_dir.join("info");

    if !trash_dir.exists() {
        return Err("Trash directory not found at ~/.local/share/Trash".to_string());
    }
    fs::create_dir_all(&files_dir).map_err(|e| e.to_string())?;
    fs::create_dir_all(&info_dir).map_err(|e| e.to_string())?;

    let file_name = path.file_name().unwrap().to_str().unwrap();
    let dest_path = files_dir.join(file_name);
    let info_path = info_dir.join(format!("{}.trashinfo", file_name));

    // Create .trashinfo file
    let original_path = path.canonicalize().unwrap().to_str().unwrap().to_string();
    let deletion_date = Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();
    let info_content = format!("[Trash Info]\nPath={}\nDeletionDate={}\n", original_path, deletion_date);
    fs::write(&info_path, info_content).map_err(|e| e.to_string())?;

    // Move the file
    fs::rename(path, &dest_path).map_err(|e| e.to_string())?;

    if options.verbose { println!("Moved '{}' to trash.", path.display()); }
    Ok(())
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
        if let Err(e) = delete_recursive(path, options) { eprintln!("{}", e); }
    } else {
        let c_path = CString::new(path_str).unwrap();
        if options.verbose { println!("Removing file: {}", path.display()); }
        if unsafe { unlink(c_path.as_ptr()) } != 0 {
            eprintln!("Failed to delete file '{}': {}", path.display(), Error::last_os_error());
        }
    }
}
