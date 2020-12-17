use crate::{HitRecord, Ray};

pub trait Shape {
    fn intersect(&self, ray: Ray, epsilon: f64) -> Option<HitRecord>;
}
