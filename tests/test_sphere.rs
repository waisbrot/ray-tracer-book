use std::{error::Error, f64::consts::PI};

use book_renderer::{sphere::Sphere, matrix::Matrix, ray::{Ray, Intersectable}, tuple::Tuple, util::Float, material::Material};

#[test]
fn test_book_sphere_default_transform() {
    let s = Sphere::new_unit();
    assert_eq!(s.transform, Matrix::identity(4));
}

#[test]
fn test_book_sphere_mutable_transform() {
    let mut s = Sphere::new_unit();
    s.set_transform(Matrix::translation(2.0, 3.0, 4.0));
    assert_eq!(s.transform, Matrix::translation(2.0, 3.0, 4.0));
}

#[test]
fn test_book_intersect_scaled_sphere() {
    let r = Ray::new(Tuple::new_point(0.0, 0.0, -5.0), Tuple::new_vector(0.0, 0.0, 1.0));
    let mut s = Sphere::new_unit();
    s.set_transform(Matrix::scaling(2.0, 2.0, 2.0));
    let xs = s.intersections(&r);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].t, 3.0);
    assert_eq!(xs[1].t, 7.0);
}

#[test]
fn test_book_intersect_translated_sphere() {
    let r = Ray::new(Tuple::new_point(0.0, 0.0, -5.0), Tuple::new_vector(0.0, 0.0, 1.0));
    let mut s = Sphere::new_unit();
    s.set_transform(Matrix::translation(5.0, 0.0, 0.0));
    let xs = s.intersections(&r);
    assert_eq!(xs.len(), 0);
}

#[test]
fn test_book_surface_normal_x() {
    let s = Sphere::new_unit();
    assert_eq!(s.surface_normal(&Tuple::new_point(1.0, 0.0, 0.0)), Tuple::new_vector(1.0, 0.0, 0.0));
}

#[test]
fn test_book_surface_normal_y() {
    let s = Sphere::new_unit();
    assert_eq!(s.surface_normal(&Tuple::new_point(0.0, 1.0, 0.0)), Tuple::new_vector(0.0, 1.0, 0.0));
}

#[test]
fn test_book_surface_normal_z() {
    let s = Sphere::new_unit();
    assert_eq!(s.surface_normal(&Tuple::new_point(0.0, 0.0, 1.0)), Tuple::new_vector(0.0, 0.0, 1.0));
}

#[test]
fn test_book_surface_normal_nonaxial() {
    let s = Sphere::new_unit();
    let s33 = (3.0 as Float).sqrt() / 3.0;
    assert_eq!(s.surface_normal(&Tuple::new_point(s33.clone(), s33.clone(), s33.clone())), Tuple::new_vector(s33.clone(), s33.clone(), s33.clone()));
}

#[test]
fn test_book_surface_normal_normalized() -> Result<(), Box<dyn Error>> {
    let s = Sphere::new_unit();
    let s33 = (3.0 as Float).sqrt() / 3.0;
    let normal = s.surface_normal(&Tuple::new_point(s33.clone(), s33.clone(), s33.clone()));
    let normalized = normal.normalize()?;
    assert_eq!(normal, normalized);
    Ok(())
}

#[test]
fn test_book_translated_normal() {
    let mut s = Sphere::new_unit();
    s.set_transform(Matrix::translation(0.0, 1.0, 0.0));
    let n = s.surface_normal(&Tuple::new_point(0.0, 1.70711, -0.70711));
    assert_eq!(n, Tuple::new_vector(0.0, 0.70711, -0.70711));
}

#[test]
fn test_book_scaled_rotated_normal() {
    let mut s = Sphere::new_unit();
    s.set_transform(Matrix::scaling(1.0, 0.5, 1.0) * Matrix::rotation_z(PI/5.0));
    let n = s.surface_normal(&Tuple::new_point(0.0, 2.0_f64.sqrt()/2.0, -2.0_f64.sqrt()/2.0));
    assert_eq!(n, Tuple::new_vector(0.0, 0.97014, -0.24254));
}

#[test]
fn test_book_default_material() {
    let s = Sphere::new_unit();
    assert_eq!(s.material, Material::default());
}

#[test]
fn test_book_modify_material() {
    let mut s = Sphere::new_unit();
    assert_eq!(s, Sphere::new_unit());
    s.material.ambient = 1.0;
    assert_ne!(s.material, Material::default());
    assert_ne!(s, Sphere::new_unit());
}