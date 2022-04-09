use ignore::WalkBuilder;

fn main() {
    let threads = num_cpus::get();

    let walker = WalkBuilder::new(".")
        .follow_links(false)
        .same_file_system(true)
        .threads(threads)
        .build_parallel();

    walker.run(|| {
        Box::new(move |entry_res| {
            match entry_res {
                Ok(entry) => {
                    if entry.file_type().map_or(false, |ft| ft.is_file()) {
                        let path = entry.path();
                        println!("{}", path.display());
                    }
                }
                Err(e) => eprintln!("{}", e),
            }
            ignore::WalkState::Continue
        })
    });
}

