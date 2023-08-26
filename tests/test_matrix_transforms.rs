use std::{error::Error, f64::consts::PI};
// use proptest::{prelude::*, num::f64::{POSITIVE, NEGATIVE,NORMAL}};
use book_renderer::{matrix::Matrix, tuple::Tuple, util::Float};

#[test]
fn test_book_translation_mult() {
    let transform = Matrix::translation(5.0, -3.0, 2.0);
    let p = Tuple::new_point(-3.0, 4.0, 5.0);
    assert_eq!(transform * p, Tuple::new_point(2.0, 1.0, 7.0));
}

#[test]
fn test_book_inverse_translation_mult() -> Result<(), Box<dyn Error>> {
    let transform = Matrix::translation(5.0, -3.0, 2.0);
    let inv = transform.inverse()?;
    let p = Tuple::new_point(-3.0, 4.0, 5.0);
    assert_eq!(inv * p, Tuple::new_point(-8.0, 7.0, 3.0));
    Ok(())
}

#[test]
fn test_book_transform_vector() {
    let transform = Matrix::translation(5.0, -3.0, 2.0);
    let v = Tuple::new_vector(-3.0, 4.0, 5.0);
    assert_eq!(transform * v, v);
}

#[test]
fn test_book_scale_point() {
    let transform = Matrix::scaling(2.0, 3.0, 4.0);
    let p = Tuple::new_point(-4.0, 6.0, 8.0);
    assert_eq!(transform * p, Tuple::new_point(-8.0, 18.0, 32.0));
}

#[test]
fn test_book_scale_vector() {
    let transform = Matrix::scaling(2.0, 3.0, 4.0);
    let v = Tuple::new_vector(-4.0, 6.0, 8.0);
    assert_eq!(transform * v, Tuple::new_vector(-8.0, 18.0, 32.0));
}

#[test]
fn test_book_scale_vector_inverse() -> Result<(), Box<dyn Error>> {
    let transform = Matrix::scaling(2.0, 3.0, 4.0);
    let inv = transform.inverse()?;
    let v = Tuple::new_vector(-4.0, 6.0, 8.0);
    assert_eq!(inv * v, Tuple::new_vector(-2.0, 2.0, 2.0));
    Ok(())
}

#[test]
fn test_book_reflect_point() {
    let transform = Matrix::scaling(-1.0, 1.0, 1.0);
    let p = Tuple::new_point(2.0, 3.0, 4.0);
    assert_eq!(transform * p, Tuple::new_point(-2.0, 3.0, 4.0));
}

#[test]
fn test_book_rotate_x_axis() {
    let p = Tuple::new_point(0.0, 1.0, 0.0);
    let half_quarter = Matrix::rotation_x(PI / 4.0);
    let full_quarter = Matrix::rotation_x(PI / 2.0);
    assert_eq!(half_quarter * p, Tuple::new_point(0.0, Float::sqrt(2.0)/2.0, Float::sqrt(2.0)/2.0));
    assert_eq!(full_quarter * p, Tuple::new_point(0.0, 0.0, 1.0));
}

#[test]
fn test_book_inverse_x_rotation() -> Result<(), Box<dyn Error>> {
    let p = Tuple::new_point(0.0, 1.0, 0.0);
    let half_quarter = Matrix::rotation_x(PI / 4.0);
    let inv = half_quarter.inverse()?;
    assert_eq!(inv * p, Tuple::new_point(0.0, Float::sqrt(2.0)/2.0, -Float::sqrt(2.0)/2.0));
    Ok(())
}

#[test]
fn test_book_y_rotation() {
    let p = Tuple::new_point(0.0, 0.0, 1.0);
    let half_quarter = Matrix::rotation_y(PI/4.0);
    let full_quarter = Matrix::rotation_y(PI/2.0);
    assert_eq!(half_quarter * p, Tuple::new_point(Float::sqrt(2.0)/2.0, 0.0, Float::sqrt(2.0)/2.0));
    assert_eq!(full_quarter * p, Tuple::new_point(1.0, 0.0, 0.0));
}

#[test]
fn test_book_z_rotation() {
    let p = Tuple::new_point(0.0, 1.0, 0.0);
    let half_quarter = Matrix::rotation_z(PI/4.0);
    let full_quarter = Matrix::rotation_z(PI/2.0);
    assert_eq!(half_quarter * p, Tuple::new_point(-Float::sqrt(2.0)/2.0, Float::sqrt(2.0)/2.0, 0.0));
    assert_eq!(full_quarter * p, Tuple::new_point(-1.0, 0.0, 0.0));
}

macro_rules! shearing_tests {
    ($($test_name:ident: $shear:expr, $expected:expr)*) => {
        $(
            #[test]
            fn $test_name() {
                let transform = Matrix::shear($shear.0, $shear.1, $shear.2, $shear.3, $shear.4, $shear.5);
                let p = Tuple::new_point(2.0, 3.0, 4.0);
                assert_eq!(transform * p, Tuple::new_point($expected.0, $expected.1, $expected.2));
            }
        )*
    }
}
shearing_tests! {
    x_y: (1.0, 0.0, 0.0, 0.0, 0.0, 0.0),(5.0, 3.0, 4.0)
    x_z: (0.0, 1.0, 0.0, 0.0, 0.0, 0.0),(6.0, 3.0, 4.0)
    y_x: (0.0, 0.0, 1.0, 0.0, 0.0, 0.0),(2.0, 5.0, 4.0)
    y_z: (0.0, 0.0, 0.0, 1.0, 0.0, 0.0),(2.0, 7.0, 4.0)
    z_x: (0.0, 0.0, 0.0, 0.0, 1.0, 0.0),(2.0, 3.0, 6.0)
    z_y: (0.0, 0.0, 0.0, 0.0, 0.0, 1.0),(2.0, 3.0, 7.0)
}

#[test]
fn test_book_transform_seq() {
    let p = Tuple::new_point(1.0, 0.0, 1.0);
    let a = Matrix::rotation_x(PI/2.0);
    let b = Matrix::scaling(5.0, 5.0, 5.0);
    let c = Matrix::translation(10.0, 5.0, 7.0);
    let p2 = a * p;
    assert_eq!(p2, Tuple::new_point(1.0, -1.0, 0.0));
    let p3 = b * p2;
    assert_eq!(p3, Tuple::new_point(5.0, -5.0, 0.0));
    let p4 = c * p3;
    assert_eq!(p4, Tuple::new_point(15.0, 0.0, 7.0));
}

#[test]
fn test_book_transform_chain() {
    let p = Tuple::new_point(1.0, 0.0, 1.0);
    let a = Matrix::rotation_x(PI/2.0);
    let b = Matrix::scaling(5.0, 5.0, 5.0);
    let c = Matrix::translation(10.0, 5.0, 7.0);
    let t = c * b * a;
    assert_eq!(t * p, Tuple::new_point(15.0, 0.0, 7.0));
}