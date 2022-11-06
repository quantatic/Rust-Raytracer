use crate::{Buffer, Camera, Color, Ray, Scene};

use image::RgbImage;
use rand::{thread_rng, Rng};
use rayon::prelude::*;

const EPSILON: f64 = 1e-8;

pub struct Renderer {
    scene: Scene,
    camera: Camera,
    width: usize,
    height: usize,
    iterations: usize,
}

impl Renderer {
    pub fn new(
        scene: Scene,
        camera: Camera,
        width: usize,
        height: usize,
        iterations: usize,
    ) -> Self {
        Self {
            scene,
            camera,
            width,
            height,
            iterations,
        }
    }

    pub fn render(&self) -> RgbImage {
        let mut b = Buffer::new(self.width, self.height);
        b.enumerate_pixels()
            .par_bridge()
            .for_each(|(x, y, mut pixel)| {
                for _ in 0..self.iterations {
                    pixel.add_sample(self.sample_pixel(x, y))
                }
            });
        /*
        for y in 0..self.height {
            let row_samples = (0..self.width)
                .into_par_iter()
                .map(move |x| {
                    (0..self.iterations)
                        .map(move |_| self.sample_pixel(x, y))
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();

            for (x, pixel_samples) in row_samples.into_iter().enumerate() {
                for sample in pixel_samples {
                    b.add_sample(x as usize, y, sample);
                }
            }
        }*/

        b.into()
    }

    // x and y represent rendered (x, y) of pixel in final image
    fn sample_pixel(&self, x: usize, y: usize) -> Color {
        let dim = usize::max(self.width, self.height) as f64;

        let _color = Color::default();

        // map input (x, y) to [-1, 1]
        let xn = (((2 * x) as f64) - (self.width as f64)) / dim;
        let yn = ((self.height as f64) - ((2 * y) as f64)) / dim;

        let mut rng = thread_rng();

        let dx = rng.gen_range(0.0..(1.0 / dim));
        let dy = rng.gen_range(0.0..(1.0 / dim));
        self.trace_ray(self.camera.cast_ray(xn + dx, yn + dy))
    }

    fn trace_ray(&self, ray: Ray) -> Color {
        if let Some(hit_record) = self.scene.get_closest_hit(ray, EPSILON) {
            let mut hit_color = Color::new(
                hit_record.normal.x,
                hit_record.normal.y,
                hit_record.normal.z,
            );

            for illumination in self.scene.illuminations(ray.eval(hit_record.time)) {
                // if shadow ray has less travel time than illumination ray, we're in shadow
                let shadow = self
                    .scene
                    .get_closest_hit(illumination.to_light, EPSILON)
                    .map_or(false, |shadow_hit| shadow_hit.time < illumination.time);

                if shadow {
                    hit_color = Color::hex(0x000000);
                }
            }

            hit_color
        } else {
            Color::hex(0x000000)
        }
    }
}
