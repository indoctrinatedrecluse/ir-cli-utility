use crate::WhoamiOptions;
use std::ffi::CStr;
use std::mem::MaybeUninit;

pub fn whoami(_options: WhoamiOptions) {
    unsafe {
        let uid = libc::geteuid();
        let mut pwd = MaybeUninit::<libc::passwd>::uninit();
        let mut buf = vec![0; 1024];
        let mut result = std::ptr::null_mut();

        if libc::getpwuid_r(
            uid,
            pwd.as_mut_ptr(),
            buf.as_mut_ptr() as *mut libc::c_char,
            buf.len(),
            &mut result,
        ) == 0 && !result.is_null() {
            let pwd = pwd.assume_init();
            let username = CStr::from_ptr(pwd.pw_name).to_string_lossy().into_owned();
            println!("{}", username);
        } else {
            // Fallback to reading environment variables if system lookup fails
            if let Ok(user) = std::env::var("USER") {
                println!("{}", user);
            } else if let Ok(user) = std::env::var("LOGNAME") {
                println!("{}", user);
            } else {
                eprintln!("Error: Failed to retrieve user identity.");
            }
        }
    }
}
