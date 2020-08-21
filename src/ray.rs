use crate::vec3::Vec3;

use num::Float;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Ray<T: Float> {
    pub origin: Vec3<T>,
    pub dir: Vec3<T>,
}

impl<T: Float> Ray<T> {
    pub fn new(origin: Vec3<T>, dir: Vec3<T>) -> Self {
        Self { origin, dir }
    }

    pub fn eval(&self, t: T) -> Vec3<T> {
        self.origin + (self.dir * t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let ray = Ray::new(Vec3::new(1.0, 2.0, 3.0), Vec3::new(4.0, 5.0, 6.0));

        assert_eq!(ray.origin, Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(ray.dir, Vec3::new(4.0, 5.0, 6.0));
    }

    #[test]
    fn test_eval() {
        let ray = Ray::new(Vec3::new(1.0, 2.0, 3.0), Vec3::new(4.0, 5.0, 6.0));

        let ray_evaled = ray.eval(0.5);

        assert_eq!(ray_evaled, Vec3::new(3.0, 4.5, 6.0));
    }
}
