use std::fmt::Debug;
use std::path::Path;

use image::{ImageBuffer, Rgba};
use indicatif::ProgressBar;

use crate::util::TerrariaFile;
use crate::xnb::XNBFile::Texture;
use crate::Asset::Image;
use crate::{util, xnb};

pub type Sprite = ImageBuffer<Rgba<u8>, Vec<u8>>;

#[derive(Debug)]
pub enum Asset {
    Image(ImageAsset),
}

pub fn collect_assets(content: &Path, progress_bar: &mut ProgressBar) -> Vec<Asset> {
    let images_dir = content.join("Images");

    util::seek_files(&images_dir, progress_bar, "Importing Images")
        .map(|item| Image(ImageAsset::new(&images_dir, &item)))
        .collect()
}

#[derive(Debug)]
pub enum ImageAsset {
    Tile(u32, Sprite),
    Wall(u32, Sprite),
    Item(u32, Sprite),
    Unknown,
}

impl ImageAsset {
    pub fn new(images: &Path, file: &TerrariaFile) -> ImageAsset {
        let mut path = images.join(&file.name);
        path.set_file_name(".xnb");
        let sprite = xnb::convert_xnb_file(path);

        if let Ok(Texture(sprite)) = sprite {
            if file.name.starts_with("Tiles_") {
                if let Ok(id) = file.name[6..].parse() {
                    return ImageAsset::Tile(id, sprite);
                }
            } else if file.name.starts_with("Walls_") {
                if let Ok(id) = file.name[6..].parse() {
                    return ImageAsset::Wall(id, sprite);
                }
            } else if file.name.starts_with("Item_") {
                if let Ok(id) = file.name[5..].parse() {
                    return ImageAsset::Item(id, sprite);
                }
            }
        }

        ImageAsset::Unknown
    }
}
