use crate::asset::Asset;
use indicatif::{ProgressBar, ProgressStyle};

mod asset;
mod util;
mod xnb;

fn main() {
    let cwd = std::env::current_dir().expect("Could not access current working directory");
    if cwd.file_stem().unwrap() != "Content" {
        panic!("Not launched from Terraria's \"Content\" directory.")
    }

    let mut progress_bar = ProgressBar::new(1).with_style(
        ProgressStyle::default_bar()
            .template("{msg} {wide_bar:.cyan} {pos}/{len}")
            .progress_chars("=> "),
    );

    let assets = asset::collect_assets(&cwd, &mut progress_bar);

    for x in assets {}
}
