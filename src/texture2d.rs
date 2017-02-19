use device;

use glium;
use image;
use std::io;
use std::io::Cursor;
use std::io::prelude::*;
use std::fs::File;

pub struct Texture2d {
    tex: glium::texture::Texture2d
}

impl Texture2d {
    pub fn from_bytes(bytes: Vec<u8>, dev: &device::Device) -> Self {
        let image = image::load(Cursor::new(bytes), image::PNG).unwrap().to_rgba();
        let image_dimensions = image.dimensions();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions);
        let texture = glium::texture::Texture2d::new(dev.display(), image).unwrap();

        Texture2d {
            tex: texture
        }
    }

    pub fn from_file(path: &str, dev: &device::Device) -> Self {
        let mut file = File::open(path).unwrap();
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes);
        Self::from_bytes(bytes, dev)
    }
}