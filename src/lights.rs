use crate::{Color, Ray};

use nalgebra::Point3;

pub struct IlluminationRecord {
    pub color: Color,
    pub to_light: Ray,
}

pub trait Light {
    fn illuminate(&self, point: Point3<f64>) -> IlluminationRecord;
}

pub struct PointLight {
    location: Point3<f64>,
    color: Color,
}

impl PointLight {
    pub fn new(location: Point3<f64>, color: Color) -> Self {
        Self { location, color }
    }
}

impl Light for PointLight {
    fn illuminate(&self, point: Point3<f64>) -> IlluminationRecord {
        let to_light = Ray {
            origin: point,
            direction: point - self.location,
        };

        IlluminationRecord {
            color: self.color,
            to_light,
        }
    }
}
