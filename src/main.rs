use std::path::Path;

#[macro_use]
mod log;

mod articles;
mod git;

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

    let mut frontend = matches.value_of("frontend").unwrap().split(":");

    let out = Path::new(matches.value_of("out").unwrap());
    let articles = matches.value_of("articles").unwrap();

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
    }

    // 2. Parse the articles
    // TODO
}
