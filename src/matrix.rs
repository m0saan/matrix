//! # mini_matrix
//!
//! A mini linear algebra library implemented in Rust.

use num::{Float, Num};
use std::fmt::{Debug, Display};

use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub, SubAssign};
use std::ops::{Deref, DerefMut, Index, IndexMut};

use crate::Vector;

/// A generic matrix type with `M` rows and `N` columns.
///
/// # Examples
///
/// ```
/// use mini_matrix::Matrix;
///
/// let matrix = Matrix::<f64, 2, 2>::from([[1.0, 2.0], [3.0, 4.0]]);
/// assert_eq!(matrix.size(), (2, 2));
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix<T, const M: usize, const N: usize> {
    /// The underlying storage for the matrix elements.
    pub store: [[T; N]; M],
}

impl<T, const M: usize, const N: usize> Matrix<T, M, N>
where
    T: Copy + Default,
{
    /// Creates a new `Matrix` from the given 2D array.
    ///
    /// # Examples
    ///
    /// ```
    /// use mini_matrix::Matrix;
    ///
    /// let matrix = Matrix::<i32, 2, 3>::from([[1, 2, 3], [4, 5, 6]]);
    /// ```
    pub fn from(data: [[T; N]; M]) -> Self {
        Self { store: data }
    }

    /// Returns the dimensions of the matrix as a tuple `(rows, columns)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use mini_matrix::Matrix;
    ///
    /// let matrix = Matrix::<f64, 3, 4>::zero();
    /// assert_eq!(matrix.size(), (3, 4));
    /// ```
    pub const fn size(&self) -> (usize, usize) {
        (M, N)
    }

    /// Creates a new `Matrix` with all elements set to the default value of type `T`.
    ///
    /// # Examples
    ///
    /// ```
    /// use mini_matrix::Matrix;
    ///
    /// let matrix = Matrix::<f64, 2, 2>::zero();
    /// assert_eq!(matrix.store, [[0.0, 0.0], [0.0, 0.0]]);
    /// ```
    pub fn zero() -> Self {
        Self {
            store: [[T::default(); N]; M],
        }
    }

    pub fn from_vecs(vecs: Vec<Vec<T>>) -> Self {
        let mut store = [[T::default(); N]; M];
        for (i, vec) in vecs.iter().enumerate() {
            for (j, elem) in vec.iter().enumerate() {
                store[i][j] = *elem;
            }
        }
        Self { store }
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
    /// Adds another matrix to this matrix in-place.
    ///
    /// # Examples
    ///
    /// ```
    /// use mini_matrix::Matrix;
    ///
    /// let mut a = Matrix::<i32, 2, 2>::from([[1, 2], [3, 4]]);
    /// let b = Matrix::<i32, 2, 2>::from([[5, 6], [7, 8]]);
    /// a.add(&b);
    /// assert_eq!(a.store, [[6, 8], [10, 12]]);
    /// ```
    pub fn add(&mut self, other: &Self) {
        for (l_row, r_row) in self.store.iter_mut().zip(other.store.iter()) {
            for (l, r) in l_row.iter_mut().zip(r_row.iter()) {
                *l += *r;
            }
        }
    }

    /// Subtracts another matrix from this matrix in-place.
    ///
    /// # Examples
    ///
    /// ```
    /// use mini_matrix::Matrix;
    ///
    /// let mut a = Matrix::<i32, 2, 2>::from([[5, 6], [7, 8]]);
    /// let b = Matrix::<i32, 2, 2>::from([[1, 2], [3, 4]]);
    /// a.sub(&b);
    /// assert_eq!(a.store, [[4, 4], [4, 4]]);
    /// ```
    pub fn sub(&mut self, other: &Self) {
        for (l_row, r_row) in self.store.iter_mut().zip(other.store.iter()) {
            for (l, r) in l_row.iter_mut().zip(r_row.iter()) {
                *l -= *r;
            }
        }
    }

    /// Multiplies this matrix by a scalar value in-place.
    ///
    /// # Examples
    ///
    /// ```
    /// use mini_matrix::Matrix;
    ///
    /// let mut a = Matrix::<i32, 2, 2>::from([[1, 2], [3, 4]]);
    /// a.scl(2);
    /// assert_eq!(a.store, [[2, 4], [6, 8]]);
    /// ```
    pub fn scl(&mut self, scalar: T) {
        for row in self.store.iter_mut() {
            for elem in row.iter_mut() {
                *elem *= scalar;
            }
        }
    }
}

impl<T, const M: usize, const N: usize> IndexMut<(usize, usize)> for Matrix<T, M, N> {
    /// Mutably indexes into the matrix, allowing modification of its elements.
    ///
    /// # Arguments
    ///
    /// * `index` - A tuple `(row, column)` specifying the element to access.
    ///
    /// # Examples
    ///
    /// ```
    /// use mini_matrix::Matrix;
    ///
    /// let mut matrix = Matrix::<i32, 2, 2>::from([[1, 2], [3, 4]]);
    /// matrix[(0, 1)] = 5;
    /// assert_eq!(matrix.store, [[1, 5], [3, 4]]);
    /// ```
    fn index_mut(&mut self, index: (usize, usize)) -> &mut T {
        &mut self.store[index.0][index.1]
    }
}

impl<T, const M: usize, const N: usize> Index<(usize, usize)> for Matrix<T, M, N> {
    type Output = T;

    /// Immutably indexes into the matrix, allowing read access to its elements.
    ///
    /// # Arguments
    ///
    /// * `index` - A tuple `(row, column)` specifying the element to access.
    ///
    /// # Examples
    ///
    /// ```
    /// use mini_matrix::Matrix;
    ///
    /// let matrix = Matrix::<i32, 2, 2>::from([[1, 2], [3, 4]]);
    /// assert_eq!(matrix[(1, 0)], 3);
    /// ```
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.store.index(index.0).index(index.1)
    }
}

impl<T, const M: usize, const N: usize> Deref for Matrix<T, M, N> {
    type Target = [[T; N]; M];

    /// Dereferences the matrix, allowing it to be treated as a slice.
    ///
    /// # Note
    ///
    /// This implementation is currently unfinished.
    ///
    /// # Examples
    ///
    /// ```
    /// use mini_matrix::Matrix;
    ///
    /// let matrix = Matrix::<i32, 2, 2>::from([[1, 2], [3, 4]]);
    /// // Usage example will be available once implementation is complete
    /// ```
    fn deref(&self) -> &Self::Target {
        &self.store
    }
}

impl<T, const M: usize, const N: usize> DerefMut for Matrix<T, M, N> {
    /// Mutably dereferences the matrix, allowing it to be treated as a mutable slice.
    ///
    /// # Note
    ///
    /// This implementation is currently unfinished.
    ///
    /// # Examples
    ///
    /// ```
    /// use mini_matrix::Matrix;
    ///
    /// let mut matrix = Matrix::<i32, 2, 2>::from([[1, 2], [3, 4]]);
    /// // Usage example will be available once implementation is complete
    /// ```
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.store
    }
}

impl<T, const M: usize, const N: usize> Add for Matrix<T, M, N>
where
    T: AddAssign + Copy + Num,
{
    type Output = Self;

    /// Adds two matrices element-wise.
    ///
    /// # Examples
    ///
    /// ```
    /// use mini_matrix::Matrix;
    ///
    /// let a = Matrix::<i32, 2, 2>::from([[1, 2], [3, 4]]);
    /// let b = Matrix::<i32, 2, 2>::from([[5, 6], [7, 8]]);
    /// let c = a + b;
    /// assert_eq!(c.store, [[6, 8], [10, 12]]);
    /// ```
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

    /// Subtracts one matrix from another element-wise.
    ///
    /// # Examples
    ///
    /// ```
    /// use mini_matrix::Matrix;
    ///
    /// let a = Matrix::<i32, 2, 2>::from([[5, 6], [7, 8]]);
    /// let b = Matrix::<i32, 2, 2>::from([[1, 2], [3, 4]]);
    /// let c = a - b;
    /// assert_eq!(c.store, [[4, 4], [4, 4]]);
    /// ```
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

    /// Multiplies a matrix by a scalar value.
    ///
    /// # Examples
    ///
    /// ```
    /// use mini_matrix::Matrix;
    ///
    /// let a = Matrix::<i32, 2, 2>::from([[1, 2], [3, 4]]);
    /// let b = a * 2;
    /// assert_eq!(b.store, [[2, 4], [6, 8]]);
    /// ```
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

    /// Multiplies a matrix by a vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use mini_matrix::{Matrix, Vector};
    ///
    /// let a = Matrix::<i32, 2, 2>::from([[1, 2], [3, 4]]);
    /// let v = Vector::<i32, 2>::from([5, 6]);
    /// let result = a * v;
    /// assert_eq!(result.store, [17, 39]);
    /// ```
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

    /// Multiplies two matrices.
    ///
    /// # Examples
    ///
    /// ```
    /// use mini_matrix::Matrix;
    ///
    /// let a = Matrix::<i32, 2, 2>::from([[1, 2], [3, 4]]);
    /// let b = Matrix::<i32, 2, 2>::from([[5, 6], [7, 8]]);
    /// let c = a * b;
    /// assert_eq!(c.store, [[17, 23], [39, 53]]);
    /// ```
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

    /// Negates all elements of the matrix.
    ///
    /// # Examples
    ///
    /// ```
    /// use mini_matrix::Matrix;
    ///
    /// let a = Matrix::<i32, 2, 2>::from([[1, -2], [-3, 4]]);
    /// let b = -a;
    /// assert_eq!(b.store, [[-1, 2], [3, -4]]);
    /// ````
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
    /// Formats the matrix for display.
    ///
    /// Each row of the matrix is displayed on a new line, with elements separated by commas.
    /// Elements are formatted with one decimal place precision.
    ///
    /// # Examples
    ///
    /// ```
    /// use mini_matrix::Matrix;
    ///
    /// let a = Matrix::<f32, 2, 2>::from([[1.0, 2.5], [3.7, 4.2]]);
    /// println!("{}", a);
    /// // Output:
    /// // // [1.0, 2.5]
    /// // // [3.7, 4.2]
    /// ```
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
    /// Multiplies the matrix by a vector.
    ///
    /// # Arguments
    /// * `vec` - The vector to multiply with the matrix.
    ///
    /// # Returns
    /// The resulting vector of the multiplication.
    pub fn mul_vec(&mut self, vec: &Vector<T, N>) -> Vector<T, N> {
        let mut result = Vector::zero();
        for (idx, row) in self.store.iter_mut().enumerate() {
            for (e1, e2) in row.iter_mut().zip(vec.store.iter()) {
                result[idx] += *e1 * *e2;
            }
        }
        result
    }

    /// Multiplies the matrix by another matrix.
    ///
    /// # Arguments
    /// * `mat` - The matrix to multiply with.
    ///
    /// # Returns
    /// The resulting matrix of the multiplication.
    pub fn mul_mat(&mut self, mat: &Matrix<T, M, N>) -> Matrix<T, M, N> {
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
    /// Calculates the trace of the matrix.
    ///
    /// The trace is defined as the sum of the elements on the main diagonal.
    ///
    /// # Panics
    ///
    /// Panics if the matrix is not square (i.e., if M != N).
    ///
    /// # Examples
    ///
    /// ```
    /// use mini_matrix::Matrix;
    ///
    /// let mut a = Matrix::<i32, 3, 3>::from([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
    /// assert_eq!(a.trace(), 15);
    /// ```
    pub fn trace(&self) -> T {
        assert!(M == N, "Matrix must be square to calculate trace");

        let mut result = T::default();
        for i in 0..M {
            result += self[(i, i)];
        }
        result
    }
}

/* ********************************************* */
/*            Exercise 09 - Transpose            */
/* ********************************************* */
impl<T, const M: usize, const N: usize> Matrix<T, M, N>
where
    T: Copy + Default,
{
    /// Computes the transpose of the matrix.
    ///
    /// # Examples
    ///
    /// ```
    /// use mini_matrix::Matrix;
    ///
    /// let mut a = Matrix::<i32, 2, 3>::from([[1, 2, 3], [4, 5, 6]]);
    /// let b = a.transpose();
    /// assert_eq!(b.store, [[1, 4], [2, 5], [3, 6]]);
    /// ```
    pub fn transpose(&mut self) -> Matrix<T, N, M> {
        let mut result = Matrix::zero();
        for i in 0..M {
            for j in 0..N {
                result[(j, i)] = self[(i, j)];
            }
        }
        result
    }
}

/* ********************************************** */
/*             Exercise XX - Identity             */
/* ********************************************** */
impl<T, const M: usize, const N: usize> Matrix<T, M, N>
where
    T: Copy + Default + Num,
{
    /// Creates an identity matrix.
    ///
    /// # Examples
    ///
    /// ```
    /// use mini_matrix::Matrix;
    ///
    /// let a = Matrix::<i32, 3, 3>::identity();
    /// assert_eq!(a.store, [[1, 0, 0], [0, 1, 0], [0, 0, 1]]);
    /// ```
    pub fn identity() -> Matrix<T, M, N> {
        let mut result = Matrix::zero();
        for i in 0..M {
            result[(i, i)] = T::one();
        }
        result
    }
}

/* ************************************************* */
/*             Exercise 12 - Row Echelon             */
/* ************************************************* */

impl<T, const M: usize, const N: usize> Matrix<T, M, N>
where
    T: Copy + Default + Mul<Output = T> + PartialEq + Num + Div<Output = T> + Sub<Output = T>,
{
    /// Computes the row echelon form of the matrix.
    ///
    /// # Examples
    ///
    /// ```
    /// use mini_matrix::Matrix;
    ///
    /// let a = Matrix::<f64, 3, 4>::from([
    ///     [1.0, 2.0, 3.0, 4.0],
    ///     [5.0, 6.0, 7.0, 8.0],
    ///     [9.0, 10.0, 11.0, 12.0]
    /// ]);
    /// let b = a.row_echelon();
    /// // Check the result (approximate due to floating-point arithmetic)
    /// ```
    pub fn row_echelon(&self) -> Matrix<T, M, N> {
        let mut result = self.clone();
        let mut pivot = 0;

        for r in 0..M {
            if pivot >= N {
                break;
            }

            // Find the row with a non-zero pivot
            let mut i = r;
            while i < M && result[(i, pivot)] == T::default() {
                i += 1;
            }

            if i == M {
                pivot += 1;
                if pivot >= N {
                    break;
                }
                // No non-zero element found in this column, continue to the next column
                continue;
            }

            // Swap the current row with the row containing the non-zero pivot
            if i != r {
                for j in 0..N {
                    let temp = result[(r, j)];
                    result[(r, j)] = result[(i, j)];
                    result[(i, j)] = temp;
                }
            }

            // Normalize the pivot row
            let divisor = result[(r, pivot)];
            if divisor != T::default() {
                for j in 0..N {
                    result[(r, j)] = result[(r, j)] / divisor;
                }
            }

            // Eliminate the pivot column in all other rows
            for i in 0..M {
                if i != r {
                    let factor = result[(i, pivot)];
                    for j in 0..N {
                        result[(i, j)] = result[(i, j)] - factor * result[(r, j)];
                    }
                }
            }

            pivot += 1;
        }

        result
    }
}

/************************************************ * */
/*            Exercise 12 - Determinant            */
/************************************************ */

impl<T, const M: usize, const N: usize> Matrix<T, M, N>
where
    T: Copy + Default + Mul + Num + Neg<Output = T> + AddAssign + Debug,
{
    /// Calculates the determinant of the matrix.
    ///
    /// This method supports matrices up to 4x4 in size.
    ///
    /// # Panics
    ///
    /// Panics if the matrix is larger than 4x4.
    ///
    /// # Examples
    ///
    /// ```
    /// use mini_matrix::Matrix;
    ///
    /// let a = Matrix::<i32, 3, 3>::from([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
    /// assert_eq!(a.determinant(), 0);
    /// ```
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

    fn get_cofactor(&self, row: usize, col: usize) -> Matrix<T, 3, 3> {
        let mut cofactor_matrix = Matrix::<T, 3, 3>::zero();
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

/********************************************* */
/*            Exercise 12 - Inverse            */
/********************************************* */

impl<T, const M: usize, const N: usize> Matrix<T, M, N>
where
    T: Copy + Default + Mul + Num + Neg<Output = T> + AddAssign + Debug + Float,
{
    /// Calculates the inverse of the matrix.
    ///
    /// This method supports matrices up to 3x3 in size.
    ///
    /// # Returns
    ///
    /// Returns `Ok(Matrix)` if the inverse exists, or an `Err` with a descriptive message if not.
    ///
    /// # Examples
    ///
    /// ```
    /// use mini_matrix::Matrix;
    ///
    /// let a = Matrix::<f64, 2, 2>::from([[1.0, 2.0], [3.0, 4.0]]);
    /// let inv = a.inverse().unwrap();
    /// // Check the result (approximate due to floating-point arithmetic)
    /// ```
    pub fn inverse(&self) -> Result<Self, &'static str> {
        if M != N {
            return Err("Matrix must be square to calculate inverse");
        }

        let det = self.determinant();

        if det == T::zero() {
            return Err("Matrix is singular and has no inverse");
        }

        let mut inv = Matrix::<T, N, M>::zero();
        for i in 0..M {
            for j in 0..N {
                let coffactor = match M {
                    2 => self.cofactor1x1(i, j).determinant(),
                    3 => self.cofactor2x2(i, j).determinant(),
                    _ => return Err("Inverse not implemented for matrices larger than 3x3"),
                };
                let base: i32 = -1;
                inv[(i, j)] = (coffactor * T::from(base.pow((i + j) as u32)).unwrap()) / det;
            }
        }
        let inv = inv.transpose();
        Ok(inv)
    }
}

/***************************************** */
/*            Exercise 13 - Rank            */
/***************************************** */

impl<T, const M: usize, const N: usize> Matrix<T, M, N>
where
    T: Copy + Default + Mul + Num + Neg<Output = T> + AddAssign + PartialEq,
{
    /// Calculates the rank of the matrix.
    ///
    /// The rank is determined by computing the row echelon form and counting non-zero rows.
    ///
    /// # Examples
    ///
    /// ```
    /// use mini_matrix::Matrix;
    ///
    /// let a = Matrix::<i32, 3, 3>::from([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
    /// assert_eq!(a.rank(), 2);
    /// ```
    pub fn rank(&self) -> usize {
        let mut rank = M;
        let row_echelon = self.row_echelon();
        for i in 0..M {
            let mut is_zero = true;
            for j in 0..N {
                if row_echelon[(i, j)] != T::default() {
                    is_zero = false;
                    break;
                }
            }
            if is_zero {
                rank -= 1;
            }
        }
        rank
    }
}
