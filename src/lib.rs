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
    T:  Copy + Default
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
    T: Num + Sum + Copy + Clone ,
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

// O(n)
pub fn linear_combination<T, const N: usize>(
    vectors: &mut [Vector<T, N>],
    scalars: &[T],
) -> Vector<T, N>
where
    T: Add<Output = T> + Mul<Output = T>  + Copy + Clone + Default + PartialOrd, Vector<T, N>: Mul<T>
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

impl<T, const N: usize> Vector<T, N>
where
    T: Float + Sum,
{
    pub fn norm_1(&mut self) -> T {
        self.0.iter().map(|x| x.abs()).sum()
    }
    pub fn norm(&mut self) -> T {
        self.0.iter().map(|x| x.powi(2)).sum::<T>().sqrt()
    }
    pub fn norm_inf(&mut self) -> T {
        self.0.iter().map(|x| x.abs()).fold(T::zero(), |a, b| a.max(b))
    }
}
