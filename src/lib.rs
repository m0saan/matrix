use std::{
    fmt::Display,
    ops::{Add, Div, Index, IndexMut, Mul, Sub},
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
