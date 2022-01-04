use image::ImageError;
use image::imageops::FilterType;

use crate::asset::{collect_assets, ResourceManager, ResourcePath};
use crate::progress_bar::Progress;
use crate::xnb::XNBFile::Texture;

mod asset;
mod util;
mod xnb;
mod mapper;
mod registry;
mod progress_bar;

fn main() {
    let cwd = std::env::current_dir().expect("Could not access current working directory");
    if cwd.file_stem().unwrap() != "Content" {
        panic!("Not launched from Terraria's \"Content\" directory.")
    }

    let mut progress_bar = Progress::new();

    let out = cwd.join("rustaria");
    let sprite = out.join("sprite");

    let manager = collect_assets(&cwd, &mut progress_bar);

    let tile = sprite.join("tile");
    std::fs::create_dir_all(&tile).unwrap();

    for id in progress_bar.iter(registry::BLOCK_TILES.iter(), "Mapping Tiles") {
        if let Some(sprite) = manager.get_sprite(ResourcePath::Tile(id.0)) {
            let sprite = mapper::remap_tile(sprite);
            let result = sprite.save(tile.join(format!("{}.png", id.1)));

            result.map_err(|error| {
                progress_bar.print(format!("Failed to export {} {}", id.1, error));
            }).unwrap();
        }
    }

    let wall = sprite.join("wall");
    std::fs::create_dir_all(&wall).unwrap();

    for id in progress_bar.iter(registry::WALLS.iter(), "Mapping Walls") {
        if let Some(sprite) = manager.get_sprite(ResourcePath::Wall(id.0)) {
            let sprite = mapper::remap_wall(sprite);
            let result = sprite.save(wall.join(format!("{}.png", id.1)));

            result.map_err(|error| {
                progress_bar.print(format!("Failed to export {} {}", id.1, error));
            }).unwrap();
        }
    }


    //let test = sprite.join("test");
    //std::fs::create_dir_all(&test).unwrap();
    //for i in progress_bar.iter(1..5124, "Mapping Items") {
    //    if let Some(sprite) =  manager.get_sprite(ResourcePath::Item(i)) {
    //        let sprite = image::imageops::resize(&sprite, sprite.width() / 2, sprite.height() / 2, FilterType::Nearest);
    //        sprite.save(test.join(format!("{}.png", i)));
    //    }
    //}
}

