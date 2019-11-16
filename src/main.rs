mod color;
mod hit;
mod ray;
mod shapes;
mod vector;

use crate::color::Color;
use crate::hit::Hit;
use crate::vector::Vec3;
use crate::ray::Ray;
use crate::shapes::{Hitable, Sphere};
use image::{ColorType, ImageBuffer, Rgb};
use image::png::PNGEncoder;
use std::fs::File;

// Renders a square 2x2 view plane, 1 unit away from the camera. The camera is located at (0, 0).
fn render(rendered_dims: (u32, u32), objects: &[Box<dyn Hitable>]) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let (rendered_width, rendered_height) = rendered_dims;

    let mut result: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(rendered_width, rendered_height);

    for (pixel_x, pixel_y, pixel) in result.enumerate_pixels_mut() {
        let x = ((pixel_x as f64) - ((rendered_width as f64) / 2.0) + 0.5) / (rendered_width as f64);
        let y = ((pixel_y as f64) - ((rendered_height as f64) / 2.0) + 0.5) / (rendered_height as f64);

        let ray = Ray {
            pos: Vec3::new(0.0, 0.0, 0.0),
            dir: Vec3::new(x, y, -1.0),
        };

        let mut closest_hit: Option<Hit> = None;
        for obj in objects.iter() {
            if let Some(new_hit_record) = obj.hit(&ray) {
                if let Some(old_hit_record) = &closest_hit {
                    if old_hit_record.dist > new_hit_record.dist {
                        closest_hit = Some(new_hit_record);
                    }
                } else {
                    closest_hit = Some(new_hit_record);
                }
            }
        }

        *pixel = match closest_hit {
            Some(Hit{hit, ..}) => {
                hit.color().into()
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
            pos: Vec3::new(0.0, 0.0, -7.0),
            radius: 1.0,
            color: Color::new(255, 0, 0),
        }),
        Box::new(Sphere {
            pos: Vec3::new(2.0, 0.0, -7.0),
            radius: 2.5,
            color: Color::new(0, 255, 0),
        }),
        Box::new(Sphere {
            pos: Vec3::new(1.0, 1.0, -5.0),
            radius: 1.0,
            color: Color::new(0, 0, 255),
        }),
    ];

    let dims = (1_000, 1_000);

    let pixels = render(dims, &objects);
    write_image("output.png", &pixels, dims)
        .expect("Failed to write to image");
}
