use std::fs::File;
use std::io::{Write, Read};
use zip::{ZipWriter, CompressionMethod};
use zip::write::FileOptions;

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let file = File::create(&path).unwrap();
    let mut zip = ZipWriter::new(file);

    zip.start_file("file1.txt", FileOptions::default().compression_method(CompressionMethod::Stored)).unwrap();
    zip.write_all(b"Hello, world!").unwrap();

    zip.finish().unwrap();
}
