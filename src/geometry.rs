pub struct Ray {
    pub pos: Vec3,
    pub dir: Vec3
}

pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vec3 {
    #[allow(dead_code)]
    pub fn add(&self, other: &Self) -> Self {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    pub fn sub(&self, other: &Self) -> Self {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    pub fn mult(&self, val: f64) -> Self {
        Vec3 {
            x: self.x * val,
            y: self.y * val,
            z: self.z * val
        }
    }

    pub fn div(&self, val: f64) -> Self {
        Vec3 {
            x: self.x / val,
            y: self.y / val,
            z: self.z / val
        }
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }

    pub fn len(&self) -> f64 {
        ((self.x * self.x) + (self.y * self.y) + (self.z * self.z)).sqrt()
    }

    pub fn unit(&self) -> Self {
        self.div(self.len())
    }
}
