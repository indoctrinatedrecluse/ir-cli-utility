use crate::LnOptions;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub fn ln(target: &str, link_name: &str, options: LnOptions) {
    linux::ln(target, link_name, options);
}

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub fn ln(target: &str, link_name: &str, options: LnOptions) {
    windows::ln(target, link_name, options);
}
