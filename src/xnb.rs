use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use bytes::Buf;
use image::{ColorType, ImageBuffer, Rgba, RgbaImage, RgbImage};
use lzxd::{Lzxd, WindowSize};
use crate::asset::Sprite;

const HEADER_SIZE: i32 = 14;

pub enum XNBFile {
    Texture(Sprite),
    Unknown
}

pub fn convert_xnb_file(file_in: PathBuf) -> Option<XNBFile> {
    if let Ok(mut file) = File::open(file_in) {
        let mut vec = Vec::new();
        file.read_to_end(&mut vec);
        let mut data = vec.as_slice();


        if data.get_u8() != b'X' || data.get_u8() != b'N' || data.get_u8() != b'B' {
            panic!("Not an XNB File");
        }

        // platform / version
        data.get_u8();
        data.get_u8();

        let compressed = (data.get_u8() & 0x80) != 0;

        let compressed_size = data.get_i32_le();
        let decompressed_size = if compressed { data.get_i32_le() } else {
            compressed_size
        };

        if compressed {
            let vec = decompress_lzx(&mut data, (compressed_size - HEADER_SIZE) as u32, decompressed_size as u32);
            Some(read_xnb_data(&mut vec.as_slice()))
        } else {
            Some(read_xnb_data(&mut data))
        }
    } else {
        None
    }
}

fn read_xnb_data(input: &mut &[u8]) -> XNBFile {
    let type_reader_count = xnb_get_int(input);
    let mut type_reader_name = xnb_get_string(input);

    // reader version
    input.get_i32();


    let asem_info_index = type_reader_name.find(',');
    if let Some(index) = asem_info_index {
        type_reader_name = (&type_reader_name[0..index]).parse().unwrap()
    }

    for i in 1..type_reader_count {
        xnb_get_string(input);
        input.get_i32();
    }

    if xnb_get_int(input) != 0 {
        panic!("Shared Resources are not Supported.")
    }

    if xnb_get_int(input) != 1 {
        panic!("Primary Asset is null; wtf")
    }

    match type_reader_name.as_str() {
        "Microsoft.Xna.Framework.Content.Texture2DReader" => {
            let surface_format = input.get_u32_le();
            let width = input.get_u32_le();
            let height = input.get_u32_le();
            let mipmaps = input.get_u32_le();
            let size = input.get_u32_le();

            XNBFile::Texture(RgbaImage::from_raw(width, height, input.to_vec()).unwrap())
        }
        _ => XNBFile::Unknown,
    }
}

//ByteBuffer input, int inputLength, ByteBuffer output, int outputLength
fn decompress_lzx(input: &mut &[u8], input_length: u32, output_length: u32) -> Vec<u8> {
    let mut remaining = input_length;
    let mut out = Vec::with_capacity(output_length as usize);
    let mut chunk_decompressor = Lzxd::new(WindowSize::KB64);

    while remaining > 0 {
        let mut hi = input.get_u8();
        let mut lo = input.get_u8();
        let mut block_size: u32 = ((hi as u32) << 8u32) | lo as u32;
        let mut frame_size: u32 = 0x8000;

        if hi == 0xFF {
            hi = lo;
            lo = input.get_u8();
            frame_size = ((hi as u32) << 8u32) | lo as u32;
            hi = input.get_u8();
            lo = input.get_u8();
            block_size = ((hi as u32) << 8u32) | lo as u32;
            remaining -= 5;
        } else {
            remaining -= 2;
        }

        if block_size == 0 || frame_size == 0 {
            return out;
        }


        let chunk = &input[0..block_size as usize];
        let result = chunk_decompressor.decompress_next(chunk);
        input.advance(block_size as usize);
        if let Ok(data) = result {
            out.write_all(data).unwrap();
        } else {
            panic!("Failed {}", result.unwrap_err());
        }

        remaining -= block_size as u32;
    }
    out
}

fn xnb_get_int(input: &mut &[u8]) -> i32 {
    let mut result: i32 = 0;
    let mut bits_read = 0;
    loop {
        let value = input.get_u8();
        result |= (value as i32 & 0x7f) << bits_read;
        bits_read += 7;

        if (value & 0x80) == 0 {
            break;
        }
    }

    result
}

fn xnb_get_string(input: &mut &[u8]) -> String {
    let length = xnb_get_int(input);
    let bytes = input.copy_to_bytes(length as usize);
    let out = String::from_utf8(bytes.to_vec()).unwrap();
    out
}
