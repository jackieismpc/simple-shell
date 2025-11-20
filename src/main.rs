#[allow(unused_imports)]
use std::io::{self, Write};
const built_in_commands: [&str; 3] = ["echo", "exit", "type"];
enum Command {
    ExitCommand,
    EchoCommand { display_string: String },
    TypeCommand { command_name: String },
    CommandNotFound,
}
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
            _ => Command::CommandNotFound,
        }
    }
}
fn check_environment_command(command_name: &str) -> Option<String> {
    if let Ok(paths) = std::env::var("PATH") {
        for path in paths.split(':') {
            let full_path = format!("{}/{}", path, command_name);
            // if std::path::Path::new(&full_path).exists(){
            //     return Some(full_path);
            // }
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
                // if built_in_commands.contains(&command_name.as_str()) {
                //     println!("{} is a shell builtin", command_name);
                // }else if check_environment_command(&command_name) {
                //     println!("{} is an external command", command_name);

                // }
                // else {
                //     println!("{}: not found", command_name);
                // }

                match built_in_commands.contains(&command_name.as_str()) {
                    true => println!("{} is a shell builtin", command_name),
                    false => match check_environment_command(&command_name) {
                        Some(path) => println!("{} is {}", command_name, path),
                        None => println!("{}: not found", command_name),
                    },
                }
            }
            Command::CommandNotFound => {
                println!("{}: command not found", command.trim());
            }
        }
    }
}
