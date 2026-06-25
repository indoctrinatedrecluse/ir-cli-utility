use crate::CopyOptions;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub fn copy(source: &str, destination: &str, options: CopyOptions) {
    linux::copy(source, destination, options);
}

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub fn copy(source: &str, destination: &str, options: CopyOptions) {
    windows::copy(source, destination, options);
}
