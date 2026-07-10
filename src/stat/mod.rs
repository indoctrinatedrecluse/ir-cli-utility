use std::fs;
#[cfg(target_os = "windows")]
use std::fs::File;
use std::io;
#[cfg(target_os = "windows")]
use std::path::Path;
use chrono::{DateTime, Local};

pub struct StatOptions {
    pub files: Vec<String>,
    pub file_system: bool,
    pub format: Option<String>,
    pub terse: bool,
}

#[derive(Debug, Clone)]
struct StatInfo {
    name: String,
    size: u64,
    blocks: u64,
    block_size: u64,
    device_dec: u64,
    device_hex: u64,
    inode: u64,
    links: u64,
    mode: u32,
    file_type: String,
    uid: u32,
    gid: u32,
    user_name: String,
    group_name: String,
    optimal_io: u64,
    access_time: String,
    modify_time: String,
    change_time: String,
    birth_time: String,
}

#[derive(Debug, Clone)]
struct FsStatInfo {
    name: String,
    fs_id: String,
    namelen: u64,
    fs_type: String,
    block_size: u64,
    fundamental_block_size: u64,
    blocks_total: u64,
    blocks_free: u64,
    blocks_avail: u64,
}

pub fn run_stat(options: StatOptions) -> io::Result<()> {
    if options.files.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "missing operand",
        ));
    }

    for file_path in &options.files {
        if options.file_system {
            match get_fs_stat(file_path) {
                Ok(info) => print_fs_stat(&info, &options),
                Err(e) => eprintln!("ir stat: cannot read file system status for '{}': {}", file_path, e),
            }
        } else {
            match get_file_stat(file_path) {
                Ok(info) => print_file_stat(&info, &options),
                Err(e) => eprintln!("ir stat: cannot stat '{}': {}", file_path, e),
            }
        }
    }

    Ok(())
}

fn print_file_stat(info: &StatInfo, options: &StatOptions) {
    if let Some(ref fmt) = options.format {
        println!("{}", format_stat(fmt, info));
    } else if options.terse {
        println!(
            "{} {} {} {:x} {} {} {} {} {} {} {} {} {} {} {} {}",
            info.name, info.size, info.blocks, info.mode, info.uid, info.gid,
            info.device_dec, info.inode, info.links, 0, 0,
            info.access_time, info.modify_time, info.change_time, info.birth_time,
            info.optimal_io
        );
    } else {
        println!("  File: {}", info.name);
        println!(
            "  Size: {:<10}  Blocks: {:<10} IO Block: {:<10} {}",
            info.size, info.blocks, info.optimal_io, info.file_type
        );
        println!(
            "Device: {:x}h/{}d\tInode: {:<10} Links: {}",
            info.device_hex, info.device_dec, info.inode, info.links
        );
        println!(
            "Access: ({:04o}/{})  Uid: ( {}/ {:>8})   Gid: ( {}/ {:>8})",
            info.mode & 0o7777,
            format_mode_human(info.mode),
            info.uid,
            info.user_name,
            info.gid,
            info.group_name
        );
        println!("Access: {}", info.access_time);
        println!("Modify: {}", info.modify_time);
        println!("Change: {}", info.change_time);
        println!(" Birth: {}", info.birth_time);
    }
}

fn print_fs_stat(info: &FsStatInfo, options: &StatOptions) {
    if let Some(ref fmt) = options.format {
        // Simple fallback formatting for fs
        println!("{}", fmt.replace("%n", &info.name).replace("%t", &info.fs_type));
    } else if options.terse {
        println!(
            "{} {} {} {} {} {} {} {} {}",
            info.name, info.fs_id, info.namelen, info.fs_type,
            info.block_size, info.blocks_total, info.blocks_free, info.blocks_avail, 0
        );
    } else {
        println!("  File: \"{}\"", info.name);
        println!(
            "    ID: {:<16} Namelen: {:<10} Type: {}",
            info.fs_id, info.namelen, info.fs_type
        );
        println!(
            "Block size: {:<10} Fundamental block size: {}",
            info.block_size, info.fundamental_block_size
        );
        println!(
            "Blocks: Total: {:<10} Free: {:<10} Available: {}",
            info.blocks_total, info.blocks_free, info.blocks_avail
        );
    }
}

fn format_stat(format_str: &str, info: &StatInfo) -> String {
    let mut result = String::new();
    let mut chars = format_str.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '%' {
            if let Some(&next_c) = chars.peek() {
                let replaced = match next_c {
                    'a' => Some(format!("{:04o}", info.mode & 0o7777)),
                    'A' => Some(format_mode_human(info.mode)),
                    'b' => Some(info.blocks.to_string()),
                    'B' => Some(info.block_size.to_string()),
                    'd' => Some(info.device_dec.to_string()),
                    'D' => Some(format!("{:x}", info.device_hex)),
                    'f' => Some(format!("{:x}", info.mode)),
                    'F' => Some(info.file_type.clone()),
                    'g' => Some(info.gid.to_string()),
                    'G' => Some(info.group_name.clone()),
                    'h' => Some(info.links.to_string()),
                    'i' => Some(info.inode.to_string()),
                    'n' => Some(info.name.clone()),
                    'o' => Some(info.optimal_io.to_string()),
                    's' => Some(info.size.to_string()),
                    'u' => Some(info.uid.to_string()),
                    'U' => Some(info.user_name.clone()),
                    'x' => Some(info.access_time.clone()),
                    'y' => Some(info.modify_time.clone()),
                    'z' => Some(info.change_time.clone()),
                    'w' => Some(info.birth_time.clone()),
                    '%' => Some("%".to_string()),
                    _ => None,
                };
                if let Some(r) = replaced {
                    result.push_str(&r);
                    chars.next();
                } else {
                    result.push('%');
                }
            } else {
                result.push('%');
            }
        } else if c == '\\' {
            if let Some(&next_c) = chars.peek() {
                let esc = match next_c {
                    'n' => Some('\n'),
                    't' => Some('\t'),
                    '\\' => Some('\\'),
                    _ => None,
                };
                if let Some(e) = esc {
                    result.push(e);
                    chars.next();
                } else {
                    result.push('\\');
                }
            } else {
                result.push('\\');
            }
        } else {
            result.push(c);
        }
    }
    result
}

fn format_mode_human(mode: u32) -> String {
    let mut s = String::with_capacity(10);
    // Determine type prefix
    let type_char = if (mode & 0o170000) == 0o040000 {
        'd'
    } else if (mode & 0o170000) == 0o120000 {
        'l'
    } else {
        '-'
    };
    s.push(type_char);

    // Owner permissions
    s.push(if (mode & 0o400) != 0 { 'r' } else { '-' });
    s.push(if (mode & 0o200) != 0 { 'w' } else { '-' });
    s.push(if (mode & 0o100) != 0 { 'x' } else { '-' });

    // Group permissions
    s.push(if (mode & 0o040) != 0 { 'r' } else { '-' });
    s.push(if (mode & 0o020) != 0 { 'w' } else { '-' });
    s.push(if (mode & 0o010) != 0 { 'x' } else { '-' });

    // Other permissions
    s.push(if (mode & 0o004) != 0 { 'r' } else { '-' });
    s.push(if (mode & 0o002) != 0 { 'w' } else { '-' });
    s.push(if (mode & 0o001) != 0 { 'x' } else { '-' });

    s
}

fn format_time(t: std::time::SystemTime) -> String {
    let dt: DateTime<Local> = t.into();
    dt.format("%Y-%m-%d %H:%M:%S.%f %z").to_string()
}

// ─── Windows Stat Implementation ──────────────────────────────────────────────
#[cfg(target_os = "windows")]
fn get_file_stat(file_path: &str) -> io::Result<StatInfo> {
    let meta = fs::metadata(file_path)?;

    let is_dir = meta.is_dir();
    let is_symlink = meta.file_type().is_symlink();
    let file_type = if is_symlink {
        "symbolic link"
    } else if is_dir {
        "directory"
    } else {
        "regular file"
    };

    let mode = if is_dir {
        0o040755
    } else if meta.permissions().readonly() {
        0o100444
    } else {
        0o100666
    };

    // Stably query file index, volume serial number, and number of links on Windows via FFI
    let (inode, dev, links) = query_win_handle_info(file_path).unwrap_or((0, 0, 1));

    let size = meta.len();
    let block_size = 4096;
    let blocks = (size + 511) / 512; // best effort 512-byte blocks

    let access_time = meta.accessed().map(format_time).unwrap_or_else(|_| "-".to_string());
    let modify_time = meta.modified().map(format_time).unwrap_or_else(|_| "-".to_string());
    let birth_time = meta.created().map(format_time).unwrap_or_else(|_| "-".to_string());

    Ok(StatInfo {
        name: file_path.to_string(),
        size,
        blocks,
        block_size,
        device_dec: dev,
        device_hex: dev,
        inode,
        links,
        mode,
        file_type: file_type.to_string(),
        uid: 0,
        gid: 0,
        user_name: std::env::var("USERNAME").unwrap_or_else(|_| "None".to_string()),
        group_name: "None".to_string(),
        optimal_io: 4096,
        access_time,
        modify_time: modify_time.clone(),
        change_time: modify_time,
        birth_time,
    })
}

#[cfg(target_os = "windows")]
fn query_win_handle_info(file_path: &str) -> io::Result<(u64, u64, u64)> {
    use std::os::windows::io::AsRawHandle;
    use windows_sys::Win32::Storage::FileSystem::{GetFileInformationByHandle, BY_HANDLE_FILE_INFORMATION};

    let file = File::open(file_path)?;
    let handle = file.as_raw_handle();
    let mut info = std::mem::MaybeUninit::<BY_HANDLE_FILE_INFORMATION>::uninit();
    let res = unsafe { GetFileInformationByHandle(handle as _, info.as_mut_ptr()) };
    if res == 0 {
        return Err(io::Error::last_os_error());
    }
    let info = unsafe { info.assume_init() };
    let file_index = ((info.nFileIndexHigh as u64) << 32) | (info.nFileIndexLow as u64);
    Ok((file_index, info.dwVolumeSerialNumber as u64, info.nNumberOfLinks as u64))
}

#[cfg(target_os = "windows")]
fn get_fs_stat(file_path: &str) -> io::Result<FsStatInfo> {
    use std::ptr::null_mut;
    use windows_sys::Win32::Storage::FileSystem::{GetDiskFreeSpaceExW, GetVolumeInformationW};

    let path = Path::new(file_path);
    let root = match path.ancestors().last() {
        Some(r) => r.to_str().unwrap_or("C:\\"),
        None => "C:\\",
    };

    let mut root_w: Vec<u16> = root.encode_utf16().collect();
    if !root_w.ends_with(&[92]) { // ASCII '\' is 92
        root_w.push(92);
    }
    root_w.push(0);

    let mut free_bytes_avail = 0u64;
    let mut total_bytes = 0u64;
    let mut total_free_bytes = 0u64;

    let res = unsafe {
        GetDiskFreeSpaceExW(
            root_w.as_ptr(),
            &mut free_bytes_avail,
            &mut total_bytes,
            &mut total_free_bytes,
        )
    };

    if res == 0 {
        return Err(io::Error::last_os_error());
    }

    let mut fs_name = [0u16; 256];
    let vol_res = unsafe {
        GetVolumeInformationW(
            root_w.as_ptr(),
            null_mut(),
            0,
            null_mut(),
            null_mut(),
            null_mut(),
            fs_name.as_mut_ptr(),
            fs_name.len() as u32,
        )
    };

    let fs_type = if vol_res != 0 {
        let len = fs_name.iter().position(|&c| c == 0).unwrap_or(fs_name.len());
        String::from_utf16_lossy(&fs_name[..len])
    } else {
        "NTFS".to_string()
    };

    Ok(FsStatInfo {
        name: file_path.to_string(),
        fs_id: "0".to_string(),
        namelen: 255,
        fs_type,
        block_size: 4096,
        fundamental_block_size: 4096,
        blocks_total: total_bytes / 4096,
        blocks_free: total_free_bytes / 4096,
        blocks_avail: free_bytes_avail / 4096,
    })
}

// ─── Linux/Unix Stat Implementation ───────────────────────────────────────────
#[cfg(not(target_os = "windows"))]
fn get_file_stat(file_path: &str) -> io::Result<StatInfo> {
    use std::os::unix::fs::MetadataExt;
    let meta = fs::metadata(file_path)?;

    let is_dir = meta.is_dir();
    let is_symlink = meta.file_type().is_symlink();
    let file_type = if is_symlink {
        "symbolic link"
    } else if is_dir {
        "directory"
    } else {
        "regular file"
    };

    let mode = meta.mode();
    let inode = meta.ino();
    let dev = meta.dev();
    let links = meta.nlink();
    let size = meta.len();
    let blocks = meta.blocks();
    let blksize = meta.blksize();

    let uid = meta.uid();
    let gid = meta.gid();
    let (user_name, group_name) = get_user_group(uid, gid);

    let access_time = meta.accessed().map(format_time).unwrap_or_else(|_| "-".to_string());
    let modify_time = meta.modified().map(format_time).unwrap_or_else(|_| "-".to_string());
    let birth_time = meta.created().map(format_time).unwrap_or_else(|_| "-".to_string());

    Ok(StatInfo {
        name: file_path.to_string(),
        size,
        blocks,
        block_size: 512,
        device_dec: dev,
        device_hex: dev,
        inode,
        links,
        mode,
        file_type: file_type.to_string(),
        uid,
        gid,
        user_name,
        group_name,
        optimal_io: blksize,
        access_time,
        modify_time: modify_time.clone(),
        change_time: modify_time,
        birth_time,
    })
}

#[cfg(not(target_os = "windows"))]
fn get_user_group(uid: u32, gid: u32) -> (String, String) {
    let mut user_name = uid.to_string();
    let mut group_name = gid.to_string();
    unsafe {
        let pw = libc::getpwuid(uid);
        if !pw.is_null() {
            if let Ok(s) = std::ffi::CStr::from_ptr((*pw).pw_name).to_str() {
                user_name = s.to_string();
            }
        }
        let gr = libc::getgrgid(gid);
        if !gr.is_null() {
            if let Ok(s) = std::ffi::CStr::from_ptr((*gr).gr_name).to_str() {
                group_name = s.to_string();
            }
        }
    }
    (user_name, group_name)
}

#[cfg(not(target_os = "windows"))]
fn get_fs_stat(file_path: &str) -> io::Result<FsStatInfo> {
    let c_path = std::ffi::CString::new(file_path)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid file path"))?;

    let mut stat = std::mem::MaybeUninit::<libc::statvfs>::uninit();
    let res = unsafe { libc::statvfs(c_path.as_ptr(), stat.as_mut_ptr()) };
    if res != 0 {
        return Err(io::Error::last_os_error());
    }

    let stat = unsafe { stat.assume_init() };

    let block_size = if stat.f_frsize > 0 {
        stat.f_frsize as u64
    } else {
        stat.f_bsize as u64
    };

    Ok(FsStatInfo {
        name: file_path.to_string(),
        fs_id: format!("{:x}", stat.f_fsid),
        namelen: stat.f_namemax as u64,
        fs_type: "Unknown".to_string(),
        block_size: stat.f_bsize as u64,
        fundamental_block_size: block_size,
        blocks_total: stat.f_blocks as u64,
        blocks_free: stat.f_bfree as u64,
        blocks_avail: stat.f_bavail as u64,
    })
}
