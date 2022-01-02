use std::path::{Path, PathBuf};
use indicatif::{ProgressBar, ProgressStyle};
use crate::asset::Asset;

mod asset;
mod util;
mod xnb;

fn main() {
    let content = Path::new("./");
    if !content.canonicalize().unwrap().to_str().unwrap().ends_with("Content") {
        panic!("Not launched from Terraria's \"Content\" directory.")
    }

    let mut progress_bar = ProgressBar::new(1).with_style(ProgressStyle::default_bar().template("{msg} {wide_bar:.cyan} {pos}/{len}").progress_chars("=> "));

    let assets = asset::collect_assets(content, &mut progress_bar);


    for x in assets {

    }
}
