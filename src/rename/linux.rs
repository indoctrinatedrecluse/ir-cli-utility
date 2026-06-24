use crate::RenameOptions;
use std::ffi::CString;
use std::io::{self, Write, Error};
use std::path::Path;
use libc::{lstat, S_IFDIR, S_IFLNK};
use std::mem::MaybeUninit;

pub fn rename(source: &str, destination: &str, options: RenameOptions) {
    let source_path = Path::new(source);
    let dest_path = Path::new(destination);

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

    // 5. Handle force overwrite
    if options.force {
        // On Linux, `rename` overwrites by default. However, we should check if the destination
        // exists to provide a consistent experience with the `-f` flag.
        if Path::new(destination).exists() {
            // If we are here, it means we are overwriting.
        }
    } else {
        // If not forcing, check if destination exists and fail if it does.
        if Path::new(destination).exists() {
            eprintln!("Error: Destination '{}' already exists. Use -f or --force to overwrite.", destination);
            return;
        }
    }

    // 6. Perform the rename
    let c_dest = match CString::new(destination) {
        Ok(s) => s,
        Err(_) => { eprintln!("Error: Destination name contains invalid characters."); return; }
    };

    if unsafe { libc::rename(c_source.as_ptr(), c_dest.as_ptr()) } != 0 {
        eprintln!("Error renaming '{}' to '{}': {}", source, destination, Error::last_os_error());
    } else {
        println!("Successfully renamed '{}' to '{}'.", source, destination);
    }
}
