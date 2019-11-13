use crate::geometry::{Ray, Vec3};

pub trait Hitable {
    fn hit(&self, ray: &Ray) -> bool;
}

pub struct Sphere {
    pub pos: Vec3,
    pub radius: f64
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray) -> bool {
        let eqn_c = &self.pos;
        let eqn_r = &self.radius;
        let eqn_l = &ray.dir;
        let eqn_o = &ray.pos;

        let o_minus_c = eqn_o.sub(eqn_c);

        let a = eqn_l.dot(eqn_l);
        let b = 2.0 * eqn_l.dot(&o_minus_c);
        let c = o_minus_c.dot(&o_minus_c) - eqn_r;

        (b.powi(2) - 4.0 * a * c) >= 0.0
    }
}
