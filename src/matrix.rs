use num::Num;
use std::fmt::Display;

use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};
use std::ops::{Deref, DerefMut, Index, IndexMut};

use crate::Vector;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix<T, const M: usize, const N: usize> {
    pub store: [[T; N]; M],
}

impl<T, const M: usize, const N: usize> Matrix<T, M, N>
where
    T: Copy + Default,
{
    pub fn from(data: [[T; N]; M]) -> Self {
        Self { store: data }
    }

    pub const fn size(&self) -> (usize, usize) {
        (M, N)
    }

    pub fn zero() -> Self {
        Self {
            store: [[T::default(); N]; M],
        }
    }
}

impl<T, const M: usize, const N: usize> Matrix<T, M, N>
where
    T: AddAssign + SubAssign + MulAssign + Copy + Default,
{
    pub fn add(&mut self, other: &Self) {
        for (l_row, r_row) in self.store.iter_mut().zip(other.store.iter()) {
            for (l, r) in l_row.iter_mut().zip(r_row.iter()) {
                *l += *r;
            }
        }
    }

    pub fn sub(&mut self, other: &Self) {
        for (l_row, r_row) in self.store.iter_mut().zip(other.store.iter()) {
            for (l, r) in l_row.iter_mut().zip(r_row.iter()) {
                *l -= *r;
            }
        }
    }
    pub fn scl(&mut self, scalar: T) {
        for row in self.store.iter_mut() {
            for elem in row.iter_mut() {
                *elem *= scalar;
            }
        }
    }
}
impl<T, const M: usize, const N: usize> IndexMut<(usize, usize)> for Matrix<T, M, N> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut T {
        &mut self.store[index.0][index.1]
    }
}

impl<T, const M: usize, const N: usize> Index<(usize, usize)> for Matrix<T, M, N> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.store.index(index.0).index(index.1)
    }
}

impl<T, const M: usize, const N: usize> Deref for Matrix<T, M, N> {
    type Target = [T; N];

    fn deref(&self) -> &Self::Target {
        todo!()
    }
}

impl<T, const M: usize, const N: usize> DerefMut for Matrix<T, M, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        todo!()
    }
}

impl<T, const M: usize, const N: usize> Add for Matrix<T, M, N>
where
    T: AddAssign + Copy + Num,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = self.clone();
        for (l_row, r_row) in result.store.iter_mut().zip(rhs.store.iter()) {
            for (l, r) in l_row.iter_mut().zip(r_row.iter()) {
                *l += *r;
            }
        }
        result
    }
}

impl<T, const M: usize, const N: usize> Sub for Matrix<T, M, N>
where
    T: SubAssign + Copy + Num,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = self.clone();
        for (l_row, r_row) in result.store.iter_mut().zip(rhs.store.iter()) {
            for (l, r) in l_row.iter_mut().zip(r_row.iter()) {
                *l -= *r;
            }
        }
        result
    }
}

impl<T, const M: usize, const N: usize> Mul<T> for Matrix<T, M, N>
where
    T: MulAssign + Copy + Num,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let mut result = self.clone();
        for row in result.store.iter_mut() {
            for elem in row.iter_mut() {
                *elem *= rhs;
            }
        }
        result
    }
}

impl<T, const M: usize, const N: usize> Display for Matrix<T, M, N>
where
    T: AddAssign + SubAssign + MulAssign + Copy + std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (i, row) in self.store.iter().enumerate() {
            if i > 0 {
                writeln!(f)?;
            }
            write!(f, "// [")?;
            for (j, val) in row.iter().enumerate() {
                if j > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{:.1}", val)?;
            }
            write!(f, "]")?;
        }
        write!(f, "\n")?;
        Ok(())
    }
}

impl<T, const M: usize, const N: usize> Matrix<T, M, N>
where
    T: Num + Copy + AddAssign + Default,
{
    pub fn mul_vec(&mut self, vec: Vector<T, N>) -> Vector<T, N> {
        let mut result = Vector::zero();
        for (idx, row) in self.store.iter_mut().enumerate() {
            for (e1, e2) in row.iter_mut().zip(vec.store.iter()) {
                result[idx] += *e1 * *e2;
            }
        }
        result
    }

    pub fn mul_mat(&mut self, mat: Matrix<T, M, N>) -> Matrix<T, M, N> {
        let mut result = Matrix::zero();
        for i in 0..M {
            for j in 0..N {
                for k in 0..N {
                    result[(i, j)] += self[(i, k)] * mat[(k, j)];
                }
            }
        }
        result
    }
}

impl<T, const M: usize, const N: usize> Matrix<T, M, N>
where
    T: Num + Copy + AddAssign + Default,
{
    pub fn trace(&mut self) -> T {
        assert!(M == N, "Matrix must be square to calculate trace");

        let mut result = T::default();
        for i in 0..M {
            result += self[(i, i)];
        }
        result
    }
}
