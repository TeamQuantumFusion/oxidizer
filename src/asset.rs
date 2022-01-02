use std::fmt::{Debug, Formatter};
use std::path::{Display, Path, PathBuf};

use image::{ImageBuffer, Rgba};
use indicatif::ProgressBar;

use crate::{util, xnb};
use crate::Asset::Image;
use crate::util::TerrariaFile;
use crate::xnb::XNBFile::Texture;

pub type Sprite = ImageBuffer<Rgba<u8>, Vec<u8>>;

#[derive(Debug)]
pub enum Asset {
    Image(ImageAsset)
}

pub fn collect_assets(content: &Path, progress_bar: &mut ProgressBar) -> Vec<Asset> {
    let mut out = Vec::new();

    let images_dir = content.join("Images");
    for item in util::seek_files(&images_dir, progress_bar, "Importing Images") {
        out.push(Image(ImageAsset::new(&images_dir, &item)));
    };

    out
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
        let sprite = xnb::convert_xnb_file(images.join(file.name.clone() + ".xnb"));
        if let Some(Texture(sprite)) = sprite {
            if file.name.starts_with("Tiles_") {
                if let Ok(id) = file.name[6..file.name.len()].parse() {
                    return ImageAsset::Tile(id, sprite);
                }
            } else if file.name.starts_with("Walls_") {
                if let Ok(id) = file.name[6..file.name.len()].parse() {
                    return ImageAsset::Wall(id, sprite);
                }
            } else if file.name.starts_with("Item_") {
                if let Ok(id) = file.name[5..file.name.len()].parse() {
                    return ImageAsset::Item(id, sprite);
                }
            }
        }

        ImageAsset::Unknown
    }
}