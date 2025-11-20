#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // TODO: Uncomment the code below to pass the first stage
    loop{
    print!("$ ");
    io::stdout().flush().unwrap();
    // 从标准输入读取一行命令
    let mut command = String::new();
    io::stdin().read_line(&mut command).unwrap();
    match command.trim() {
        "exit" => break,
        _ =>println!("{}: command not found", command.trim())
    }
}
}
