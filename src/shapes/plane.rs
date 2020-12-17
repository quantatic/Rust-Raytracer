use super::Shape;

use crate::{HitRecord, Ray};

use nalgebra::{Point3, Unit, Vector3};

pub struct Plane {
    normal: Unit<Vector3<f64>>,
}

impl Plane {
    pub fn new(normal: Unit<Vector3<f64>>) -> Self {
        Self { normal }
    }
}

impl Shape for Plane {
    fn intersect(&self, ray: Ray, epsilon: f64) -> Option<HitRecord> {
        let cos = self.normal.dot(&ray.direction);

        if cos == 0.0 {
            return None;
        };

        let time = -(ray.origin - Point3::origin()).dot(&self.normal) / cos;

        if time > epsilon {
            Some(HitRecord {
                time,
                normal: self.normal,
            })
        } else {
            None
        }
    }
}
