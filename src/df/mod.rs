use crate::DfOptions;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub fn df(options: DfOptions) {
    linux::df(options);
}

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub fn df(options: DfOptions) {
    windows::df(options);
}

pub fn format_size_human(bytes: u64) -> String {
    if bytes == 0 { return "0B".to_string(); }
    const UNITS: [&str; 5] = ["B", "K", "M", "G", "T"];
    let bytes_f = bytes as f64;
    let digit_groups = (bytes_f.log10() / 1024.0f64.log10()).floor() as i32;
    let unit_index = digit_groups.min(UNITS.len() as i32 - 1).max(0);
    
    if unit_index == 0 {
        return format!("{}B", bytes);
    }
    
    let size = bytes_f / 1024.0f64.powi(unit_index);
    if size >= 10.0 {
        format!("{:.0}{}", size.round(), UNITS[unit_index as usize])
    } else {
        format!("{:.1}{}", size, UNITS[unit_index as usize])
    }
}
