use shell_words::split;

mod cd;
mod exit;
mod clear;
mod ls;

pub fn handle_command(args: &str) {
    let commands: Vec<&str> = args.split("&&").map(str::trim).collect();

    for command in commands {
        let Ok(parts) = split(command) else {
            eprintln!("Parse error in command: {}", command);
            continue;
        };

        if parts.is_empty() {
            continue;
        }

        let cmd = &parts[0];
        let args = &parts[1..];

        match cmd.as_str() {
            "cd" => {
                if args.len() == 1 {
                    cd::cd(&args[0]);
                } else {
                    eprintln!("Usage: cd <path>");
                }
            }
            "exit" => {
                exit::exit();
            }
            "clear" => {
                clear::clear();
            }
            "echo" => {
                println!("{}", args.join(" "));
            }
            "ls" => {
                if args.is_empty() {
                    ls::ls(".");
                } else {
                    ls::ls(&args[0]);
                }
            }
            _ => {
                println!("Command not found: {}", cmd);
            }
        }
    }
}
