use nalgebra::{UnitVector3, Vector3};
use rand::{distributions::Distribution, Rng};
use rand_distr::StandardNormal;

use crate::{Ray, ShapeCollisionInformation};

use super::{Material, MaterialCollisionInformation, MaterialReflectionInformation};

#[derive(Clone, Copy, Debug)]
pub struct DiffuseMaterial {
    pub albedo: Vector3<f32>,
}

impl DiffuseMaterial {
    fn random_unit_sphere<R: Rng + ?Sized>(rng: &mut R) -> UnitVector3<f32> {
        let [x, y, z] = std::array::from_fn(|_| StandardNormal.sample(rng));
        UnitVector3::new_normalize(Vector3::new(x, y, z))
    }
}

impl Material for DiffuseMaterial {
    fn evaluate(
        &self,
        _incoming: Ray,
        info: ShapeCollisionInformation,
    ) -> MaterialCollisionInformation {
        let mut rng = rand::thread_rng();
        let random_dir = Self::random_unit_sphere(&mut rng);
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
