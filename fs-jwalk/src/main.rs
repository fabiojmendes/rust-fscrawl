use std::env;

use jwalk::WalkDir;

fn main() {
    let root = env::args().skip(1).next().unwrap_or(String::from("."));

    for entry in WalkDir::new(root) {
        match entry {
            Ok(entry) => {
                let path = entry.path();
                if !path.is_dir() {
                    println!("{}", path.display());
                }
            }
            Err(e) => eprintln!("error: {}", e),
        }
    }
}
