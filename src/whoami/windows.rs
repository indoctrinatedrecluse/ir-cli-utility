use crate::WhoamiOptions;
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use windows_sys::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};
use windows_sys::Win32::Security::{
    GetTokenInformation, LookupAccountSidW, TokenUser, TOKEN_QUERY, TOKEN_USER,
};
use windows_sys::Win32::Foundation::{CloseHandle, GetLastError};

pub fn whoami(_options: WhoamiOptions) {
    unsafe {
        let mut token_handle = 0isize;
        if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token_handle) == 0 {
            eprintln!("Error: OpenProcessToken failed (code: {}).", GetLastError());
            return;
        }

        let mut len = 0u32;
        // First query for size
        GetTokenInformation(token_handle, TokenUser, std::ptr::null_mut(), 0, &mut len);
        if len == 0 {
            eprintln!("Error: GetTokenInformation size query failed.");
            CloseHandle(token_handle);
            return;
        }

        let mut buffer = vec![0u8; len as usize];
        if GetTokenInformation(token_handle, TokenUser, buffer.as_mut_ptr() as *mut _, len, &mut len) == 0 {
            eprintln!("Error: GetTokenInformation failed (code: {}).", GetLastError());
            CloseHandle(token_handle);
            return;
        }

        let token_user = &*(buffer.as_ptr() as *const TOKEN_USER);
        let sid = token_user.User.Sid;

        let mut name_len = 256u32;
        let mut name = vec![0u16; 256];
        let mut domain_len = 256u32;
        let mut domain = vec![0u16; 256];
        let mut use_sid = 0i32;

        if LookupAccountSidW(
            std::ptr::null(),
            sid,
            name.as_mut_ptr(),
            &mut name_len,
            domain.as_mut_ptr(),
            &mut domain_len,
            &mut use_sid,
        ) == 0 {
            eprintln!("Error: LookupAccountSidW failed (code: {}).", GetLastError());
            CloseHandle(token_handle);
            return;
        }

        let user_str = OsString::from_wide(&name[..name_len as usize]).to_string_lossy().into_owned();
        let domain_str = OsString::from_wide(&domain[..domain_len as usize]).to_string_lossy().into_owned();

        println!("{}\\{}", domain_str, user_str);

        CloseHandle(token_handle);
    }
}
