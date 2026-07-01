use windows_sys::Win32::System::DataExchange::{
    OpenClipboard, CloseClipboard, EmptyClipboard, SetClipboardData, GetClipboardData,
};
use windows_sys::Win32::System::Memory::{
    GlobalAlloc, GlobalLock, GlobalUnlock, GMEM_MOVEABLE,
};

extern "system" {
    fn GlobalFree(hmem: *mut std::ffi::c_void) -> *mut std::ffi::c_void;
}

pub fn get_clipboard() -> Result<String, String> {
    unsafe {
        if OpenClipboard(0) == 0 {
            return Err("Failed to open clipboard".to_string());
        }
        
        let handle = GetClipboardData(13); // CF_UNICODETEXT is 13
        if handle == 0 {
            CloseClipboard();
            return Ok(String::new()); // Empty or non-text content
        }
        
        let ptr = GlobalLock(handle as *mut std::ffi::c_void);
        if ptr.is_null() {
            CloseClipboard();
            return Err("Failed to lock global memory".to_string());
        }
        
        let ptr_u16 = ptr as *const u16;
        let mut len = 0;
        while *ptr_u16.add(len) != 0 {
            len += 1;
        }
        
        let slice = std::slice::from_raw_parts(ptr_u16, len);
        let text = String::from_utf16_lossy(slice);
        
        GlobalUnlock(handle as *mut std::ffi::c_void);
        CloseClipboard();
        
        Ok(text)
    }
}

pub fn set_clipboard(text: &str) -> Result<(), String> {
    unsafe {
        let wide: Vec<u16> = text.encode_utf16().chain(std::iter::once(0)).collect();
        let bytes_len = wide.len() * 2;
        
        let handle = GlobalAlloc(GMEM_MOVEABLE, bytes_len);
        if handle.is_null() {
            return Err("Failed to allocate global memory".to_string());
        }
        
        let ptr = GlobalLock(handle);
        if ptr.is_null() {
            GlobalFree(handle);
            return Err("Failed to lock global memory".to_string());
        }
        
        std::ptr::copy_nonoverlapping(wide.as_ptr(), ptr as *mut u16, wide.len());
        
        GlobalUnlock(handle);
        
        if OpenClipboard(0) == 0 {
            GlobalFree(handle);
            return Err("Failed to open clipboard".to_string());
        }
        
        if EmptyClipboard() == 0 {
            CloseClipboard();
            GlobalFree(handle);
            return Err("Failed to empty clipboard".to_string());
        }
        
        if SetClipboardData(13, handle as isize) == 0 { // CF_UNICODETEXT is 13
            CloseClipboard();
            GlobalFree(handle);
            return Err("Failed to set clipboard data".to_string());
        }
        
        CloseClipboard();
        Ok(())
    }
}

pub fn clear_clipboard() -> Result<(), String> {
    unsafe {
        if OpenClipboard(0) == 0 {
            return Err("Failed to open clipboard".to_string());
        }
        if EmptyClipboard() == 0 {
            CloseClipboard();
            return Err("Failed to empty clipboard".to_string());
        }
        CloseClipboard();
        Ok(())
    }
}
