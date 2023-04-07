use egui::Slider;
use nalgebra::{UnitVector3, Vector3};

use super::{Shape, ShapeCollisionInformation};
use crate::{Material, Ray};

#[derive(Clone, Copy, Debug)]
pub struct Plane<T: Material> {
    pub point: Vector3<f32>,
    pub normal: UnitVector3<f32>,
    pub material: T,
}

impl<T: Material> Shape for Plane<T> {
    fn intersect(&self, ray: Ray) -> Option<ShapeCollisionInformation> {
        let l_dot_n = ray.dir.dot(&self.normal);

        // Iff line and ray are parallel, no intersection occurs.
        if l_dot_n == 0.0 {
            return None;
        }

        let t = (self.point - ray.pos).dot(&self.normal) / l_dot_n;

        if t < 0.0 {
            return None;
        }

        let intersection_point =
            ray.pos + (ray.dir.into_inner() * t) + (self.normal.into_inner() * 0.00001);
        Some(ShapeCollisionInformation {
            distance: t,
            normal: self.normal,
            intersection_point: intersection_point,
        })
    }

    fn render(&mut self, ui: &mut egui::Ui) -> bool {
        let mut needs_render = false;
        ui.horizontal(|ui| {
            ui.label("x");
            needs_render |= ui
                .add(Slider::new(&mut self.point.x, (-10.0)..=10.0))
                .changed();
        });
        ui.horizontal(|ui| {
            ui.label("y");
            needs_render |= ui
                .add(Slider::new(&mut self.point.y, (-10.0)..=10.0).vertical())
                .changed();
        });
        ui.horizontal(|ui| {
            ui.label("z");
            needs_render |= ui
                .add(Slider::new(&mut self.point.z, (-10.0)..=0.0))
                .changed();
        });

        {
            let mut normal_raw = self.normal.into_inner();
            ui.horizontal(|ui| {
                ui.label("normal x");
                needs_render |= ui.add(Slider::new(&mut normal_raw.x, -1.0..=1.0)).changed();
            });
            ui.horizontal(|ui| {
                ui.label("normal y");
                needs_render |= ui.add(Slider::new(&mut normal_raw.y, -1.0..=1.0)).changed();
            });
            ui.horizontal(|ui| {
                ui.label("normal z");
                needs_render |= ui.add(Slider::new(&mut normal_raw.z, -1.0..=1.0)).changed();
            });
            self.normal = UnitVector3::new_normalize(normal_raw);
        }

        needs_render
    }

    fn material(&self) -> &dyn Material {
        &self.material
    }

    fn material_mut(&mut self) -> &mut dyn Material {
        &mut self.material
    }
}
