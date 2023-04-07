use crate::{Ray, ShapeCollisionInformation};
use nalgebra::{UnitVector3, Vector3};

use super::{Material, MaterialCollisionInformation, MaterialReflectionInformation};

#[derive(Clone, Copy, Debug)]
pub struct MetalMaterial {
    pub albedo: Vector3<f32>,
}

impl Material for MetalMaterial {
    fn evaluate(
        &self,
        incoming: Ray,
        info: ShapeCollisionInformation,
    ) -> MaterialCollisionInformation {
        let d = incoming.dir.into_inner();
        let n = info.normal;

        let reflection = UnitVector3::new_normalize(d - (2.0 * d.dot(&n) * n.into_inner()));

        let reflection_info = MaterialReflectionInformation {
            albedo: self.albedo,
            reflection,
        };

        MaterialCollisionInformation {
            reflection_information: Some(reflection_info),
            emission_information: None,
        }
    }

    fn color(&self) -> Vector3<f32> {
        self.albedo
    }

    fn color_mut(&mut self) -> &mut Vector3<f32> {
        &mut self.albedo
    }
}
