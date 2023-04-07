use nalgebra::Vector3;

use crate::{Ray, ShapeCollisionInformation};

use super::{Material, MaterialCollisionInformation, MaterialEmissionInformation};

#[derive(Clone, Copy, Debug)]
pub struct LightMaterial {
    pub color: Vector3<f32>,
}

impl Material for LightMaterial {
    fn evaluate(
        &self,
        _incoming: Ray,
        info: ShapeCollisionInformation,
    ) -> MaterialCollisionInformation {
        let emission_information = MaterialEmissionInformation { color: self.color };

        MaterialCollisionInformation {
            reflection_information: None,
            emission_information: Some(emission_information),
        }
    }

    fn color(&self) -> Vector3<f32> {
        self.color
    }

    fn color_mut(&mut self) -> &mut Vector3<f32> {
        &mut self.color
    }
}
