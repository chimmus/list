use std::fs;

pub fn ls(path: &String, all: bool) {
    let mut items: Vec<String> = Vec::new();
    match fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().into_owned();
                if !all && name.starts_with('.') {
                    continue;
                }
                items.push(name);
            }
            for item in items {
                println!("{item}");
            }
        }
        Err(e) => eprintln!("ls: cannot read '{}': {}", path, e),
    }
}
