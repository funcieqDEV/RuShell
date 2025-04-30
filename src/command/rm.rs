pub fn rm(filepath: &str) {
    let filepath = std::path::Path::new(filepath);
    if filepath.is_file() {
        match std::fs::remove_file(filepath) {
            Ok(_) => {
                println!("File {} removed successfully", filepath.display());
            }
            Err(e) => eprintln!("Error removing file: {}", e),
        }
    } else {
        eprintln!("{} is not a file", filepath.display());
    }
}