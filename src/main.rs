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
fn main() {
    // TODO: Uncomment the code below to pass the first stage
    loop{
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
            if built_in_commands.contains(&command_name.as_str()) {
                println!("{} is a shell built-in", command_name);
            } else {
                println!("{}: not found", command_name);
            }
        }
        Command::CommandNotFound => {
            println!("{}: command not found", command.trim());
        }
    }   
}
}
