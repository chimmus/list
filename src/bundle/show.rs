use std::fs;

pub fn ls(path: &String) {
    match fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().into_owned();
                if !name.starts_with('.') {
                    println!("{}", name);
                }
            }
        }
        Err(e) => eprintln!("ls: cannot read '{}': {}", path, e),
    }
}
