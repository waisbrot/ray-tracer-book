use core::panic;

use crate::{
    material::Material,
    matrix::Matrix,
    ray::{Intersectable, Intersection, Ray},
    tuple::{Point, Tuple, Vector},
    util::Float,
};

#[derive(Debug, PartialEq)]
pub struct Sphere {
    pub origin: Tuple,
    pub radius: Float,
    pub transform: Matrix,
    pub material: Material,
}

#[derive(Debug, PartialEq)]
struct Discriminant {
    a: Float,
    b: Float,
    #[allow(dead_code)]
    c: Float,
    d: Float,
}

impl Sphere {
    pub fn new_unit() -> Sphere {
        Sphere {
            origin: Tuple::new_point(0.0, 0.0, 0.0),
            radius: 1.0,
            transform: Matrix::identity(4),
            material: Material::default(),
        }
    }

    fn discriminant(&self, ray: &Ray) -> Discriminant {
        let sphere_to_ray = ray.origin - self.origin;
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;
        let d = b.powi(2) - 4.0 * a * c;
        // eprintln!("a={}; b={}; c={}; d={}", a, b, c, d);
        Discriminant { a, b, c, d }
    }
    pub fn set_transform(&mut self, trans: Matrix) -> &Self {
        self.transform = trans;
        self
    }
    fn transformed_ray(&self, ray: &Ray) -> Ray {
        &self.transform.inverse().unwrap() * ray // todo: avoid unwrap
    }
}

impl Intersectable for Sphere {
    fn is_intersecting(&self, ray: &Ray) -> bool {
        self.discriminant(&self.transformed_ray(ray)).d >= 0.0
    }

    fn intersection_count(&self, ray: &Ray) -> usize {
        match self.discriminant(&self.transformed_ray(ray)).d {
            i if i < 0.0 => 0,
            i if i == 0.0 => 1,
            i if i > 0.0 => 2,
            i if i.is_infinite() => panic!("Discriminant is Inf"),
            i if i.is_nan() => panic!("Discriminant is NaN"),
            i => panic!("Unknow error for discriminant `{:?}`", i),
        }
    }

    fn intersections(&self, ray: &Ray) -> Vec<Intersection> {
        let discriminant = self.discriminant(&self.transformed_ray(ray));
        match discriminant.d {
            d if d < 0.0 => vec![],
            d if d == 0.0 => vec![Intersection {
                t: -discriminant.b / (2.0 * discriminant.a),
                object: self,
            }],
            d if d > 0.0 => vec![
                Intersection {
                    t: (-discriminant.b - d.sqrt()) / (2.0 * discriminant.a),
                    object: self,
                },
                Intersection {
                    t: (-discriminant.b + d.sqrt()) / (2.0 * discriminant.a),
                    object: self,
                },
            ],
            _ => panic!("Bad discriminant `{:?}`", discriminant),
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn transformation(&self) -> &Matrix {
        &self.transform
    }

    fn surface_normal(&self, point: &Point) -> Vector {
        let object_point = self.transform.inverse().unwrap() * point;
        let object_normal = object_point - self.origin;
        let transform = self.transform.inverse().unwrap().transpose();
        let mut world_normal = transform * object_normal;
        world_normal.w = 0.0;
        world_normal.normalize().unwrap()
    }

    fn material(&self) -> &Material {
        &self.material
    }
}
