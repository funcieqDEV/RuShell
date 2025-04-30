pub fn touch(path: &str) {
    let path = std::path::Path::new(path);
    if path.exists() {
        eprintln!("{} already exists", path.display());
    } else {
        match std::fs::File::create(path) {
            Ok(_) => println!("{} created", path.display()),
            Err(e) => eprintln!("Error creating file: {}", e),
        }
    }
}