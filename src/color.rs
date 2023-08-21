use auto_ops::*;

use crate::util::feq;

#[derive(Debug,Clone,Copy)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

impl Color {
    pub fn new(red: f32, green: f32, blue: f32) -> Color {
        Color{red, green, blue}
    }
}

pub const BLACK: Color = Color { red: 0.0, green: 0.0, blue: 0.0 };

impl_op_ex!(+ |a: &Color, b: &Color| -> Color { Color::new(a.red + b.red, a.green + b.green, a.blue + b.blue)});
impl_op_ex!(- |a: &Color, b: &Color| -> Color { Color::new(a.red - b.red, a.green - b.green, a.blue - b.blue)});
impl_op_ex!(* |a: &Color, b: &Color| -> Color { Color::new(a.red * b.red, a.green * b.green, a.blue * b.blue)});
impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        feq(&self.red, &other.red) && feq(&self.green, &other.green) && feq(&self.blue, &other.blue)
    }
}
