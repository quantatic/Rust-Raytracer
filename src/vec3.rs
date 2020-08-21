use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use num::Float;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

impl<T: Float> Vec3<T> {
    pub fn size(&self) -> T {
        ((self.x * self.x) + (self.y * self.y) + (self.z * self.z)).sqrt()
    }

    pub fn dot(&self, other: Self) -> T {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }

    pub fn cross(&self, other: Self) -> Self {
        Self {
            x: (self.y * other.z - self.z * other.y),
            y: (self.z * other.x - self.x * other.z),
            z: (self.x * other.y - self.y - other.x),
        }
    }

    pub fn unit(&self) -> Self {
        *self / self.size()
    }
}

impl<T> Add for Vec3<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T> AddAssign for Vec3<T>
where
    T: AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl<T> Sub for Vec3<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T> SubAssign for Vec3<T>
where
    T: SubAssign,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl<T> Mul<T> for Vec3<T>
where
    T: Mul<Output = T> + Clone,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs.clone(),
            y: self.y * rhs.clone(),
            z: self.z * rhs,
        }
    }
}

impl<T> MulAssign<T> for Vec3<T>
where
    T: MulAssign<T> + Clone,
{
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs.clone();
        self.y *= rhs.clone();
        self.z *= rhs
    }
}

impl<T> Div<T> for Vec3<T>
where
    T: Div<Output = T> + Clone,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs.clone(),
            y: self.y / rhs.clone(),
            z: self.z / rhs,
        }
    }
}

impl<T> DivAssign<T> for Vec3<T>
where
    T: DivAssign<T> + Clone,
{
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs.clone();
        self.y /= rhs.clone();
        self.z /= rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let vec = Vec3::new(1, 2, 3);
        assert_eq!(vec.x, 1);
        assert_eq!(vec.y, 2);
        assert_eq!(vec.z, 3);
    }

    #[test]
    fn test_equals() {
        let vec = Vec3::new(1, 2, 3);
        let vec2 = Vec3::new(1, 2, 3);
        assert_eq!(vec, vec2);

        let not_equal = Vec3::new(4, 2, 3);
        assert_ne!(vec, not_equal);
    }

    #[test]
    fn test_add_positive() {
        let start = Vec3::new(1, 2, 3);
        let to_add = Vec3::new(7, 3, 10);

        let final_expected = Vec3::new(8, 5, 13);

        assert_eq!(start + to_add, final_expected);
    }

    #[test]
    fn test_add_negative() {
        let start = Vec3::new(1, 2, 3);
        let to_add = Vec3::new(-4, -15, 2);

        let final_expected = Vec3::new(-3, -13, 5);
        assert_eq!(start + to_add, final_expected);
    }

    #[test]
    fn test_add_assign_positive() {
        let mut start = Vec3::new(3, 4, 5);
        start += Vec3::new(5, 6, 7);

        let final_expected = Vec3::new(8, 10, 12);
        assert_eq!(start, final_expected);
    }

    #[test]
    fn test_add_assign_negative() {
        let mut start = Vec3::new(3, 4, 5);
        start += Vec3::new(-1, -4, -33);

        let final_expected = Vec3::new(2, 0, -28);
        assert_eq!(start, final_expected);
    }

    #[test]
    fn test_sub_positive() {
        let start = Vec3::new(1, 2, 3);
        let to_sub = Vec3::new(7, 3, 10);

        let final_expected = Vec3::new(-6, -1, -7);

        assert_eq!(start - to_sub, final_expected);
    }

    #[test]
    fn test_sub_negative() {
        let start = Vec3::new(1, 2, 3);
        let to_sub = Vec3::new(-4, -15, 2);

        let final_expected = Vec3::new(5, 17, 1);
        assert_eq!(start - to_sub, final_expected);
    }

    #[test]
    fn test_sub_assign_positive() {
        let mut start = Vec3::new(3, 4, 5);
        start -= Vec3::new(5, 6, 7);

        let final_expected = Vec3::new(-2, -2, -2);
        assert_eq!(start, final_expected);
    }

    #[test]
    fn test_sub_assign_negative() {
        let mut start = Vec3::new(3, 4, 5);
        start -= Vec3::new(-1, -4, -33);

        let final_expected = Vec3::new(4, 8, 38);
        assert_eq!(start, final_expected);
    }

    #[test]
    fn test_mul() {
        let start = Vec3::new(1, 7, 4);
        let to_mul = 5;

        let final_expected = Vec3::new(5, 35, 20);
        assert_eq!(start * to_mul, final_expected);
    }

    #[test]
    fn test_mul_assign() {
        let mut start = Vec3::new(1, 7, 4);
        let to_mul = 5;

        let final_expected = Vec3::new(5, 35, 20);

        start *= to_mul;

        assert_eq!(start, final_expected);
    }

    #[test]
    fn test_div() {
        let start = Vec3::new(44, 16, 4);
        let to_div = 4;

        let final_expected = Vec3::new(11, 4, 1);
        assert_eq!(start / to_div, final_expected);
    }

    #[test]
    fn test_div_assign() {
        let mut start = Vec3::new(44, 16, 4);
        let to_div = 4;

        let final_expected = Vec3::new(11, 4, 1);

        start /= to_div;
        assert_eq!(start, final_expected);
    }

    #[test]
    fn test_size() {
        let start = Vec3::new(3.0, -4.0, -12.0);

        let expected_size = 13.0;

        assert_eq!(start.size(), expected_size);
    }

    #[test]
    fn test_dot() {
        let vec1 = Vec3::new(2.0, 3.0, 9.0);
        let vec2 = Vec3::new(0.5, 1.5, 6.0);

        let expected_dot = 59.5;

        assert_eq!(vec1.dot(vec2), expected_dot);
    }
}
