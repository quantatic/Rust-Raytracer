use crate::{HitRecord, Ray};

pub trait Shape {
    fn intersect(&self, ray: Ray) -> Option<HitRecord>;
}
