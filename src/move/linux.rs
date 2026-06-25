use crate::MoveOptions;
use std::ffi::CString;
use std::io::Error;
use std::path::{Path, PathBuf};
use libc;

pub fn move_item(source: &str, destination: &str, options: MoveOptions) {
    let source_path = Path::new(source);
    let dest_path = Path::new(destination);

    if !source_path.exists() {
        eprintln!("Error: Source '{}' not found.", source);
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

    if !options.force && final_dest_path.exists() {
        eprintln!("Error: Destination '{}' already exists. Use --force to overwrite.", final_dest_path.display());
        return;
    }

    let c_source = CString::new(source).unwrap();
    let c_dest = CString::new(final_dest_path.as_os_str().to_str().unwrap()).unwrap();

    if unsafe { libc::rename(c_source.as_ptr(), c_dest.as_ptr()) } != 0 {
        eprintln!("Error moving '{}' to '{}': {}", source, final_dest_path.display(), Error::last_os_error());
    } else {
        println!("Successfully moved '{}' to '{}'.", source, final_dest_path.display());
    }
}
