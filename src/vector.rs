//! # matrix
//!
//! A mini linear algebra library implemented in Rust.

use num::{Float, Num, Signed};
use std::fmt::Display;
use std::iter::Sum;
use std::ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign};
use std::ops::{Deref, DerefMut, Index, IndexMut};

/// Represents a vector of `N` elements of type `T`.
///
/// # Examples
///
/// ```
/// use matrix::Vector;
///
/// let v = Vector::from([1, 2, 3]);
/// assert_eq!(v[0], 1);
/// assert_eq!(v[1], 2);
/// assert_eq!(v[2], 3);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Vector<T, const N: usize> {
    pub store: [T; N],
}

impl<T, const N: usize> IndexMut<usize> for Vector<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        &mut self.store[index]
    }
}

impl<T, const N: usize> Index<usize> for Vector<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.store.index(index)
    }
}

impl<T, const N: usize> Deref for Vector<T, N> {
    type Target = [T; N];

    fn deref(&self) -> &Self::Target {
        &self.store
    }
}

impl<T, const N: usize> DerefMut for Vector<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.store
    }
}

impl<T, const N: usize> Vector<T, N>
where
    T: Copy + Default,
{
    /// Creates a new `Vector` from an array of elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use matrix::Vector;
    ///
    /// let v = Vector::from([1, 2, 3]);
    /// ```
    pub fn from(data: [T; N]) -> Self {
        Self { store: data }
    }

    /// Returns the number of elements in the `Vector`.
    ///
    /// # Examples
    ///
    /// ```
    /// use matrix::Vector;
    ///
    /// let v: Vector<i32, 3> = Vector::from([1, 2, 3]);
    /// assert_eq!(v.size(), 3);
    /// ```
    pub const fn size(&self) -> usize {
        N
    }

    /// Creates a new `Vector` with all elements set to the default value of type `T`.
    ///
    /// # Examples
    ///
    /// ```
    /// use matrix::Vector;
    ///
    /// let v: Vector<i32, 3> = Vector::zero();
    /// assert_eq!(v[0], 0);
    /// assert_eq!(v[1], 0);
    /// assert_eq!(v[2], 0);
    /// ```
    pub fn zero() -> Self {
        Self {
            store: [T::default(); N],
        }
    }
}

impl<T, const N: usize> Vector<T, N>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Copy + Clone + Default + PartialOrd,
{
    /// Adds another vector to this vector in-place.
    ///
    /// # Examples
    ///
    /// ```
    /// use matrix::Vector;
    ///
    /// let mut v1 = Vector::from([1, 2, 3]);
    /// let v2 = Vector::from([4, 5, 6]);
    /// v1.add(&v2);
    /// assert_eq!(v1[0], 5);
    /// assert_eq!(v1[1], 7);
    /// assert_eq!(v1[2], 9);
    /// ```
    pub fn add(&mut self, rhs: &Self) {
        for (lsh_e, rhs_e) in self.store.iter_mut().zip(rhs.store.iter()) {
            *lsh_e = *lsh_e + *rhs_e;
        }
    }

    /// Subtracts another vector from this vector in-place.
    ///
    /// # Examples
    ///
    /// ```
    /// use matrix::Vector;
    ///
    /// let mut v1 = Vector::from([4, 5, 6]);
    /// let v2 = Vector::from([1, 2, 3]);
    /// v1.sub(&v2);
    /// assert_eq!(v1[0], 3);
    /// assert_eq!(v1[1], 3);
    /// assert_eq!(v1[2], 3);
    /// ```
    pub fn sub(&mut self, rhs: &Self) {
        for (lsh_e, rhs_e) in self.store.iter_mut().zip(rhs.store.iter()) {
            *lsh_e = *lsh_e - *rhs_e;
        }
    }

    /// Multiplies this vector by a scalar value in-place.
    ///
    /// # Examples
    ///
    /// ```
    /// use matrix::Vector;
    ///
    /// let mut v = Vector::from([1, 2, 3]);
    /// v.scl(2);
    /// assert_eq!(v[0], 2);
    /// assert_eq!(v[1], 4);
    /// assert_eq!(v[2], 6);
    /// ```
    pub fn scl(&mut self, scalar: T) {
        for elem in self.store.iter_mut() {
            *elem = *elem * scalar;
        }
    }
}

impl<T, const N: usize> Vector<T, N>
where
    T: Num + Sum + Copy + Clone,
{
    /// Computes the dot product of two vectors.
    ///
    /// The dot product is the sum of the products of corresponding elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use matrix::Vector;
    ///
    /// let v1 = Vector::from([1, 2, 3]);
    /// let v2 = Vector::from([4, 5, 6]);
    /// assert_eq!(v1.dot(v2), 32); // 1*4 + 2*5 + 3*6 = 32
    /// ```
    pub fn dot(&self, v: Self) -> T {
        self.store
            .iter()
            .zip(v.store.iter())
            .map(|(a, b)| *a * *b)
            .sum()
    }
}

impl<T, const N: usize> Add for Vector<T, N>
where
    T: Add<Output = T> + Default + Clone + Copy,
{
    type Output = Self;
    /// Adds two vectors element-wise.
    ///
    /// # Examples
    ///
    /// ```
    /// use matrix::Vector;
    ///
    /// let v1 = Vector::from([1, 2, 3]);
    /// let v2 = Vector::from([4, 5, 6]);
    /// let v3 = v1 + v2;
    /// assert_eq!(v3, Vector::from([5, 7, 9]));
    /// ```
    fn add(self, rhs: Self) -> Self::Output {
        let mut result = self.clone();
        for (e1, e2) in result.store.iter_mut().zip(rhs.store.iter()) {
            *e1 = *e1 + *e2;
        }
        result
    }
}

impl<T, const N: usize> AddAssign for Vector<T, N>
where
    T: Num + Default + Clone + Copy,
{
    /// Adds another vector to this vector in-place.
    ///
    /// # Examples
    ///
    /// ```
    /// use matrix::Vector;
    ///
    /// let mut v1 = Vector::from([1, 2, 3]);
    /// let v2 = Vector::from([4, 5, 6]);
    /// v1 += v2;
    /// assert_eq!(v1, Vector::from([5, 7, 9]));
    /// ```
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<T, const N: usize> Sub for Vector<T, N>
where
    T: Sub<Output = T> + Default + Clone + Copy,
{
    type Output = Self;
    /// Subtracts one vector from another element-wise.
    ///
    /// # Examples
    ///
    /// ```
    /// use matrix::Vector;
    ///
    /// let v1 = Vector::from([4, 5, 6]);
    /// let v2 = Vector::from([1, 2, 3]);
    /// let v3 = v1 - v2;
    /// assert_eq!(v3, Vector::from([3, 3, 3]));
    /// ```
    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = self.clone();
        for (e1, e2) in result.store.iter_mut().zip(rhs.store.iter()) {
            *e1 = *e1 - *e2;
        }
        result
    }
}

impl<T, const N: usize> SubAssign for Vector<T, N>
where
    T: Num + Default + Clone + Copy,
{
    /// Subtracts another vector from this vector in-place.
    ///
    /// # Examples
    ///
    /// ```
    /// use matrix::Vector;
    ///
    /// let mut v1 = Vector::from([4, 5, 6]);
    /// let v2 = Vector::from([1, 2, 3]);
    /// v1 -= v2;
    /// assert_eq!(v1, Vector::from([3, 3, 3]));
    /// ```
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

// Implement the Mul trait for Vector<T, N> with f32
impl<T, const N: usize> Mul<f32> for Vector<T, N>
where
    T: Mul<f32, Output = T> + Clone,
{
    type Output = Self;
    /// Multiplies a vector by a scalar (f32).
    ///
    /// # Examples
    ///
    /// ```
    /// use matrix::Vector;
    ///
    /// let v1 = Vector::from([1.0, 2.0, 3.0]);
    /// let v2 = v1 * 2.0;
    /// assert_eq!(v2, Vector::from([2.0, 4.0, 6.0]));
    /// ```
    fn mul(self, scalar: f32) -> Self::Output {
        let mut result = self.clone();
        for element in result.store.iter_mut() {
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
    /// Computes the dot product of two vectors.
    ///
    /// This is an alternative way to compute the dot product using the * operator.
    ///
    /// # Examples
    ///
    /// ```
    /// use matrix::Vector;
    ///
    /// let v1 = Vector::from([1, 2, 3]);
    /// let v2 = Vector::from([4, 5, 6]);
    /// assert_eq!(v1 * v2, 32); // 1*4 + 2*5 + 3*6 = 32
    /// ```
    fn mul(self, rhs: Self) -> Self::Output {
        self.store
            .into_iter()
            .zip(rhs.store.into_iter())
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
    /// Negates a vector, inverting the sign of all its elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use matrix::Vector;
    ///
    /// let v1 = Vector::from([1, -2, 3]);
    /// let v2 = -v1;
    /// assert_eq!(v2, Vector::from([-1, 2, -3]));
    /// ```
    fn neg(self) -> Self::Output {
        let mut result = self.clone();
        for element in result.store.iter_mut() {
            *element = -(*element);
        }
        result
    }
}

impl<T, const N: usize> Display for Vector<T, N>
where
    T: Display,
{
    /// Formats the vector for display.
    ///
    /// # Examples
    ///
    /// ```
    /// use matrix::Vector;
    ///
    /// let v = Vector::from([1.0, 2.5, 3.7]);
    /// println!("{}", v); // Outputs: //-> [1.0, 2.5, 3.7]
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "//-> [")?;
        for (i, val) in self.store.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{:.1}", val)?;
        }
        write!(f, "]\n")
    }
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
        self.store.iter().map(|x| x.abs()).sum()
    }

    /// Calculates the L2 norm (Euclidean norm) of the vector.
    ///
    /// The L2 norm is the square root of the sum of the squared components.
    ///
    /// # Returns
    /// The L2 norm as a value of type `T`.
    pub fn norm(&self) -> T {
        self.store.iter().map(|x| x.powi(2)).sum::<T>().sqrt()
    }

    /// Calculates the L-infinity norm (maximum norm) of the vector.
    ///
    /// The L-infinity norm is the maximum of the absolute values of the vector's components.
    ///
    /// # Returns
    /// The L-infinity norm as a value of type `T`.
    pub fn norm_inf(&self) -> T {
        self.store
            .iter()
            .map(|x| x.abs())
            .fold(T::zero(), |a, b| a.max(b))
    }
}
