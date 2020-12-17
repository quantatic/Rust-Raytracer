use nalgebra::{Unit, Vector3};

#[derive(Copy, Clone, Debug)]
pub struct HitRecord {
    pub time: f64,
    pub normal: Unit<Vector3<f64>>,
}
