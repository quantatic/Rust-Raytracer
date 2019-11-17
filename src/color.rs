#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color {
            r,
            g,
            b
        }
    }

	pub fn blend(c1: Self, part_c1: f64, c2: Self, part_c2: f64) -> Self {
		// make sure the percentages add up to 100%
		assert_eq!(part_c1 + part_c2, 1.0);

		Color {
			r: ((c1.r as f64) * part_c1 + (c2.r as f64) * part_c2) as u8,
			g: ((c1.g as f64) * part_c1 + (c2.g as f64) * part_c2) as u8,
			b: ((c1.b as f64) * part_c1 + (c2.b as f64) * part_c2) as u8,
		}
	}
}

impl Into<image::Rgb<u8>> for Color {
    fn into(self) -> image::Rgb<u8> {
        image::Rgb([self.r, self.g, self.b])
    }
}
