//! # mini_matrix
//!
//! A mini linear algebra library implemented in Rust.
//!
use num::{Float, Num};
use std::iter::Sum;
use std::ops::{Add, AddAssign, Mul};

use super::Vector;

/* ***************************** */
/*      Linear Combination      */
/* *************************** */

/// Computes the linear combination of a set of vectors.
///
/// A linear combination is the sum of scalar multiples of vectors.
///
/// # Arguments
///
/// * `vectors` - A mutable slice of vectors to be combined
/// * `scalars` - A slice of scalar values to multiply with each vector
///
/// # Returns
///
/// A new `Vector<T, N>` representing the linear combination of the input vectors.
///
/// # Examples
///
/// ```
/// use mini_matrix::{Vector, linear_combination};
///
/// let mut v1 = Vector::from([1.0, 2.0, 3.0]);
/// let mut v2 = Vector::from([4.0, 5.0, 6.0]);
/// let scalars = [2.0, 3.0];
///
/// let result = linear_combination(&mut [v1, v2], &scalars);
/// assert_eq!(result, Vector::from([14.0, 19.0, 24.0]));
/// ```
///
/// # Panics
///
/// This function will panic if:
/// - The number of vectors and scalars are not equal
/// - The vector slice is empty

// O(n)

pub fn linear_combination<T, const N: usize>(
    vectors: &[Vector<T, N>],
    scalars: &[T],
) -> Vector<T, N>
where
    T: Num + Copy + Clone + Default + PartialOrd + AddAssign + Mul<T>,
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
    for (v, s) in vectors.iter().zip(scalars.iter()) {
        result += *v * *s;
    }
    result
}

/// Computes the linear interpolation between two vectors.
///
/// Linear interpolation (lerp) finds a point that is between two vectors, based on a given parameter `t`.
/// It computes a weighted average of the vectors where `t` determines the weight of the second vector.
///
/// # Arguments
///
/// * `u` - The starting vector.
/// * `v` - The ending vector.
/// * `t` - A scalar parameter between 0.0 and 1.0 that determines the interpolation point.
///         If `t` is 0.0, the result is `u`. If `t` is 1.0, the result is `v`.
///         For values between 0.0 and 1.0, the result is a point between `u` and `v`.
///
/// # Returns
///
/// A new vector that represents the interpolated result between `u` and `v`.
///
/// # Examples
///
/// ```
/// use mini_matrix::{Vector, lerp};
///
/// let u = Vector::from([1.0, 2.0, 3.0]);
/// let v = Vector::from([4.0, 5.0, 6.0]);
///
/// let result = lerp(u, v, 0.5);
/// assert_eq!(result, Vector::from([2.5, 3.5, 4.5]));
/// ```
///
/// # Panics
///
/// This function will panic if:
/// - The type `V` does not implement the required traits (`Add`, `Mul`).
///
/// # Notes
///
/// - This function assumes `t` is a floating-point number (`f32`) and works with vectors where the `Add`
///   and `Mul` traits are implemented.
pub fn lerp<V>(u: V, v: V, t: f32) -> V
where
    V: Add<V, Output = V> + Mul<f32, Output = V>,
{
    u * (1.0 - t) + v * t
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
    let dot_product = u.dot(v);
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
pub fn cross_product<T, const N: usize>(u: &Vector<T, N>, v: &Vector<T, N>) -> Vector<T, 3>
where
    T: Float + Default,
{
    assert_eq!(N, 3, "Cross product is only defined for 3D vectors");

    Vector::from([
        u[1] * v[2] - u[2] * v[1],
        u[2] * v[0] - u[0] * v[2],
        u[0] * v[1] - u[1] * v[0],
    ])
}
