use std::path::Path;
use std::process::Command;
use crate::utils::is_executable;

pub fn resolve_executable(program: &str) -> Option<(String, String)> {
    let argv0 = Path::new(program)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or(program)
        .to_string();

    if program.contains('/') {
        let p = Path::new(program);
        if is_executable(p) { return Some((program.to_string(), argv0)); }
        return None;
    }

    if let Ok(paths) = std::env::var("PATH") {
        for dir in paths.split(':') {
            let candidate = Path::new(dir).join(program);
            if is_executable(&candidate) {
                if let Some(s) = candidate.to_str() { return Some((s.to_string(), argv0)); }
            }
        }
    }
    None
}

pub fn spawn_and_wait(exe_path: String, argv0: String, args: &[String], presented_name: &str) {
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        let mut cmd = Command::new(exe_path);
        cmd.args(args);
        cmd.arg0(argv0);
        match cmd.spawn() {
            Ok(mut child) => { let _ = child.wait(); }
            Err(e) => eprintln!("{}: failed to execute: {}", presented_name, e),
        }
    }
    #[cfg(not(unix))]
    {
        match Command::new(exe_path).args(args).spawn() {
            Ok(mut child) => { let _ = child.wait(); }
            Err(e) => eprintln!("{}: failed to execute: {}", presented_name, e),
        }
    }
}