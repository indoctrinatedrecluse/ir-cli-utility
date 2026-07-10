use std::io::Result;
use std::path::Path;

pub fn get_mode(path: &Path) -> Result<u32> {
    let metadata = std::fs::metadata(path)?;
    let readonly = metadata.permissions().readonly();
    if readonly {
        Ok(0o444)
    } else {
        Ok(0o666)
    }
}

pub fn set_mode(path: &Path, mode: u32) -> Result<()> {
    let mut permissions = std::fs::metadata(path)?.permissions();
    let readonly = (mode & 0o200) == 0;
    permissions.set_readonly(readonly);
    std::fs::set_permissions(path, permissions)?;
    Ok(())
}
