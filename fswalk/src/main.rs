use std::env;

use rayon::prelude::*;
use walkdir::WalkDir;

fn main() {
    let root = env::args().skip(1).next().unwrap_or(String::from("."));

    WalkDir::new(root)
        .into_iter()
        .par_bridge()
        .for_each(|entry| match entry {
            Ok(entry) => {
                if entry.file_type().is_file() {
                    let path = entry.path();
                    println!("{}", path.display());
                }
            }
            Err(e) => eprintln!("{}", e),
        });
}
