use crate::ListOptions;
use std::ffi::{OsStr, OsString};
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use std::path::Path;
use std::mem::zeroed;

use windows_sys::Win32::Foundation::{
    INVALID_HANDLE_VALUE,
    FILETIME,
    SYSTEMTIME,
};
use windows_sys::Win32::Storage::FileSystem::{
    FindFirstFileW,
    FindNextFileW,
    FindClose,
    WIN32_FIND_DATAW,
    FILE_ATTRIBUTE_DIRECTORY,
    FILE_ATTRIBUTE_READONLY,
    FILE_ATTRIBUTE_REPARSE_POINT,
    FILE_ATTRIBUTE_HIDDEN,
    FILE_ATTRIBUTE_SYSTEM,
};
use windows_sys::Win32::System::Time::FileTimeToSystemTime;


struct FileInfo {
    name: String,
    permissions: String,
    size: u64,
    size_bytes_raw: String,   // always raw bytes for non-human mode
    size_human: String,       // IEC (KiB/MiB/GiB) for -h mode
    created: String,
    modified: String,
    modified_ft: FILETIME,
    is_dir: bool,
    is_symlink: bool,
    symlink_target: Option<String>,
}

// --- Helper functions ---
fn format_permissions(attributes: u32) -> String {
    let dir    = if attributes & FILE_ATTRIBUTE_DIRECTORY    != 0 { 'd' } else { '-' };
    let reparse= if attributes & FILE_ATTRIBUTE_REPARSE_POINT != 0 { 'l' } else { '-' };
    let readonly= if attributes & FILE_ATTRIBUTE_READONLY    != 0 { 'r' } else { '-' };
    let hidden  = if attributes & FILE_ATTRIBUTE_HIDDEN      != 0 { 'h' } else { '-' };
    let system  = if attributes & FILE_ATTRIBUTE_SYSTEM      != 0 { 's' } else { '-' };
    format!("{}{}{}{}{}", dir, reparse, readonly, hidden, system)
}

/// Format bytes as a compact decimal string with a unit suffix (KB/MB/GB).
fn format_size_decimal(bytes: u64) -> String {
    if bytes == 0 { return "0 B".to_string(); }
    const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];
    let digit_groups = (bytes as f64).log10() / (1024.0f64).log10();
    let unit_index = (digit_groups.floor() as i32).min(UNITS.len() as i32 - 1);
    let size = bytes as f64 / 1024.0f64.powi(unit_index);
    format!("{:.2} {}", size, UNITS[unit_index as usize])
}

/// Format bytes as an IEC human-readable string (KiB/MiB/GiB).
fn format_size_iec(bytes: u64) -> String {
    match bytes {
        0                                 => "0 B".to_string(),
        b if b >= 1024 * 1024 * 1024     => format!("{:.2} GiB", b as f64 / (1024.0 * 1024.0 * 1024.0)),
        b if b >= 1024 * 1024            => format!("{:.2} MiB", b as f64 / (1024.0 * 1024.0)),
        b if b >= 1024                   => format!("{:.2} KiB", b as f64 / 1024.0),
        b                                => format!("{} B", b),
    }
}

fn format_filetime(ft: &FILETIME) -> String {
    let mut st: SYSTEMTIME = unsafe { zeroed() };
    if unsafe { FileTimeToSystemTime(ft, &mut st) } != 0 {
        format!("{:04}-{:02}-{:02} {:02}:{:02}", st.wYear, st.wMonth, st.wDay, st.wHour, st.wMinute)
    } else { "N/A".to_string() }
}

/// Read the symlink target for a path using `std::fs::read_link`.
fn read_symlink_target(name: &str) -> Option<String> {
    std::fs::read_link(name)
        .ok()
        .map(|p| p.to_string_lossy().into_owned())
}
// --- End of helper functions ---

pub fn list(options: ListOptions) {
    let mut files: Vec<FileInfo> = Vec::new();
    let path: Vec<u16> = OsStr::new(".\\*").encode_wide().chain(Some(0)).collect();
    let mut find_data: WIN32_FIND_DATAW = unsafe { zeroed() };
    let find_handle = unsafe { FindFirstFileW(path.as_ptr(), &mut find_data) };

    if find_handle == INVALID_HANDLE_VALUE {
        eprintln!("Error listing files.");
        return;
    }

    loop {
        let filename_len = find_data.cFileName.iter().position(|&c| c == 0).unwrap_or(0);
        if filename_len > 0 {
            let filename_os = OsString::from_wide(&find_data.cFileName[..filename_len]);
            let filename_str = filename_os.to_string_lossy();

            if filename_str != "." && filename_str != ".." {
                let is_hidden = find_data.dwFileAttributes & FILE_ATTRIBUTE_HIDDEN != 0;
                if options.show_all || !is_hidden {
                    let is_dir = (find_data.dwFileAttributes & FILE_ATTRIBUTE_DIRECTORY) != 0;
                    let is_symlink = (find_data.dwFileAttributes & FILE_ATTRIBUTE_REPARSE_POINT) != 0;
                    let size = if is_dir { 0 } else {
                        (find_data.nFileSizeHigh as u64) << 32 | find_data.nFileSizeLow as u64
                    };

                    let symlink_target = if is_symlink {
                        read_symlink_target(filename_str.as_ref())
                    } else {
                        None
                    };

                    files.push(FileInfo {
                        name: filename_str.to_string(),
                        permissions: format_permissions(find_data.dwFileAttributes),
                        size,
                        size_bytes_raw: if is_dir { "---".to_string() } else { format_size_decimal(size) },
                        size_human:     if is_dir { "---".to_string() } else { format_size_iec(size) },
                        created:  format_filetime(&find_data.ftCreationTime),
                        modified: format_filetime(&find_data.ftLastWriteTime),
                        modified_ft: find_data.ftLastWriteTime,
                        is_dir,
                        is_symlink,
                        symlink_target,
                    });
                }
            }
        }

        if unsafe { FindNextFileW(find_handle, &mut find_data) } == 0 {
            break;
        }
    }
    unsafe { FindClose(find_handle) };

    // Filtering
    let filtered_files: Vec<FileInfo> = if let Some(ext) = options.filter {
        files.into_iter().filter(|file| {
            Path::new(&file.name).extension().and_then(OsStr::to_str) == Some(&ext)
        }).collect()
    } else {
        files
    };

    // Apply files-only or folders-only filter
    let filtered_files: Vec<FileInfo> = if options.files_only {
        filtered_files.into_iter().filter(|file| !file.is_dir).collect()
    } else if options.folders_only {
        filtered_files.into_iter().filter(|file| file.is_dir).collect()
    } else {
        filtered_files
    };

    // Sorting
    let mut sorted_files = filtered_files;
    if options.sort_by_size {
        sorted_files.sort_by(|a, b| b.size.cmp(&a.size));
    } else if options.sort_by_time {
        sorted_files.sort_by(|a, b| {
            let a_val = (a.modified_ft.dwHighDateTime as u64) << 32 | a.modified_ft.dwLowDateTime as u64;
            let b_val = (b.modified_ft.dwHighDateTime as u64) << 32 | b.modified_ft.dwLowDateTime as u64;
            b_val.cmp(&a_val)
        });
    }

    // Printing
    println!("{:<12} {:<10} {:<20} {:<20} {}", "Permissions", "Size", "Created", "Modified", "Name");
    println!("{:-<12} {:-<10} {:-<20} {:-<20} {:-<30}", "", "", "", "", "");
    for file in sorted_files {
        let size_col = if options.human_readable { &file.size_human } else { &file.size_bytes_raw };
        let name_col = if file.is_symlink {
            match &file.symlink_target {
                Some(target) => format!("{} -> {}", file.name, target),
                None         => format!("{} -> ?", file.name),
            }
        } else {
            file.name.clone()
        };
        println!(
            "{:<12} {:<10} {:<20} {:<20} {}",
            file.permissions, size_col, file.created, file.modified, name_col
        );
    }
}
