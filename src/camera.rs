use crate::Ray;

use nalgebra::{Point3, Vector3};

#[derive(Copy, Clone, Debug)]
pub struct Camera {
    // location of camera
    location: Point3<f64>,

    // look direction of camera
    direction: Vector3<f64>,

    // up vector for camera
    up: Vector3<f64>,

    // fov in radians
    fov: f64,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            location: Point3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(0.0, 0.0, -1.0),
            up: Vector3::new(0.0, 1.0, 0.0),
            fov: std::f64::consts::FRAC_PI_2,
        }
    }
}

impl Camera {
    pub fn look_at(
        location: Point3<f64>,
        direction: Vector3<f64>,
        up: Vector3<f64>,
        fov: f64,
    ) -> Self {
        Self {
            location,
            direction,
            up,
            fov,
        }
    }

    // sample rays from this camera
    // both x and y are from the range [-1, 1]
    pub fn cast_ray(&self, x: f64, y: f64) -> Ray {
        let right = self.direction.cross(&self.up).normalize();

        // look direction, adjusted to viewing plane by FOV
        let fov_dir = self.direction * (self.fov / 2.0).tan().recip();

        Ray {
            origin: self.location,
            direction: fov_dir + (right * x) + (self.up * y),
        }
    }
}
