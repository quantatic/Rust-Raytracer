use crate::Vec3;

pub struct HitRecord {
    pub location: Vec3<f64>,
    pub normal: Vec3<f64>,
    pub distance: f64,
}
