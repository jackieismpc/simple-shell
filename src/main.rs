mod parser;
mod utils;
mod executor;
mod builtins;

use std::io::{self, Write};
use parser::CommandKind;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        if io::stdin().read_line(&mut line).is_err() { break; }
        let trimmed = line.trim();
        let cmd = CommandKind::parse(trimmed);

        // 内置命令处理；run_builtin 返回 false 表示要退出
        match cmd {
            CommandKind::External { program, args } => {
                match executor::resolve_executable(&program) {
                    Some((exe_path, argv0)) => executor::spawn_and_wait(exe_path, argv0, &args, &program),
                    None => println!("{}: not found", program),
                }
            }
            _ => {
                let keep_running = builtins::run_builtin(cmd, trimmed);
                if !keep_running { break; }
            }
        }
    }
}