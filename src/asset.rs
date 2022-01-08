use std::collections::{HashMap, HashSet};
use std::ffi::OsString;
use std::fmt::Debug;
use std::path::{Path, PathBuf};

use image::{ImageBuffer, Rgba};
use image::DynamicImage::ImageLumaA8;
use indicatif::ProgressBar;

use crate::{Progress, registry, util, xnb};
use crate::util::TerrariaFile;
use crate::xnb::XNBFile;
use crate::xnb::XNBFile::Texture;

pub type Sprite = ImageBuffer<Rgba<u8>, Vec<u8>>;

pub struct ResourceManager {
    pub sprites: HashMap<String, PathBuf>,
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
        let path = self.sprites.get(&*path.get_string())?;
        let xnb = xnb::convert_xnb_file(path.to_path_buf()).unwrap();
        match xnb {
            Texture(sprite) => Some(sprite),
            _ => None
        }
    }
}



pub fn collect_assets(content: &Path, progress: &mut Progress) -> ResourceManager {
    let mut manager = ResourceManager { sprites: Default::default() };

    let images = content.join("Images");
    let dir = images.read_dir().unwrap();
    for entry in progress.iter_unsized(dir, 12000, "Indexing Images") {
        let dir = entry.unwrap();
        let string = dir.file_name();
        if string.to_str().unwrap().ends_with(".xnb") {
            manager.sprites.insert(string.to_str().unwrap().trim_end_matches(".xnb").to_string(), dir.path());
        }
    };

    progress.print(format!("Indexed {} images", manager.sprites.len()));


    manager
}