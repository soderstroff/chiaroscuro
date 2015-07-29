use std::ops::{Index, IndexMut, Add, Sub, Mul, Neg};
use ::super::vector::Vector;

pub struct Matrix<T: Copy>{elts: Vec<T>, rows: usize, cols: usize}

impl<T: Copy> Matrix<T> {
    fn new(r: usize, c: usize) -> Matrix<T> {
        Matrix { elts: Vec::new(), rows: r, cols: c }
    }

    fn cols(&self) -> usize {
        self.cols
    }

    fn rows(&self) -> usize {
        self.rows
    }
}

/// Matrix indexing
impl<T: Copy> Index<(usize,usize)> for Matrix<T> {
    type Output = T;
    fn index(&self, (r,c): (usize,usize)) -> &T {
        &self.elts[r + c * self.rows]
    }
}

/// Mutable matrix indexing
impl<T: Copy> IndexMut<(usize,usize)> for Matrix<T> {
    fn index_mut(&mut self, (r,c): (usize,usize)) -> &mut T {
        &mut self.elts[r + c*self.rows]
    }
}

/// Matrix addition
impl <T: Copy + Add<Output = T>> Add<Matrix<T>> for Matrix<T> {
    type Output = Matrix<T>;
    fn add(mut self, rhs: Matrix<T>) -> Matrix<T> {
        let (c, r) = (self.cols(), self.rows());
        assert!(c == rhs.cols() && r == rhs.rows());

        for j in 0..c {
            for i in 0..r {
                self[(i,j)] = self[(i,j)] + rhs[(i,j)];
            }
        }
        self
    }
}

/// Matrix subtraction
impl<T: Copy + Sub<Output = T>> Sub<Matrix<T>> for Matrix<T> {
    type Output = Matrix<T>;
    fn sub(mut self, rhs: Matrix<T>) -> Matrix<T> {
        let (c, r) = (self.cols(), self.rows());
        assert!(c == rhs.cols() && r == rhs.rows());

        for j in 0..c {
            for i in 0..r {
                self[(i,j)] = self[(i,j)] - rhs[(i,j)];
            }
        }
        self
    }
}

/// Matrix negation
impl<T: Copy + Neg<Output = T>> Neg for Matrix<T> {
    type Output = Self;
    fn neg(mut self) -> Matrix<T> {
        let (c, r) = (self.cols(), self.rows());
        for j in 0..c {
            for i in 0..r {
                self[(i,j)] = -self[(i,j)];
            }
        }
        self
    }
}

/// Scalar multiplication of a matrix
impl<T: Copy + Mul<Output = T>> Mul<T> for Matrix<T> {
    type Output = Matrix<T>;
    fn mul(mut self, rhs: T) -> Matrix<T> {
        let(c,r) = (self.cols(), self.rows());
        for j in 0..c {
            for i in 0..r {
                self[(i,j)] = self[(i,j)] * rhs;
            }
        }
        self
    }
}
