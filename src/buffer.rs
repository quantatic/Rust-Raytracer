use crate::Color;

use image::RgbImage;

use std::convert::TryInto;

pub struct Buffer {
    width: usize,
    height: usize,

    // laid out as: samples[(y * width) + x][sample_num] or samples[(row * width) + col][sample_num]
    samples: Vec<Vec<Color>>,
}

impl Buffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            samples: vec![vec![]; width * height],
        }
    }

    pub fn enumerate_pixels(&mut self) -> EnumeratePixels {
        let pixels = self.samples.iter_mut().map(move |pixel_samples| Pixel {
            samples: pixel_samples,
        });

        EnumeratePixels {
            pixels: Box::new(pixels),
            row: 0,
            col: 0,
            width: self.width,
        }
    }

    fn average_samples(&self, x: usize, y: usize) -> Color {
        let mut color = Color::default();
        let mut num_samples = 0.0;
        //for sample in &self.samples[((y * self.width) as usize) + (x as usize)] {
        for sample in &self.samples[(y * self.width) + x] {
            color += *sample;
            num_samples += 1.0;
        }

        let res = color / num_samples;
        res
    }
}

pub struct Pixel<'a> {
    samples: &'a mut Vec<Color>,
}

impl Pixel<'_> {
    pub fn add_sample(&mut self, sample: Color) {
        self.samples.push(sample);
    }
}

pub struct EnumeratePixels<'a> {
    pixels: Box<dyn Iterator<Item = Pixel<'a>> + 'a + Send + Sync>,
    col: usize,
    row: usize,
    width: usize,
}

impl<'a> Iterator for EnumeratePixels<'a> {
    type Item = (usize, usize, Pixel<'a>);

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        self.pixels.next().map(|pixel| {
            let res = (self.col, self.row, pixel);
            self.col += 1;
            if self.col >= self.width {
                self.col = 0;
                self.row += 1;
            }

            res
        })
    }
}

impl From<Buffer> for RgbImage {
    fn from(val: Buffer) -> RgbImage {
        let mut res = RgbImage::new(
            val.width.try_into().unwrap(),
            val.height.try_into().unwrap(),
        );

        for row in 0..val.height {
            for col in 0..val.width {
                res.put_pixel(
                    col.try_into().unwrap(),
                    row.try_into().unwrap(),
                    val.average_samples(col, row).into(),
                );
            }
        }

        res
    }
}
