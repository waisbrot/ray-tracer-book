use crate::{tuple::Point, color::Color};

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Light {
    pub position: Point,
    pub intensity: Color,
}

impl Light {
    pub fn new_point(position: Point, intensity: Color) -> Light {
        Light{ position, intensity }
    }
}