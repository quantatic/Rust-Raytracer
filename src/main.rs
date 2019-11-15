mod geometry;
mod shapes;

use crate::geometry::{Ray, Vec3};
use crate::shapes::{Hitable, Sphere};

use image::ColorType;
use image::png::PNGEncoder;
use std::fs::File;

fn pixel_to_location((pixel_width, pixel_height): (usize, usize), (pixel_x, pixel_y): (usize, usize)) -> (f64, f64) {
	let loc_x = ((pixel_x as f64) - ((pixel_width as f64) / 2.0)) / (pixel_width as f64) * 2.0;
	let loc_y = ((pixel_y as f64) - ((pixel_height as f64) / 2.0)) / (pixel_height as f64) * 2.0;

	(loc_x, loc_y)
}

fn render((width, height): (usize, usize), objects: &[Sphere]) -> Vec<u8> {
	let mut result = Vec::new();

	for i in 0..height {
		for j in 0..width {
			let (x, y) = pixel_to_location((width, height), (i, j));

            let ray = Ray {
                pos: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0
                },
                dir: Vec3 {
                    x,
                    y,
                    z: -1.0
                }
            };

            let mut hit = false;
            for obj in objects.iter() {
                if obj.hit(&ray) {
                    hit = true;
                    break;
                }
            }

			result.push(if hit {255} else {0});
        }
    }

	result
}

fn write_image(filename: &str, pixels: &[u8], (width, height): (usize, usize)) -> Result<(), std::io::Error> {
	let output = File::create(filename)?;

	let encoder = PNGEncoder::new(output);

	encoder.encode(pixels, width as u32, height as u32, ColorType::Gray(8))
}

fn main() {
    println!("Hello, world!");

    let objects = vec![
        Sphere {
            pos: Vec3 {
                x: -1.0,
                y: -1.0,
                z: -1.5
            },
            radius: 1.0
        },
        Sphere {
            pos: Vec3 {
                x: 1.0,
                y: 1.0,
                z: -2.0
            },
            radius: 1.0
        }
    ];

	let dims = (500, 500);

    let pixels = render(dims, &objects);
	write_image("output.png", &pixels, dims)
		.expect("Failed to write to file");

}
