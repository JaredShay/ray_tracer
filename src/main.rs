extern crate image as extern_image;

use std::path::Path;

mod pixels;
use pixels::{RGBPixel};

mod image;
use image::{Image};

mod vector;
use vector::{Vector3, IntoVector3};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

fn main() {
    let vector_1 = Vector3(1.0, 1.0, 1.0);
    let vector_2 = Vector3(1.0, 2.0, 1.0);

    println!("v1 + v2: {:?}", vector_1.add(&vector_2));
    println!("v1 - v2: {:?}", vector_1.subtract(&vector_2));
    println!("v1 * v2: {:?}", vector_1.multiply(&vector_2));
    println!("v1 * v2: {:?}", vector_1.multiply(&vector_2));
    println!("v1 * 2.0: {:?}", vector_1.multiply(&2.0));
    //let path = Path::new("out/result.png");

    //let mut image: Image<RGBPixel> = Image::new(WIDTH, HEIGHT);

    //for i in 0..WIDTH {
    //    for j in 0..HEIGHT {
    //        let mut pixel = image.pixel_at_mut(i, j).unwrap();

    //        pixel.r = (((1.0 / WIDTH as f32) * 255.0) * i as f32) as u8;
    //        pixel.g = (((1.0 / HEIGHT as f32) * 255.0) * (HEIGHT as f32 - j as f32)) as u8;
    //        pixel.b = (0.3 * 255.0) as u8;
    //    }
    //}

    //extern_image::save_buffer(
    //    path, &image.raw_buffer(), WIDTH, HEIGHT, extern_image::RGB(8)).unwrap()
}
