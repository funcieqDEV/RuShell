pub fn mkdir(path: &str) {
    let path = std::path::Path::new(path);
    if path.exists() {
        eprintln!("{} already exists", path.display());
        return;
    }
    match std::fs::create_dir_all(path) {
        Ok(_) => {
            println!("Created directory: {}", path.display());
        }
        Err(e) => eprintln!("Error creating directory: {}", e),
    }
}