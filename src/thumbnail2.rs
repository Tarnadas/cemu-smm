//! Super Mario Maker 2 thumbnail file manipulation.

use crate::{decrypt, encrypt, key_tables::*};

use image::{jpeg::JPEGEncoder, load_from_memory, DynamicImage, ImageError};
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Clone, Debug, PartialEq)]
pub struct Thumbnail2 {
    encrypted: Vec<u8>,
    jpeg: Option<Vec<u8>>,
    jpeg_opt: Option<Vec<u8>>,
}

impl Thumbnail2 {
    pub fn new(bytes: Vec<u8>) -> Thumbnail2 {
        Thumbnail2 {
            encrypted: bytes,
            jpeg: None,
            jpeg_opt: None,
        }
    }

    pub fn from_decrypted(bytes: Vec<u8>) -> Thumbnail2 {
        Thumbnail2 {
            encrypted: encrypt(bytes.clone(), &THUMBNAIL_KEY_TABLE, false),
            jpeg: Some(bytes),
            jpeg_opt: None,
        }
    }

    pub fn encrypt(bytes: Vec<u8>) -> Vec<u8> {
        encrypt(bytes, &THUMBNAIL_KEY_TABLE, false)
    }

    pub fn decrypt(bytes: Vec<u8>) -> Vec<u8> {
        decrypt(bytes, &THUMBNAIL_KEY_TABLE)
    }

    pub fn get_encrypted(&self) -> &Vec<u8> {
        &self.encrypted
    }

    pub fn move_jpeg(&mut self) -> Vec<u8> {
        self.lazy_load_jpeg();
        self.jpeg.clone().unwrap()
    }

    pub fn get_jpeg(&mut self) -> &[u8] {
        self.lazy_load_jpeg();
        if let Some(jpeg) = &self.jpeg_opt {
            &jpeg[..]
        } else {
            self.jpeg.as_ref().unwrap()
        }
    }

    pub fn get_jpeg_no_opt(&mut self) -> &[u8] {
        self.lazy_load_jpeg();
        self.jpeg.as_ref().unwrap()
    }

    pub fn optimize_jpeg(&mut self) -> Result<(), ImageError> {
        let jpeg = self.get_jpeg();

        let image = load_from_memory(jpeg)?;
        let color = image.color();

        match image {
            DynamicImage::ImageRgb8(buffer) => {
                let (width, height) = buffer.dimensions();
                let mut opt = vec![];
                let mut encoder = JPEGEncoder::new_with_quality(&mut opt, 80);
                encoder
                    .encode(&buffer.into_raw()[..], width, height, color)
                    .map_err(ImageError::from)?;
                self.jpeg_opt = if opt.len() < jpeg.len() {
                    Some(opt)
                } else {
                    Some(jpeg.to_vec())
                };
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn lazy_load_jpeg(&mut self) {
        let decrypted = if self.jpeg.is_none() {
            Some(decrypt(self.encrypted.clone(), &THUMBNAIL_KEY_TABLE))
        } else {
            None
        };
        if decrypted.is_some() {
            self.jpeg = decrypted;
        }
    }
}