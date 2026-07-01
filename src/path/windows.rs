use windows_sys::Win32::System::Registry::{
    RegOpenKeyExW, RegQueryValueExW, RegSetValueExW, RegCloseKey,
    HKEY_CURRENT_USER, KEY_READ, KEY_WRITE, REG_EXPAND_SZ,
};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    SendMessageTimeoutW, HWND_BROADCAST, WM_SETTINGCHANGE, SMTO_ABORTIFHUNG,
};

fn get_user_path() -> Result<String, String> {
    unsafe {
        let key_name: Vec<u16> = "Environment".encode_utf16().chain(std::iter::once(0)).collect();
        let value_name: Vec<u16> = "Path".encode_utf16().chain(std::iter::once(0)).collect();
        
        let mut hkey = 0isize;
        if RegOpenKeyExW(HKEY_CURRENT_USER, key_name.as_ptr(), 0, KEY_READ, &mut hkey) != 0 {
            return Err("Failed to open registry key HKCU\\Environment".to_string());
        }
        
        let mut val_type = 0u32;
        let mut buf_size = 0u32;
        
        if RegQueryValueExW(hkey, value_name.as_ptr(), std::ptr::null_mut(), &mut val_type, std::ptr::null_mut(), &mut buf_size) != 0 {
            RegCloseKey(hkey);
            return Ok(String::new()); // Path might not exist
        }
        
        let mut buf = vec![0u16; (buf_size / 2) as usize];
        if RegQueryValueExW(hkey, value_name.as_ptr(), std::ptr::null_mut(), &mut val_type, buf.as_mut_ptr() as *mut u8, &mut buf_size) != 0 {
            RegCloseKey(hkey);
            return Err("Failed to query registry value PATH".to_string());
        }
        
        RegCloseKey(hkey);
        if let Some(last) = buf.last() {
            if *last == 0 {
                buf.pop();
            }
        }
        
        Ok(String::from_utf16_lossy(&buf))
    }
}

fn set_user_path(path_str: &str) -> Result<(), String> {
    unsafe {
        let key_name: Vec<u16> = "Environment".encode_utf16().chain(std::iter::once(0)).collect();
        let value_name: Vec<u16> = "Path".encode_utf16().chain(std::iter::once(0)).collect();
        
        let mut hkey = 0isize;
        if RegOpenKeyExW(HKEY_CURRENT_USER, key_name.as_ptr(), 0, KEY_WRITE, &mut hkey) != 0 {
            return Err("Failed to open registry key HKCU\\Environment for writing".to_string());
        }
        
        let wide_val: Vec<u16> = path_str.encode_utf16().chain(std::iter::once(0)).collect();
        let bytes_len = wide_val.len() * 2;
        
        if RegSetValueExW(hkey, value_name.as_ptr(), 0, REG_EXPAND_SZ, wide_val.as_ptr() as *const u8, bytes_len as u32) != 0 {
            RegCloseKey(hkey);
            return Err("Failed to write registry value PATH".to_string());
        }
        
        RegCloseKey(hkey);
        
        let env_str: Vec<u16> = "Environment".encode_utf16().chain(std::iter::once(0)).collect();
        let mut result = 0usize;
        SendMessageTimeoutW(
            HWND_BROADCAST as isize,
            WM_SETTINGCHANGE,
            0,
            env_str.as_ptr() as isize,
            SMTO_ABORTIFHUNG,
            5000,
            &mut result,
        );
        
        Ok(())
    }
}

pub fn list_path() {
    println!("User Registry PATH (HKCU\\Environment):");
    match get_user_path() {
        Ok(path) => {
            if path.is_empty() {
                println!("  <Empty>");
            } else {
                for entry in path.split(';') {
                    let trimmed = entry.trim();
                    if !trimmed.is_empty() {
                        println!("  {}", trimmed);
                    }
                }
            }
        }
        Err(e) => println!("  Error: {}", e),
    }

    println!("\nActive Process PATH:");
    if let Ok(process_path) = std::env::var("PATH") {
        for entry in process_path.split(';') {
            let trimmed = entry.trim();
            if !trimmed.is_empty() {
                println!("  {}", trimmed);
            }
        }
    } else {
        println!("  <None>");
    }
}

pub fn add_path(dir: &str) {
    let normalized = dir.replace("/", "\\");
    
    let path = match get_user_path() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    let mut entries: Vec<String> = path.split(';')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    // Case-insensitive duplicates check on Windows
    let exists = entries.iter().any(|e| e.to_lowercase() == normalized.to_lowercase());
    if exists {
        println!("Info: Directory '{}' is already in user PATH.", normalized);
        return;
    }

    entries.push(normalized.clone());
    let new_path = entries.join(";");

    if let Err(e) = set_user_path(&new_path) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }

    println!("Success: Added '{}' to user PATH.", normalized);
}

pub fn remove_path(dir: &str) {
    let normalized = dir.replace("/", "\\");
    
    let path = match get_user_path() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    let entries: Vec<String> = path.split(';')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    let initial_count = entries.len();
    let filtered_entries: Vec<String> = entries.into_iter()
        .filter(|e| e.to_lowercase() != normalized.to_lowercase())
        .collect();

    if filtered_entries.len() == initial_count {
        println!("Info: Directory '{}' was not found in user PATH.", normalized);
        return;
    }

    let new_path = filtered_entries.join(";");

    if let Err(e) = set_user_path(&new_path) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }

    println!("Success: Removed '{}' from user PATH.", normalized);
}
