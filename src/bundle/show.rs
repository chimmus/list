use std::fs;

pub fn ls(path: &String, all: bool) {
    match fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().into_owned();
                if !all && name.starts_with('.') {
                    continue;
                }
                println!("{name}");
            }
        }
        Err(e) => eprintln!("ls: cannot read '{}': {}", path, e),
    }
}
