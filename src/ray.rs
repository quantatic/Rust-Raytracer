use crate::vector::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub pos: Vec3,
    pub dir: Vec3
}

impl Ray {
    pub fn eval(&self, t: f64) -> Vec3 {
        self.pos + (self.dir * t)
    }
}
