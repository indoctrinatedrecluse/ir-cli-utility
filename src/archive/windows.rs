use crate::ArchiveOptions;

pub fn archive(path: &str, options: ArchiveOptions) {
    crate::archive::archive(path, options);
}
