use crate::ListOptions;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub fn list(options: ListOptions) {
    linux::list(options);
}

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub fn list(options: ListOptions) {
    windows::list(options);
}
