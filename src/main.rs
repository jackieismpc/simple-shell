#[allow(unused_imports)]
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

const built_in_commands: [&str; 5] = ["echo", "exit", "type", "pwd", "cd"]; // shell 内置命令列表

enum CommandKind {
    Exit,
    Echo { display_string: String },
    Type { command_name: String },
    Pwd,
    Cd { directory: String },
    External { program: String, args: Vec<String> },
    NotFound,
}

fn split_args(line: &str) -> Vec<String> {
    // 简单分词器：支持单引号，单引号内字符按字面处理，单引号外以空白分隔
    let mut res = Vec::new();
    let mut cur = String::new();
    let mut in_single = false;
    let mut chars = line.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\'' {
            in_single = !in_single;
            continue;
        }
        if c.is_whitespace() && !in_single {
            if !cur.is_empty() {
                res.push(cur);
                cur = String::new();
            }
            // 吃掉连续空白
            while let Some(&nc) = chars.peek() {
                if nc.is_whitespace() {
                    chars.next();
                } else {
                    break;
                }
            }
        } else {
            cur.push(c);
        }
    }
    if !cur.is_empty() {
        res.push(cur);
    }
    res
}

impl CommandKind {
    fn parse(line: &str) -> CommandKind {
        let parts_owned = split_args(line);
        let parts: Vec<&str> = parts_owned.iter().map(|s| s.as_str()).collect();

        match parts.as_slice() {
            ["exit", "0"] => CommandKind::Exit,
            ["echo", rest @ ..] => {
                // rest 已保留单引号内的空格
                let display = if parts_owned.len() >= 2 {
                    parts_owned[1..].join(" ")
                } else {
                    String::new()
                };
                CommandKind::Echo {
                    display_string: display,
                }
            }
            ["type", name] => CommandKind::Type {
                command_name: name.to_string(),
            },
            ["pwd"] => CommandKind::Pwd,
            ["cd", dir] => CommandKind::Cd {
                directory: dir.to_string(),
            },
            [] => CommandKind::NotFound,
            _ => {
                let program = parts_owned.get(0).cloned().unwrap_or_default();
                let args = if parts_owned.len() >= 2 {
                    parts_owned[1..].to_vec()
                } else {
                    Vec::new()
                };
                CommandKind::External { program, args }
            }
        }
    }
}

// 检查路径是否为可执行文件（Unix 平台检查执行位，非 Unix 只判断为文件）
fn is_executable(path: &Path) -> bool {
    if let Ok(meta) = std::fs::metadata(path) {
        if !meta.is_file() {
            return false;
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            return meta.permissions().mode() & 0o111 != 0;
        }
        #[cfg(not(unix))]
        {
            return true;
        }
    }
    false
}

// 根据用户输入的 program（可能带路径或只是名字）返回可执行文件的真实路径以及希望传给子进程的 argv0（basename）
fn resolve_executable(program: &str) -> Option<(String, String)> {
    let argv0 = Path::new(program)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or(program)
        .to_string();

    if program.contains('/') {
        // 作为直接路径处理
        let p = Path::new(program);
        if is_executable(p) {
            return Some((program.to_string(), argv0));
        }
        return None;
    }

    // 在 PATH 中查找
    if let Ok(paths) = std::env::var("PATH") {
        for dir in paths.split(':') {
            let candidate = Path::new(dir).join(program);
            if is_executable(&candidate) {
                if let Some(s) = candidate.to_str() {
                    return Some((s.to_string(), argv0));
                }
            }
        }
    }
    None
}

fn spawn_and_wait(exe_path: String, argv0: String, args: &[String], presented_name: &str) {
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        let mut cmd = Command::new(exe_path);
        cmd.args(args);
        // 把 argv[0] 设置为用户期望的名字（basename / 原始输入的最后一段）
        cmd.arg0(argv0);
        match cmd.spawn() {
            Ok(mut child) => {
                let _ = child.wait();
            }
            Err(e) => eprintln!("{}: failed to execute: {}", presented_name, e),
        }
    }
    #[cfg(not(unix))]
    {
        match Command::new(exe_path).args(args).spawn() {
            Ok(mut child) => {
                let _ = child.wait();
            }
            Err(e) => eprintln!("{}: failed to execute: {}", presented_name, e),
        }
    }
}

fn expand_home(path: &str) -> Option<String> {
    // 支持 "~" 和 "~/" 开头展开 HOME
    if path == "~" {
        std::env::var_os("HOME").map(|h| h.to_string_lossy().into_owned())
    } else if path.starts_with("~/") {
        std::env::var_os("HOME").map(|h| {
            let mut base = h.to_string_lossy().into_owned();
            base.push_str(&path[1..]); // 把 "~/..." 拼接成 "/home/user/..."
            base
        })
    } else {
        Some(path.to_string())
    }
}

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(0) => break, // EOF -> 退出
            Ok(_) => {}
            Err(_) => break,
        }

        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue; // 忽略空行
        }

        let cmd = CommandKind::parse(trimmed);

        match cmd {
            CommandKind::Exit => break,
            CommandKind::Echo { display_string } => println!("{}", display_string),
            CommandKind::Type { command_name } => {
                if built_in_commands.contains(&command_name.as_str()) {
                    println!("{} is a shell builtin", command_name);
                } else if let Some(path) = resolve_executable(&command_name) {
                    println!("{} is {}", command_name, path.0);
                } else {
                    println!("{}: not found", command_name);
                }
            }
            CommandKind::Pwd => {
                let current_path = std::env::current_dir().unwrap();
                // 更安全的转换，避免 unwrap on non-UTF8
                let display_string = current_path.to_string_lossy().into_owned();
                println!("{}", display_string);
            }
            CommandKind::Cd { directory } => {
                if directory == "~" || directory.starts_with("~/") {
                    match expand_home(&directory) {
                        Some(target) => {
                            if std::env::set_current_dir(&target).is_err() {
                                println!("cd: {}: No such file or directory", target);
                            }
                        }
                        None => println!("cd: HOME not set"),
                    }
                } else {
                    if std::env::set_current_dir(&directory).is_err() {
                        println!("cd: {}: No such file or directory", directory);
                    }
                }
            }
            CommandKind::External { program, args } => match resolve_executable(&program) {
                Some((exe_path, argv0)) => {
                    spawn_and_wait(exe_path, argv0, &args, &program);
                }
                None => println!("{}: not found", program),
            },
            CommandKind::NotFound => {
                println!("{}: command not found", trimmed);
            }
        }
    }
}
