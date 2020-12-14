use nalgebra::Vector3;

#[derive(Copy, Clone, Debug)]
pub struct HitRecord {
    pub time: f64,
    pub normal: Vector3<f64>,
}
