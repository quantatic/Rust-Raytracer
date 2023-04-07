use std::{
    iter::Sum,
    ops::{Add, AddAssign, Div, Mul, Sub},
};

use nalgebra::{UnitVector3, Vector3};
use rand::Rng;
use rand_distr::{Distribution, StandardNormal};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec3(pub f32, pub f32, pub f32);

impl Vec3 {
    pub fn len(self) -> f32 {
        let len_squared = (self.0 * self.0) + (self.1 * self.1) + (self.2 * self.2);
        len_squared.sqrt()
    }

    pub fn norm(self) -> UnitVec3 {
        UnitVec3 {
            inner: self / self.len(),
        }
    }

    pub fn dot(self, other: Vec3) -> f32 {
        (self.0 * other.0) + (self.1 * other.1) + (self.2 * other.2)
    }

    pub fn reflect(self, normal: UnitVec3) -> Vec3 {
        self - (2.0 * self.dot(normal.to_vec3()) * normal.to_vec3())
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3(self * rhs.0, self * rhs.1, self * rhs.2)
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Self::Output {
        Vec3(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl Sum<Vec3> for Vec3 {
    fn sum<I: Iterator<Item = Vec3>>(iter: I) -> Self {
        let mut result = Vec3::default();
        for item in iter {
            result += item;
        }

        result
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UnitVec3 {
    inner: Vec3, // this value must always be normalized (have length 1)
}

impl UnitVec3 {
    pub fn to_vec3(self) -> Vec3 {
        self.inner
    }

    pub fn random_unit_sphere<R: Rng + ?Sized>(rng: &mut R) -> UnitVec3 {
        let [x, y, z] = std::array::from_fn(|_| StandardNormal.sample(rng));
        Vec3(x, y, z).norm()
    }
}

pub fn random_unit_sphere<R: Rng + ?Sized>(rng: &mut R) -> UnitVector3<f32> {
    let [x, y, z] = std::array::from_fn(|_| StandardNormal.sample(rng));
    UnitVector3::new_normalize(Vector3::new(x, y, z))
}

#[cfg(test)]
mod tests {
    use super::*;

    const MARGIN_OF_ERROR: f32 = 0.00001;

    fn assert_approx_equal(vec_one: Vec3, vec_two: Vec3, margin: f32) {
        let approx_equal = if (vec_one.0 - vec_two.0).abs() > margin {
            false
        } else if (vec_one.1 - vec_two.1).abs() > margin {
            false
        } else if (vec_one.2 - vec_two.2).abs() > margin {
            false
        } else {
            true
        };

        if !approx_equal {
            assert_eq!(approx_equal, true, "{vec_one:?} is not approximately equal to {vec_two:?} with a margin of error {margin}");
        }
    }

    #[test]
    fn test_eq() {
        let vec_one = Vec3(1.0, -3.0, 5.3);
        let vec_two = Vec3(1.0, -3.0, 5.3);
        assert_approx_equal(vec_one, vec_two, MARGIN_OF_ERROR);
    }

    #[test]
    fn test_add() {
        let vec_one = Vec3(1.0, -3.0, 5.3);
        let vec_two = Vec3(4.9, -0.1, -6.7);
        let expected_result = Vec3(5.9, -3.1, -1.4);
        assert_approx_equal(vec_one + vec_two, expected_result, MARGIN_OF_ERROR);
    }

    #[test]
    fn test_sub() {
        let vec_one = Vec3(1.0, -3.0, 5.3);
        let vec_two = Vec3(4.9, -0.1, -6.7);
        let expected_result = Vec3(-3.9, -2.9, 12.0);
        assert_approx_equal(vec_one - vec_two, expected_result, MARGIN_OF_ERROR);
    }

    #[test]
    fn test_mul() {
        let vec_in = Vec3(0.6, 7.9, -3.2);
        let multiplier = 9.8;
        let expected_result = Vec3(5.88, 77.42, -31.36);
        assert_approx_equal(vec_in * multiplier, expected_result, MARGIN_OF_ERROR);
        assert_approx_equal(multiplier * vec_in, expected_result, MARGIN_OF_ERROR);
    }

    #[test]
    fn test_div() {
        let vec_in = Vec3(0.6, 7.9, -3.2);
        let dividend = -0.25;
        let expected_result = Vec3(-2.4, -31.6, 12.8);
        assert_approx_equal(vec_in / dividend, expected_result, MARGIN_OF_ERROR);
    }
}
