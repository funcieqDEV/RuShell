use shell_words::split;
use std::process::Command;

mod cd;
mod exit;
mod clear;
mod cat;
mod ls;
mod mkdir;
mod rmdir;
mod rm;
mod touch;
mod cp;
mod mv;
mod help;

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
            "cat" => {
                if args.len() == 1 {
                    cat::cat(&args[0]);
                } else {
                    eprintln!("Usage: cat <file>");
                }
            }
            "mkdir" => {
                if args.len() == 1 {
                    mkdir::mkdir(&args[0]);
                } else {
                    eprintln!("Usage: mkdir <directory>");
                }
            }
            "rmdir" => {
                if args.len() == 1 {
                    rmdir::rmdir(&args[0]);
                } else {
                    eprintln!("Usage: rmdir <directory>");
                }
            }
            "rm" => {
                if args.len() == 1 {
                    rm::rm(&args[0]);
                } else {
                    eprintln!("Usage: rm <file>");
                }
            }
            "touch" => {
                if args.len() == 1 {
                    touch::touch(&args[0]);
                } else {
                    eprintln!("Usage: touch <file>");
                }
            }
            "cp" => {
                if args.len() == 2 {
                    cp::cp(&args[0], &args[1]);
                } else {
                    eprintln!("Usage: cp <source> <destination>");
                }
            }
            "mv" => {
                if args.len() == 2 {
                    mv::mv(&args[0], &args[1]);
                } else {
                    eprintln!("Usage: mv <source> <destination>");
                }
            }
            "help" => {
                help::help();
            }

            _ => {
                if cmd.starts_with("./") {
                    match Command::new(cmd).args(args).status() {
                        Ok(status) => {
                            if !status.success() {
                                eprintln!("Process exited with status: {}", status);
                            }
                        }
                        Err(err) => {
                            eprintln!("Failed to execute {}: {}", cmd, err);
                        }
                    }
                } else {
                    println!("Command not found: {}", cmd);
                }
            }
        }
    }
}
