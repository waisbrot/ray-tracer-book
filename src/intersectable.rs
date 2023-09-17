use std::any::Any;

use crate::{ray::Ray, intersection::Intersection, matrix::Matrix, material::Material, tuple::{Point, Vector}};

pub trait Intersectable: std::fmt::Debug + Any {
    fn is_intersecting(&self, ray: &Ray) -> bool;
    fn intersection_count(&self, ray: &Ray) -> usize;
    fn intersections(&self, ray: &Ray) -> Vec<Intersection>;
    fn as_any(&self) -> &dyn Any;
    fn transformation(&self) -> &Matrix;
    fn surface_normal(&self, point: &Point) -> Vector;
    fn mut_material(&mut self) -> &mut Material;
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