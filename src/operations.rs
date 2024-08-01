use num::Float;
use std::iter::Sum;
use std::ops::{Add, Mul};

use super::Vector;

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
        for (result_elem, vector_elem) in result.data.iter_mut().zip(vector.data.iter()) {
            *result_elem = *result_elem + *vector_elem * *scalar;
        }
    }
    result
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
