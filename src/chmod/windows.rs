use std::io::Result;
use std::path::Path;

pub fn chmod_single(path: &Path, mode: u32) -> Result<()> {
    let mut permissions = std::fs::metadata(path)?.permissions();
    let readonly = (mode & 0o200) == 0;
    permissions.set_readonly(readonly);
    std::fs::set_permissions(path, permissions)?;
    Ok(())
}
