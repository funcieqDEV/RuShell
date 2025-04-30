pub fn mv(src: &str, dest: &str) {
    let src_path = std::path::Path::new(src);
    let dest_path = std::path::Path::new(dest);

    if src_path.exists() {
        if src_path.is_file() {
            if dest_path.is_dir() {
                let new_dest = dest_path.join(src_path.file_name().unwrap());
                match std::fs::rename(src_path, new_dest) {
                    Ok(_) => println!("Moved file to directory"),
                    Err(e) => eprintln!("Error moving file: {}", e),
                }
            } else {
                match std::fs::rename(src_path, dest_path) {
                    Ok(_) => println!("Moved file"),
                    Err(e) => eprintln!("Error moving file: {}", e),
                }
            }
        } else if src_path.is_dir() {
            if dest_path.is_dir() {
                let new_dest = dest_path.join(src_path.file_name().unwrap());
                match std::fs::rename(src_path, new_dest) {
                    Ok(_) => println!("Moved directory to directory"),
                    Err(e) => eprintln!("Error moving directory: {}", e),
                }
            } else {
                match std::fs::rename(src_path, dest_path) {
                    Ok(_) => println!("Moved directory"),
                    Err(e) => eprintln!("Error moving directory: {}", e),
                }
            }
        } else {
            eprintln!("{} is neither a file nor a directory", src);
        }
    } else {
        eprintln!("{} does not exist", src);
    }
}