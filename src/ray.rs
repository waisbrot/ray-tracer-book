use auto_ops::*;
use crate::{tuple::Tuple, util::Float, matrix::Matrix};

#[derive(Debug, PartialEq)]
pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

impl Ray {
    // In book: ray(point, vector)
    pub fn new(origin: Tuple, direction: Tuple) -> Ray {
        Ray{origin, direction}
    }

    pub fn position(&self, distance: &Float) -> Tuple {
        self.origin + self.direction * distance
    }
}

impl_op_ex!(* |transform: &Matrix, ray: &Ray| -> Ray {
    Ray{
        origin: transform * &ray.origin,
        direction: transform * &ray.direction,
    }
});