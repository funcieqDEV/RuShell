use std::io::{self, Write};
mod command;

fn setup() {
    println!("Setting up the environment...");
    #[cfg(windows)]
    {
        use winapi::um::consoleapi::{GetConsoleMode, SetConsoleMode};
        use winapi::um::handleapi::INVALID_HANDLE_VALUE;
        use winapi::um::processenv::GetStdHandle;
        use winapi::um::winbase::STD_OUTPUT_HANDLE;
        use winapi::um::wincon::ENABLE_VIRTUAL_TERMINAL_PROCESSING;

        unsafe {
            let h_stdout = GetStdHandle(STD_OUTPUT_HANDLE);
            if h_stdout == INVALID_HANDLE_VALUE {
               return;
            }

            let mut mode = 0;
            if GetConsoleMode(h_stdout, &mut mode) == 0 {
             return;
            }

            SetConsoleMode(h_stdout, mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING);
        }
    }


    println!("Environment setup complete.");
    println!("Welcome to RuShell!");
}



fn main() {
    setup();
    io::stdout().flush().unwrap();

    let mut com = String::new();
    loop {
        print!("RS ~ {}> ", std::env::current_dir().unwrap().display());
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut com).unwrap();
        com = com.trim().to_string();

        if com.is_empty() {
            println!();
            continue;
        } else {
            command::handle_command(&com.as_str());
        }

        com.clear();
    }
}
