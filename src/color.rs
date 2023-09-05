use auto_ops::*;

use crate::util::{feq, Float};

#[derive(Debug, Clone, Copy, PartialOrd)]
pub struct Color {
    pub red: Float,
    pub green: Float,
    pub blue: Float,
}

impl Color {
    pub fn new(red: Float, green: Float, blue: Float) -> Color {
        Color{red, green, blue}
    }

    pub fn white(intensity: Float) -> Color {
        Self::new(intensity, intensity, intensity)
    }
}

pub const BLACK: Color = Color { red: 0.0, green: 0.0, blue: 0.0 };
pub const RED: Color = Color { red: 255.0, green: 0.0, blue: 0.0 };
pub const GREEN: Color = Color { red: 0.0, green: 255.0, blue: 0.0 };
pub const BLUE: Color = Color { red: 0.0, green: 0.0, blue: 255.0 };

impl_op_ex!(+ |a: &Color, b: &Color| -> Color { Color::new(a.red + b.red, a.green + b.green, a.blue + b.blue)});
impl_op_ex!(- |a: &Color, b: &Color| -> Color { Color::new(a.red - b.red, a.green - b.green, a.blue - b.blue)});
impl_op_ex!(* |a: &Color, b: &Color| -> Color { Color::new(a.red * b.red, a.green * b.green, a.blue * b.blue)});
impl_op_ex!(* |a: &Color, b: &Float| -> Color { Color::new(a.red * b, a.green * b, a.blue * b)});
impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        feq(&self.red, &other.red) && feq(&self.green, &other.green) && feq(&self.blue, &other.blue)
    }
}
