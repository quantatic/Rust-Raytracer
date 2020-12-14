use nalgebra::{Point3, Vector3};

pub struct HitRecord {
    pub position: Point3<f64>,
    pub time: f64,
    pub normal: Vector3<f64>,
}
