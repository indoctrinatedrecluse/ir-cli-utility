use crate::LnOptions;
use std::ffi::CString;
use std::io::Error;
use std::path::Path;

pub fn ln(target: &str, link_name: &str, options: LnOptions) {
    let link_path = Path::new(link_name);
    
    if options.force && (link_path.exists() || link_path.is_symlink()) {
        if let Err(e) = std::fs::remove_file(link_path) {
            if let Err(e2) = std::fs::remove_dir_all(link_path) {
                eprintln!("Error: Failed to force remove existing destination: {} (also tried dir: {})", e, e2);
                return;
            }
        }
    }

    let c_target = match CString::new(target) {
        Ok(s) => s,
        Err(_) => {
            eprintln!("Error: Invalid target name (contains null byte).");
            return;
        }
    };

    let c_link = match CString::new(link_name) {
        Ok(s) => s,
        Err(_) => {
            eprintln!("Error: Invalid link name (contains null byte).");
            return;
        }
    };

    let res = unsafe {
        if options.symbolic {
            libc::symlink(c_target.as_ptr(), c_link.as_ptr())
        } else {
            libc::link(c_target.as_ptr(), c_link.as_ptr())
        }
    };

    if res != 0 {
        eprintln!("Error: Link creation failed: {}", Error::last_os_error());
    }
}
