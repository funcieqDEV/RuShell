pub fn rmdir(path: &str) {
    let path = std::path::Path::new(path);
    if path.is_dir() {
        match std::fs::remove_dir(path) {
            Ok(_) => {
                println!("Directory removed successfully");
            }
            Err(e) => eprintln!("Error removing directory: {}", e),
        }
    } else {
        eprintln!("{} is not a directory", path.display());
    }
}