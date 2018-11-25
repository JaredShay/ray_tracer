extern crate image as extern_image;

use std::path::Path;

mod pixels;
use pixels::{RGBPixel};

mod image;
use image::{Image};

mod vector;
use vector::{Vector3, Vector3Operations};

mod ray;
use ray::{Ray};

const WIDTH: u32 = 200;
const HEIGHT: u32 = 100;

fn color(ray: &Ray) -> Vector3 {
    let t: f32 = 0.5 * (ray.direction.unit_vector().y() + 1.0);

    let white = Vector3(1.0, 1.0, 1.0);
    let blue = Vector3(0.5, 0.7, 1.0);

    // Vec representing unit vec white
    white.lerp(&blue, t)
}

fn main() {
    let path = Path::new("out/result.png");

    let mut image: Image<RGBPixel> = Image::new(WIDTH, HEIGHT);

    let lower_left_corner = Vector3(-2.0, -1.0, -1.0);
    let horizontal = Vector3(4.0, 0.0, 0.0);
    let vertical = Vector3(0.0, 2.0, 0.0);
    let origin = Vector3(0.0, 0.0, 0.0);

    for y in (0..HEIGHT) {
        for x in (0..WIDTH) {
            // flip the x and y when fetching pixels here. This is just a quirk
            // of the tutorial I'm using having a different co-ord system
            let mut pixel = image.pixel_at_mut(199 - x, 99 - y).unwrap();

            let ray_x = x as f32 / WIDTH as f32;
            let ray_y = y as f32 / HEIGHT as f32;

            let ray: Ray = Ray {
                origin: &origin,
                direction: lower_left_corner.add(&horizontal.multiply(&ray_x)).add(&vertical.multiply(&ray_y)) };

            let color: Vector3 = color(&ray);

            pixel.r = (255.99 * color.x()) as u8;
            pixel.g = (255.99 * color.y()) as u8;
            pixel.b = (255.99 * color.z()) as u8;
        }
    }

    extern_image::save_buffer(
        path, &image.raw_buffer(), WIDTH, HEIGHT, extern_image::RGB(8)).unwrap()
}
