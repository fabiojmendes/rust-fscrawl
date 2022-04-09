use std::{env, fs, io, path::PathBuf};

fn main() -> io::Result<()> {
    let root = env::args().skip(1).next().unwrap_or(String::from("."));

    let mut stack = Vec::new();
    stack.push(PathBuf::from(root));
    while let Some(dir) = stack.pop() {
        for entry in fs::read_dir(&dir)? {
            match entry {
                Ok(entry) => {
                    if entry.file_type()?.is_dir() {
                        stack.push(entry.path());
                    } else {
                        println!("{}", entry.path().display());
                    }
                }
                Err(e) => eprintln!("error reading: {}: {}", dir.display(), e),
            }
        }
    }
    Ok(())
}
