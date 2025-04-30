pub fn help() {
    println!("Available commands:");
    println!("  help - Show this help message");
    println!("  clear - Clear the terminal screen");
    println!("  exit - Exit the shell");
    println!("  cd <path> - Change directory to <path>");
    println!("  cat <file> - Display the contents of <file>");
    println!("  echo <text> - Print <text> to the terminal");
    println!("  ls [path] - List files in [path] (default: current directory)");
    println!("  mkdir <directory> - Create a new directory <directory>");
    println!("  rmdir <directory> - Remove an empty directory <directory>");
    println!("  rm <file> - Remove a file <file>");
    println!("  touch <file> - Create an empty file <file>");
    println!("  cp <source> <destination> - Copy file from <source> to <destination>");
    println!("  mv <source> <destination> - Move file from <source> to <destination>");
}