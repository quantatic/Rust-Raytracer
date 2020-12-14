use crate::{HitRecord, Ray};

use super::Shape;

use nalgebra::Point3;

#[derive(Default)]
pub struct Sphere;

impl Shape for Sphere {
    fn intersect(&self, ray: Ray) -> Option<HitRecord> {
        let o_minus_c = ray.origin - Point3::origin(); // center is implicitly at 0, 0, 0

        let a = ray.direction.norm().powi(2);
        let b = 2.0 * ray.direction.dot(&o_minus_c);
        let c = o_minus_c.norm().powi(2) - 1.0; // radius is implicitly 1.0

        let disc = b.powi(2) - (4.0 * a * c);

        if disc < 0.0 {
            return None;
        }

        let root_disc = disc.sqrt();

        let q = if b < 0.0 {
            -0.5 * (b - root_disc)
        } else {
            -0.5 * (b + root_disc)
        };

        let sol1 = q / a;
        let sol2 = c / q;

        let solution = if sol1 > 0.0 && sol1 < sol2 {
            sol1
        } else if sol2 > 0.0 {
            sol2
        } else {
            return None;
        };

        // sphere has radius of 1, so normal is normalized already
        let normal = ray.eval(solution) - Point3::origin();

        Some(HitRecord {
            time: solution,
            normal,
        })
    }
}
