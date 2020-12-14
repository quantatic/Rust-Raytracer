use crate::{Buffer, Camera, Color, Ray, Scene};

use image::RgbImage;
use rand::{thread_rng, Rng};

pub struct Renderer {
    scene: Scene,
    camera: Camera,
    width: u32,
    height: u32,
    iterations: u32,
}

impl Renderer {
    pub fn new(scene: Scene, camera: Camera, width: u32, height: u32, iterations: u32) -> Self {
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
        for y in 0..self.height {
            for x in 0..self.width {
                for _ in 0..self.iterations {
                    b.add_sample(x, y, self.sample_pixel(x, y));
                }
            }
        }

        b.into()
    }

    // x and y represent rendered (x, y) of pixel in final image
    fn sample_pixel(&self, x: u32, y: u32) -> Color {
        let dim = u32::max(self.width, self.height) as f64;

        let _color = Color::default();

        // map input (x, y) to [-1, 1]
        let xn = (((2 * x) as f64) - (self.width as f64)) / dim;
        let yn = (((2 * y) as f64) - (self.height as f64)) / dim;

        let mut rng = thread_rng();

        let dx = rng.gen_range(-1.0 / dim, 1.0 / dim);
        let dy = rng.gen_range(-1.0 / dim, 1.0 / dim);
        self.trace_ray(self.camera.cast_ray(xn + dx, yn + dy))
    }

    fn trace_ray(&self, ray: Ray) -> Color {
        if self.scene.get_closest_hit(ray).is_some() {
            Color::hex(0xFFFFFF)
        } else {
            Color::hex(0x000000)
        }
    }
}
