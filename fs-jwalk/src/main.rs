use std::env;

use jwalk::WalkDir;

fn main() {
    let root = env::args().skip(1).next().unwrap_or(String::from("."));

    for entry in WalkDir::new(root) {
        match entry {
            Ok(entry) => {
                if entry.file_type().is_file() {
                    println!("{}", entry.path().display());
                }
            }
            Err(e) => eprintln!("error: {}", e),
        }
    }
}
