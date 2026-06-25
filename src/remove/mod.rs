use crate::RemoveOptions;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub fn remove(path: &str, options: &RemoveOptions) {
    linux::remove(path, options);
}

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub fn remove(path: &str, options: &RemoveOptions) {
    windows::remove(path, options);
}
