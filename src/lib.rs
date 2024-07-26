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
}

// O(n)
pub fn linear_combination<T>(vectors: &mut [Vector<T>], scalars: &[T]) -> Vector<T>
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
    assert_eq!(
        vectors.len(),
        scalars.len(),
        "The number of vectors and scalars must be the same"
    );

    if vectors.is_empty() {
        panic!("The number of vectors must be greater than 0");
    }

    let mut result = Vector::from(&vec![T::default(); vectors[0].size()]);
    for (vector, scalar) in vectors.iter_mut().zip(scalars.iter()) {
        vector.scl(scalar.clone());
        result.add(&vector);
    }
    result
}

// pub fn lerp<V>(u: V, v: V, t: f32) -> V
// where
//     V: Clone
//         + std::ops::AddAssign<V>
//         + std::ops::SubAssign<V>
//         + std::ops::MulAssign<f32>
//         + std::ops::Add<V, Output = V>
//         + std::ops::Sub<V, Output = V>
//         + std::ops::Mul<f32, Output = V>,
// {
//     let mut result = u;
//     result *= 1.0 - t;
//     result += v * t;
//     result
// }
