use std::ops::{Index, IndexMut, Add, Sub, Mul, Neg};

pub struct Vector<T>(pub Vec<T>);

/// Vector addition
impl<T: Copy + Add<Output=T>> Add<Vector<T>> for Vector<T> {
    type Output = Vector<T>;
    fn add(mut self, rhs: Vector<T>) -> Vector<T> {
        let l = self.0.len();
        assert_eq!(l, rhs.0.len());

        for i in 0..l {
            self.0[i] = self.0[i] + rhs.0[i];
        }
        self
    }
}

/// Vector subtraction
impl<T: Copy + Sub<Output=T>> Sub<Vector<T>> for Vector<T> {
    type Output = Vector<T>;
    fn sub(mut self, rhs: Vector<T>) -> Vector<T> {
        let l = self.0.len();
        assert_eq!(l, rhs.0.len());

        for i in 0..l {
            self.0[i] = self.0[i] - rhs.0[i];
        }
        self
    }
}

/// Vector dot product
impl<T: Copy + Default + Mul<Output=T> + Add <Output=T>> Mul<Vector<T>> for Vector<T> {
    type Output = T;
    fn mul(self, rhs: Vector<T>) -> T {
        let l = self.0.len();
        assert_eq!(l, rhs.0.len());
        let mut accum:T = Default::default();
        for i in 0..l {
            accum = accum + self.0[i] * rhs.0[i];
        }
        accum
    }
}

/// Scalar multiplication of a vector
impl<T:Copy + Mul<Output = T>> Mul<T> for Vector<T> {
    type Output = Vector<T>;
    fn mul(mut self, rhs: T) -> Vector<T> {
        for i in 0..self.0.len() {
            self.0[i] = self.0[i] * rhs;
        }
        self
    }
}

/// Vector negation
impl<T: Neg<Output=T> + Copy> Neg for Vector<T> {
    type Output = Self;
    fn neg(mut self) -> Self {
        for i in 0..self.0.len() {
            self.0[i] = -self.0[i];
        }
        self
    }
}


/// Vector indexing
impl<T> Index<usize> for Vector<T> {
    type Output = T;
    fn index<'a>(&'a self, index: usize) -> &T {
        &self.0[index]
    }
}

/// Mutable vector indexing
impl<T> IndexMut<usize> for Vector<T> {
    fn index_mut<'a>(&'a mut self, index: usize) -> &mut T {
        &mut self.0[index]
    }
}
