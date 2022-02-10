use console::{Color, style};
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::asset::{ResourceManager, ResourcePath};

mod asset;
mod util;
mod xnb;
mod mapper;
mod registry;

fn main() {
    let cwd = std::env::current_dir().expect("Could not access current working directory");
    if cwd.file_stem().unwrap() != "Content" {
        panic!("Not launched from Terraria's \"Content\" directory.")
    }

    let mut progress_bar = new_progress_bar();

    let out = cwd.join("rustaria");
    let sprite = out.join("sprite");

    let manager = ResourceManager {
        sprite_path: cwd.join("Images")
    };

    let tile = sprite.join("tile");
    std::fs::create_dir_all(&tile).unwrap();
    progress_bar.set_length((registry::BLOCK_TILES.len() + registry::WALLS.len()) as u64);


    registry::BLOCK_TILES.par_iter().progress_with(progress_bar.clone()).for_each(|id| {
        if let Some(sprite) = manager.get_sprite(ResourcePath::Tile(id.0)) {
            let sprite = mapper::remap_tile(sprite);
            let result = sprite.save(tile.join(format!("{}.png", id.1)));

            if let Err(error) = result {
                progress_bar.println(format!("Failed to export {} {}", id.1, error));
            }
        }
    });

    let wall = sprite.join("wall");
    std::fs::create_dir_all(&wall).unwrap();


    registry::WALLS.par_iter().progress_with(progress_bar.clone()).for_each(|id| {
        if let Some(sprite) = manager.get_sprite(ResourcePath::Wall(id.0)) {
            let sprite = mapper::remap_wall(sprite);
            let result = sprite.save(wall.join(format!("{}.png", id.1)));

            if let Err(error) = result {
                progress_bar.println(format!("Failed to export {} {}", id.1, error));
            }
        }
    });


    //let test = sprite.join("test");
    //std::fs::create_dir_all(&test).unwrap();
    //for i in progress_bar.iter(1..5124, "Mapping Items") {
    //    if let Some(sprite) =  manager.get_sprite(ResourcePath::Item(i)) {
    //        let sprite = image::imageops::resize(&sprite, sprite.width() / 2, sprite.height() / 2, FilterType::Nearest);
    //        sprite.save(test.join(format!("{}.png", i)));
    //    }
    //}
}




pub fn new_progress_bar() -> ProgressBar {
    let header = "{msg} [{prefix}".to_owned() + "{percent}% ";
    let footer = "{eta} {pos}".to_owned() + &*style("/").fg(Color::White).to_string() + "{len}";

    let format = header + "{wide_bar:.cyan}\x1b[m] " + &*footer;
    let progress_bar = ProgressBar::new(1).with_style(
        ProgressStyle::default_bar()
            .template(&*format)
            .progress_chars("##-"),
    );


    progress_bar
}