mod mesh;
mod sphere;

use egui::{Slider, Ui};

use crate::{Material, Ray};
use nalgebra::{UnitVector3, Vector3};

pub use mesh::Mesh;
pub use sphere::Sphere;

#[derive(Clone, Copy, Debug)]
pub struct ShapeCollisionInformation {
    pub distance: f32,
    pub normal: UnitVector3<f32>,
    pub intersection_point: Vector3<f32>,
}

pub trait Shape {
    fn intersect(&self, ray: Ray) -> Option<ShapeCollisionInformation>;
    fn render(&mut self, ui: &mut Ui) -> bool;

    fn material(&self) -> &dyn Material;
    fn material_mut(&mut self) -> &mut dyn Material;
}
