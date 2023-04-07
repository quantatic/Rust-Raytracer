mod diffuse;
mod light;
mod metal;

use crate::{Ray, ShapeCollisionInformation};
use nalgebra::{UnitVector3, Vector3};

pub use diffuse::DiffuseMaterial;
pub use light::LightMaterial;
pub use metal::MetalMaterial;

#[derive(Clone, Copy, Debug)]
pub struct MaterialReflectionInformation {
    pub reflection: UnitVector3<f32>,
    pub albedo: Vector3<f32>, // 0.0 to 1.0 RGB space
}

#[derive(Clone, Copy, Debug)]
pub struct MaterialEmissionInformation {
    pub color: Vector3<f32>, // 0.0 to 1.0 in RGB space for unclipped emission
}

#[derive(Clone, Copy, Debug)]
pub struct MaterialCollisionInformation {
    pub reflection_information: Option<MaterialReflectionInformation>,
    pub emission_information: Option<MaterialEmissionInformation>,
}

pub trait Material {
    fn evaluate(
        &self,
        incoming: Ray,
        collision: ShapeCollisionInformation,
    ) -> MaterialCollisionInformation;

    fn color(&self) -> Vector3<f32>;
    fn color_mut(&mut self) -> &mut Vector3<f32>;
}
