use num::Num;
use std::fmt::{Debug, Display};

use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub, SubAssign};
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

    #[allow(dead_code)]
    fn map<F>(&self, mut f: F) -> Matrix<T, M, N>
    where
        F: FnMut(T) -> T,
    {
        let mut result = Matrix::<T, M, N>::zero();
        for i in 0..M {
            for j in 0..N {
                result[(i, j)] = f(self[(i, j)]);
            }
        }
        result
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

impl<T, const M: usize, const N: usize> Mul<Vector<T, N>> for Matrix<T, M, N>
where
    T: MulAssign + AddAssign + Copy + Num + Default,
{
    type Output = Vector<T, M>;

    fn mul(self, rhs: Vector<T, N>) -> Self::Output {
        let mut result = Vector::zero();
        for (idx, row) in self.store.iter().enumerate() {
            for (e1, e2) in row.iter().zip(rhs.store.iter()) {
                result.store[idx] += *e1 * *e2;
            }
        }
        result
    }
}

impl<T, const M: usize, const N: usize> Mul<Matrix<T, N, N>> for Matrix<T, M, N>
where
    T: MulAssign + AddAssign + Copy + Num + Default,
{
    type Output = Self;

    fn mul(self, rhs: Matrix<T, N, N>) -> Self::Output {
        let mut result = Matrix::zero();
        for (i, row) in self.store.iter().enumerate() {
            for (j, col) in rhs.store.iter().enumerate() {
                for (e1, e2) in row.iter().zip(col.iter()) {
                    result.store[i][j] += *e1 * *e2;
                }
            }
        }
        result
    }
}

impl<T, const M: usize, const N: usize> Neg for Matrix<T, M, N>
where
    T: Neg<Output = T> + Copy,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        let mut result = self.clone();
        for row in result.store.iter_mut() {
            for elem in row.iter_mut() {
                *elem = -*elem;
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

impl<T, const M: usize, const N: usize> Matrix<T, M, N>
where
    T: Copy + Default,
{
    pub fn transpose(&mut self) -> Matrix<T, M, N> {
        let mut result = Matrix::zero();
        for i in 0..M {
            for j in 0..N {
                result[(j, i)] = self[(i, j)];
            }
        }
        result
    }
}

impl<T, const M: usize, const N: usize> Matrix<T, M, N>
where
    T: Copy + Default,
{
    pub fn identity() -> Matrix<T, M, N> {
        let mut result = Matrix::zero();
        for i in 0..M {
            result[(i, i)] = T::default();
        }
        result
    }
}

impl<T, const M: usize, const N: usize> Matrix<T, M, N>
where
    T: Copy + Default + Mul + PartialEq + Num + Div<Output = T> + Sub<Output = T>,
{
    pub fn row_echelon(&self) -> Matrix<T, M, N> {
        let mut result = self.clone();
        // let mut matrix_out = result.store;
        let mut pivot = 0;
        let row_count = M;
        let column_count = N;

        'outer: for r in 0..row_count {
            if column_count <= pivot {
                break;
            }
            let mut i = r;
            while result[(i, pivot)] == T::default() {
                i = i + 1;
                if i == row_count {
                    i = r;
                    pivot = pivot + 1;
                    if column_count == pivot {
                        break 'outer;
                    }
                }
            }
            for j in 0..row_count {
                let temp = result[(r, j)];
                result[(r, j)] = result[(i, j)];
                result[(i, j)] = temp;
            }
            let divisor = result[(r, pivot)];
            if divisor != T::default() {
                for j in 0..column_count {
                    result[(r, j)] = result[(r, j)] / divisor;
                }
            }
            for j in 0..row_count {
                if j != r {
                    let hold = result[(j, pivot)];
                    for k in 0..column_count {
                        result[(j, k)] = result[(j, k)] - (hold * result[(r, k)]);
                    }
                }
            }
            pivot = pivot + 1;
        }
        result
    }
}

impl<T, const M: usize, const N: usize> Matrix<T, M, N>
where
    T: Copy + Default + Mul + Num + Neg<Output = T> + AddAssign,
{
    pub fn determinant(&self) -> T {
        match M {
            1 => self[(0, 0)],
            2 => self[(0, 0)] * self[(1, 1)] - self[(0, 1)] * self[(1, 0)],
            3 => self.determinant_3x3(),
            4 => (0..4)
                .map(|i| {
                    let sign = if i % 2 == 0 { T::one() } else { -T::one() };
                    let cofactor = self.get_cofactor(0, i);
                    sign * self[(0, i)] * cofactor.determinant_3x3()
                })
                .fold(T::default(), |acc, x| acc + x),
            _ => panic!("Determinant not implemented for matrices larger than 4x4"),
        }
    }

    fn determinant_3x3(&self) -> T {
        self[(0, 0)] * (self[(1, 1)] * self[(2, 2)] - self[(1, 2)] * self[(2, 1)])
            - self[(0, 1)] * (self[(1, 0)] * self[(2, 2)] - self[(1, 2)] * self[(2, 0)])
            + self[(0, 2)] * (self[(1, 0)] * self[(2, 1)] - self[(1, 1)] * self[(2, 0)])
    }

    fn get_cofactor(&self, row: usize, col: usize) -> Matrix<T, M, N> {
        let mut cofactor_matrix = Matrix::<T, M, N>::zero();
        let mut row_index = 0;

        for r in 0..M {
            if r == row {
                continue;
            }
            let mut col_index = 0;
            for c in 0..N {
                if c == col {
                    continue;
                }
                cofactor_matrix[(row_index, col_index)] = self[(r, c)];
                col_index += 1;
            }
            row_index += 1;
        }

        cofactor_matrix
    }
}

impl<T, const M: usize, const N: usize> Matrix<T, M, N>
where
    T: Copy + Default + Mul + Num + Neg<Output = T> + AddAssign + Debug,
{
    pub fn inverse(&self) -> Result<Matrix<T, M, N>, &'static str> {
        if M != N {
            return Err("Matrix must be square to calculate inverse");
        }

        let det = self.determinant();

        if det == T::zero() {
            return Err("Matrix is singular and has no inverse");
        }

        let inv = Matrix::<T, M, N>::zero();
        // for i in 0..M {
        //     for j in 0..N {
        //         let sign = if (i + j) % 2 == 0 { T::one() } else { -T::one() };
        //         let cofactor = self.get_cofactor(i, j);
        //         result[(j, i)] = sign * cofactor.determinant_3x3() / det;
        //     }
        // }
        Ok(inv)
    }
}
