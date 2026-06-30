use crate::DuOptions;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub fn du(paths: Vec<String>, options: DuOptions) {
    linux::du(paths, options);
}

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub fn du(paths: Vec<String>, options: DuOptions) {
    windows::du(paths, options);
}
