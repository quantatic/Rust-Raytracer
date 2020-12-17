use crate::{HitRecord, Ray};

use super::Shape;

use nalgebra::{Point3, Unit};

#[derive(Default)]
pub struct Sphere;

impl Shape for Sphere {
    fn intersect(&self, ray: Ray, epsilon: f64) -> Option<HitRecord> {
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

        let sol_1 = q / a;
        let sol_2 = c / q;

        let sol_min = f64::min(sol_1, sol_2);
        let sol_max = f64::max(sol_1, sol_2);

        let solution = if sol_min > epsilon {
            sol_min
        } else if sol_max > epsilon {
            sol_max
        } else {
            return None;
        };

        let normal = Unit::new_normalize(ray.eval(solution) - Point3::origin());

        Some(HitRecord {
            time: solution,
            normal,
        })
    }
}
