pub fn ls(path: &str) {
    let path = std::path::Path::new(path);
    if path.is_dir() {
        match std::fs::read_dir(path) {
            Ok(entries) => {
                for entry in entries {
                    match entry {
                        Ok(entry) => {
                            let file_name = entry.file_name();
                            println!("{}", file_name.to_string_lossy());
                        }
                        Err(e) => eprintln!("Error reading directory entry: {}", e),
                    }
                }
            }
            Err(e) => eprintln!("Error reading directory: {}", e),
        }
    } else {
        eprintln!("{} is not a directory", path.display());
    }

}