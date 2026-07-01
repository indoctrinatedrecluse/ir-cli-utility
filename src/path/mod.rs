use crate::PathOptions;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub fn list_path() {
    linux::list_path();
}
#[cfg(target_os = "linux")]
pub fn add_path(dir: &str) {
    linux::add_path(dir);
}
#[cfg(target_os = "linux")]
pub fn remove_path(dir: &str) {
    linux::remove_path(dir);
}

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub fn list_path() {
    windows::list_path();
}
#[cfg(target_os = "windows")]
pub fn add_path(dir: &str) {
    windows::add_path(dir);
}
#[cfg(target_os = "windows")]
pub fn remove_path(dir: &str) {
    windows::remove_path(dir);
}

pub fn run_path(options: PathOptions) {
    if let Some(ref dir) = options.add {
        add_path(dir);
    } else if let Some(ref dir) = options.remove {
        remove_path(dir);
    } else {
        list_path();
    }
}
