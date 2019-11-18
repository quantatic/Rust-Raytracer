mod camera;
mod color;
mod hit;
mod ray;
mod shapes;
mod vector;

use crate::color::Color;
use crate::vector::Vec3;
use crate::ray::Ray;
use crate::shapes::{Hitable, Sphere, Plane};
use image::{ColorType, ImageBuffer, Rgb};
use image::png::PNGEncoder;
use std::fs::File;

use rand::Rng;

// Renders a square 2x2 view plane, 1 unit away from the camera. The camera is located at (0, 0).
fn render(rendered_dims: (u32, u32), objects: &[Box<dyn Hitable>], samples: u32, depth: u32) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    assert_ne!(samples, 0);

    let (rendered_width, rendered_height) = rendered_dims;

    let mut result: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(rendered_width, rendered_height);

    for (pixel_x, pixel_y, pixel) in result.enumerate_pixels_mut() {
        let mut pixel_color: (u32, u32, u32) = (0, 0, 0);
        for _ in 0..samples {

            let x = ((pixel_x as f64) - ((rendered_width as f64) / 2.0) + rand::thread_rng().gen_range(0.0, 1.0)) / (rendered_width as f64);
            let y = (((rendered_height as f64) / 2.0) - (pixel_y as f64) + rand::thread_rng().gen_range(0.0, 1.0)) / (rendered_height as f64);

            let ray = Ray {
                pos: Vec3::new(0.0, 0.0, 0.0),
                dir: Vec3::new(x, y, -1.0).unit(),
            };


            match camera::cast_ray(objects, ray, depth) {
                Some(Color {r, g, b}) => {
                    pixel_color.0 += r as u32;
                    pixel_color.1 += g as u32;
                    pixel_color.2 += b as u32;
                },
                None => {
                    pixel_color.0 += 127;
                    pixel_color.1 += 127;
                    pixel_color.2 += 127;
                }
            };
        }

        *pixel = image::Rgb([
            (pixel_color.0 / samples) as u8,
            (pixel_color.1 / samples) as u8,
            (pixel_color.2 / samples) as u8,
        ]);
    }

    result
}

fn write_image(filename: &str, pixels: &[u8], (width, height): (u32, u32)) -> Result<(), std::io::Error> {
    let output = File::create(filename)?;

    let encoder = PNGEncoder::new(output);

    encoder.encode(pixels, width, height, ColorType::RGB(8))
}

fn main() {
    println!("Hello, world!");

    let objects: Vec<Box<dyn Hitable>> = vec![
        Box::new(Sphere {
            pos: Vec3::new(0.0, -1.4, -10.0),
            radius: 1.0,
            color: Color::new(255, 0, 0),
        }),
        Box::new(Sphere {
            pos: Vec3::new(3.0, 0.0, -12.0),
            radius: 1.0,
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
        Box::new(Plane {
            point: Vec3::new(0.0, 0.0, -20.0),
            normal: Vec3::new(0.0, 0.0, 1.0),
            color: Color::new(127, 127, 127),
        }),
    ];

    let dims = (2_000, 2_000);

    let pixels = render(dims, &objects, 10, 5);
    write_image("output.png", &pixels, dims)
        .expect("Failed to write to image");
}
