use crate::{HitRecord, Light, Object, Ray};

#[derive(Default)]
pub struct Scene {
    lights: Vec<Box<dyn Light>>,
    objects: Vec<Object>,
}

impl Scene {
    pub fn add_object(&mut self, object: Object) {
        self.objects.push(object)
    }

    pub fn get_closest_hit(&self, ray: Ray) -> Option<HitRecord> {
        for object in &self.objects {
            if let Some(record) = object.intersect(ray) {
                return Some(record);
            }
        }

        None
    }
}
