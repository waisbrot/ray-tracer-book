use std::error::Error;
use proptest::{prelude::*, num::f64::{POSITIVE, NEGATIVE,NORMAL}};
use book_renderer::{matrix::Matrix, tuple::Tuple};

#[test]
fn test_book_from_str_matrix_4_4() -> Result<(), Box<dyn Error>> {
    let m = Matrix::from_str(4,4,
    "01,2,3,4
    5.5,6.5,7.5,8.5
    9,10,11,12
    13.5,14.5,15.5,16.5")?;
    assert_eq!(m[(0,0)],1.0);
    assert_eq!(m[(0,3)],4.0);
    assert_eq!(m[(1,0)],5.5);
    assert_eq!(m[(1,2)],7.5);
    assert_eq!(m[(2,2)],11.0);
    assert_eq!(m[(3,0)],13.5);
    assert_eq!(m[(3,2)],15.5);
    Ok(())
}

#[test]
fn test_book_from_str_matrix_2_2() -> Result<(), Box<dyn Error>> {
    let m = Matrix::from_str(2, 2, "
    -3,5
    1,-2
    ")?;
    assert_eq!(m[(0,0)], -3.0);
    assert_eq!(m[(0,1)], 5.0);
    assert_eq!(m[(1,0)], 1.0);
    assert_eq!(m[(1,1)], -2.0);
    Ok(())
}

#[test]
fn test_book_from_str_matrix_3_3() -> Result<(), Box<dyn Error>> {
    let m = Matrix::from_str(3, 3, "
    -3, 5, 0
    1 | -2 | -7
    | 0 | 1 | 1 |
    ")?;
    assert_eq!(m[(0,0)], -3.0);
    assert_eq!(m[(1,1)], -2.0);
    assert_eq!(m[(2,2)], 1.0);
    Ok(())
}

#[test]
fn test_book_equal_matrices() -> Result<(), Box<dyn Error>> {
    let m1 = Matrix::from_str(4, 4, "
    | 1 | 2 | 3 | 4 |
    | 5 | 6 | 7 | 8 |
    | 9 | 8 | 7 | 6 |
    | 5 | 4 | 3 | 2 |
    ")?;
    let m2 = Matrix::from_str(4, 4, "
    1 | 2 | 3 | 4
    5 | 6 | 7 | 8
    9 | 8 | 7 | 6
    5 | 4 | 3 | 2
    ")?;
    assert!(m1 == m2);
    Ok(())
}

#[test]
fn test_book_unequal_matrices() -> Result<(), Box<dyn Error>> {
    let m1 = Matrix::from_str(4, 4, "
    | 1 | 2 | 3 | 4 |
    | 5 | 6 | 7 | 8 |
    | 9 | 8 | 7 | 6 |
    | 5 | 4 | 3 | 2 |
    ")?;
    let m2 = Matrix::from_str(4, 4, "
    5 | 6 | 7 | 8
    1 | 2 | 3 | 4
    9 | 8 | 7 | 6
    5 | 4 | 3 | 2
    ")?;
    assert!(m1 != m2);
    Ok(())
}

#[test]
fn test_book_matrix_mult() -> Result<(), Box<dyn Error>> {
    let m1 = Matrix::from_str(4, 4, "
    | 1 | 2 | 3 | 4 |
    | 5 | 6 | 7 | 8 |
    | 9 | 8 | 7 | 6 |
    | 5 | 4 | 3 | 2 |
    ")?;
    let m2 = Matrix::from_str(4, 4, "
    | -2 | 1 | 2 |  3 |
    |  3 | 2 | 1 | -1 |
    |  4 | 3 | 6 |  5 |
    |  1 | 2 | 7 |  8 |
    ")?;
    let res = Matrix::from_str(4, 4, "
    | 20 | 22 |  50 |  48 |
    | 44 | 54 | 114 | 108 |
    | 40 | 58 | 110 | 102 |
    | 16 | 26 |  46 |  42 |
    ")?;
    assert_eq!(m1 * m2, res);
    Ok(())
}

#[test]
fn test_book_matrix_tuple_mult() -> Result<(), Box<dyn Error>> {
    let m = Matrix::from_str(4, 4, "
    1 | 2 | 3 | 4
    2 | 4 | 4 | 2
    8 | 6 | 4 | 1
    0 | 0 | 0 | 1
    ")?;
    let t = Tuple::new_point(1.0, 2.0, 3.0);
    let res = Tuple::new_point(18.0, 24.0, 33.0);
    assert_eq!(m * t, res);
    Ok(())
}

#[test]
fn test_book_identity_matrix_matrix() -> Result<(), Box<dyn Error>> {
    let m = Matrix::from_str(4, 4, "
    0 | 1 |  2 | 4
    1 | 2 |  4 | 6
    2 | 4 |  0 | 16
    4 | 8 | 16 | 32
    ")?;
    assert_eq!(Matrix::identity(4) * &m, m);
    assert_eq!(&m * Matrix::identity(4), m);
    Ok(())
}

#[test]
fn test_book_identity_matrix_tuple() -> Result<(), Box<dyn Error>> {
    let t = Tuple { x: 1.0, y: 2.0, z: 3.0, w: 4.0 };
    assert_eq!(Matrix::identity(4) * &t, t);
    assert_eq!(&t * Matrix::identity(4), t);
    Ok(())
}

#[test]
fn test_book_transpose() -> Result<(), Box<dyn Error>> {
    let m1 = Matrix::from_str(4, 4, "
    0 | 9 | 3 | 0
    9 | 8 | 0 | 8
    1 | 8 | 5 | 3
    0 | 0 | 5 | 8
    ")?;
    let m2 = Matrix::from_str(4, 4, "
    0 | 9 | 1 | 0
    9 | 8 | 8 | 0
    3 | 0 | 5 | 5
    0 | 8 | 3 | 8
    ")?;
    assert_eq!(m1.transpose(), m2);
    assert_eq!(m2.transpose(), m1);
    Ok(())
}

#[test]
fn test_book_transpose_identity() -> Result<(), Box<dyn Error>> {
    assert_eq!(Matrix::identity(4).transpose(), Matrix::identity(4));
    Ok(())
}

#[test]
fn test_book_det_2() -> Result<(), Box<dyn Error>> {
    let m = Matrix::from_str(2, 2, "
     1 | 5
    -3 | 2
    ")?;
    assert_eq!(m.determinant(), 17.0);
    Ok(())
}

#[test]
fn test_book_submatrix_3() -> Result<(), Box<dyn Error>> {
    let m = Matrix::from_str(3, 3, "
     1 | 5 |  0
    -3 | 2 |  7
     0 | 6 | -3
    ")?;
    let sub = Matrix::from_str(2, 2, "
    -3 | 2
     0 | 6
    ")?;
    assert_eq!(m.submatrix(0, 2), sub);
    Ok(())
}

#[test]
fn test_book_submatrix_4() -> Result<(), Box<dyn Error>> {
    let m = Matrix::from_str(4, 4, "
    -6 | 1 |  1 | 6
    -8 | 5 |  8 | 6
    -1 | 0 |  8 | 2
    -7 | 1 | -1 | 1
    ")?;
    let sub = Matrix::from_str(3, 3, "
    -6 |  1 | 6
    -8 |  8 | 6
    -7 | -1 | 1
    ")?;
    assert_eq!(m.submatrix(2, 1), sub);
    Ok(())
}

#[test]
fn test_book_minor_3() -> Result<(), Box<dyn Error>> {
    let m = Matrix::from_str(3, 3, "
    3 |  5 |  0
    2 | -1 | -7
    6 | -1 |  5
    ")?;
    assert_eq!(m.submatrix(1, 0).determinant(), 25.0);
    assert_eq!(m.minor(1, 0), 25.0);
    Ok(())
}

#[test]
fn test_book_cofactor_3() -> Result<(), Box<dyn Error>> {
    let m = Matrix::from_str(3, 3, "
    3 |  5 |  0
    2 | -1 | -7
    6 | -1 |  5
    ")?;
    assert_eq!(m.minor(0, 0), -12.0);
    assert_eq!(m.cofactor(0, 0), -12.0);
    assert_eq!(m.minor(1, 0), 25.0);
    assert_eq!(m.cofactor(1, 0), -25.0);
    Ok(())
}

#[test]
fn test_book_determinant_3() -> Result<(), Box<dyn Error>> {
    let m = Matrix::from_str(3, 3, "
     1 | 2 |  6
    -5 | 8 | -4
     2 | 6 |  4
    ")?;
    assert_eq!(m.cofactor(0, 0), 56.0);
    assert_eq!(m.cofactor(0, 1), 12.0);
    assert_eq!(m.cofactor(0, 2), -46.0);
    assert_eq!(m.determinant(), -196.0);
    Ok(())
}

#[test]
fn test_book_determinant_4() -> Result<(), Box<dyn Error>> {
    let m = Matrix::from_str(4, 4, "
    -2 | -8 |  3 |  5
    -3 |  1 |  7 |  3
     1 |  2 | -9 |  6
    -6 |  7 |  7 | -9
    ")?;
    assert_eq!(m.cofactor(0, 0), 690.0);
    assert_eq!(m.cofactor(0, 1), 447.0);
    assert_eq!(m.cofactor(0, 2), 210.0);
    assert_eq!(m.cofactor(0, 3), 51.0);
    assert_eq!(m.determinant(), -4071.0);
    Ok(())
}

#[test]
fn test_book_invertable_success() -> Result<(), Box<dyn Error>> {
    let m = Matrix::from_str(4, 4, "
    6 |  4 | 4 |  4
    5 |  5 | 7 |  6
    4 | -9 | 3 | -7
    9 |  1 | 7 | -6
    ")?;
    assert_eq!(m.determinant(), -2120.0);
    assert!(m.is_invertible());
    Ok(())
}

#[test]
fn test_book_invertable_failure() -> Result<(), Box<dyn Error>> {
    let m = Matrix::from_str(4, 4, "
    -4 |  2 | -2 | -3
     9 |  6 |  2 |  6
     0 | -5 |  1 | -5
     0 |  0 |  0 |  0
    ")?;
    assert_eq!(m.determinant(), 0.0);
    assert!(!m.is_invertible());
    Ok(())
}

#[test]
fn test_book_inverse_4() -> Result<(), Box<dyn Error>> {
    let a = Matrix::from_str(4, 4, "
    -5 |  2 |  6 | -8
     1 | -5 |  1 |  8
     7 |  7 | -6 | -7
     1 | -3 |  7 |  4
    ")?;
    let b = a.inverse()?;

    assert_eq!(a.determinant(), 532.0);
    assert_eq!(a.cofactor(2, 3), -160.0);
    assert_eq!(b[(3,2)], -160.0/532.0);
    assert_eq!(a.cofactor(3, 2), 105.0);
    assert_eq!(b[(2,3)], 105.0/532.0);
    assert_eq!(b, Matrix::from_str(4, 4, "
     0.21805 |  0.45113 |  0.24060 | -0.04511
    -0.80827 | -1.45677 | -0.44361 |  0.52068
    -0.07895 | -0.22368 | -0.05263 |  0.19737
    -0.52256 | -0.81391 | -0.30075 |  0.30639
    ")?);
    Ok(())
}

#[test]
fn test_book_inverse_4b() -> Result<(), Box<dyn Error>> {
    let a = Matrix::from_str(4, 4, "
     8 | -5 |  9 |  2
     7 |  5 |  6 |  1
    -6 |  0 |  9 |  6
    -3 |  0 | -9 | -4
    ")?;
    assert_eq!(a.inverse()?, Matrix::from_str(4, 4, "
    -0.15385 | -0.15385 | -0.28205 | -0.53846
    -0.07692 |  0.12308 |  0.02564 |  0.03077
     0.35897 |  0.35897 |  0.43590 |  0.92308
    -0.69231 | -0.69231 | -0.76923 | -1.92308
    ")?);
    Ok(())
}

#[test]
fn test_book_inverse_4c() -> Result<(), Box<dyn Error>> {
    let a = Matrix::from_str(4, 4, "
     9 |  3 |  0 |  9
    -5 | -2 | -6 | -3
    -4 |  9 |  6 |  4
    -7 |  6 |  6 |  2
    ")?;
    assert_eq!(a.inverse()?, Matrix::from_str(4, 4, "
    -0.04074 | -0.07778 |  0.14444 | -0.22222
    -0.07778 |  0.03333 |  0.36667 | -0.33333
    -0.02901 | -0.14630 | -0.10926 |  0.12963
     0.17778 |  0.06667 | -0.26667 |  0.33333
    ")?);
    Ok(())
}

#[test]
fn test_book_inverse_mult_property() -> Result<(), Box<dyn Error>> {
    let a = Matrix::from_str(4, 4, "
     3 | -9 |  7 |  3
     3 | -8 |  2 | -9
    -4 |  4 |  4 |  1
    -6 |  5 | -1 |  1
    ")?;
    let b = Matrix::from_str(4, 4, "
    8 |  2 | 2 | 2
    3 | -1 | 7 | 0
    7 |  0 | 5 | 4
    6 | -2 | 0 | 5
    ")?;
    let c = &a * &b;
    assert_eq!(c * b.inverse()?, a);
    Ok(())
}

prop_compose! {
    fn matrix_strategy(n: usize)(values in [[NORMAL|POSITIVE|NEGATIVE;4];4]) -> Matrix {
        if n > 4 {
            panic!("Can't make more than a 4x4 matrix");
        }
        let mut m = Matrix::new(n, n);
        for r in 0..n {
            for c in 0..n {
                m[(r,c)] = values[r][c];
            }
        }
        m
    }
}
prop_compose! {
    fn finite_matrix_strategy(n: usize)(values in [[NORMAL|POSITIVE|NEGATIVE;4];4]) -> Matrix {
        if n > 4 {
            panic!("Can't make more than a 4x4 matrix");
        }
        let mut m = Matrix::new(n, n);
        for r in 0..n {
            for c in 0..n {
                m[(r,c)] = values[r][c].clamp(-1e5, 1e5);
            }
        }
        m
    }
}
proptest! {

    #[test]
    fn identity_multiplier(m in matrix_strategy(4)) {
        let id = Matrix::identity(4);
        prop_assert_eq!(&m * id, m);
    }

    #[test]
    fn inverse_multiplier(a in finite_matrix_strategy(2), b in finite_matrix_strategy(2).prop_filter("B must be invertable", |m| m.is_invertible())) {
        for row in 0..2 {
            for col in 0..2 {
                let submatrix = b.submatrix(row, col);
                let bri = if row == 0 {1} else {0};
                let bci = if col == 0 {1} else {0};
                prop_assert!(submatrix[(0,0)] == b[(bri,bci)]);
                prop_assert_eq!(submatrix.height(), 1);
                prop_assert_eq!(submatrix.width(), 1);
                let cofactor = b.cofactor(row, col);
                prop_assert!(cofactor > 0.0 || cofactor < 0.0);
                prop_assert!(cofactor.is_finite());
            }
        }
        let c = &a * &b;
        prop_assume!(c.is_invertible());
        let inv = b.inverse()?;
        let d = &c * &inv;
        prop_assume!(!d.contains_nan());
        prop_assert_eq!(d, a, "c={:?} inv={:?}", c, inv);
    }
}
