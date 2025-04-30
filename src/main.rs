use std::io::Write;
use std::io;
mod command;


fn setup() {
    println!("Setting up the environment...");

    println!("Environment setup complete.");
    println!("welcome in RuShell!");
}

fn main() {

    setup();
    io::stdout().flush().unwrap();
    let mut com = String::new();
        loop {
            print!("RS ~ {}> ",std::env::current_dir().unwrap().display());
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut com).unwrap();
            com = com.trim().to_string();
            if com.is_empty() {
                println!();
                continue;
            }
            else {
                command::handle_command(&com.as_str());
            }
            
            com.clear();
        }
}

