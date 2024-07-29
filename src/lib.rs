use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

#[derive(Debug, Clone, Default, PartialEq, PartialOrd)]
pub struct Vector<T> {
    pub data: Vec<T>,
}

impl<T> Display for Vector<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "//-> [")?;
        for (i, val) in self.data.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{:.1}", val)?;
        }
        write!(f, "]\n")
    }
}

impl<T> Vector<T>
where
    T: Default + Clone,
{
    pub fn from(data: &[T]) -> Self {
        let data = data.to_vec();
        Self { data }
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }
}

impl<T> Vector<T>
where
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Copy
        + Clone
        + Default
        + PartialOrd,
{
    pub fn add(&mut self, rhs: &Self) {
        for (lsh_e, rhs_e) in self.data.iter_mut().zip(rhs.data.iter()) {
            *lsh_e = *lsh_e + *rhs_e;
        }
    }

    pub fn sub(&mut self, rhs: &Self) {
        for (lsh_e, rhs_e) in self.data.iter_mut().zip(rhs.data.iter()) {
            *lsh_e = *lsh_e - *rhs_e;
        }
    }

    pub fn scl(&mut self, scalar: T) {
        for elem in self.data.iter_mut() {
            *elem = *elem * scalar;
        }
    }

    pub fn dot(&self, v: Self) -> T {
        let mut prod = T::default();
        for (e1, e2) in self.data.iter().zip(v.data.iter()) {
            prod = prod + (*e1 * *e2);
        }
        prod
    }
}

impl<T> Add for Vector<T>
where
    T: Add<Output = T> + Default + Clone + Copy,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = Vector::from(&vec![T::default(); 0]);
        for elem in self.data.iter().zip(rhs.data.iter()) {
            result.data.push(*elem.0 + *elem.1)
        }
        result
    }
}

impl<T> Sub for Vector<T>
where
    T: Sub<Output = T> + Default + Clone + Copy,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = Vector::from(&vec![T::default(); 0]);
        for elem in self.data.iter().zip(rhs.data.iter()) {
            result.data.push(*elem.0 - *elem.1)
        }
        result
    }
}

// Implement the Mul trait for Vector<T> with f32
impl<T> Mul<f32> for Vector<T>
where
    T: Mul<f32, Output = T> + Clone,
{
    type Output = Self;

    fn mul(self, scalar: f32) -> Self::Output {
        let mut result = self.clone();
        for element in result.data.iter_mut() {
            *element = element.clone() * scalar;
        }
        result
    }
}

impl Vector<f32> {
    pub fn norm_1(&mut self) -> f32 {
        let mut norm = 0.0;
        for e in self.data.iter() {
            norm = norm + e.abs();
        }
        norm
    }

    pub fn norm(&mut self) -> f32 {
        let mut norm = 0.0;
        for e in self.data.iter() {
            norm = norm + (*e * *e);
        }
        norm.sqrt()
    }

    pub fn norm_inf(&mut self) -> f32 {
        let mut norm = 0.0;
        for e in self.data.iter() {
            if e.abs() > norm {
                norm = e.abs();
            }
        }
        norm
    }
}

//
// Matrix
//
#[derive(Debug, Clone, Default, PartialEq, PartialOrd)]
pub struct Matrix<T> {
    pub data: Vec<Vec<T>>,
}

impl<T> std::fmt::Display for Matrix<T>
where
    T: Clone + Display + std::ops::AddAssign<T> + std::ops::SubAssign<T> + std::ops::MulAssign,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (i, row) in self.data.iter().enumerate() {
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

impl<T> Matrix<T>
where
    T: Default + Clone,
{
    pub fn from(data: &[&[T]]) -> Self {
        let data = data.iter().map(|row| row.to_vec()).collect();
        Self { data }
    }

    pub fn size(&self) -> (usize, usize) {
        (self.data.len(), self.data[0].len())
    }
}

impl<T> Matrix<T>
where
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Copy
        + Clone
        + Default
        + PartialOrd,
{
    pub fn add(&mut self, rhs: &Self) {
        for (r_lhs, r_rhs) in self.data.iter_mut().zip(rhs.data.iter()) {
            for (e_rhs, e_lhs) in r_lhs.iter_mut().zip(r_rhs.iter()) {
                *e_rhs = *e_rhs + *e_lhs
            }
        }
    }

    pub fn sub(&mut self, rhs: &Self) {
        for (r_lhs, r_rhs) in self.data.iter_mut().zip(rhs.data.iter()) {
            for (e_rhs, e_lhs) in r_lhs.iter_mut().zip(r_rhs.iter()) {
                *e_rhs = *e_rhs - *e_lhs
            }
        }
    }

    pub fn scl(&mut self, scalar: T) {
        for vec in self.data.iter_mut() {
            for element in vec.iter_mut() {
                *element = *element * scalar;
            }
        }
    }
}

impl<T> Add for Matrix<T>
where
    T: Add<Output = T> + Default + Clone + Copy,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = self.clone();
        for (l_vec, r_vec) in result.data.iter_mut().zip(rhs.data.iter()) {
            for (e_lhs, e_rhs) in l_vec.iter_mut().zip(r_vec.iter()) {
                *e_lhs = *e_lhs + *e_rhs;
            }
        }
        result
    }
}

impl<T> Sub for Matrix<T>
where
    T: Sub<Output = T> + Default + Clone + Copy,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = self.clone();
        for (l_vec, r_vec) in result.data.iter_mut().zip(rhs.data.iter()) {
            for (e_lhs, e_rhs) in l_vec.iter_mut().zip(r_vec.iter()) {
                *e_lhs = *e_lhs - *e_rhs;
            }
        }
        result
    }
}

// Implement the Mul trait for Vector<T> with f32
impl<T> Mul<f32> for Matrix<T>
where
    T: Mul<f32, Output = T> + Clone,
{
    type Output = Self;

    fn mul(self, scalar: f32) -> Self::Output {
        let mut result = self.clone();
        for vec in result.data.iter_mut() {
            for element in vec.iter_mut() {
                *element = element.clone() * scalar;
            }
        }
        result
    }
}

// O(n)
// pub fn linear_combination<T>(vectors: &mut [Vector<T>], scalars: &[T]) -> Vector<T>
// where
//     T: Add<Output = T>
//         + Sub<Output = T>
//         + Mul<Output = T>
//         + Div<Output = T>
//         + Copy
//         + Clone
//         + Default
//         + PartialOrd,
// {
//     assert_eq!(
//         vectors.len(),
//         scalars.len(),
//         "The number of vectors and scalars must be the same"
//     );

//     if vectors.is_empty() {
//         panic!("The number of vectors must be greater than 0");
//     }

//     let mut result = Vector::from(&vec![T::default(); vectors[0].size()]);
//     for (vector, scalar) in vectors.iter().zip(scalars.iter()) {
//         vector.scl(scalar.clone());
//         result.add(vector);
//     }
//     result
// }

pub trait Lerp {
    fn lerp(u: Self, v: Self, t: f32) -> Self;
}

impl Lerp for f32 {
    fn lerp(u: Self, v: Self, t: Self) -> Self {
        u + (v - u) * t
    }
}

impl<T> Lerp for Vector<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<f32, Output = T> + Clone + Default,
{
    fn lerp(u: Self, v: Self, t: f32) -> Self {
        let mut result: Vector<T> = Vector {
            data: vec![T::default(); u.size()],
        };
        for (e_u, e_v) in u.data.iter().zip(v.data.iter()) {
            result
                .data
                .push(e_u.clone() + (e_v.clone() - e_u.clone()) * t)
        }
        result
    }
}

pub fn lerp<V>(u: V, v: V, t: f32) -> V
where
    V: Add<Output = V> + Sub<Output = V> + Mul<f32, Output = V> + Clone + Default,
{
    u.clone() + (v - u) * t
}
