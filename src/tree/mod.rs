use crate::TreeOptions;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub fn tree(path: &str, options: TreeOptions) {
    linux::tree(path, options);
}

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub fn tree(path: &str, options: TreeOptions) {
    windows::tree(path, options);
}
