#[allow(unused_imports)]
use std::io::{self, Write};
const built_in_commands: [&str; 3] = ["echo", "exit", "type"];//shell 内置命令列表
enum Command {
    ExitCommand,
    EchoCommand { display_string: String },
    TypeCommand { command_name: String },
    ExternalCommand { program: String, args: Vec<String> },//外部命令
    CommandNotFound,
}// 解析命令字符串并返回对应的 Command 枚举
impl Command {
    fn parse(command: &str) -> Command {
        let parts: Vec<&str> = command.trim().split_whitespace().collect();
        match parts.as_slice() {
            ["exit", "0"] => Command::ExitCommand,
            ["echo", rest @ ..] => Command::EchoCommand {
                display_string: rest.join(" "),
            },
            ["type", command_name] => Command::TypeCommand {
                command_name: command_name.to_string(),
            },
            [] => Command::CommandNotFound,
            _ => {
                let program = parts[0].to_string();
                let args = parts[1..].iter().map(|s| s.to_string()).collect();
                Command::ExternalCommand { program, args }  
            }
        }
    }
}
fn check_environment_command(command_name: &str) -> Option<String> {
    if let Ok(paths) = std::env::var("PATH") {
        for path in paths.split(':') {
            let full_path = format!("{}/{}", path, command_name);

            if let Ok(metadata) = std::fs::metadata(&full_path) {
                if metadata.is_file() {
                    #[cfg(unix)]
                    {
                        use std::os::unix::fs::PermissionsExt;
                        if metadata.permissions().mode() & 0o111 != 0 {
                            return Some(full_path);
                        }
                    }
                    #[cfg(not(unix))]
                    {
                        // 非 Unix 平台只能判断为文件（没有可执行位概念）
                        return Some(full_path);
                    }
                }
            }
        }
    }
    None
}
fn main() {
    // TODO: Uncomment the code below to pass the first stage
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        // 从标准输入读取一行命令
        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();
        let cmd = Command::parse(&command);
        match cmd {
            Command::ExitCommand => break,
            Command::EchoCommand { display_string } => {
                println!("{}", display_string);
            }
            Command::TypeCommand { command_name } => {

                match built_in_commands.contains(&command_name.as_str()) {
                    true => println!("{} is a shell builtin", command_name),
                    false => match check_environment_command(&command_name) {
                        Some(path) => println!("{} is {}", command_name, path),
                        None => println!("{}: not found", command_name),
                    },
                }
            }
            Command::ExternalCommand { program, args } => {
                // 如果包含 '/' 则当作直接路径处理，否则在 PATH 中查找
                let maybe_path = if program.contains('/') {
                    // 直接路径：检查是否存在且可执行（Unix 检查可执行位）
                    if let Ok(meta) = std::fs::metadata(&program) {
                        if meta.is_file() {
                            #[cfg(unix)]
                            {
                                use std::os::unix::fs::PermissionsExt;
                                if meta.permissions().mode() & 0o111 != 0 {
                                    Some(program.clone())
                                } else {
                                    None
                                }
                            }
                            #[cfg(not(unix))]
                            {
                                Some(program.clone())
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    check_environment_command(&program)
                };

                match maybe_path {
                    Some(path) => {
                        match std::process::Command::new(path).args(&args).spawn() {
                            Ok(mut child) => {
                                let _ = child.wait();
                            }
                            Err(e) => println!("{}: failed to execute: {}", program, e),
                        }
                    }
                    None => println!("{}: not found", program),
                }
            }
            Command::CommandNotFound => {
                println!("{}: command not found", command.trim());
            }
        }
    }
}
