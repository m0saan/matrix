//
// Matrix
//
#![allow(dead_code)]
#![allow(unused_imports)]

use num::{Float, Num, Signed, Zero};
use std::fmt::Display;
use std::iter::Sum;
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub, SubAssign};
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug, Clone)]
pub struct Matrix<T, const M: usize, const N: usize>([[T; M]; N]);

impl<T, const M: usize, const N: usize> Matrix<T, M, N>
where
    T: AddAssign + SubAssign + MulAssign + Copy,
{
    pub fn from(data: [[T; M]; N]) -> Self {
        Self(data)
    }

    pub const fn size(&self) -> (usize, usize) {
        (M, N)
    }

    pub fn add(&mut self, other: &Self) {
        for (l_row, r_row) in self.0.iter_mut().zip(other.0.iter()) {
            for (l, r) in l_row.iter_mut().zip(r_row.iter()) {
                *l += *r;
            }
        }
    }

    pub fn sub(&mut self, other: &Self) {
        for (l_row, r_row) in self.0.iter_mut().zip(other.0.iter()) {
            for (l, r) in l_row.iter_mut().zip(r_row.iter()) {
                *l -= *r;
            }
        }
    }
    pub fn scl(&mut self, scalar: T) {
        for row in self.0.iter_mut() {
            for elem in row.iter_mut() {
                *elem *= scalar;
            }
        }
    }
}

impl<T, const M: usize, const N: usize> IndexMut<(usize, usize)> for Matrix<T, M, N> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut T {
        &mut self.0[index.0][index.1]
    }
}

impl<T, const M: usize, const N: usize> Index<(usize, usize)> for Matrix<T, M, N> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.0.index(index.0).index(index.1)
    }
}

impl<T, const M: usize, const N: usize> Add for Matrix<T, M, N>
where
    T: AddAssign + Copy + Num,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = self.clone();
        for (l_row, r_row) in result.0.iter_mut().zip(rhs.0.iter()) {
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
        for (l_row, r_row) in result.0.iter_mut().zip(rhs.0.iter()) {
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
        for row in result.0.iter_mut() {
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
        for (i, row) in self.0.iter().enumerate() {
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

#[derive(Debug, Clone, Copy)]
pub struct Vector<T, const N: usize>([T; N]);

impl<T, const N: usize> IndexMut<usize> for Vector<T,N> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        &mut self.0[index]
    }
}

impl<T, const N: usize> Index<usize> for Vector<T,N> {
    type Output = T;

    fn index(&self, index:usize) -> &Self::Output {
        &self.0.index(index)
    }
}


impl<T, const N: usize> Deref for Vector<T, N> {
    type Target = [T; N];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, const N: usize> DerefMut for Vector<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T, const N: usize> Vector<T, N>
where
    T: Copy + Default,
{
    pub fn from(data: [T; N]) -> Self {
        Self(data)
    }

    pub const fn size(&self) -> usize {
        N
    }

    pub fn zero() -> Self {
        Self([T::default(); N])
    }
}

impl<T, const N: usize> Vector<T, N>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Copy + Clone + Default + PartialOrd,
{
    pub fn add(&mut self, rhs: &Self) {
        for (lsh_e, rhs_e) in self.0.iter_mut().zip(rhs.0.iter()) {
            *lsh_e = *lsh_e + *rhs_e;
        }
    }

    pub fn sub(&mut self, rhs: &Self) {
        for (lsh_e, rhs_e) in self.0.iter_mut().zip(rhs.0.iter()) {
            *lsh_e = *lsh_e - *rhs_e;
        }
    }

    pub fn scl(&mut self, scalar: T) {
        for elem in self.0.iter_mut() {
            *elem = *elem * scalar;
        }
    }
}

impl<T, const N: usize> Vector<T, N>
where
    T: Num + Sum + Copy + Clone,
{
    pub fn dot(&self, v: Self) -> T {
        self.0.iter().zip(v.0.iter()).map(|(a, b)| *a * *b).sum()
    }
}

impl<T, const N: usize> Add for Vector<T, N>
where
    T: Add<Output = T> + Default + Clone + Copy,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = self.clone();
        for (e1, e2) in result.0.iter_mut().zip(rhs.0.iter()) {
            *e1 = *e1 + *e2;
        }
        result
    }
}

impl<T, const N: usize> AddAssign for Vector<T, N>
where
    T: Num + Default + Clone + Copy,
{
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<T, const N: usize> Sub for Vector<T, N>
where
    T: Sub<Output = T> + Default + Clone + Copy,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = self.clone();
        for (e1, e2) in result.0.iter_mut().zip(rhs.0.iter()) {
            *e1 = *e1 - *e2;
        }
        result
    }
}

impl<T, const N: usize> SubAssign for Vector<T, N>
where
    T: Num + Default + Clone + Copy,
{
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

// Implement the Mul trait for Vector<T, N> with f32
impl<T, const N: usize> Mul<f32> for Vector<T, N>
where
    T: Mul<f32, Output = T> + Clone,
{
    type Output = Self;

    fn mul(self, scalar: f32) -> Self::Output {
        let mut result = self.clone();
        for element in result.0.iter_mut() {
            *element = element.clone() * scalar;
        }
        result
    }
}

// Implement the Mul trait for Vector<T, N> with f32
impl<T, const N: usize> Mul for Vector<T, N>
where
    T: Mul + Num + Sum + Copy,
{
    type Output = T;

    fn mul(self, rhs: Self) -> Self::Output {
        self.0
            .into_iter()
            .zip(rhs.0.into_iter())
            .map(|(a, b)| a * b)
            .sum()
    }
}

// Implement the Neg trait for Vector<T, N>
impl<T, const N: usize> Neg for Vector<T, N>
where
    T: Num + Copy + Signed,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        let mut result = self.clone();
        for element in result.0.iter_mut() {
            *element = -(*element);
        }
        result
    }
}

impl<T, const N: usize> Display for Vector<T, N>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "//-> [")?;
        for (i, val) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{:.1}", val)?;
        }
        write!(f, "]\n")
    }
}

/* ***************************** */
/*      Linear Combination      */
/* *************************** */
// O(n)

pub fn linear_combination<T, const N: usize>(
    vectors: &mut [Vector<T, N>],
    scalars: &[T],
) -> Vector<T, N>
where
    T: Add<Output = T> + Mul<Output = T> + Copy + Clone + Default + PartialOrd,
    Vector<T, N>: Mul<T>,
{
    assert_eq!(
        vectors.len(),
        scalars.len(),
        "The number of vectors and scalars must be the same"
    );

    if vectors.is_empty() {
        panic!("The number of vectors must be greater than 0");
    }

    let mut result = Vector::zero();
    for (vector, scalar) in vectors.iter_mut().zip(scalars.iter()) {
        for (result_elem, vector_elem) in result.0.iter_mut().zip(vector.0.iter()) {
            *result_elem = *result_elem + *vector_elem * *scalar;
        }
    }
    result
}

/* *************** */
/*      Norms      */
/* *************** */
impl<T, const N: usize> Vector<T, N>
where
    T: Float + Sum,
{
    /// Calculates the L1 norm (Manhattan norm) of the vector.
    ///
    /// The L1 norm is the sum of the absolute values of the vector's components.
    ///
    /// # Returns
    /// The L1 norm as a value of type `T`.
    pub fn norm_1(&self) -> T {
        self.0.iter().map(|x| x.abs()).sum()
    }

    /// Calculates the L2 norm (Euclidean norm) of the vector.
    ///
    /// The L2 norm is the square root of the sum of the squared components.
    ///
    /// # Returns
    /// The L2 norm as a value of type `T`.
    pub fn norm(&self) -> T {
        self.0.iter().map(|x| x.powi(2)).sum::<T>().sqrt()
    }

    /// Calculates the L-infinity norm (maximum norm) of the vector.
    ///
    /// The L-infinity norm is the maximum of the absolute values of the vector's components.
    ///
    /// # Returns
    /// The L-infinity norm as a value of type `T`.
    pub fn norm_inf(&self) -> T {
        self.0
            .iter()
            .map(|x| x.abs())
            .fold(T::zero(), |a, b| a.max(b))
    }
}


/* *********************** */
/*      Cosine Angle       */
/* *********************** */

/// Calculates the cosine of the angle between two vectors.
///
/// For vectors a and b, the cosine of the angle θ between them is:
/// cos(θ) = (a · b) / (||a|| ||b||)
/// Where (a · b) is the dot product and ||a|| and ||b|| are the magnitudes of the vectors.
///
/// # Arguments
/// * `u` - A reference to the first vector
/// * `v` - A reference to the second vector
///
/// # Returns
/// The cosine of the angle between the two vectors as a value of type `T`.
///
/// # Type Parameters
/// * `T` - The floating-point type of the vector components
/// * `N` - The dimensionality of the vectors
///
/// # Type Constraints
/// * `T: Float` - The component type must be a floating-point type
/// * `T: Sum` - The component type must support summation
pub fn angle_cos<T, const N: usize>(u: &Vector<T, N>, v: &Vector<T, N>) -> T
where
    T: Float,
    T: Sum,
{
    let dot_product = u.dot(*v);
    let norm_u = u.norm();
    let norm_v = v.norm();
    dot_product / (norm_u * norm_v)
}


/// Computes the cross product of two 3-dimensional vectors.
///
/// The cross product u × v is defined for 3D vectors as:
/// u × v = [u2v3 - u3v2, u3v1 - u1v3, u1v2 - u2v1]
///
/// # Arguments
/// * `u` - A reference to the first 3D vector
/// * `v` - A reference to the second 3D vector
///
/// # Returns
/// A new `Vector<T, 3>` representing the cross product of `u` and `v`.
///
/// # Type Parameters
/// * `T` - The floating-point type of the vector components
/// * `N` - The dimensionality of the vectors (should be 3)
///
/// # Panics
/// This function will panic if the input vectors are not 3-dimensional.
///
/// # Examples
/// ```
/// let u = Vector::from([0., 0., 1.]);
/// let v = Vector::from([1., 0., 0.]);
/// assert_eq!(cross_product(&u, &v), Vector::from([0., 1., 0.]));
///
/// let u = Vector::from([1., 2., 3.]);
/// let v = Vector::from([4., 5., 6.]);
/// assert_eq!(cross_product(&u, &v), Vector::from([-3., 6., -3.]));
///
/// let u = Vector::from([4., 2., -3.]);
/// let v = Vector::from([-2., -5., 16.]);
/// assert_eq!(cross_product(&u, &v), Vector::from([17., -58., -16.]));
/// ```
pub fn cross_product<T, const N: usize>(u: &Vector<T, N>, v: &Vector<T, N>) -> Vector<T, 3>
where
    T: Float + Default
{
    assert_eq!(N, 3, "Cross product is only defined for 3D vectors");

    Vector::from([
        u[1] * v[2] - u[2] * v[1],
        u[2] * v[0] - u[0] * v[2],
        u[0] * v[1] - u[1] * v[0],
    ])
}
