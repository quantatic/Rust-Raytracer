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

// Renders a square 2x2 view plane, 1 unit away from the camera. The camera is located at (0, 0).
fn render(rendered_dims: (u32, u32), objects: &[Box<dyn Hitable>]) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let (rendered_width, rendered_height) = rendered_dims;

    let mut result: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(rendered_width, rendered_height);

    for (pixel_x, pixel_y, pixel) in result.enumerate_pixels_mut() {
        let x = ((pixel_x as f64) - ((rendered_width as f64) / 2.0) + 0.5) / (rendered_width as f64);
        let y = (((rendered_height as f64) / 2.0) - (pixel_y as f64) + 0.5) / (rendered_height as f64);

        let ray = Ray {
            pos: Vec3::new(0.0, 0.0, 0.0),
            dir: Vec3::new(x, y, -1.0).unit(),
        };


        *pixel = match camera::cast_ray(objects, ray, 3) {
			Some(color) => {
				color.into()
            },
            None => image::Rgb([127; 3])
        };
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

    let dims = (5_000, 5_000);

    let pixels = render(dims, &objects);
    write_image("output.png", &pixels, dims)
        .expect("Failed to write to image");
}
