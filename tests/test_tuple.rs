use std::error::Error;

use book_renderer::{tuple::*, util::{feq, Float}};
use proptest::prelude::*;

#[test]
fn test_book_point() {
    let t = Tuple {
        x: 4.3,
        y: -4.2,
        z: 3.1,
        w: 1.0,
    };
    assert!(t.is_point());
}

#[test]
fn test_book_vector() {
    let t = Tuple {
        x: 4.3,
        y: -4.2,
        z: 3.1,
        w: 0.0,
    };
    assert!(t.is_vector());
}

#[test]
fn test_book_addition() {
    let t1 = Tuple {
        x: 3.,
        y: -2.,
        z: 5.,
        w: 1.,
    };
    let t2: Tuple = Tuple {
        x: -2.,
        y: 3.,
        z: 1.,
        w: 0.,
    };
    let expect = Tuple {
        x: 1.,
        y: 1.,
        z: 6.,
        w: 1.,
    };
    assert_eq!(t1 + t2, expect);
}

#[test]
fn test_book_subtract_points() {
    let p1 = Tuple::new_point(3., 2., 1.);
    let p2 = Tuple::new_point(5., 6., 7.);
    assert_eq!(p1 - p2, Tuple::new_vector(-2., -4., -6.));
}

#[test]
fn test_book_subtract_vector_point() {
    let p = Tuple::new_point(3., 2., 1.);
    let v = Tuple::new_vector(5., 6., 7.);
    assert_eq!(p - v, Tuple::new_point(-2., -4., -6.))
}

#[test]
fn test_book_subtract_vectors() {
    let v1 = Tuple::new_vector(3., 2., 1.);
    let v2 = Tuple::new_vector(5., 6., 7.);
    assert_eq!(v1 - v2, Tuple::new_vector(-2., -4., -6.));
}

#[test]
fn test_book_negate_tuple() {
    let t = Tuple{x:1.,y:-2.,z:3.,w:-4.};
    let n = Tuple{x:-1.,y:2.,z:-3.,w:4.};
    assert_eq!(-t, n);
}

#[test]
fn test_book_multiply_tuple_scalar() {
    let t = Tuple{x:1., y:-2., z:3., w:-4.};
    assert_eq!(t * 3.5, Tuple{x:3.5, y:-7., z:10.5, w:-14.});
}

#[test]
fn test_book_divide_tuple_scalar() {
    let t = Tuple{x:1., y:-2., z:3., w:-4.};
    assert_eq!(t/2., Tuple{x:0.5, y:-1., z:1.5, w:-2.});
}

#[test]
fn test_book_magnitude() -> Result<(), Box<dyn Error>> {
    assert_eq!(Tuple::new_vector(1., 0., 0.).magnitude()?, 1.);
    assert_eq!(Tuple::new_vector(0., 1., 0.).magnitude()?, 1.);
    assert_eq!(Tuple::new_vector(0., 0., 1.).magnitude()?, 1.);
    assert_eq!(Tuple::new_vector(1., 2., 3.).magnitude()?, Float::sqrt(14.));
    assert_eq!(Tuple::new_vector(-1., -2., -3.).magnitude()?, Float::sqrt(14.));
    Ok(())
}

#[test]
fn test_book_normalization() -> Result<(), Box<dyn Error>> {
    assert_eq!(Tuple::new_vector(4., 0., 0.).normalize()?, Tuple::new_vector(1., 0., 0.));
    assert_eq!(Tuple::new_vector(1., 2., 3.).normalize()?, Tuple::new_vector(0.26726, 0.53452, 0.80178));
    Ok(())
}

#[test]
fn test_book_dot_product() {
    let a = Tuple::new_vector(1., 2., 3.);
    let b = Tuple::new_vector(2., 3., 4.);
    assert_eq!(a.dot(&b), 20.)
}

#[test]
fn test_book_cross_product() -> Result<(), Box<dyn Error>> {
    let a = Tuple::new_vector(1., 2., 3.);
    let b = Tuple::new_vector(2., 3., 4.);
    assert_eq!(a.cross(&b)?, Tuple::new_vector(-1.0, 2.0, -1.));
    assert_eq!(b.cross(&a)?, Tuple::new_vector(1.0, -2.0, 1.0));    
    Ok(())
}

#[test]
fn test_cross_product_point_failure() {
    let a = Tuple::new_point(1., 2., 3.);
    let b = Tuple::new_point(2., 3., 4.);
    assert!(a.cross(&b).is_err());
    assert!(b.cross(&a).is_err());    
}

#[test]
fn test_eq_neq_small_numbers() {
    assert_eq!(Tuple::new_vector(0., 0., 0.), Tuple::new_vector(0., -5.2844766e-36, 0.));
    assert_ne!(Tuple::new_vector(0., 0., 0.), Tuple::new_vector(0., -5.2844766e36, 0.));
}

#[test]
fn test_regression_large_number_normalize() {
    let v = Tuple::new_vector(0., -5.2844766e36, 0.);
    let nv = v.normalize();
    assert!(nv.is_err());
}

proptest! {
    #[test]
    fn create_point(x: Float, y: Float, z: Float) {
        let t = Tuple::new_point(x,y,z);
        prop_assert!(t.is_point());
    }

    #[test]
    fn create_vector(x: Float, y: Float, z: Float) {
        let t = Tuple::new_vector(x,y,z);
        prop_assert!(t.is_vector());
    }

    #[test]
    fn normalized_vectors_are_magnitude_1(x: Float, y: Float, z: Float) {
        let v = Tuple::new_vector(x, y, z);
        prop_assume!(v != Tuple::new_vector(0.,0.,0.));
        match v.normalize() {
            Ok(nv) => prop_assert!(feq(&nv.magnitude()?, &1.0), "nv={:?}", nv),
            Err(_) => prop_assert!(true),
        }
    }
}
