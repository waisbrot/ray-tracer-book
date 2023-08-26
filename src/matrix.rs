use std::{ops::{IndexMut, Index}, error::Error, num::ParseFloatError};
use auto_ops::impl_op_ex;
use regex::Regex;
use array2d::Array2D;
use crate::{tuple::Tuple, util::{feq, Float}};

#[derive(Debug)]
pub struct NotInvertibleError {}
impl Error for NotInvertibleError {}
impl std::fmt::Display for NotInvertibleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Matrix is not invertible")
    }
}

#[derive(Debug)]
pub struct MatrixParseError { token: String, err: ParseFloatError }
impl Error for MatrixParseError {}
impl std::fmt::Display for MatrixParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unable to parse matrix. Failed at token {} with error {}", self.token, self.err)
    }
}


#[derive(Debug)]
pub struct Matrix {
    n: Array2D<Float>,
}

impl Matrix {
    pub fn new(height:usize, width:usize) -> Matrix {
        Matrix { n: Array2D::filled_with(0.0, height, width) }
    }
    pub fn from_str(height:usize, width:usize, contents:&str) -> Result<Matrix, MatrixParseError> {
        let re = Regex::new(r"[^\d.\-]+").unwrap();
        let mut array = Array2D::filled_with(0.0,height,width);
        for (r,line) in contents.split('\n').map(|l| l.trim()).filter(|l| ! l.is_empty()).enumerate() {
            for (c,n) in re.split(line).map(|w| w.trim()).filter(|w| ! w.is_empty()).enumerate() {
                array[(r,c)] = n.trim().parse::<Float>().or_else(|e| Err(MatrixParseError{token: n.to_string(), err: e}))?;
            }
        }
        Ok(Matrix{n:array})
    }
    pub fn identity(size: usize) -> Self {
        let mut m = Self::new(size, size);
        for i in 0..size {
            m[(i,i)] = 1.0;
        }
        m
    }
    pub fn translation(x: Float, y: Float, z: Float) -> Self {
        let mut m = Self::identity(4);
        m[(0,3)] = x;
        m[(1,3)] = y;
        m[(2,3)] = z;
        m
    }
    pub fn scaling(x: Float, y: Float, z: Float) -> Self {
        let mut m = Self::identity(4);
        m[(0,0)] = x;
        m[(1,1)] = y;
        m[(2,2)] = z;
        m
    }
    pub fn rotation_x(radians: Float) -> Self {
        let mut m = Self::identity(4);
        m[(1,1)] = radians.cos();
        m[(2,1)] = radians.sin();
        m[(1,2)] = -m[(2,1)];
        m[(2,2)] = m[(1,1)];
        m
    }
    pub fn rotation_y(radians: Float) -> Self {
        let mut m = Self::identity(4);
        m[(0,0)] = radians.cos();
        m[(0,2)] = radians.sin();
        m[(2,0)] = -m[(0,2)];
        m[(2,2)] = m[(0,0)];
        m
    }
    pub fn rotation_z(radians: Float) -> Self {
        let mut m = Self::identity(4);
        m[(0,0)] = radians.cos();
        m[(1,0)] = radians.sin();
        m[(0,1)] = -m[(1,0)];
        m[(1,1)] = m[(0,0)];
        m
    }
    pub fn shear(xy: Float, xz: Float, yx: Float, yz: Float, zx: Float, zy: Float) -> Self {
        let mut m = Self::identity(4);
        m[(0,1)] = xy;
        m[(0,2)] = xz;
        m[(1,0)] = yx;
        m[(1,2)] = yz;
        m[(2,0)] = zx;
        m[(2,1)] = zy;
        m
    }


    pub fn is_square(&self) -> bool {
        self.height() == self.width()
    }

    fn assert_square(&self) -> () {
        if ! self.is_square() {
            panic!("Matrix is not square: {} x {}", self.n.num_columns(), self.n.num_rows());
        }
    }

    pub fn transpose(&self) -> Matrix {
        self.assert_square();
        let mut m = Matrix::new(self.height(), self.width());
        for r in 0..self.height() {
            for c in 0..self.width() {
                m[(r,c)] = self[(c,r)];
            }
        }
        m
    }

    pub fn determinant(&self) -> Float {
        self.assert_square();
        match self.n.num_columns() {
            1 =>
                self[(0,0)],
            2 => 
                self[(0,0)] * self[(1,1)] - self[(0,1)] * self[(1,0)],
            n => {
                let mut d:Float = 0.0; 
                for c in 0..n {
                    d += self[(0,c)] * self.cofactor(0, c);
                }
                d
            }
        }
    }

    pub fn height(&self) -> usize {
        self.n.num_rows()
    }

    pub fn width(&self) -> usize {
        self.n.num_columns()
    }

    pub fn contains_nan(&self) -> bool {
        self.n.elements_row_major_iter().any(|f| f.is_nan())
    }

    pub fn is_invertible(&self) -> bool {
        let det = self.determinant();
        det != 0.0 && det.is_finite()
    }

    pub fn inverse(&self) -> Result<Matrix, NotInvertibleError> {
        if !self.is_invertible() {
            return Err(NotInvertibleError{})
        }
        let det = self.determinant();
        let mut m = Matrix::new(self.height(), self.width());
        for r in 0..self.height() {
            for c in 0..self.width() {
                let cf = self.cofactor(r, c);
                m[(c,r)] = cf / det;  // transposing by swapping r,c -> c,r
            }
        }
        Ok(m)
    }

    pub fn submatrix(&self, r: usize, c: usize) -> Matrix {
        self.assert_square();
        let size = self.n.num_columns();
        let mut m = Matrix::new(size - 1, size - 1);
        let mut rt:usize = 0;
        for ri in 0..size {
            if ri == r {
                continue;
            }
            let mut ct:usize = 0;
            for ci in 0..size {
                if ci == c {
                    continue;
                }
                m[(rt,ct)] = self[(ri,ci)];
                ct += 1;
            }
            rt += 1;
        }
        m
    }

    pub fn minor(&self, r: usize, c: usize) -> Float {
        self.assert_square();
        self.submatrix(r, c).determinant()
    }

    pub fn cofactor(&self, r: usize, c: usize) -> Float {
        let sign: Float = if (r + c) % 2 == 0 { 1.0 } else { -1.0 };
        self.minor(r, c) * sign
    }
}

impl Index<(usize,usize)> for Matrix{
    type Output=Float;

    fn index(&self, index: (usize,usize)) -> &Self::Output {
        &self.n[index]
    }
}
impl IndexMut<(usize,usize)> for Matrix {
    fn index_mut(&mut self, index: (usize,usize)) -> &mut Float {
    &mut self.n[index]
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        self.n.elements_row_major_iter()
            .zip(other.n.elements_row_major_iter())
            .all(|(a,b)| { feq(a, b) })
    }
}

impl_op_ex!(* |a: &Matrix, b: &Matrix| -> Matrix {
    if a.width() != b.width() {
        panic!("Number of columns does not match: {} != {}", a.width(), b.width());
    }
    if a.height() != b.height() {
        panic!("Number of rows does not match: {} != {}", a.height(), b.height());
    }
    let rows = a.height();
    let cols = a.width();
    let mut product = Matrix::new(rows, cols);
    for r in 0..rows {
        for c in 0..cols {
            let v = a.n.row_iter(r).unwrap()
                        .zip(b.n.column_iter(c).unwrap())
                        .map(|(ar,bc)| ar * bc)
                        .reduce(|acc, f| acc + f)
                        .unwrap();
            product[(r,c)] = v;
        }
    }
    product
});

impl_op_ex!(* |a: &Matrix, b: &Tuple| -> Tuple {
    if a.height() != 4 {
        panic!("Expected 4 rows in the matrix to multiply by the tuple but found {}", a.height());
    }
    let sum_row = |r:usize| -> Float {
        a.n.row_iter(r).unwrap()
            .zip(&[b.x, b.y, b.z, b.w])
            .map(|(ar,bc)| {ar * bc})
            .reduce(|acc, f| acc + f)
            .unwrap()
    };
    Tuple {x: sum_row(0), y: sum_row(1), z: sum_row(2), w: sum_row(3) }
});

impl_op_ex!(* |a: &Tuple, b: &Matrix| -> Tuple {
    b * a
});