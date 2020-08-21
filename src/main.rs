use raytrace::Ray;
use raytrace::Vec3;
use raytrace::{Shape, Sphere};

use std::error::Error;

use image::{ImageBuffer, Rgb};

use rayon::prelude::*;

use std::ops::Add;

use rand::Rng;

const WIDTH: u32 = 5_000;
const ASPECT_RATIO: f64 = 1.0; // ratio of WIDTH / HEIGHT
const SAMPLES: u32 = 100;

const HEIGHT: u32 = ((WIDTH as f64) * ASPECT_RATIO) as u32;

const SCALE_FACTOR: f64 = 2.0 / (WIDTH as f64);

fn main() -> Result<(), Box<dyn Error>> {
    let mut shapes: Vec<Box<dyn Shape + Sync>> = Vec::new();

    shapes.push(Box::new(Sphere::new(Vec3::new(-0.95, 0.0, -2.0), 1.0)));
    shapes.push(Box::new(Sphere::new(Vec3::new(0.95, 0.0, -2.0), 1.0)));

    let mut image_buffer: ImageBuffer<Rgb<u8>, _> = ImageBuffer::new(WIDTH, HEIGHT);

    image_buffer
        .enumerate_rows_mut()
        .par_bridge()
        .for_each(|(_, row)| {
            for (x, y, pixel) in row {
                let pixel_color = (0..SAMPLES)
                    .map(|_| {
                        let mut rng = rand::thread_rng();
                        let y_offset = rng.gen_range(-1.0, 1.0);
                        let x_offset = rng.gen_range(-1.0, 1.0);

                        // y_coord is from -1.0 to 1.0. 0 is actually the top of the screen, so
                        // reverse it here. Also offset each pixel by between -1.0 and 1.0 before
                        // scaling.
                        let y_coord =
                            -(((y as f64) - (HEIGHT as f64 / 2.0) + y_offset) * SCALE_FACTOR);
                        let x_coord = ((x as f64) - (WIDTH as f64 / 2.0) + x_offset) * SCALE_FACTOR;

                        let mut projected_ray =
                            Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(x_coord, y_coord, -1.0));
                        let mut result: Vec3<f64> = Vec3::new(255.0, 255.0, 255.0);

                        while let Some(closest_hit) = shapes
                            .iter()
                            .filter_map(|shape| shape.intersects(projected_ray))
                            .min_by(|hit_1, hit_2| {
                                hit_1
                                    .distance
                                    .partial_cmp(&hit_2.distance)
                                    .expect("Distances shouldn't be NaN")
                            })
                        {
                            result *= 0.7;
                            if result.size() < 10.0 {
                                break;
                            }

                            projected_ray = Ray::new(
                                closest_hit.location,
                                projected_ray.dir.reflect_using_normal(closest_hit.normal),
                            );
                        }

                        result
                    })
                    .fold(Vec3::new(0.0, 0.0, 0.0), Vec3::add)
                    / (SAMPLES as f64);

                *pixel = Rgb([
                    pixel_color.x as u8,
                    pixel_color.y as u8,
                    pixel_color.z as u8,
                ]);
            }
        });

    image_buffer.save("render.png")?;

    Ok(())
}
