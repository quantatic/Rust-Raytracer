use image::Rgb;

use std::ops::{Add, AddAssign, Div};

#[derive(Debug, Default, Clone, Copy)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64,
}

impl Color {
    // creates a color from a hex value
    // 0x__RRGGBB
    //
    pub fn hex(value: u32) -> Self {
        Self {
            r: (((value >> 16) & 0xFF) as f64) / 255.0,
            g: (((value >> 8) & 0xFF) as f64) / 255.0,
            b: ((value & 0xFF) as f64) / 255.0,
        }
    }

    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self {
            r: r.max(0.0).min(1.0),
            g: g.max(0.0).min(1.0),
            b: b.max(0.0).min(1.0),
        }
    }
}

impl Div<f64> for Color {
    type Output = Color;

    fn div(self, other: f64) -> Self::Output {
        Self {
            r: self.r / other,
            g: self.g / other,
            b: self.b / other,
        }
    }
}

impl Add<Color> for Color {
    type Output = Color;

    fn add(self, other: Color) -> Self::Output {
        Self {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl AddAssign<Color> for Color {
    fn add_assign(&mut self, other: Color) {
        self.r += other.r;
        self.g += other.g;
        self.b += other.b;
    }
}

impl From<Color> for Rgb<u8> {
    fn from(val: Color) -> Rgb<u8> {
        let r = (val.r.max(0.0).min(1.0) * 255.0) as u8;
        let g = (val.g.max(0.0).min(1.0) * 255.0) as u8;
        let b = (val.b.max(0.0).min(1.0) * 255.0) as u8;

        Self([r, g, b])
    }
}
