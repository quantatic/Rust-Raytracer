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
        let mut closest_record: Option<HitRecord> = None;
        for object in &self.objects {
            if let Some(record) = object.intersect(ray) {
                closest_record = closest_record
                    .map_or(record, |cur_closest| {
                        if record.time < cur_closest.time {
                            record
                        } else {
                            cur_closest
                        }
                    })
                    .into();
            }
        }

        closest_record
    }
}
