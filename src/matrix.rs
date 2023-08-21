use std::{ops::{IndexMut, Index}, error::Error, num::ParseFloatError};
use auto_ops::impl_op_ex;
use regex::Regex;
use array2d::Array2D;
use crate::{tuple::Tuple, util::feq};

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
    n: Array2D<f32>,
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
                array[(r,c)] = n.trim().parse::<f32>().or_else(|e| Err(MatrixParseError{token: n.to_string(), err: e}))?;
            }
        }
        Ok(Matrix{n:array})
    }
    pub fn identity(size: usize) -> Matrix {
        let mut m = Matrix::new(size, size);
        for i in 0..size {
            m[(i,i)] = 1.0;
        }
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

    pub fn determinant(&self) -> f32 {
        self.assert_square();
        match self.n.num_columns() {
            2 => 
                self[(0,0)] * self[(1,1)] - self[(0,1)] * self[(1,0)],
            n => {
                let mut d:f32 = 0.0; 
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
        self.determinant() != 0.0
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

    pub fn minor(&self, r: usize, c: usize) -> f32 {
        self.assert_square();
        self.submatrix(r, c).determinant()
    }

    pub fn cofactor(&self, r: usize, c: usize) -> f32 {
        let sign: f32 = if (r + c) % 2 == 0 { 1.0 } else { -1.0 };
        self.minor(r, c) * sign
    }
}

impl Index<(usize,usize)> for Matrix{
    type Output=f32;

    fn index(&self, index: (usize,usize)) -> &Self::Output {
        &self.n[index]
    }
}
impl IndexMut<(usize,usize)> for Matrix {
    fn index_mut(&mut self, index: (usize,usize)) -> &mut f32 {
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
    let sum_row = |r:usize| -> f32 {
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