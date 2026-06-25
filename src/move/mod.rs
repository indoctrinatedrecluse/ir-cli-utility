use crate::MoveOptions;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub fn move_item(source: &str, destination: &str, options: MoveOptions) {
    linux::move_item(source, destination, options);
}

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub fn move_item(source: &str, destination: &str, options: MoveOptions) {
    windows::move_item(source, destination, options);
}
