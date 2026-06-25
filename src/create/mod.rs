use crate::CreateOptions;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub fn create(path: &str, options: CreateOptions) {
    linux::create(path, options);
}

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub fn create(path: &str, options: CreateOptions) {
    windows::create(path, options);
}
