use crate::parser::CommandKind;

const BUILT_INS: [&str; 5] = ["echo", "exit", "type", "pwd", "cd"];

pub fn is_builtin(name: &str) -> bool {
    BUILT_INS.contains(&name)
}

pub fn run_builtin(cmd: CommandKind, raw_line: &str) -> bool {
    match cmd {
        CommandKind::Exit => return false, // 返回 false 表示退出 REPL
        CommandKind::Echo { display_string } => { println!("{}", display_string); }
        CommandKind::Type { command_name } => {
            if is_builtin(&command_name) {
                println!("{} is a shell builtin", command_name);
            } else if let Some(path) = crate::executor::resolve_executable(&command_name) {
                println!("{} is {}", command_name, path.0);
            } else {
                println!("{}: not found", command_name);
            }
        }
        CommandKind::Pwd => {
            if let Ok(current_path) = std::env::current_dir() {
                if let Some(s) = current_path.to_str() { println!("{}", s); }
            }
        }
        CommandKind::Cd { directory } => {
            if directory == "~" {
                if let Some(home_dir) = std::env::var_os("HOME") {
                    if std::env::set_current_dir(&home_dir).is_err() {
                        println!("cd: {}: No such file or directory", home_dir.to_string_lossy());
                    }
                } else { println!("cd: HOME not set"); }
            } else {
                if std::env::set_current_dir(&directory).is_err() {
                    println!("cd: {}: No such file or directory", directory);
                }
            }
        }
        CommandKind::NotFound => println!("{}: command not found", raw_line),
        CommandKind::External { .. } => unreachable!(),
    }
    true
}
