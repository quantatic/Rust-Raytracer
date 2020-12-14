use crate::{HitRecord, Ray};

use super::Shape;

use nalgebra::{Point3, Vector3};

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

        let q = if b < 0.0 {
            -0.5 * (b - (b.powi(2) - (4.0 * a * c)))
        } else {
            -0.5 * (b + (b.powi(2) - (4.0 * a * c)))
        };

        let sol1 = q / a;
        let sol2 = c / q;

        if sol1 > 0.0 && sol1 < sol2 {
            Some(HitRecord {
                position: ray.eval(sol1),
                time: sol1,
                normal: Vector3::new(0.0, 0.0, 0.0),
            })
        } else if sol2 > 0.0 {
            Some(HitRecord {
                position: ray.eval(sol2),
                time: sol2,
                normal: Vector3::new(0.0, 0.0, 0.0),
            })
        } else {
            None
        }
    }
}
