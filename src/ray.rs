use nalgebra::{Point3, Transform3, Unit, Vector3};

use simba::scalar::SupersetOf;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ray {
    pub origin: Point3<f64>,
    pub direction: Unit<Vector3<f64>>,
}

impl Ray {
    pub fn eval(&self, t: f64) -> Point3<f64> {
        self.origin + (*self.direction * t)
    }

    pub fn transform<T>(self, transform: T) -> Self
    where
        Transform3<f64>: SupersetOf<T>,
    {
        Self {
            origin: Transform3::from_subset(&transform).transform_point(&self.origin),
            direction: Unit::new_normalize(
                Transform3::from_subset(&transform).transform_vector(&self.direction),
            ),
        }
    }
}
