extern crate image as extern_image;

use std::path::Path;

mod pixels;
use pixels::{RGBAPixel};

mod image;
use image::{Image};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
const PIXELS_SIZE: usize = (WIDTH * HEIGHT) as usize;

fn main() {
    let path = Path::new("out/test.png");

    let image: Image<RGBAPixel> = Image::new(PIXELS_SIZE);

    extern_image::save_buffer(path, &image.raw_buffer(), WIDTH, HEIGHT, extern_image::RGBA(8)).unwrap()
}
