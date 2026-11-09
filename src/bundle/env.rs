use std::env;

pub fn current() -> (String, bool){
    match env::current_dir() {
        Ok(path) => {
            let path = path.to_string_lossy();
            (path.to_string(), true)
        },
        Err(e) => {
            (e.to_string(), false)
        },
    }
}
