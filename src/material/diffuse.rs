use nalgebra::{UnitVector3, Vector3};

use crate::{Ray, ShapeCollisionInformation};

use super::{Material, MaterialCollisionInformation, MaterialReflectionInformation};

#[derive(Clone, Copy, Debug)]
pub struct DiffuseMaterial {
    pub albedo: Vector3<f32>,
}

impl Material for DiffuseMaterial {
    fn evaluate(
        &self,
        _incoming: Ray,
        info: ShapeCollisionInformation,
    ) -> MaterialCollisionInformation {
        let mut rng = rand::thread_rng();
        let random_dir = crate::vec3::random_unit_sphere(&mut rng);
        let reflection_raw = if random_dir.dot(&info.normal) >= 0.0 {
            random_dir.into_inner()
        } else {
            random_dir.into_inner() * -1.0
        };

        let reflection = UnitVector3::new_normalize(reflection_raw);

        let reflection_information = MaterialReflectionInformation {
            albedo: self.albedo,
            reflection,
        };

        MaterialCollisionInformation {
            reflection_information: Some(reflection_information),
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
