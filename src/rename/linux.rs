use crate::RenameOptions;
use std::ffi::CString;
use std::io::{self, Write, Error};
use std::path::{Path, PathBuf};
use libc::{lstat, S_IFDIR, S_IFLNK};
use std::mem::MaybeUninit;

pub fn rename(source: &str, new_name: &str, options: RenameOptions) {
    let source_path = Path::new(source);

    // Construct the full destination path
    let mut dest_path = if let Some(parent) = source_path.parent() {
        parent.to_path_buf()
    } else {
        PathBuf::from(".")
    };
    dest_path.push(new_name);

    // 1. Check if source exists
    let c_source = match CString::new(source) {
        Ok(s) => s,
        Err(_) => { eprintln!("Error: Source name contains invalid characters."); return; }
    };

    let mut stat_buf = MaybeUninit::uninit();
    if unsafe { lstat(c_source.as_ptr(), stat_buf.as_mut_ptr()) } != 0 {
        eprintln!("Error: Source '{}' not found. ({})", source, Error::last_os_error());
        return;
    }
    let stat_info = unsafe { stat_buf.assume_init() };
    let source_is_dir = (stat_info.st_mode & libc::S_IFMT) == S_IFDIR;
    let source_is_link = (stat_info.st_mode & libc::S_IFMT) == S_IFLNK;

    // 2. Handle symbolic links
    if source_is_link && !options.force_links {
        eprintln!("Error: Source '{}' is a symbolic link. Use --force-links to rename it.", source);
        return;
    }

    // 3. Prevent renaming a folder to a file
    if source_is_dir && dest_path.extension().is_some() {
        eprintln!("Error: Cannot rename a folder ('{}') to a file with an extension ('{}').", source, new_name);
        return;
    }

    // 4. Interactive prompt
    if options.interactive {
        print!("Rename '{}' to '{}'? [y/N] ", source, dest_path.display());
        io::stdout().flush().unwrap();
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() || input.trim().to_lowercase() != "y" {
            println!("Operation cancelled.");
            return;
        }
    }

    // 5. Handle force overwrite
    if !options.force && dest_path.exists() {
        eprintln!("Error: Destination '{}' already exists. Use -f or --force to overwrite.", dest_path.display());
        return;
    }

    // 6. Perform the rename
    let c_dest = match CString::new(dest_path.as_os_str().to_str().unwrap_or_default()) {
        Ok(s) => s,
        Err(_) => { eprintln!("Error: Destination name contains invalid characters."); return; }
    };

    if unsafe { libc::rename(c_source.as_ptr(), c_dest.as_ptr()) } != 0 {
        eprintln!("Error renaming '{}' to '{}': {}", source, dest_path.display(), Error::last_os_error());
    } else {
        println!("Successfully renamed '{}' to '{}'.", source, dest_path.display());
    }
}
