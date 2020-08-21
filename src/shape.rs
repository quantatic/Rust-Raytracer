use crate::HitRecord;
use crate::Ray;
use crate::Vec3;

pub trait Shape {
    fn intersects(&self, ray: Ray<f64>) -> Option<HitRecord>;
}

#[derive(Debug)]
pub struct Sphere {
    center: Vec3<f64>,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3<f64>, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Shape for Sphere {
    fn intersects(&self, ray: Ray<f64>) -> Option<HitRecord> {
        let o_sub_c = ray.origin - self.center;
        let disc =
            ray.dir.unit().dot(o_sub_c).powi(2) - (o_sub_c.size().powi(2) - self.radius.powi(2));

        if disc >= 0.0 {
            let hit_dist = -ray.dir.unit().dot(o_sub_c) - disc.sqrt();
            let hit_loc = ray.eval(hit_dist / ray.dir.size());
            let hit_normal = (hit_loc - self.center).unit();
            let hit_record = HitRecord {
                location: hit_loc,
                normal: hit_normal,
                distance: hit_dist,
            };
            Some(hit_record)
        } else {
            None
        }
    }
}
