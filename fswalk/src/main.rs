use rayon::prelude::*;
use std::env;

use walkdir::WalkDir;

fn main() {
    let root = env::args().skip(1).next().unwrap_or(String::from("."));

    WalkDir::new(root)
        .into_iter()
        .par_bridge()
        .for_each(|entry| match entry {
            Ok(entry) => {
                let path = entry.path();
                if !path.is_dir() {
                    println!("{}", path.display());
                }
            }
            Err(e) => eprintln!("{e}"),
        });
}
