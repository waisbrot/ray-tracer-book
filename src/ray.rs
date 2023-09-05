use std::any::Any;
use auto_ops::*;
use crate::{tuple::{Tuple, Point, Vector}, util::Float, matrix::Matrix, material::Material};

#[derive(Debug, Clone)]
pub struct Intersection<'a> {
    pub t: Float,
    pub object:  &'a dyn Intersectable,
}

impl <'a> Intersection<'a> {
    pub fn hit(intersections: &Vec<Intersection<'a>>) -> Option<Intersection<'a>> {
        intersections.iter()
        .filter(|i| i.t >= 0.0)
        .reduce(|acc, e| if e < acc { e } else { acc })
        .map(|i| i.clone())
    }
}

impl PartialOrd for Intersection<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.t.partial_cmp(&other.t)
    }
}

impl PartialEq<Intersection<'_>> for Intersection<'_> {
    fn eq(&self, other: &Intersection<'_>) -> bool {
        self.t == other.t && self.object as *const dyn Intersectable as *const () == other.object as *const dyn Intersectable as *const ()
    }
}
impl PartialEq<Intersection<'_>> for &Intersection<'_> {
    fn eq(&self, other: &Intersection<'_>) -> bool {
        self.t == other.t && self.object as *const dyn Intersectable as *const () == other.object as *const dyn Intersectable as *const ()
    }
}
impl PartialEq<&Intersection<'_>> for Intersection<'_> {
    fn eq(&self, other: &&Intersection) -> bool {
        self.t == other.t && self.object as *const dyn Intersectable as *const () == other.object as *const dyn Intersectable as *const ()
    }
}

pub trait Intersectable: std::fmt::Debug + Any {
    fn is_intersecting(&self, ray: &Ray) -> bool;
    fn intersection_count(&self, ray: &Ray) -> usize;
    fn intersections(&self, ray: &Ray) -> Vec<Intersection>;
    fn as_any(&self) -> &dyn Any;
    fn transformation(&self) -> &Matrix;
    fn surface_normal(&self, point: &Point) -> Vector;
    fn material(&self) -> &Material;
}

impl<T: PartialEq + Any> PartialEq<T> for dyn Intersectable {
    fn eq(&self, other: &T) -> bool {
        if let Some(this) = self.as_any().downcast_ref::<T>() {
            this == other
        } else {
            false
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

impl Ray {
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