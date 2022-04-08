use ignore::WalkBuilder;

// fn visit()

fn main() {
    let walker = WalkBuilder::new(".")
        .follow_links(false)
        .same_file_system(true)
        .build_parallel();

    walker.run(|| {
        Box::new(move |entry_res| {
            match entry_res {
                Ok(entry) => {
                    let path = entry.path();
                    if !path.is_dir() {
                        println!("{}", path.display());
                    }
                }
                Err(e) => eprintln!("{}", e),
            }
            ignore::WalkState::Continue
        })
    });
}
