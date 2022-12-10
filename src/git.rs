use std::{fmt::Write, path::Path};

use anyhow::Ok;
use git2::{
    build::{CheckoutBuilder, RepoBuilder},
    FetchOptions,
};
use indicatif::{ProgressBar, ProgressState, ProgressStyle};

pub fn clone_to<S: AsRef<str>>(repo: S, branch: S, out: &Path) -> anyhow::Result<()> {
    if out.is_dir() {
        // Delete the directory if it exists
        warn!(
            "Deleting existing directory: {}...",
            out.display().to_string().light_cyan()
        );
        std::fs::remove_dir_all(out)?;
    }
    info!(
        "Cloning {} to {}",
        repo.as_ref().light_cyan(),
        out.display().to_string().light_cyan()
    );
    let pb = ProgressBar::new(1);
    pb.set_style(
        ProgressStyle::with_template(
            "[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta}) {msg}",
        )
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| {
            write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()
        })
        .progress_chars("#>-"),
    );
    let mut co = CheckoutBuilder::new();
    co.progress(|path, current, total| {
        pb.set_position(current as u64);
        pb.set_message(if let Some(path) = path {
            path.display().to_string()
        } else {
            String::new()
        });
        pb.set_length(total as u64);
    });

    RepoBuilder::new()
        .fetch_options(FetchOptions::new())
        .with_checkout(co)
        .branch(branch.as_ref())
        .clone(repo.as_ref(), out)?;
    pb.finish_with_message("Done");
    Ok(())
}

#[test]
fn test_clone() -> anyhow::Result<()> {
    info!("Testing clone");
    clone_to(
        "https://cdn.bdbmzwsc.top/BAID-CSClub/baid-website-next.git",
        "build",
        Path::new("./dist-original"),
    )
}
