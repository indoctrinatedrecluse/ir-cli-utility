use std::ffi::CString;
use std::io::{Error, Result};
use std::os::unix::ffi::OsStrExt;
use std::path::Path;

pub fn chmod_single(path: &Path, mode: u32) -> Result<()> {
    let c_path = CString::new(path.as_os_str().as_bytes())
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, e))?;
    let res = unsafe { libc::chmod(c_path.as_ptr(), mode as libc::mode_t) };
    if res != 0 {
        Err(Error::last_os_error())
    } else {
        Ok(())
    }
}
