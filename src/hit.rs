use crate::shapes::Hitable;
use crate::vector::Vec3;

pub struct Hit<'a> {
    pub hit: &'a dyn Hitable,
    pub dist: f64,
    pub normal: Vec3,
    pub hit_point: Vec3,
}
