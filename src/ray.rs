use crate::vector::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub pos: Vec3,
    pub dir: Vec3
}

impl Ray {
    pub fn eval(&self, t: f64) -> Vec3 {
        self.pos + (self.dir * t)
    }
}

#[test]
fn test_eval() {
    let a: Ray = Ray {
        pos: Vec3::new(1.0, 2.0, 3.0),
        dir: Vec3::new(5.0, -1.0, 6.0),
    };

    let evaled: Vec3 = a.eval(1.5);

    approx::assert_relative_eq!(evaled.x, 1.0 + 5.0 * 1.5);
    approx::assert_relative_eq!(evaled.y, 2.0 + -1.0 * 1.5);
    approx::assert_relative_eq!(evaled.z, 3.0 + 6.0 * 1.5);
}
