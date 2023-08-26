use std::{error::Error, path::Path, fs::File, io::{Write, BufWriter}, ops::{Index, IndexMut}};

use crate::{color::{Color, BLACK}, util::Float};
use array2d::Array2D;
use png;

#[derive(Debug, Clone)]
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub pixels: Array2D<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        let pixels = Array2D::filled_with(BLACK, height, width);
        Canvas { width, height, pixels: pixels }
    }

    fn scale_and_clamp_255(number: Float) -> u8 {
        (number * 255.0).clamp(0.0, 255.0).round() as u8
    }

    pub fn ppm_data(&self) -> Vec<String> {
        let mut v: Vec<String> = Vec::new();
        v.push(String::from("P3"));
        v.push(format!("{} {}", self.width, self.height));
        v.push(String::from("255"));
        for y in 0..self.height {
            let strings:Vec<String> = self.pixels.row_iter(y)
                .unwrap()
                .map(|c| { 
                    let red = Self::scale_and_clamp_255(c.red);
                    let green = Self::scale_and_clamp_255(c.green);
                    let blue = Self::scale_and_clamp_255(c.blue);
                    format!("{} {} {}", red, green, blue) 
                })
                .collect();
            v.push(strings.join(" "));
        }
        v.push(String::from("\n"));
        return v;
    }

    pub fn write_ppm<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn Error>> {
        let fh = File::create(path)?;
        let mut file = BufWriter::new(fh);
        for line in self.ppm_data() {
            file.write_all(line.as_bytes())?;
            file.write_all(b"\n")?;
        }
        Ok(())
    }

    pub fn write_png<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn Error>> {
        let fh = File::create(path)?;
        let writer = BufWriter::new(fh);
        let mut encoder = png::Encoder::new(writer, self.width as u32, self.height as u32);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        encoder.set_source_gamma(png::ScaledFloat::from_scaled(45455)); // 1.0 / 2.2, scaled by 100000
        let mut writer = encoder.write_header()?;
        let data:Vec<u8> = self.pixels
            .as_row_major()
            .iter()
            .flat_map(|c| { 
                [Self::scale_and_clamp_255(c.red), Self::scale_and_clamp_255(c.green), Self::scale_and_clamp_255(c.blue), 255 as u8]
            })
            .collect();
        writer.write_image_data(&data)?;
        Ok(())
    }
}

impl Index<(usize,usize)> for Canvas {
    type Output=Color;

    fn index(&self, index: (usize,usize)) -> &Self::Output {
        &self.pixels[(index.1, index.0)]
    }
}
impl IndexMut<(usize,usize)> for Canvas {
    fn index_mut(&mut self, index: (usize,usize)) -> &mut Color {
        &mut self.pixels[(index.1, index.0)]
    }
}