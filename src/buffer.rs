use crate::Color;

use image::RgbImage;

pub struct Buffer {
    width: u32,
    height: u32,

    // laid out as: samples[y][x][sample_num] or samples[row][col][sample_num]
    samples: Vec<Vec<Vec<Color>>>,
}

impl Buffer {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            //samples: vec![vec![]; (width as usize) * (height as usize)],
            samples: vec![vec![vec![]; width as usize]; height as usize],
        }
    }

    pub fn add_sample(&mut self, x: u32, y: u32, sample: Color) {
        //self.samples[((y * self.width) as usize) + (x as usize)].push(sample);
        self.samples[y as usize][x as usize].push(sample);
    }

    pub fn image(&self) -> RgbImage {
        let mut res = RgbImage::new(self.width, self.height);

        for row in 0..self.height {
            for col in 0..self.width {
                res.put_pixel(col, row, self.average_samples(col, row).into());
            }
        }

        res
    }

    fn average_samples(&self, x: u32, y: u32) -> Color {
        let mut color = Color::default();
        let mut num_samples = 0.0;
        //for sample in &self.samples[((y * self.width) as usize) + (x as usize)] {
        for sample in &self.samples[y as usize][x as usize] {
            color += *sample;
            num_samples += 1.0;
        }

        let res = color / num_samples;
        res
    }
}

impl From<Buffer> for RgbImage {
    fn from(val: Buffer) -> RgbImage {
        let mut res = RgbImage::new(val.width, val.height);

        for row in 0..val.height {
            for col in 0..val.width {
                res.put_pixel(col, row, val.average_samples(col, row).into());
            }
        }

        res
    }
}
