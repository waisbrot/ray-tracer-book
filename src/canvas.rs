use std::{error::Error, path::Path, fs::File, io::{LineWriter, Write, BufWriter}};

use crate::color::{Color, BLACK};
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

    pub fn ppm_data(&self) -> Vec<String> {
        let mut v: Vec<String> = Vec::new();
        v.push(String::from("P3"));
        v.push(format!("{} {}", self.width, self.height));
        v.push(String::from("255"));
        for y in 0..self.height {
            let strings:Vec<String> = self.pixels.row_iter(y)
                .unwrap()
                .map(|c| { format!("{} {} {}", (c.red * 255.0) as i32, (c.green * 255.0) as i32, (c.blue * 255.0) as i32) })
                .collect();
            v.push(strings.join(" "));
        }
        return v;
    }

    pub fn write_ppm<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn Error>> {
        let fh = File::create(path)?;
        let mut file = BufWriter::new(fh);
        for line in self.ppm_data() {
            file.write_all(line.as_bytes());
            file.write_all(b"\n");
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
            .flat_map(|c| { [(c.red * 255.0) as u8, (c.green * 255.0) as u8, (c.blue * 255.0) as u8, 255] })
            .collect();
        writer.write_image_data(&data)?;
        Ok(())
    }
}