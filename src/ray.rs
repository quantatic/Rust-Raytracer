use nalgebra::{UnitVector3, Vector3};

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub pos: Vector3<f32>,
    pub dir: UnitVector3<f32>,
}
