use crate::Ray;

use nalgebra::{Isometry3, Perspective3, Point3, Projective3, Unit, Vector3};

const Z_NEAR: f64 = 100.0;
const Z_FAR: f64 = 1e5;

#[derive(Copy, Clone, Debug)]
pub struct Camera {
    ndc_to_camera: Projective3<f64>,
    camera_to_world: Isometry3<f64>,
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.0, 0.0, -1.0),
            Vector3::y_axis(),
            std::f64::consts::FRAC_PI_2,
            1.0,
        )
    }
}

impl Camera {
    pub fn new(
        camera_location: Point3<f64>,
        look_location: Point3<f64>,
        up: Unit<Vector3<f64>>,
        fov_y: f64,
        aspect_ratio: f64,
    ) -> Self {
        let ndc_to_camera = Perspective3::new(aspect_ratio, fov_y, Z_NEAR, Z_FAR)
            .to_projective()
            .inverse();

        let camera_to_world =
            Isometry3::look_at_rh(&camera_location, &look_location, &up).inverse();

        Self {
            ndc_to_camera,
            camera_to_world,
        }
    }

    // sample rays from this camera
    // input x and y are in NDC space
    pub fn cast_ray(&self, x: f64, y: f64) -> Ray {
        let ndc_film_location = Point3::new(x, y, -1.0);

        let camera_film_location = self.ndc_to_camera.transform_point(&ndc_film_location);

        let camera_ray = Ray {
            origin: Point3::origin(),
            direction: Unit::new_normalize(camera_film_location - Point3::origin()),
        };

        camera_ray.transform(self.camera_to_world)
    }
}
