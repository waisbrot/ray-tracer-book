use auto_ops::*;
use std::{error::Error, fmt::Display, num::ParseFloatError};
use crate::util::{feq, Float};

#[derive(Debug, Clone, Copy)]
pub struct Tuple {
    pub x: Float,
    pub y: Float,
    pub z: Float,
    pub w: Float,
}

impl Display for Tuple {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}

impl From<&Tuple> for String {
    fn from(value: &Tuple) -> Self {
        std::format!("({:7.3}, {:7.3}, {:7.3}, {:7.3})", value.x, value.y, value.z, value.w)
    }
}

pub type Vector = Tuple;
pub type Point = Tuple;

impl Tuple {
    pub fn new_point(x: Float, y: Float, z: Float) -> Point {
        Tuple { x, y, z, w: 1.0 }
    }

    pub fn parse_point(s: &str) -> Result<Point, ParseFloatError> {
        let pieces:Vec<&str> = s.split(',').collect();
        Ok(Self::new_point(pieces[0].parse::<Float>()?, pieces[1].parse::<Float>()?, pieces[2].parse::<Float>()?))
    }

    pub fn new_vector(x: Float, y: Float, z: Float) -> Vector {
        Tuple { x, y, z, w: 0.0 }
    }

    pub fn parse_vector(s: &str) -> Result<Vector, ParseFloatError> {
        let pieces:Vec<&str> = s.split(',').collect();
        Ok(Self::new_vector(pieces[0].parse::<Float>()?, pieces[1].parse::<Float>()?, pieces[2].parse::<Float>()?))
    }

    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }

    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }

    pub fn magnitude(&self) -> Result<Float, FloatingPointFiniteError> {
        let x = self.x.powi(2);
        let y = self.y.powi(2);
        let z = self.z.powi(2);
        let w = self.w.powi(2);
        let sum = x + y + z + w;
        match sum.is_finite() {
            true => Ok(sum.sqrt()),
            false => Err(FloatingPointFiniteError {}),
        }
    }

    pub fn normalize(&self) -> Result<Tuple, FloatingPointFiniteError> {
        Ok(self / self.magnitude()?)
    }

    pub fn dot(&self, other: &Tuple) -> Float {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    pub fn cross(&self, other: &Tuple) -> Result<Tuple, NotVector> {
        match (self.is_vector(), other.is_vector()) {
            (true, false) => Err(NotVector::new(other)),
            (false, true) => Err(NotVector::new(self)),
            (false, false) => Err(NotVector::new(self)),
            (true, true) => Ok(Self::new_vector(
                self.y * other.z - self.z * other.y,
                self.z * other.x - self.x * other.z,
                self.x * other.y - self.y * other.x,
            )),
        }
    }
}

impl_op_ex!(+ |a: &Tuple, b: &Tuple| -> Tuple { Tuple{x: a.x + b.x, y: a.y + b.y, z: a.z + b.z, w: a.w + b.w} });
impl_op_ex!(- |a: &Tuple, b: &Tuple| -> Tuple {
    Tuple {
        x: a.x - b.x,
        y: a.y - b.y,
        z: a.z - b.z,
        w: a.w - b.w,
    }
});
impl_op_ex!(* |a: &Tuple, b: &Float| -> Tuple {
    Tuple {
        x: a.x * b,
        y: a.y * b,
        z: a.z * b,
        w: a.w * b,
    }
});
impl_op_ex!(/ |a: &Tuple, b: &Float| -> Tuple { Tuple{x: a.x / b, y: a.y / b, z: a.z / b, w:a.w / b} });
impl_op_ex!(- |a: &Tuple| -> Tuple {
    Tuple {
        x: -a.x,
        y: -a.y,
        z: -a.z,
        w: -a.w,
    }
});
impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        feq(&self.x, &other.x) && feq(&self.y, &other.y) && feq(&self.z, &other.z) && feq(&self.w, &other.w)
    }
}

#[derive(Debug)]
pub struct FloatingPointFiniteError {}
impl Error for FloatingPointFiniteError {}
impl Display for FloatingPointFiniteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Floating point number is NaN or Inf")
    }
}

#[derive(Debug)]
pub struct NotVector {
    tuple: Tuple,
}
impl NotVector {
    pub fn new(tuple: &Tuple) -> NotVector {
        NotVector{tuple: tuple.clone()}
    }
}
impl Error for NotVector {}
impl Display for NotVector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Expecting {:?} to be a vector but it is not", self.tuple)
    }
}
