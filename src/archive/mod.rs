use crate::ArchiveOptions;
use std::path::{Path, PathBuf};
use std::fs::{self, File};
use std::io::{self, Write, Read};
use zip::{ZipWriter, CompressionMethod};
use zip::write::FileOptions;
use walkdir::WalkDir;
use tar::Builder;
use flate2::write::GzEncoder;
use flate2::read::GzDecoder;
use flate2::Compression;

fn zip_dir(dir: &Path, writer: &mut ZipWriter<File>, options: &ArchiveOptions) -> zip::result::ZipResult<()> {
    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        let name = path.strip_prefix(dir).unwrap();
        if options.verbose {
            println!("Adding: {}", name.display());
        }
        if path.is_file() {
            // FIX: Use start_file instead of deprecated start_file_from_path
            writer.start_file(name.to_str().unwrap(), FileOptions::default().compression_method(CompressionMethod::Stored))?;
            let mut f = File::open(path)?;
            let mut buffer = Vec::new();
            f.read_to_end(&mut buffer)?;
            writer.write_all(&buffer)?;
        } else if name.as_os_str().len() != 0 {
            // FIX: Use add_directory instead of deprecated add_directory_from_path
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

    if source_path.is_file() {
        let file_name = source_path.file_name().unwrap().to_str().unwrap();
        zip.start_file(file_name, FileOptions::default().compression_method(CompressionMethod::Stored)).map_err(|e| e.to_string())?;
        let mut f = File::open(source_path).map_err(|e| e.to_string())?;
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer).map_err(|e| e.to_string())?;
        zip.write_all(&buffer).map_err(|e| e.to_string())?;
    } else {
        zip_dir(source_path, &mut zip, options).map_err(|e| e.to_string())?;
    }
    zip.finish().map_err(|e| e.to_string())?;
    Ok(())
}

fn create_tar_gz_archive(source_path: &Path, dest_path: &Path, options: &ArchiveOptions) -> Result<(), String> {
    if !options.force && dest_path.exists() {
        return Err(format!("Destination '{}' already exists. Use --force to overwrite.", dest_path.display()));
    }
    let file = File::create(&dest_path).map_err(|e| e.to_string())?;
    let enc = GzEncoder::new(file, Compression::default());
    let mut tar = Builder::new(enc);

    if source_path.is_dir() {
        // tar.append_dir_all expects a relative path for the entry name
        // and the full path to the directory on the filesystem.
        // We need to ensure the entry name is just the directory name.
        let dir_name = source_path.file_name().unwrap().to_str().unwrap();
        tar.append_dir_all(dir_name, source_path).map_err(|e| e.to_string())?;
    } else {
        // For a single file, append_path expects the path to the file on the filesystem
        // and uses its file_name as the entry name in the archive.
        tar.append_path(source_path).map_err(|e| e.to_string())?;
    }
    tar.finish().map_err(|e| e.to_string())?;
    Ok(())
}

fn unarc_zip(source_path: &Path, dest_path: &Path, options: &ArchiveOptions) -> Result<(), String> {
    let file = File::open(source_path).map_err(|e| e.to_string())?;
    let mut archive = zip::ZipArchive::new(file).map_err(|e| e.to_string())?;

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
            io::copy(&mut file, &mut outfile).map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

fn unarc_tar_gz(source_path: &Path, dest_path: &Path, _options: &ArchiveOptions) -> Result<(), String> {
    let file = File::open(source_path).map_err(|e| e.to_string())?;
    let tar = GzDecoder::new(file);
    let mut archive = tar::Archive::new(tar);
    archive.unpack(dest_path).map_err(|e| e.to_string())?;
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
        },
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
        },
        "iso" => {
            let file = File::open(source_path).map_err(|e| e.to_string())?;
            let fs = iso9660::ISO9660::new(file).map_err(|e| e.to_string())?;
            for entry in fs.root.contents() {
                // FIX: Correctly handle the Result and access the identifier method
                if let Ok(entry) = entry {
                     if options.verbose {
                        println!("Found: {}", entry.identifier());
                    }
                }
            }
            Ok(())
        },
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
