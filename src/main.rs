mod color;
mod hit;
mod ray;
mod scene;
mod shapes;
mod vector;

use crate::scene::Scene;
use crate::color::Color;
use crate::vector::Vec3;
use crate::shapes::{Sphere, Plane};
use image::ColorType;
use image::png::PNGEncoder;
use std::fs::File;

fn write_image(filename: &str, pixels: &[u8], (width, height): (u32, u32)) -> Result<(), std::io::Error> {
    let output = File::create(filename)?;

    let encoder = PNGEncoder::new(output);

    encoder.encode(pixels, width, height, ColorType::RGB(8))
}

fn main() {
    println!("Hello, world!");

    let scene = Scene {
        objects: vec![
            Box::new(Sphere {
                pos: Vec3::new(0.0, -1.4, -10.0),
                radius: 1.0,
                color: Color::new(255, 0, 0),
            }),
            Box::new(Sphere {
                pos: Vec3::new(5.0, -2.0, -20.0),
                radius: 2.0,
                color: Color::new(255, 255, 0),
            }),
            Box::new(Sphere {
                pos: Vec3::new(3.0, 3.0, -12.0),
                radius: 2.0,
                color: Color::new(127, 255, 127),
            }),
            Box::new(Sphere {
                pos: Vec3::new(-3.0, 0.0, -12.0),
                radius: 1.0,
                color: Color::new(255, 0, 255),
            }),
            Box::new(Plane {
                point: Vec3::new(0.0, -2.0, 0.0),
                normal: Vec3::new(0.0, 1.0, 0.0),
                color: Color::new(0, 0, 255),
            }),
            Box::new(Sphere {
                pos: Vec3::new(-5.0, 5.0, -20.0),
                radius: 3.0,
                color: Color::new(255, 125, 0),
            }),
        ],
    };

    let dims = (2500, 2500);

    let pixels = scene.render(dims, 10, 10);
    write_image("output.png", &pixels, dims)
        .expect("Failed to write to image");
}
