use crate::color::Color;
use crate::hit::Hit;
use crate::ray::Ray;
use crate::shapes::Hitable;


pub fn cast_ray(objects: &[Box<dyn Hitable>], ray: Ray, depth_left: u32) -> Option<Color> {
	if depth_left <= 0 {
		return None;
	}

	let mut closest_hit: Option<Hit> = None;
	for obj in objects.iter() {
		if let Some(new_hit_record) = obj.hit(ray) {
			if let Some(old_hit_record) = &closest_hit {
				if old_hit_record.dist > new_hit_record.dist {
					closest_hit = Some(new_hit_record);
				}
			} else {
				closest_hit = Some(new_hit_record);
			}
		}
	}

	match closest_hit {
		Some(hit_record) => {
			let bounced_ray = Ray {
				pos: hit_record.hit_point,
				dir: ray.dir - (hit_record.normal * 2.0 * ray.dir.dot(hit_record.normal)),
			};

			match cast_ray(objects, bounced_ray, depth_left - 1) {
				Some(recurse_color) => {
					Some(Color::blend(hit_record.hit.color(), 0.8, recurse_color, 0.2))
				},
				None => {
					Some(hit_record.hit.color())
				},
			}
		},
		None => None
	}
}
