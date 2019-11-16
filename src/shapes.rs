use crate::color::Color;
use crate::vector::Vec3;
use crate::ray::Ray;
use crate::hit::Hit;

pub trait Hitable {
    fn hit(&self, ray: &Ray) -> Option<Hit>;
    fn color(&self) -> Color;
}

pub struct Sphere {
    pub pos: Vec3,
    pub radius: f64,
    pub color: Color,
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray) -> Option<Hit> {
        let eqn_c = self.pos;
        let eqn_r = self.radius;
        let eqn_l = ray.dir;
        let eqn_o = ray.pos;

        let o_minus_c = eqn_o - eqn_c;

        let a = eqn_l.dot(eqn_l);
        let b = 2.0 * eqn_l.dot(o_minus_c);
        let c = o_minus_c.dot(o_minus_c) - eqn_r;

        let disc = b.powi(2) - (4.0 * a * c);

        if disc < 0.0 {
            return None;
        }

        let solution_one = (-b + disc.sqrt()) / (2.0 * a);
        let solution_two = (-b - disc.sqrt()) / (2.0 * a);

        let dist = if solution_one > 0.0 && solution_one < solution_two {
            solution_one
        } else {
            solution_two
        };

        let normal = (ray.eval(dist) - self.pos).unit();
        Some(
            Hit {
                hit: self,
                dist,
                normal,
            }
        )
    }

    fn color(&self) -> Color {
        self.color
    }
}
