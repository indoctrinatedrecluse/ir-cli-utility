use crate::WhoamiOptions;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub fn whoami(options: WhoamiOptions) {
    linux::whoami(options);
}

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub fn whoami(options: WhoamiOptions) {
    windows::whoami(options);
}
