use num::{Float, Num, Signed};
use std::fmt::Display;
use std::iter::Sum;
use std::ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign};
use std::ops::{Deref, DerefMut, Index, IndexMut};

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
    pub fn from(data: [T; N]) -> Self {
        Self { store: data }
    }

    pub const fn size(&self) -> usize {
        N
    }

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
    pub fn add(&mut self, rhs: &Self) {
        for (lsh_e, rhs_e) in self.store.iter_mut().zip(rhs.store.iter()) {
            *lsh_e = *lsh_e + *rhs_e;
        }
    }

    pub fn sub(&mut self, rhs: &Self) {
        for (lsh_e, rhs_e) in self.store.iter_mut().zip(rhs.store.iter()) {
            *lsh_e = *lsh_e - *rhs_e;
        }
    }

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
