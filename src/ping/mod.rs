use crate::PingOptions;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub fn ping(host: &str, options: PingOptions) {
    linux::ping(host, options);
}

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub fn ping(host: &str, options: PingOptions) {
    windows::ping(host, options);
}
