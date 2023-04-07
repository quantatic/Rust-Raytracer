use egui::{Slider, Ui};
use nalgebra::{UnitVector3, Vector3};

use super::Shape;
use crate::{Material, Ray, ShapeCollisionInformation};

#[derive(Clone, Copy, Debug)]
pub struct Sphere<T: Material> {
    pub center: Vector3<f32>,
    pub radius: f32,
    pub material: T,
}

impl<T: Material> Shape for Sphere<T> {
    fn intersect(&self, ray: Ray) -> Option<ShapeCollisionInformation> {
        let u = ray.dir;
        let o = ray.pos;
        let c = self.center;
        let r = self.radius;

        let o_minus_c = o - c;
        let u_dot_o_minus_c = u.into_inner().dot(&o_minus_c);

        let discriminant = u_dot_o_minus_c.powi(2) - o_minus_c.norm().powi(2) + r.powi(2);

        if discriminant > 0.0 {
            let discriminant_sqrt = discriminant.sqrt();

            let large_solution = (-u_dot_o_minus_c) + discriminant_sqrt;
            let small_solution = (-u_dot_o_minus_c) - discriminant_sqrt;

            // If the largest solution is still negative, smallest must also be negative (intersection is behind us).
            if large_solution < 0.0 {
                return None;
            }

            let distance = if small_solution > 0.0 {
                small_solution
            } else {
                large_solution
            };

            let intersection_point = ray.pos + (ray.dir.into_inner() * distance);
            let normal = UnitVector3::new_normalize(intersection_point - self.center);
            let intersection_point = intersection_point + (normal.into_inner() * 0.000001);

            Some(ShapeCollisionInformation {
                distance,
                normal,
                intersection_point,
            })
        } else {
            None
        }
    }

    fn render(&mut self, ui: &mut Ui) -> bool {
        let mut needs_render = false;
        ui.horizontal(|ui| {
            ui.label("x");
            needs_render |= ui
                .add(Slider::new(&mut self.center.x, (-10.0)..=10.0))
                .changed();
        });
        ui.horizontal(|ui| {
            ui.label("y");
            needs_render |= ui
                .add(Slider::new(&mut self.center.y, (-10.0)..=10.0).vertical())
                .changed();
        });
        ui.horizontal(|ui| {
            ui.label("z");
            needs_render |= ui
                .add(Slider::new(&mut self.center.z, (-10.0)..=0.0))
                .changed();
        });
        ui.horizontal(|ui| {
            ui.label("radius");
            needs_render |= ui.add(Slider::new(&mut self.radius, 0.0..=10.0)).changed();
        });

        needs_render
    }

    fn material(&self) -> &dyn Material {
        &self.material
    }

    fn material_mut(&mut self) -> &mut dyn Material {
        &mut self.material
    }
}
