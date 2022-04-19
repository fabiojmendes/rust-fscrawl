use std::{
    env,
    path::Path,
    sync::mpsc::{self, Sender},
    thread,
};

use jwalk::WalkDir;
use rusqlite::Connection;

fn main() -> Result<(), rusqlite::Error> {
    let root = env::args().skip(1).next().unwrap_or(String::from("."));

    let conn = init_db("./crawl.db")?;
    let mut insert = conn.prepare("insert or ignore into visited (path) values (?)")?;

    let (tx, rx) = mpsc::channel::<String>();
    start_fswalk(root, tx);

    let mut progress = 0;
    eprint!("files: {progress}");
    for path in rx {
        let count = insert.execute([&path])?;
        if count > 0 {
            progress += 1;
            eprint!("\r files: {progress}");
            println!("{}", path);
        }
    }
    eprintln!("");

    Ok(())
}

fn init_db(path: impl AsRef<Path>) -> Result<Connection, rusqlite::Error> {
    let conn = Connection::open(path)?;
    conn.execute_batch(include_str!("init.sql"))?;
    Ok(conn)
}

fn start_fswalk(root: String, tx: Sender<String>) {
    thread::spawn(move || {
        for entry in WalkDir::new(root) {
            match entry {
                Ok(entry) => {
                    if entry.file_type().is_file() {
                        let path = entry.path();
                        let path_str = path.to_string_lossy();
                        tx.send(path_str.to_string()).unwrap();
                    }
                }
                Err(e) => eprintln!("error: {}", e),
            }
        }
        // eprintln!("crawl done");
    });
}
