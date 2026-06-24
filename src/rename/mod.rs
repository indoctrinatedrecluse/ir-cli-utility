use crate::RenameOptions;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub fn rename(source: &str, destination: &str, options: RenameOptions) {
    linux::rename(source, destination, options);
}

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub fn rename(source: &str, destination: &str, options: RenameOptions) {
    windows::rename(source, destination, options);
}
