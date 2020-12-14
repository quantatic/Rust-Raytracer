use nalgebra::{Point3, Transform3, Vector3};

use std::ops::Mul;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ray {
    pub origin: Point3<f64>,
    pub direction: Vector3<f64>,
}

impl Ray {
    pub fn eval(&self, t: f64) -> Point3<f64> {
        self.origin + (self.direction * t)
    }
}

impl Mul<Ray> for Transform3<f64> {
    type Output = Ray;

    fn mul(self, other: Ray) -> Self::Output {
        Ray {
            origin: self * other.origin,
            direction: self * other.direction,
        }
    }
}
