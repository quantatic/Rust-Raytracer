use crate::{HitRecord, Material, Ray, Shape};

use nalgebra::{Projective3, Rotation, Translation3, Unit, Vector3};

pub struct Object {
    shape: Box<dyn Shape>,
    material: Material,
    transform: Projective3<f64>,
}

impl Object {
    pub fn new<T: Shape + 'static>(shape: T, material: Material) -> Self {
        Self {
            shape: Box::new(shape),
            material,
            transform: Projective3::identity(),
        }
    }

    pub fn intersect(&self, ray: Ray, epsilon: f64) -> Option<HitRecord> {
        // ray has to be scaled by inverse of transform, to convert from world to local space
        self.shape
            .intersect(ray.transform(self.transform.inverse()), epsilon)
    }

    pub fn translate(self, x: f64, y: f64, z: f64) -> Self {
        Self {
            transform: Translation3::new(x, y, z) * self.transform,
            ..self
        }
    }

    pub fn rotate(self, axis: Unit<Vector3<f64>>, angle: f64) -> Self {
        Self {
            transform: Rotation::from_axis_angle(&axis, angle) * self.transform,
            ..self
        }
    }
}
