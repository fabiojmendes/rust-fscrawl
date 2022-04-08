use std::{io, path::PathBuf};

use tokio::fs;

async fn visit(root: impl Into<PathBuf>) -> io::Result<()> {
    let mut stack = Vec::new();
    stack.push(root.into());
    while let Some(dir) = stack.pop() {
        let mut reader = fs::read_dir(dir).await?;
        while let Some(e) = reader.next_entry().await? {
            if e.file_type().await?.is_dir() {
                stack.push(e.path());
            } else {
                println!("{}", e.path().display());
            }
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> io::Result<()> {
    visit(".").await?;
    Ok(())
}
