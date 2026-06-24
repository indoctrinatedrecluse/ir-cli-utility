use crate::ListOptions;
use std::ffi::{CStr, CString};
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use libc::{opendir, readdir, closedir, stat, lstat, S_IFMT, S_IFDIR, S_IFLNK, S_IRUSR, S_IWUSR, S_IXUSR, S_IRGRP, S_IWGRP, S_IXGRP, S_IROTH, S_IWOTH, S_IXOTH, getpwuid, getgrgid};
use std::mem::MaybeUninit;
use std::time::SystemTime;
use chrono::{DateTime, Local};

struct FileInfo {
    name: String,
    permissions: String,
    size: u64,
    size_formatted: String,
    owner: String,
    group: String,
    modified: String,
    modified_sec: i64,
    changed: String,
}

// ... (helper functions remain the same)
fn format_permissions(mode: u32) -> String {
    let type_char = match mode & S_IFMT {
        S_IFDIR => 'd',
        S_IFLNK => 'l',
        _ => '-',
    };
    let user = format!("{}{}{}", if mode & S_IRUSR != 0 { 'r' } else { '-' }, if mode & S_IWUSR != 0 { 'w' } else { '-' }, if mode & S_IXUSR != 0 { 'x' } else { '-' });
    let group = format!("{}{}{}", if mode & S_IRGRP != 0 { 'r' } else { '-' }, if mode & S_IWGRP != 0 { 'w' } else { '-' }, if mode & S_IXGRP != 0 { 'x' } else { '-' });
    let other = format!("{}{}{}", if mode & S_IROTH != 0 { 'r' } else { '-' }, if mode & S_IWOTH != 0 { 'w' } else { '-' }, if mode & S_IXOTH != 0 { 'x' } else { '-' });
    format!("{}{}{}{}", type_char, user, group, other)
}

fn format_size(bytes: u64) -> String {
    if bytes == 0 { return "0 B".to_string(); }
    const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];
    let digit_groups = (bytes as f64).log10() / (1024.0f64).log10();
    let unit_index = (digit_groups.floor() as i32).min(UNITS.len() as i32 - 1);
    let size = bytes as f64 / 1024.0f64.powi(unit_index);
    format!("{:.2} {}", size, UNITS[unit_index as usize])
}

fn format_time(sec: i64, nsec: i64) -> String {
    let timestamp = SystemTime::UNIX_EPOCH + std::time::Duration::new(sec as u64, nsec as u32);
    let datetime = DateTime::<Local>::from(timestamp);
    datetime.format("%Y-%m-%d %H:%M").to_string()
}

fn get_user_name(uid: u32) -> String {
    let pwd = unsafe { getpwuid(uid) };
    if pwd.is_null() { uid.to_string() } else { unsafe { CStr::from_ptr((*pwd).pw_name) }.to_string_lossy().into_owned() }
}

fn get_group_name(gid: u32) -> String {
    let grp = unsafe { getgrgid(gid) };
    if grp.is_null() { gid.to_string() } else { unsafe { CStr::from_ptr((*grp).gr_name) }.to_string_lossy().into_owned() }
}

pub fn list(options: ListOptions) {
    let mut files: Vec<FileInfo> = Vec::new();
    let path = CString::new(".").unwrap();
    let dir = unsafe { opendir(path.as_ptr()) };

    if dir.is_null() {
        eprintln!("Error opening directory.");
        return;
    }

    loop {
        let entry = unsafe { readdir(dir) };
        if entry.is_null() { break; }

        let d_name = unsafe { (*entry).d_name.as_ptr() };
        let filename = unsafe { CStr::from_ptr(d_name) };
        let filename_str = filename.to_string_lossy();

        if !options.show_all && filename_str.starts_with('.') {
            continue;
        }
        if filename_str == "." || filename_str == ".." {
            continue;
        }

        let mut stat_buf: MaybeUninit<stat> = MaybeUninit::uninit();
        let c_path = CString::new(filename_str.as_ref()).unwrap();

        if unsafe { lstat(c_path.as_ptr(), stat_buf.as_mut_ptr()) } == 0 {
            let stat_info = unsafe { stat_buf.assume_init() };
            let size = stat_info.st_size as u64;
            files.push(FileInfo {
                name: filename_str.into_owned(),
                permissions: format_permissions(stat_info.st_mode),
                size,
                size_formatted: format_size(size),
                owner: get_user_name(stat_info.st_uid),
                group: get_group_name(stat_info.st_gid),
                modified: format_time(stat_info.st_mtime, stat_info.st_mtime_nsec),
                modified_sec: stat_info.st_mtime,
                changed: format_time(stat_info.st_ctime, stat_info.st_ctime_nsec),
            });
        }
    }
    unsafe { closedir(dir) };

    // Filtering
    let filtered_files: Vec<FileInfo> = if let Some(ext) = options.filter {
        files.into_iter().filter(|file| {
            Path::new(&file.name).extension().and_then(std::ffi::OsStr::to_str) == Some(&ext)
        }).collect()
    } else {
        files
    };

    // Sorting
    let mut sorted_files = filtered_files;
    if options.sort_by_size {
        sorted_files.sort_by(|a, b| b.size.cmp(&a.size));
    } else if options.sort_by_time {
        sorted_files.sort_by(|a, b| b.modified_sec.cmp(&a.modified_sec));
    }

    // Printing
    println!("{:<12} {:<10} {:<10} {:<10} {:<20} {:<20} {}", "Permissions", "Size", "Owner", "Group", "Modified", "Changed", "Name");
    println!("{:-<12} {:-<10} {:-<10} {:-<10} {:-<20} {:-<20} {:-<30}", "", "", "", "", "", "", "");
    for file in sorted_files {
        println!(
            "{:<12} {:<10} {:<10} {:<10} {:<20} {:<20} {}",
            file.permissions, file.size_formatted, file.owner, file.group, file.modified, file.changed, file.name
        );
    }
}
