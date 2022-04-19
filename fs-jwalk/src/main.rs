use std::{
    env,
    sync::mpsc::{self, Receiver},
    thread::{self, JoinHandle},
};

use jwalk::WalkDir;
use rusqlite::Connection;

fn main() {
    let root = env::args().skip(1).next().unwrap_or(String::from("."));

    let (tx, rx) = mpsc::channel::<String>();

    let db_handle = start_db_task(rx);

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
    drop(tx);

    eprintln!("crawl done, waiting for db...");
    db_handle.join().unwrap().expect("Error writing to db");
}

fn start_db_task(rx: Receiver<String>) -> JoinHandle<Result<(), rusqlite::Error>> {
    thread::spawn(move || -> rusqlite::Result<()> {
        let conn = Connection::open("./test.db")?;
        conn.execute(
            "create table if not exists visited (
             id integer primary key,
             path text not null unique
         )",
            [],
        )?;

        conn.execute("pragma synchronous = 0", [])?;

        let mut insert = conn.prepare("insert or ignore into visited (path) values (?)")?;
        for path in rx {
            let count = insert.execute(&[&path])?;
            if count > 0 {
                println!("{}", path);
            }
        }
        Ok(())
    })
}
