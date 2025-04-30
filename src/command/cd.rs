pub fn cd(path: &str) {
    if let Err(e) = std::env::set_current_dir(path) {
        eprintln!("Error changing directory: {}", e);
    }
}