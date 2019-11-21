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
                pos: Vec3::new(0.0, 5.0, -20.0),
                radius: 5.0,
                color: Color::new(255, 0, 0),
            }),
            Box::new(Sphere {
                pos: Vec3::new(0.0, 0.0, -13.0),
                radius: 1.0,
                color: Color::new(255, 0, 0),
            }),
            Box::new(Plane {
                point: Vec3::new(0.0, -2.0, 0.0),
                normal: Vec3::new(0.0, 1.0, 0.0),
                color: Color::new(0, 0, 255),
            }),
        ],
        lights: vec![
            Vec3::new(-10.0, 20.0, -20.0),
        ],
    };

    let dims = (1_000, 1_000);

    let pixels = scene.render(dims, 5, 10);
    write_image("output.png", &pixels, dims)
        .expect("Failed to write to image");
}
