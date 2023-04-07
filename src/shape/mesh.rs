use anyhow::{anyhow, Result};
use egui::{Slider, Ui};
use linked_hash_map::LinkedHashMap;
use nalgebra::{UnitVector3, Vector3};
use ply_rs::{
    parser::Parser,
    ply::{DefaultElement, Property},
};

use crate::{Material, Ray};

use super::{Shape, ShapeCollisionInformation};

#[derive(Clone, Copy, Debug)]
struct Triangle {
    a: Vector3<f32>,
    b: Vector3<f32>,
    c: Vector3<f32>,
}

#[derive(Clone, Copy, Debug)]
struct AABB {
    pub min: Vector3<f32>,
    pub max: Vector3<f32>,
}

impl AABB {
    fn intersects(&self, ray: Ray) -> bool {
        let mut tmin = f32::NEG_INFINITY;
        let mut tmax = f32::INFINITY;

        if ray.dir.x != 0.0 {
            let tx1 = (self.min.x - ray.pos.x) / ray.dir.x;
            let tx2 = (self.max.x - ray.pos.x) / ray.dir.x;

            tmin = f32::max(tmin, f32::min(tx1, tx2));
            tmax = f32::min(tmax, f32::max(tx1, tx2));
        }

        if ray.dir.y != 0.0 {
            let ty1 = (self.min.y - ray.pos.y) / ray.dir.y;
            let ty2 = (self.max.y - ray.pos.y) / ray.dir.y;

            tmin = f32::max(tmin, f32::min(ty1, ty2));
            tmax = f32::min(tmax, f32::max(ty1, ty2));
        }

        if ray.dir.z != 0.0 {
            let tz1 = (self.min.z - ray.pos.z) / ray.dir.z;
            let tz2 = (self.max.z - ray.pos.z) / ray.dir.z;

            tmin = f32::max(tmin, f32::min(tz1, tz2));
            tmax = f32::min(tmax, f32::max(tz1, tz2));
        }

        tmax >= tmin
    }
}

impl Triangle {
    fn intersect(&self, ray: Ray) -> Option<ShapeCollisionInformation> {
        let edge_1 = self.b - self.a;
        let edge_2 = self.c - self.a;

        let n = UnitVector3::new_normalize(edge_1.cross(&edge_2));

        let p_vec = ray.dir.cross(&edge_2);

        let det = edge_1.dot(&p_vec);

        let inv_det = 1.0 / det;

        let t_vec = ray.pos - self.a;

        let u = t_vec.dot(&p_vec) * inv_det;

        if u < 0.0 || u > 1.0 {
            return None;
        }

        let q_vec = t_vec.cross(&edge_1);

        let v = ray.dir.dot(&q_vec) * inv_det;

        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = edge_2.dot(&q_vec) * inv_det;

        Some(ShapeCollisionInformation {
            distance: t,
            normal: n,
            intersection_point: ray.pos + (ray.dir.into_inner() * t),
        })
    }
}

pub struct Mesh<T: Material> {
    pub material: T,
    pub center: Vector3<f32>,
    triangles: Vec<Triangle>,
    bounding_box: AABB,
}

impl<T: Material> Mesh<T> {
    pub fn from_ply(material: T, center: Vector3<f32>, ply_contents: &str) -> Result<Self> {
        fn try_vector3_from_point(point: &LinkedHashMap<String, Property>) -> Result<Vector3<f32>> {
            let Property::Float(x) = point["x"] else {
                return Err(anyhow!("expected x to be float, but got {:?}", point["x"]));
            };

            let Property::Float(y) = point["y"] else {
                return Err(anyhow!("expected y to be float, but got {:?}", point["y"]));
            };

            let Property::Float(z) = point["z"] else {
                return Err(anyhow!("expected z to be float, but got {:?}", point["z"]));
            };

            Ok(Vector3::new(x, y, z))
        }

        let parser: Parser<DefaultElement> = Parser::new();
        let parsed = parser.read_ply(&mut ply_contents.as_bytes())?;

        let triangles: Vec<Triangle> = parsed.payload["face"]
            .iter()
            .map(|face| match &face["vertex_indices"] {
                Property::ListInt(indices) if indices.len() == 3 => {
                    let a_idx = usize::try_from(indices[0])?;
                    let b_idx = usize::try_from(indices[1])?;
                    let c_idx = usize::try_from(indices[2])?;

                    let a = try_vector3_from_point(&parsed.payload["vertex"][a_idx])?;
                    let b = try_vector3_from_point(&parsed.payload["vertex"][b_idx])?;
                    let c = try_vector3_from_point(&parsed.payload["vertex"][c_idx])?;

                    Ok(Triangle { a, b, c })
                }
                Property::ListInt(indices) => Err(anyhow!(
                    "expected there to be 3 vertex indices, but got {}",
                    indices.len()
                )),
                other => Err(anyhow!(
                    "expected vertex indices to be int list, but instead got {:?}",
                    other
                )),
            })
            .collect::<Result<Vec<_>>>()?;

        if triangles.is_empty() {
            return Err(anyhow!(
                "Expected to get some triangles, but got no triangles"
            ));
        }

        let max_x = triangles
            .iter()
            .map(|t| t.a.x.max(t.b.x).max(t.c.x))
            .max_by(f32::total_cmp)
            .unwrap();
        let max_y = triangles
            .iter()
            .map(|t| t.a.y.max(t.b.y).max(t.c.y))
            .max_by(f32::total_cmp)
            .unwrap();
        let max_z = triangles
            .iter()
            .map(|t| t.a.z.max(t.b.z).max(t.c.z))
            .max_by(f32::total_cmp)
            .unwrap();

        let min_x = triangles
            .iter()
            .map(|t| t.a.x.min(t.b.x).min(t.c.x))
            .min_by(f32::total_cmp)
            .unwrap();
        let min_y = triangles
            .iter()
            .map(|t| t.a.y.min(t.b.y).min(t.c.y))
            .min_by(f32::total_cmp)
            .unwrap();
        let min_z = triangles
            .iter()
            .map(|t| t.a.z.min(t.b.z).min(t.c.z))
            .min_by(f32::total_cmp)
            .unwrap();

        let min_pos = Vector3::new(min_x, min_y, min_z);
        let max_pos = Vector3::new(max_x, max_y, max_z);
        let bounding_box = AABB {
            min: min_pos,
            max: max_pos,
        };

        Ok(Self {
            center,
            material,
            triangles,
            bounding_box,
        })
    }
}

impl<T: Material> Shape for Mesh<T> {
    fn intersect(&self, ray: Ray) -> Option<ShapeCollisionInformation> {
        let ray = Ray {
            dir: ray.dir,
            pos: ray.pos - self.center,
        };

        if !self.bounding_box.intersects(ray) {
            return None;
        }

        self.triangles
            .iter()
            .map(|t| t.intersect(ray))
            .find_map(|t| t)
    }

    fn material(&self) -> &dyn Material {
        &self.material
    }

    fn material_mut(&mut self) -> &mut dyn Material {
        &mut self.material
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

        needs_render
    }
}
