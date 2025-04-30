pub fn cp(src: &str, dest: &str) {
    let src_path = std::path::Path::new(src);
    let dest_path = std::path::Path::new(dest);

    if src_path.is_file() {
        match std::fs::copy(src_path, dest_path) {
            Ok(_) => println!("Copied {} to {}", src, dest),
            Err(e) => eprintln!("Error copying file: {}", e),
        }
    } else {
        eprintln!("{} is not a file", src_path.display());
    }
}