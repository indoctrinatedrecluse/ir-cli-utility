use crate::ArchiveOptions;
use std::path::{Path, PathBuf};
use std::fs::{self, File};
use std::io::{self, Write, Read, IsTerminal};
use zip::{ZipWriter, CompressionMethod};
use zip::write::FileOptions;
use walkdir::WalkDir;
use tar::Builder;
use flate2::write::GzEncoder;
use flate2::read::GzDecoder;
use flate2::Compression;

fn draw_progress(processed: u64, total: u64, label: &str) {
    if !io::stdout().is_terminal() {
        return;
    }
    let total = if total == 0 { 1 } else { total };
    let pct = (processed as f64 / total as f64 * 100.0).clamp(0.0, 100.0);
    let width: usize = 30;
    let filled = ((pct / 100.0) * width as f64).round() as usize;
    let empty = width.saturating_sub(filled);
    
    let bar = format!(
        "\x1b[32m{}\x1b[0m{}",
        "█".repeat(filled),
        "░".repeat(empty)
    );
    
    let size_str = format!("{:.1} / {:.1} MB", processed as f64 / 1024.0 / 1024.0, total as f64 / 1024.0 / 1024.0);
    print!("\r[{}] {:.1}% ({}) [{}]      ", bar, pct, size_str, label);
    let _ = io::stdout().flush();
}

fn get_total_size(path: &Path) -> u64 {
    if path.is_file() {
        path.metadata().map(|m| m.len()).unwrap_or(0)
    } else {
        WalkDir::new(path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .map(|e| e.metadata().map(|m| m.len()).unwrap_or(0))
            .sum()
    }
}

struct ProgressReader<R> {
    inner: R,
    processed: u64,
    total: u64,
    label: &'static str,
}

impl<R: Read> Read for ProgressReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let n = self.inner.read(buf)?;
        self.processed += n as u64;
        draw_progress(self.processed, self.total, self.label);
        Ok(n)
    }
}

struct TarProgressReader<'a, R> {
    inner: R,
    processed_ref: &'a mut u64,
    total: u64,
    label: &'static str,
}

impl<'a, R: Read> Read for TarProgressReader<'a, R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let n = self.inner.read(buf)?;
        *self.processed_ref += n as u64;
        draw_progress(*self.processed_ref, self.total, self.label);
        Ok(n)
    }
}

fn zip_dir(
    dir: &Path,
    writer: &mut ZipWriter<File>,
    options: &ArchiveOptions,
    total_bytes: u64,
    processed_bytes: &mut u64,
) -> zip::result::ZipResult<()> {
    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        let name = path.strip_prefix(dir).unwrap();
        if options.verbose {
            println!("Adding: {}", name.display());
        }
        if path.is_file() {
            writer.start_file(
                name.to_str().unwrap(),
                FileOptions::default().compression_method(CompressionMethod::Stored),
            )?;
            let mut f = File::open(path)?;
            let mut buffer = [0u8; 65536];
            loop {
                let n = f.read(&mut buffer)?;
                if n == 0 {
                    break;
                }
                writer.write_all(&buffer[..n])?;
                *processed_bytes += n as u64;
                draw_progress(*processed_bytes, total_bytes, "Archiving");
            }
        } else if name.as_os_str().len() != 0 {
            writer.add_directory(name.to_str().unwrap(), FileOptions::default())?;
        }
    }
    Ok(())
}

fn create_zip_archive(source_path: &Path, dest_path: &Path, options: &ArchiveOptions) -> Result<(), String> {
    if !options.force && dest_path.exists() {
        return Err(format!("Destination '{}' already exists. Use --force to overwrite.", dest_path.display()));
    }
    let file = File::create(&dest_path).map_err(|e| e.to_string())?;
    let mut zip = ZipWriter::new(file);

    let total_bytes = get_total_size(source_path);
    let mut processed_bytes = 0u64;

    if source_path.is_file() {
        let file_name = source_path.file_name().unwrap().to_str().unwrap();
        zip.start_file(
            file_name,
            FileOptions::default().compression_method(CompressionMethod::Stored),
        ).map_err(|e| e.to_string())?;
        let mut f = File::open(source_path).map_err(|e| e.to_string())?;
        let mut buffer = [0u8; 65536];
        loop {
            let n = f.read(&mut buffer).map_err(|e| e.to_string())?;
            if n == 0 {
                break;
            }
            zip.write_all(&buffer[..n]).map_err(|e| e.to_string())?;
            processed_bytes += n as u64;
            draw_progress(processed_bytes, total_bytes, "Archiving");
        }
    } else {
        zip_dir(source_path, &mut zip, options, total_bytes, &mut processed_bytes).map_err(|e| e.to_string())?;
    }
    zip.finish().map_err(|e| e.to_string())?;
    if io::stdout().is_terminal() {
        println!();
    }
    Ok(())
}

fn create_tar_gz_archive(source_path: &Path, dest_path: &Path, options: &ArchiveOptions) -> Result<(), String> {
    if !options.force && dest_path.exists() {
        return Err(format!("Destination '{}' already exists. Use --force to overwrite.", dest_path.display()));
    }
    let file = File::create(&dest_path).map_err(|e| e.to_string())?;
    let enc = GzEncoder::new(file, Compression::default());
    let mut tar = Builder::new(enc);

    let total_bytes = get_total_size(source_path);
    let mut processed_bytes = 0u64;

    if source_path.is_dir() {
        let dir_name = source_path.file_name().unwrap().to_str().unwrap();
        for entry in WalkDir::new(source_path).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            let rel_path = path.strip_prefix(source_path).unwrap();
            let mut tar_path = PathBuf::from(dir_name);
            tar_path.push(rel_path);

            if path.is_file() {
                let f = File::open(path).map_err(|e| e.to_string())?;
                let mut header = tar::Header::new_gnu();
                let metadata = path.metadata().map_err(|e| e.to_string())?;
                header.set_metadata(&metadata);

                let mut wrap_reader = TarProgressReader {
                    inner: f,
                    processed_ref: &mut processed_bytes,
                    total: total_bytes,
                    label: "Archiving",
                };
                tar.append_data(&mut header, &tar_path, &mut wrap_reader).map_err(|e| e.to_string())?;
            } else if path.is_dir() {
                tar.append_dir(&tar_path, path).map_err(|e| e.to_string())?;
            }
        }
    } else {
        let f = File::open(source_path).map_err(|e| e.to_string())?;
        let mut header = tar::Header::new_gnu();
        let metadata = source_path.metadata().map_err(|e| e.to_string())?;
        header.set_metadata(&metadata);

        let mut wrap_reader = TarProgressReader {
            inner: f,
            processed_ref: &mut processed_bytes,
            total: total_bytes,
            label: "Archiving",
        };
        let file_name = source_path.file_name().unwrap();
        tar.append_data(&mut header, file_name, &mut wrap_reader).map_err(|e| e.to_string())?;
    }
    tar.finish().map_err(|e| e.to_string())?;
    if io::stdout().is_terminal() {
        println!();
    }
    Ok(())
}

fn unarc_zip(source_path: &Path, dest_path: &Path, options: &ArchiveOptions) -> Result<(), String> {
    let file = File::open(source_path).map_err(|e| e.to_string())?;
    let mut archive = zip::ZipArchive::new(file).map_err(|e| e.to_string())?;

    let mut total_bytes = 0u64;
    for i in 0..archive.len() {
        if let Ok(file) = archive.by_index(i) {
            total_bytes += file.size();
        }
    }
    let mut processed_bytes = 0u64;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).map_err(|e| e.to_string())?;
        let outpath = dest_path.join(file.name());

        if options.verbose {
            println!("Extracting: {}", outpath.display());
        }

        if (&*file.name()).ends_with('/') {
            fs::create_dir_all(&outpath).map_err(|e| e.to_string())?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).map_err(|e| e.to_string())?;
                }
            }
            let mut outfile = fs::File::create(&outpath).map_err(|e| e.to_string())?;
            let mut buffer = [0u8; 65536];
            loop {
                let n = file.read(&mut buffer).map_err(|e| e.to_string())?;
                if n == 0 {
                    break;
                }
                outfile.write_all(&buffer[..n]).map_err(|e| e.to_string())?;
                processed_bytes += n as u64;
                draw_progress(processed_bytes, total_bytes, "Extracting");
            }
        }
    }
    if io::stdout().is_terminal() {
        println!();
    }
    Ok(())
}

fn unarc_tar_gz(source_path: &Path, dest_path: &Path, _options: &ArchiveOptions) -> Result<(), String> {
    let file = File::open(source_path).map_err(|e| e.to_string())?;
    let total = file.metadata().map(|m| m.len()).unwrap_or(0);
    let progress_reader = ProgressReader {
        inner: file,
        processed: 0,
        total,
        label: "Extracting",
    };
    let tar = GzDecoder::new(progress_reader);
    let mut archive = tar::Archive::new(tar);
    archive.unpack(dest_path).map_err(|e| e.to_string())?;
    if io::stdout().is_terminal() {
        println!();
    }
    Ok(())
}

fn unarc_7z(source_path: &Path, dest_path: &Path, _options: &ArchiveOptions) -> Result<(), String> {
    sevenz_rust::decompress_file(source_path, dest_path).map_err(|e| e.to_string())
}

fn test_archive(source_path: &Path, options: &ArchiveOptions) -> Result<(), String> {
    let format = options.format.as_deref().unwrap_or_else(|| source_path.extension().and_then(|s| s.to_str()).unwrap_or(""));
    match format {
        "zip" => {
            let file = File::open(source_path).map_err(|e| e.to_string())?;
            let mut archive = zip::ZipArchive::new(file).map_err(|e| e.to_string())?;
            for i in 0..archive.len() {
                let file = archive.by_index(i).map_err(|e| e.to_string())?;
                if options.verbose {
                    println!("Testing: {}", file.name());
                }
            }
            Ok(())
        }
        "tar.gz" => {
            let file = File::open(source_path).map_err(|e| e.to_string())?;
            let tar = GzDecoder::new(file);
            let mut archive = tar::Archive::new(tar);
            for entry in archive.entries().map_err(|e| e.to_string())? {
                let entry = entry.map_err(|e| e.to_string())?;
                if options.verbose {
                    println!("Testing: {}", entry.path().unwrap().display());
                }
            }
            Ok(())
        }
        "iso" => {
            let file = File::open(source_path).map_err(|e| e.to_string())?;
            let fs = iso9660::ISO9660::new(file).map_err(|e| e.to_string())?;
            for entry in fs.root.contents() {
                if let Ok(entry) = entry {
                    if options.verbose {
                        println!("Found: {}", entry.identifier());
                    }
                }
            }
            Ok(())
        }
        _ => Err(format!("Testing for format '{}' is not supported.", format)),
    }
}

pub fn archive(path_str: &str, options: ArchiveOptions) {
    let source_path = Path::new(path_str);
    if !source_path.exists() {
        eprintln!("Error: Source path '{}' does not exist.", path_str);
        return;
    }

    let dest_path = options.dest.as_deref().map(PathBuf::from).unwrap_or_else(|| PathBuf::from("."));
    if !dest_path.is_dir() {
        if options.force {
            if let Err(e) = fs::create_dir_all(&dest_path) {
                eprintln!("Error creating destination directory: {}", e);
                return;
            }
        } else {
            eprintln!("Error: Destination path '{}' is not a valid directory. Use --force to create it.", dest_path.display());
            return;
        }
    }

    if options.test {
        if let Err(e) = test_archive(source_path, &options) {
            eprintln!("Error testing archive: {}", e);
        } else {
            println!("Archive test completed successfully.");
        }
        return;
    }

    if options.unarc {
        let format = options.format.as_deref().unwrap_or_else(|| source_path.extension().and_then(|s| s.to_str()).unwrap_or(""));
        let result = match format {
            "zip" => unarc_zip(source_path, &dest_path, &options),
            "tar.gz" => unarc_tar_gz(source_path, &dest_path, &options),
            "7z" => unarc_7z(source_path, &dest_path, &options),
            _ => Err(format!("Unsupported format for extraction: '{}'", format)),
        };
        if let Err(e) = result {
            eprintln!("Error extracting archive: {}", e);
        } else {
            println!("Archive extracted successfully to '{}'.", dest_path.display());
        }
        return;
    }

    // Default to archiving
    let format = options.format.as_deref().unwrap_or("zip");
    let archive_name = format!("{}.{}", source_path.file_stem().unwrap().to_str().unwrap(), format);
    let final_dest_path = dest_path.join(archive_name);

    let result = match format {
        "zip" => create_zip_archive(source_path, &final_dest_path, &options),
        "tar.gz" => create_tar_gz_archive(source_path, &final_dest_path, &options),
        _ => Err(format!("Unsupported format for creation: '{}'", format)),
    };

    if let Err(e) = result {
        eprintln!("Error creating archive: {}", e);
    } else {
        println!("Archive created successfully at '{}'.", final_dest_path.display());
    }
}
