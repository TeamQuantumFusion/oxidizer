use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

use crate::asset::Sprite;
use bytes::{Buf, Bytes};
use image::RgbaImage;
use lzxd::{Lzxd, WindowSize};
use thiserror::Error;

const HEADER_SIZE: i32 = 14;

pub enum XNBFile {
    Texture(Sprite),
    Unknown,
}

#[derive(Debug, Error)]
pub enum XNBError {
    #[error("Not an XNB file")]
    NotAnXNBFile,
    #[error("Shared Resources are not supported.")]
    SharedResourcesNotSupported,
    #[error("Primary asset not found.")]
    PrimaryAssetNotFound,

    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

pub fn convert_xnb_file(file_in: PathBuf) -> Result<XNBFile, XNBError> {
    let mut file = File::open(file_in)?;
    let mut vec = Vec::new();
    file.read_to_end(&mut vec)?;
    let mut data = Bytes::from(vec);

    if data.split_to(3).as_ref() != b"XNB" {
        return Err(XNBError::NotAnXNBFile);
    }

    let _platform = data.get_u8();
    let _version = data.get_u8();

    let compressed = (data.get_u8() & 0x80) != 0;

    let compressed_size = data.get_i32_le();
    let decompressed_size = if compressed {
        data.get_i32_le()
    } else {
        compressed_size
    };

    if compressed {
        let vec = decompress_lzx(
            &mut data,
            (compressed_size - HEADER_SIZE) as u32,
            decompressed_size as u32,
        );
        read_xnb_data(&mut Bytes::from(vec))
    } else {
        read_xnb_data(&mut data)
    }
}

fn read_xnb_data(input: &mut Bytes) -> Result<XNBFile, XNBError> {
    let type_reader_count = get_varint(input);
    let mut type_reader_name = xnb_get_string(input);

    // reader version
    let _reader_version = input.get_i32();

    if let Some(asem_info_index) = type_reader_name.find(',') {
        type_reader_name.truncate(asem_info_index);
    }

    for _ in 1..type_reader_count {
        xnb_get_string(input);
        input.get_i32();
    }

    let resources = get_varint(input);
    if resources != 0 {
        return Err(XNBError::SharedResourcesNotSupported);
    }

    let assets = get_varint(input);
    if assets != 1 {
        return Err(XNBError::PrimaryAssetNotFound);
    }

    Ok(match type_reader_name.as_str() {
        "Microsoft.Xna.Framework.Content.Texture2DReader" => {
            let _surface_format = input.get_u32_le();
            let width = input.get_u32_le();
            let height = input.get_u32_le();
            let _mipmaps = input.get_u32_le();
            let _size = input.get_u32_le();

            XNBFile::Texture(RgbaImage::from_raw(width, height, input.to_vec()).unwrap())
        }
        _ => XNBFile::Unknown,
    })
}

//ByteBuffer input, int inputLength, ByteBuffer output, int outputLength
fn decompress_lzx(input: &mut Bytes, input_length: u32, output_length: u32) -> Vec<u8> {
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

fn get_varint(input: &mut Bytes) -> i32 {
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

fn xnb_get_string(input: &mut Bytes) -> String {
    let length = get_varint(input);
    let bytes = input.split_to(length as usize);
    String::from_utf8(bytes.to_vec()).unwrap()
}
