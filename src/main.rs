use std::{fs, path::Path, path::PathBuf, process::exit};

#[macro_use]
mod log;
mod articles;
mod git;
mod routes;

use std::collections::VecDeque;

fn recursive_copy<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dest: Q) -> std::io::Result<()> {
    let src = src.as_ref();
    let dest = dest.as_ref();

    if dest.exists() {
        // Clear the destination directory if it already exists.
        fs::remove_dir_all(dest)?;
    }

    if !src.is_dir() {
        // If the source is not a directory, just copy the file.
        fs::copy(src, dest)?;
        return Ok(());
    }

    // Create the destination directory.
    fs::create_dir_all(dest)?;

    // Use a queue to store the directories that need to be processed.
    let mut queue = VecDeque::new();
    queue.push_back(src.to_path_buf());

    // Process the directories in the queue.
    while let Some(dir) = queue.pop_front() {
        // Iterate over the entries in the directory.
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                // If the entry is a directory, add it to the queue to be processed later.
                queue.push_back(path);
            } else {
                // Otherwise, copy the file to the destination directory.
                let dest_path = dest.join(&path.strip_prefix(src).unwrap());

                //                fs::create_dir_all()?;
                fs::create_dir_all(dest_path.parent().unwrap())?;
                fs::copy(&path, &dest_path)?;
            }
        }
    }

    Ok(())
}

fn main() {
    let cmd = clap::Command::new("sitegen")
        .subcommand_required(false)
        .arg(
            clap::Arg::new("frontend")
                .long("frontend")
                .help("The repo of the frontend project <REPO>|<BRANCH>")
                .default_value("https://cdn.bdbmzwsc.top/BAID-CSClub/baid-website-next.git|build")
                .required(false),
        )
        .arg(
            clap::Arg::new("out")
                .long("out")
                .help("The output directory")
                .default_value("./dist")
                .required(false),
        )
        .arg(
            clap::Arg::new("articles")
                .long("articles")
                .help("The articles directory")
                .default_value("./articles")
                .required(false),
        );
    let matches = cmd.get_matches();

    let mut frontend = matches.get_one::<String>("frontend").unwrap().split('|');

    let out = Path::new(matches.get_one::<String>("out").unwrap());
    let articles = Path::new(matches.get_one::<String>("articles").unwrap());

    let repo = frontend.next().unwrap();
    let branch = if let Some(b) = frontend.next() {
        b
    } else {
        warn!("No branch specified, using 'build'");
        "build"
    };

    //    // 1. Clone the repo to the output directory
    //    if let Err(e) = git::clone_to(repo, branch, out) {
    //        error!("Fail to clone repo: {}", e);
    //        exit(1);
    //    }

    // DBG: COPY from ./dist-original to ./dist
    recursive_copy("./dist-original", "./dist").unwrap();

    // 2. Start a new thread for routes building
    let out_clone = out.to_owned();
    let articles_clone = articles.to_owned();
    let routes_thread = std::thread::spawn(move || {
        // 1. Find `routes.toml` in: `articles` / `out`
        let mut routes_path = articles_clone.join("routes.toml");
        if !routes_path.exists() {
            routes_path = out_clone.join("routes.toml");
            if !routes_path.exists() {
                anyhow::bail!("No routes.toml found");
            }
        }
        routes::build(&routes_path, &out_clone)
    });

    // === Main thread ===
    // 1. Parse articles
    // 2. Process static files (video, audio, etc)
    // 3. Generate <OUTPUT>/articles/ dir

    if let Err(e) = articles::build(&articles, &out) {
        // Do all three steps in another mod
        error!("Failed to build articles: {}", e);
    }

    // Wait for the routes thread to finish
    if let Err(e) = routes_thread.join().unwrap() {
        error!("Fail to build routes: {}", e);
        exit(1);
    }

    // === Post-build ===
    // 1. Generate sitemap.xml
    // 2. Generate robots.txt
}
