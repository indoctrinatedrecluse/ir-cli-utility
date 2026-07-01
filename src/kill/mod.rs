use crate::KillOptions;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub fn kill(target: &str, options: KillOptions) {
    linux::kill(target, options);
}

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub fn kill(target: &str, options: KillOptions) {
    windows::kill(target, options);
}
