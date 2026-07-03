use crate::DfOptions;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

pub fn df(options: DfOptions) {
    let file = match File::open("/proc/self/mounts") {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error: Failed to open /proc/self/mounts: {}", e);
            return;
        }
    };

    println!(
        "{:<20} {:<10} {:>10} {:>10} {:>10} {:>6}  {:<20}",
        "Filesystem", "Type", "Size", "Used", "Available", "Use%", "Mounted on"
    );

    let reader = BufReader::new(file);
    let mut seen_mounts = HashSet::new();

    for line_result in reader.lines() {
        let line = match line_result {
            Ok(l) => l,
            Err(_) => continue,
        };

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 3 {
            continue;
        }

        let device = parts[0];
        let mount_point = parts[1];
        let fs_type = parts[2];

        // Deduplicate mounts
        if seen_mounts.contains(mount_point) {
            continue;
        }
        seen_mounts.insert(mount_point.to_string());

        // Determine if we should show this filesystem
        let is_real = device.starts_with('/') || device == "tmpfs" || device == "udev";
        if !is_real && !options.all {
            continue;
        }

        // Call statvfs
        let mut stat = std::mem::MaybeUninit::<libc::statvfs>::uninit();
        let c_path = match std::ffi::CString::new(mount_point) {
            Ok(s) => s,
            Err(_) => continue,
        };

        let stat_res = unsafe { libc::statvfs(c_path.as_ptr(), stat.as_mut_ptr()) };
        if stat_res != 0 {
            if options.all {
                println!(
                    "{:<20} {:<10} {:>10} {:>10} {:>10} {:>6}  {:<20}",
                    device, fs_type, "-", "-", "-", "-", mount_point
                );
            }
            continue;
        }

        let stat = unsafe { stat.assume_init() };
        let block_size = if stat.f_frsize > 0 {
            stat.f_frsize as u64
        } else {
            stat.f_bsize as u64
        };

        let total_bytes = stat.f_blocks as u64 * block_size;
        let free_bytes = stat.f_bfree as u64 * block_size;
        let avail_bytes = stat.f_bavail as u64 * block_size;
        let used_bytes = total_bytes.saturating_sub(free_bytes);

        let use_percent = if total_bytes > 0 {
            (used_bytes as f64 / total_bytes as f64 * 100.0).round() as u64
        } else {
            0
        };

        let (size_str, used_str, avail_str) = if options.human_readable {
            (
                super::format_size_human(total_bytes),
                super::format_size_human(used_bytes),
                super::format_size_human(avail_bytes),
            )
        } else {
            (
                format!("{}", total_bytes / 1024),
                format!("{}", used_bytes / 1024),
                format!("{}", avail_bytes / 1024),
            )
        };

        println!(
            "{:<20} {:<10} {:>10} {:>10} {:>10} {:>5}%  {:<20}",
            device, fs_type, size_str, used_str, avail_str, use_percent, mount_point
        );
    }
}
