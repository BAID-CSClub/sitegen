use std::{path::Path, process::exit};

#[macro_use]
mod log;
mod articles;
mod git;
mod routes;

fn main() {
    let cmd = clap::Command::new("sitegen")
        .subcommand_required(false)
        .arg(
            clap::Arg::new("frontend")
                .help("The repo of the frontend project <REPO>:<BRANCH>")
                .default_value("https://github.com/BAID-CSClub/baid-website-next.git:build")
                .required(false),
        )
        .arg(
            clap::Arg::new("out")
                .help("The output directory")
                .default_value("./dist")
                .required(false),
        )
        .arg(
            clap::Arg::new("articles")
                .help("The articles directory")
                .default_value("./")
                .required(false),
        );
    let matches = cmd.get_matches();

    let mut frontend = matches.value_of("frontend").unwrap().split(':');

    let out = Path::new(matches.value_of("out").unwrap());
    let articles = Path::new(matches.value_of("articles").unwrap());

    let repo = frontend.next().unwrap();
    let branch = if let Some(b) = frontend.next() {
        b
    } else {
        warn!("No branch specified, using 'build'");
        "build"
    };

    // 1. Clone the repo to the output directory

    if let Err(e) = git::clone_to(repo, branch, out) {
        error!("Fail to clone repo: {}", e);
        exit(1);
    }

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
    // 1. Parse the articles
    // TODO

    // Wait for the routes thread to finish
    if let Err(e) = routes_thread.join().unwrap() {
        error!("Fail to build routes: {}", e);
        exit(1);
    }

    // === Post-build ===
    // 1. Generate sitemap.xml
    // TODO
}
