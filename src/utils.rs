use std::path::Path;

pub fn is_executable(path: &Path) -> bool {
    if let Ok(meta) = std::fs::metadata(path) {
        if !meta.is_file() { return false; }
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            return meta.permissions().mode() & 0o111 != 0;
        }
        #[cfg(not(unix))]
        { return true; }
    }
    false
}