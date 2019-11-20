use std::ops::{Add, Sub, Mul, Div, Neg};

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 {
            x,
            y,
            z
        }
    }

    pub fn dot(self, other: Vec3) -> f64 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }

    pub fn len(self) -> f64 {
        ((self.x * self.x) + (self.y * self.y) + (self.z * self.z)).sqrt()
    }

    pub fn unit(self) -> Self {
        self.div(self.len())
    }

    pub fn bounce_with_normal(self, other: Vec3) -> Vec3 {
        self - (other * 2.0 * self.dot(other))
    }
}

#[test]
fn test_new() {
    let a: Vec3 = Vec3::new(1.0, 2.0, 3.0);
    approx::assert_relative_eq!(a.x, 1.0);
    approx::assert_relative_eq!(a.y, 2.0);
    approx::assert_relative_eq!(a.z, 3.0);
}

#[test]
fn test_dot() {
    let a: Vec3 = Vec3::new(0.5, 2.0, 3.0);
    let b: Vec3 = Vec3::new(-2.0, 5.0, 1.0);
    approx::assert_relative_eq!(a.dot(b), 12.0);
}

#[test]
fn test_len() {
    let a: Vec3 = Vec3::new(1.0, 2.0, -3.0);
    approx::assert_relative_eq!(a.len(), (14.0 as f64).sqrt());
}

#[test]
fn test_unit() {
    let a: Vec3 = Vec3::new(5.0, 12.0, -8.0);
    approx::assert_relative_ne!(a.len(), 1.0);

    let a_unit: Vec3 = a.unit();
    approx::assert_relative_eq!(a_unit.len(), 1.0);
    approx::assert_relative_eq!(a_unit.x, a.x / (233 as f64).sqrt());
    approx::assert_relative_eq!(a_unit.y, a.y / (233 as f64).sqrt());
    approx::assert_relative_eq!(a_unit.z, a.z / (233 as f64).sqrt());
}

#[test]
fn test_bounce_with_normal() {
    let a: Vec3 = Vec3::new(3.0, 1.0, -5.4);

    // normal facing in the -x should only affect x component.
    let normal: Vec3 = Vec3::new(-1.0, 0.0, 0.0).unit();
    approx::assert_relative_eq!(normal.len(), 1.0);

    let bounced: Vec3 = a.bounce_with_normal(normal);
    approx::assert_relative_eq!(a.len(), bounced.len());

    approx::assert_relative_eq!(bounced.x, -3.0);
    approx::assert_relative_eq!(bounced.y, 1.0);
    approx::assert_relative_eq!(bounced.z, -5.4);

    // normal facing in the x should also only affect x component.
    let normal: Vec3 = Vec3::new(1.0, 0.0, 0.0).unit();
    approx::assert_relative_eq!(normal.len(), 1.0);

    let bounced: Vec3 = a.bounce_with_normal(normal);
    approx::assert_relative_eq!(a.len(), bounced.len());

    approx::assert_relative_eq!(bounced.x, -3.0);
    approx::assert_relative_eq!(bounced.y, 1.0);
    approx::assert_relative_eq!(bounced.z, -5.4);

    let a: Vec3 = Vec3::new(0.0, 1.0, -5.4);

    // bounce should properly handle a parallel vector
    let normal: Vec3 = Vec3::new(1.0, 0.0, 0.0).unit();
    approx::assert_relative_eq!(normal.len(), 1.0);

    let bounced: Vec3 = a.bounce_with_normal(normal);
    approx::assert_relative_eq!(a.len(), bounced.len());

    approx::assert_relative_eq!(bounced.x, 0.0);
    approx::assert_relative_eq!(bounced.y, 1.0);
    approx::assert_relative_eq!(bounced.z, -5.4);

    let a: Vec3 = Vec3::new(0.0, 1.0, -5.4);

    // bounce should properly handle a parallel vector
    let normal: Vec3 = Vec3::new(1.0, 0.0, 0.0).unit();
    approx::assert_relative_eq!(normal.len(), 1.0);

    let bounced: Vec3 = a.bounce_with_normal(normal);
    approx::assert_relative_eq!(a.len(), bounced.len());

    approx::assert_relative_eq!(bounced.x, 0.0);
    approx::assert_relative_eq!(bounced.y, 1.0);
    approx::assert_relative_eq!(bounced.z, -5.4);

    // bounce should properly handle bouncing off of a 45 degree angle

    let a: Vec3 = Vec3::new(1.0, 0.0, -5.4);

    let normal: Vec3 = Vec3::new(-1.0, 1.0, 0.0).unit();
    approx::assert_relative_eq!(normal.len(), 1.0);

    let bounced: Vec3 = a.bounce_with_normal(normal);
    approx::assert_relative_eq!(a.len(), bounced.len());

    approx::assert_relative_eq!(bounced.x, 0.0);
    approx::assert_relative_eq!(bounced.y, 1.0);
    approx::assert_relative_eq!(bounced.z, -5.4);
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

#[test]
fn test_add() {
    let a: Vec3 = Vec3::new(5.0, 6.0, -1.0);
    let b: Vec3 = Vec3::new(-13.0, 6.0, 2.5);

    let added = a + b;

    approx::assert_relative_eq!(added.x, 5.0 + -13.0);
    approx::assert_relative_eq!(added.y, 6.0 + 6.0);
    approx::assert_relative_eq!(added.z, -1.0 + 2.5);
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

#[test]
fn test_sub() {
    let a: Vec3 = Vec3::new(5.0, 6.0, -1.0);
    let b: Vec3 = Vec3::new(-13.0, 6.0, 2.5);

    let subbed = a - b;

    approx::assert_relative_eq!(subbed.x, 5.0 - -13.0);
    approx::assert_relative_eq!(subbed.y, 6.0 - 6.0);
    approx::assert_relative_eq!(subbed.z, -1.0 - 2.5);
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

#[test]
fn test_mul() {
    let a: Vec3 = Vec3::new(5.0, 2.0, -6.0);

    let multed = a * 3.0;

    approx::assert_relative_eq!(multed.x, 5.0 * 3.0);
    approx::assert_relative_eq!(multed.y, 2.0 * 3.0);
    approx::assert_relative_eq!(multed.z, -6.0 * 3.0);
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

#[test]
fn test_div() {
    let a: Vec3 = Vec3::new(5.0, 2.0, -6.0);

    let dived = a / 3.0;

    approx::assert_relative_eq!(dived.x, 5.0 / 3.0);
    approx::assert_relative_eq!(dived.y, 2.0 / 3.0);
    approx::assert_relative_eq!(dived.z, -6.0 / 3.0);
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

#[test]
fn test_neg() {
    let a: Vec3 = Vec3::new(12.0, 1.234, 9.04);

    let neged = -a;

    approx::assert_relative_eq!(neged.x, -12.0);
    approx::assert_relative_eq!(neged.y, -1.234);
    approx::assert_relative_eq!(neged.z, -9.04);
}
