pub fn cat(path: &str) {
    let path = std::path::Path::new(path);
    if path.is_file() {
        match std::fs::read_to_string(path) {
            Ok(contents) => {
                println!("{}", contents);
            }
            Err(e) => eprintln!("Error reading file: {}", e),
        }
    } else {
        eprintln!("{} is not a file", path.display());
    }
}