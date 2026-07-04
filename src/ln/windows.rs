use crate::LnOptions;
use std::io::Error;
use std::os::windows::ffi::OsStrExt;
use std::path::Path;

const SYMBOLIC_LINK_FLAG_DIRECTORY: u32 = 0x1;
const SYMBOLIC_LINK_FLAG_ALLOW_UNPRIVILEGED_CREATE: u32 = 0x2;

fn to_wide_chars(s: &str) -> Vec<u16> {
    Path::new(s)
        .as_os_str()
        .encode_wide()
        .chain(std::iter::once(0))
        .collect()
}

pub fn ln(target: &str, link_name: &str, options: LnOptions) {
    let link_path = Path::new(link_name);

    if options.force && (link_path.exists() || link_path.is_symlink()) {
        if let Err(e) = std::fs::remove_file(link_path) {
            if let Err(e2) = std::fs::remove_dir(link_path) {
                if let Err(e3) = std::fs::remove_dir_all(link_path) {
                    eprintln!("Error: Failed to force remove existing destination: {} (also tried dir: {}, dir_all: {})", e, e2, e3);
                    return;
                }
            }
        }
    }

    let target_wide = to_wide_chars(target);
    let link_wide = to_wide_chars(link_name);

    let success = unsafe {
        if options.symbolic {
            let is_dir = Path::new(target).is_dir() 
                || target.ends_with('\\') 
                || target.ends_with('/');
            
            let mut flags = SYMBOLIC_LINK_FLAG_ALLOW_UNPRIVILEGED_CREATE;
            if is_dir {
                flags |= SYMBOLIC_LINK_FLAG_DIRECTORY;
            }

            windows_sys::Win32::Storage::FileSystem::CreateSymbolicLinkW(
                link_wide.as_ptr(),
                target_wide.as_ptr(),
                flags,
            ) != 0
        } else {
            windows_sys::Win32::Storage::FileSystem::CreateHardLinkW(
                link_wide.as_ptr(),
                target_wide.as_ptr(),
                std::ptr::null(),
            ) != 0
        }
    };

    if !success {
        eprintln!("Error: Link creation failed: {}", Error::last_os_error());
    }
}
