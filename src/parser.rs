use std::path::Path;

pub enum CommandKind {
    Exit,
    Echo { display_string: String },
    Type { command_name: String },
    Pwd,
    Cd { directory: String },
    External { program: String, args: Vec<String> },
    NotFound,
}

fn split_args(line: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut cur = String::new();
    let mut in_single = false;
    let mut in_double = false;
    for c in line.chars() {
        if in_single {
            if c == '\'' { in_single = false; } else { cur.push(c); }
        } else if in_double {
            if c == '"' { in_double = false; } else { cur.push(c); }
        } else {
            if c == '\'' { in_single = true; }
            else if c == '"' { in_double = true; }
            else if c.is_whitespace() {
                if !cur.is_empty() { args.push(cur); cur = String::new(); }
            } else { cur.push(c); }
        }
    }
    if !cur.is_empty() || in_single || in_double { args.push(cur); }
    args
}

impl CommandKind {
    pub fn parse(line: &str) -> CommandKind {
        let parts: Vec<String> = split_args(line);
        let parts_ref: Vec<&str> = parts.iter().map(|s| s.as_str()).collect();
        match parts_ref.as_slice() {
            ["exit", ..] => CommandKind::Exit,
            ["echo", rest @ ..] => CommandKind::Echo { display_string: rest.join(" ") },
            ["type", name] => CommandKind::Type { command_name: name.to_string() },
            ["pwd"] => CommandKind::Pwd,
            ["cd", dir] => CommandKind::Cd { directory: dir.to_string() },
            [] => CommandKind::NotFound,
            _ => {
                let program = parts[0].to_string();
                let args = parts[1..].iter().map(|s| s.to_string()).collect();
                CommandKind::External { program, args }
            }
        }
    }
}