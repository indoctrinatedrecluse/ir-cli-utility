use crate::PsOptions;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub fn ps(options: PsOptions) {
    linux::ps(options);
}

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub fn ps(options: PsOptions) {
    windows::ps(options);
}
