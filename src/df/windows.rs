use crate::DfOptions;
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::ptr::null_mut;
use windows_sys::Win32::Storage::FileSystem::{
    GetDiskFreeSpaceExW, GetDriveTypeW, GetLogicalDriveStringsW, GetVolumeInformationW,
};

#[allow(dead_code)]
const DRIVE_UNKNOWN: u32 = 0;
#[allow(dead_code)]
const DRIVE_NO_ROOT_DIR: u32 = 1;
const DRIVE_REMOVABLE: u32 = 2;
const DRIVE_FIXED: u32 = 3;
const DRIVE_REMOTE: u32 = 4;
const DRIVE_CDROM: u32 = 5;
const DRIVE_RAMDISK: u32 = 6;

pub fn df(options: DfOptions) {
    let mut buffer = [0u16; 512];
    let len = unsafe { GetLogicalDriveStringsW(buffer.len() as u32, buffer.as_mut_ptr()) };
    if len == 0 {
        eprintln!("Error: Failed to retrieve logical drives.");
        return;
    }

    println!(
        "{:<8} {:<8} {:>10} {:>10} {:>10} {:>6}  {:<20}",
        "Device", "Type", "Size", "Used", "Available", "Use%", "Volume"
    );

    let mut slice = &buffer[..len as usize];
    while !slice.is_empty() {
        if let Some(null_pos) = slice.iter().position(|&c| c == 0) {
            let drive_wide = &slice[..null_pos];
            slice = &slice[null_pos + 1..];

            if drive_wide.is_empty() {
                continue;
            }

            let drive_path = OsString::from_wide(drive_wide).to_string_lossy().into_owned();
            let drive_type = unsafe { GetDriveTypeW(drive_wide.as_ptr()) };

            let should_show = match drive_type {
                DRIVE_FIXED | DRIVE_RAMDISK => true,
                DRIVE_REMOVABLE => true,
                DRIVE_REMOTE | DRIVE_CDROM => options.all,
                _ => options.all,
            };

            if !should_show {
                continue;
            }

            let mut volume_name = [0u16; 256];
            let mut fs_name = [0u16; 256];
            let vol_res = unsafe {
                GetVolumeInformationW(
                    drive_wide.as_ptr(),
                    volume_name.as_mut_ptr(),
                    volume_name.len() as u32,
                    null_mut(),
                    null_mut(),
                    null_mut(),
                    fs_name.as_mut_ptr(),
                    fs_name.len() as u32,
                )
            };

            let label = if vol_res != 0 {
                let len = volume_name.iter().position(|&c| c == 0).unwrap_or(volume_name.len());
                OsString::from_wide(&volume_name[..len]).to_string_lossy().into_owned()
            } else {
                String::new()
            };

            let fs_type = if vol_res != 0 {
                let len = fs_name.iter().position(|&c| c == 0).unwrap_or(fs_name.len());
                OsString::from_wide(&fs_name[..len]).to_string_lossy().into_owned()
            } else {
                match drive_type {
                    DRIVE_CDROM => "CDFS".to_string(),
                    DRIVE_REMOTE => "Network".to_string(),
                    DRIVE_REMOVABLE => "FAT32".to_string(),
                    _ => "Unknown".to_string(),
                }
            };

            let mut free_bytes_avail = 0u64;
            let mut total_bytes = 0u64;
            let mut total_free_bytes = 0u64;

            let mut drive_wide_null = drive_wide.to_vec();
            drive_wide_null.push(0);

            let space_res = unsafe {
                GetDiskFreeSpaceExW(
                    drive_wide_null.as_ptr(),
                    &mut free_bytes_avail,
                    &mut total_bytes,
                    &mut total_free_bytes,
                )
            };

            if space_res == 0 {
                if options.all {
                    println!(
                        "{:<8} {:<8} {:>10} {:>10} {:>10} {:>6}  {:<20}",
                        drive_path.trim_end_matches('\\'),
                        fs_type,
                        "-",
                        "-",
                        "-",
                        "-",
                        label
                    );
                }
                continue;
            }

            let used_bytes = total_bytes.saturating_sub(total_free_bytes);
            let use_percent = if total_bytes > 0 {
                (used_bytes as f64 / total_bytes as f64 * 100.0).round() as u64
            } else {
                0
            };

            let (size_str, used_str, avail_str) = if options.human_readable {
                (
                    super::format_size_human(total_bytes),
                    super::format_size_human(used_bytes),
                    super::format_size_human(free_bytes_avail),
                )
            } else {
                (
                    format!("{}", total_bytes / 1024),
                    format!("{}", used_bytes / 1024),
                    format!("{}", free_bytes_avail / 1024),
                )
            };

            println!(
                "{:<8} {:<8} {:>10} {:>10} {:>10} {:>5}%  {:<20}",
                drive_path.trim_end_matches('\\'),
                fs_type,
                size_str,
                used_str,
                avail_str,
                use_percent,
                label
            );
        } else {
            break;
        }
    }
}
