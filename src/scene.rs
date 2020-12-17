use crate::{HitRecord, IlluminationRecord, Light, Object, Ray};

use nalgebra::Point3;

#[derive(Default)]
pub struct Scene {
    lights: Vec<Box<dyn Light>>,
    objects: Vec<Object>,
}

impl Scene {
    pub fn add_object(&mut self, object: Object) {
        self.objects.push(object)
    }

    pub fn add_light<T: Light + 'static>(&mut self, light: T) {
        self.lights.push(Box::new(light))
    }

    pub fn get_closest_hit(&self, ray: Ray, epsilon: f64) -> Option<HitRecord> {
        let mut closest_record: Option<HitRecord> = None;
        for object in &self.objects {
            if let Some(record) = object.intersect(ray, epsilon) {
                closest_record = Some(closest_record.map_or(record, |cur_closest| {
                    if record.time < cur_closest.time {
                        record
                    } else {
                        cur_closest
                    }
                }));
            }
        }

        closest_record
    }

    pub fn illuminations<'a>(
        &self,
        point: Point3<f64>,
    ) -> impl Iterator<Item = IlluminationRecord> + '_ {
        self.lights.iter().map(move |light| light.illuminate(point))
    }
}

/*
pub struct Illuminations<'a> {
    lights: std::slice::Iter<'a, Box<dyn Light>>,
}

impl Iterator for Illuminations<'_> {
    type Item = IlluminationRecord;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        self.lights.next().map(
    }
}
*/
