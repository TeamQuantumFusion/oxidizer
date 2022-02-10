use std::collections::{HashMap, HashSet};
use std::ffi::OsString;
use std::fmt::Debug;
use std::fs::DirEntry;
use std::path::{Path, PathBuf};

use image::{ImageBuffer, Rgba};
use image::DynamicImage::ImageLumaA8;
use indicatif::{ParallelProgressIterator, ProgressBar};
use rayon::iter::IntoParallelRefIterator;

use crate::{ registry, util, xnb};
use crate::util::TerrariaFile;
use crate::xnb::XNBFile;
use crate::xnb::XNBFile::Texture;

pub type Sprite = ImageBuffer<Rgba<u8>, Vec<u8>>;

pub struct ResourceManager {
    pub sprite_path: PathBuf
}

pub enum ResourcePath {
    Tile(u32),
    Wall(u32),
    Item(u32),
}

impl ResourcePath {
    pub fn get_string(&self) -> String {
        match self {
            ResourcePath::Tile(id) => format!("Tiles_{}", id),
            ResourcePath::Wall(id) => format!("Wall_{}", id),
            ResourcePath::Item(id) => format!("Item_{}", id),
        }
    }
}

impl ResourceManager {
    pub fn get_sprite(&self, path: ResourcePath) -> Option<Sprite> {
        let xnb = xnb::convert_xnb_file(self.sprite_path.join(path.get_string() + ".xnb")).unwrap();
        match xnb {
            Texture(sprite) => Some(sprite),
            _ => None
        }
    }
}