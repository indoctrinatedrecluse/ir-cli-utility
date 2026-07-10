use std::io::Result;
use std::os::unix::fs::MetadataExt;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

pub fn get_mode(path: &Path) -> Result<u32> {
    let metadata = std::fs::metadata(path)?;
    Ok(metadata.mode() & 0o7777)
}

pub fn set_mode(path: &Path, mode: u32) -> Result<()> {
    let metadata = std::fs::metadata(path)?;
    let mut perms = metadata.permissions();
    perms.set_mode(mode);
    std::fs::set_permissions(path, perms)?;
    Ok(())
}
