use raytrace::Ray;
use raytrace::Vec3;
use raytrace::{Shape, Sphere};

use std::error::Error;

use image::{ImageBuffer, Rgb};

use rayon::prelude::*;

const WIDTH: u32 = 100_000;
const ASPECT_RATIO: f64 = 1.0; // ratio of WIDTH / HEIGHT

const HEIGHT: u32 = ((WIDTH as f64) * ASPECT_RATIO) as u32;

const SCALE_FACTOR: f64 = 2.0 / (WIDTH as f64);

fn main() -> Result<(), Box<dyn Error>> {
    let mut shapes: Vec<Box<dyn Shape + Sync>> = Vec::new();

    shapes.push(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -6.0), 2.0)));
    shapes.push(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -6.0), 2.0)));
    shapes.push(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -6.0), 2.0)));
    shapes.push(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -6.0), 2.0)));

    let mut image_buffer: ImageBuffer<Rgb<u8>, _> = ImageBuffer::new(WIDTH, HEIGHT);

    image_buffer
        .enumerate_rows_mut()
        .par_bridge()
        .for_each(|(_, row)| {
            for (x, y, pixel) in row {
                let y_coord = ((y as f64) - (HEIGHT as f64 / 2.0)) * SCALE_FACTOR;
                let x_coord = ((x as f64) - (WIDTH as f64 / 2.0)) * SCALE_FACTOR;

                let projected_ray =
                    Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(x_coord, y_coord, -1.0));

                if let Some(closest_hit) = shapes
                    .iter()
                    .filter_map(|shape| shape.intersects(projected_ray))
                    .max_by(|hit_1, hit_2| {
                        let dist_to_hit_1 = (projected_ray.origin - hit_1.hit_loc).size();
                        let dist_to_hit_2 = (projected_ray.origin - hit_2.hit_loc).size();
                        dist_to_hit_1
                            .partial_cmp(&dist_to_hit_2)
                            .expect("Distances shouldn't be NaN")
                    })
                {
                    let normal_color_x = ((closest_hit.hit_normal.x / 2.0 + 0.5) * 255.0) as u8;
                    let normal_color_y = ((closest_hit.hit_normal.y / 2.0 + 0.5) * 255.0) as u8;
                    let normal_color_z = ((closest_hit.hit_normal.z / 2.0 + 0.5) * 255.0) as u8;
                    *pixel = Rgb([normal_color_x, normal_color_y, normal_color_z]);
                } else {
                    *pixel = Rgb([0; 3])
                }
            }
        });

    image_buffer.save("render.png")?;

    Ok(())
}
