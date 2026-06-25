use libc::{close, open, read, O_RDONLY};
use std::ffi::CString;
use std::io::Error;

pub fn read_file(path: &str) -> Result<Vec<u8>, String> {
    let c_path = CString::new(path).map_err(|_| "path contains invalid characters".to_string())?;
    let fd = unsafe { open(c_path.as_ptr(), O_RDONLY) };

    if fd < 0 {
        return Err(Error::last_os_error().to_string());
    }

    let mut bytes = Vec::new();
    let mut buffer = [0u8; 8192];

    loop {
        let read_count = unsafe { read(fd, buffer.as_mut_ptr() as *mut _, buffer.len()) };

        if read_count < 0 {
            let message = Error::last_os_error().to_string();
            unsafe { close(fd) };
            return Err(message);
        }

        if read_count == 0 {
            break;
        }

        bytes.extend_from_slice(&buffer[..read_count as usize]);
    }

    if unsafe { close(fd) } != 0 {
        return Err(Error::last_os_error().to_string());
    }

    Ok(bytes)
}
