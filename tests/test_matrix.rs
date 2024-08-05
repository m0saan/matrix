#[cfg(test)]
mod matrix_tests {
    use mini_matrix::{Matrix, Vector};

    #[test]
    fn test_from() {
        let m = Matrix::from([[2., 3.], [5., 7.]]);
        assert_eq!(m.store, [[2., 3.], [5., 7.]]);

        let n = Matrix::from([[11., 13., 17.], [19., 23., 29.]]);
        assert_eq!(n.store, [[11., 13., 17.], [19., 23., 29.]]);
    }

    #[test]
    fn test_size() {
        let m = Matrix::from([[2., 3.], [5., 7.]]);
        assert_eq!(m.size(), (2, 2));

        let n = Matrix::from([[11., 13., 17.], [19., 23., 29.]]);
        assert_eq!(n.size(), (2, 3));
    }

    #[test]
    fn test_zero() {
        let m = Matrix::<f32, 2, 3>::zero();
        assert_eq!(m.store, [[0., 0., 0.], [0., 0., 0.]]);

        let n = Matrix::<i32, 3, 4>::zero();
        assert_eq!(n.store, [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]]);
    }

    #[test]
    fn test_add() {
        let mut m = Matrix::from([[2., 3.], [5., 7.]]);
        let n = Matrix::from([[11., 13.], [17., 19.]]);
        m.add(&n);
        assert_eq!(m.store, [[13., 16.], [22., 26.]]);
    }

    #[test]
    fn test_add_subject() {
        let m1 = Matrix::from([[0, 0], [0, 0]]);
        let m2 = Matrix::from([[0, 0], [0, 0]]);
        assert_eq!(m1 + m2, Matrix::from([[0, 0], [0, 0]]));

        let m1 = Matrix::from([[1, 0], [0, 1]]);
        let m2 = Matrix::from([[0, 0], [0, 0]]);
        assert_eq!(m1 + m2, Matrix::from([[1, 0], [0, 1]]));

        let m1 = Matrix::from([[1, 1], [1, 1]]);
        let m2 = Matrix::from([[1, 1], [1, 1]]);
        assert_eq!(m1 + m2, Matrix::from([[2, 2], [2, 2]]));

        let m1 = Matrix::from([[21, 21], [21, 21]]);
        let m2 = Matrix::from([[21, 21], [21, 21]]);
        assert_eq!(m1 + m2, Matrix::from([[42, 42], [42, 42]]));
    }

    #[test]
    fn test_sub() {
        let mut m = Matrix::from([[2., 3.], [5., 7.]]);
        let n = Matrix::from([[11., 13.], [17., 19.]]);
        m.sub(&n);
        assert_eq!(m.store, [[-9., -10.], [-12., -12.]]);
    }

    #[test]
    fn test_sub_subject() {
        assert_eq!(
            Matrix::from([[0, 0], [0, 0]]) - Matrix::from([[0, 0], [0, 0]]),
            Matrix::from([[0, 0], [0, 0]])
        );

        assert_eq!(
            Matrix::from([[1, 0], [0, 1]]) - Matrix::from([[0, 0], [0, 0]]),
            Matrix::from([[1, 0], [0, 1]])
        );

        assert_eq!(
            Matrix::from([[1, 1], [1, 1]]) - Matrix::from([[1, 1], [1, 1]]),
            Matrix::from([[0, 0], [0, 0]])
        );

        assert_eq!(
            Matrix::from([[21, 21], [21, 21]]) - Matrix::from([[21, 21], [21, 21]]),
            Matrix::from([[0, 0], [0, 0]])
        );
    }

    #[test]
    fn test_scl() {
        let mut m = Matrix::from([[2., 3.], [5., 7.]]);
        m.scl(2.);
        assert_eq!(m.store, [[4., 6.], [10., 14.]]);
    }

    #[test]
    fn test_scl_subject() {
        // Create matrices
        let mut m0 = Matrix::from([[0.0, 0.0], [0.0, 0.0]]);
        let mut m1 = Matrix::from([[1.0, 0.0], [0.0, 1.0]]);
        let mut m2 = Matrix::from([[1.0, 2.0], [3.0, 4.0]]);
        let mut m3 = Matrix::from([[21.0, 21.0], [21.0, 21.0]]);

        m0.scl(0.0);
        m1.scl(1.0);
        m2.scl(2.0);
        m3.scl(0.5);
        assert_eq!(m0, Matrix::from([[0.0, 0.0], [0.0, 0.0]]));
        assert_eq!(m1, Matrix::from([[1.0, 0.0], [0.0, 1.0]]));
        assert_eq!(m2, Matrix::from([[2.0, 4.0], [6.0, 8.0]]));
        assert_eq!(m3, Matrix::from([[10.5, 10.5], [10.5, 10.5]]));
    }

    #[test]
    fn test_add_trait() {
        let m = Matrix::from([[2., 3.], [5., 7.]]);
        let n = Matrix::from([[11., 13.], [17., 19.]]);
        let o = m + n;
        assert_eq!(o.store, [[13., 16.], [22., 26.]]);
    }

    #[test]
    fn test_sub_trait() {
        let m = Matrix::from([[2., 3.], [5., 7.]]);
        let n = Matrix::from([[11., 13.], [17., 19.]]);
        let o = m - n;
        assert_eq!(o.store, [[-9., -10.], [-12., -12.]]);
    }

    #[test]
    fn test_mul_trait() {
        let m = Matrix::from([[2., 3.], [5., 7.]]);
        let o = m * 2.;
        assert_eq!(o.store, [[4., 6.], [10., 14.]]);
    }

    #[test]
    fn test_index() {
        let m = Matrix::from([[2., 3.], [5., 7.]]);
        assert_eq!(m[(0, 0)], 2.);
        assert_eq!(m[(0, 1)], 3.);
        assert_eq!(m[(1, 0)], 5.);
        assert_eq!(m[(1, 1)], 7.);
    }

    #[test]
    fn test_index_mut() {
        let mut m = Matrix::from([[2., 3.], [5., 7.]]);
        m[(0, 0)] = 11.;
        m[(0, 1)] = 13.;
        m[(1, 0)] = 17.;
        m[(1, 1)] = 19.;
        assert_eq!(m.store, [[11., 13.], [17., 19.]]);
    }

    #[test]
    fn test_matrix_vec_mul() {
        let mut u = Matrix::from([[1., 0.], [0., 1.]]);
        let v = Vector::from([4., 2.]);
        assert_eq!(u.mul_vec(&v).store, [4., 2.]); // [4.] [2.]

        let mut u = Matrix::from([[2., 0.], [0., 2.]]);
        let v = Vector::from([4., 2.]);
        assert_eq!(u.mul_vec(&v).store, [8., 4.]); // [8.] [4.]

        let mut u = Matrix::from([[2., -2.], [-2., 2.]]);
        let v = Vector::from([4., 2.]);
        assert_eq!(u.mul_vec(&v).store, [4., -4.]); // [4.] [-4.]
    }

    #[test]
    fn test_mul_vec_subject() {
        let mut mat1 = Matrix::from([[0, 0], [0, 0]]);
        let vec1 = Vector::from([1, 2]);
        assert_eq!(mat1.mul_vec(&vec1), Vector::from([0, 0]));

        let mut mat2 = Matrix::from([[1, 0], [0, 1]]);
        let vec2 = Vector::from([3, 4]);
        assert_eq!(mat2.mul_vec(&vec2), Vector::from([3, 4]));

        let mut mat3 = Matrix::from([[1, 1], [1, 1]]);
        let vec3 = Vector::from([4, 2]);
        assert_eq!(mat3.mul_vec(&vec3), Vector::from([6, 6]));

        let mut mat4 = Matrix::from([[2, 0], [0, 2]]);
        let vec4 = Vector::from([2, 1]);
        assert_eq!(mat4.mul_vec(&vec4), Vector::from([4, 2]));

        let mut mat5 = Matrix::from([[0.5, 0.0], [0.0, 0.5]]);
        let vec5 = Vector::from([4.0, 2.0]);
        assert_eq!(mat5.mul_vec(&vec5), Vector::from([2.0, 1.0]));
    }

    #[test]
    fn test_mul_mat_subject() {
        // Matrix-matrix multiplication
        let mut mat1 = Matrix::from([[0, 0], [0, 0]]);
        let mat2 = Matrix::from([[1, 2], [3, 4]]);
        assert_eq!(mat1.mul_mat(&mat2), Matrix::from([[0, 0], [0, 0]]));

        let mut mat3 = Matrix::from([[1, 0], [0, 1]]);
        let mat4 = Matrix::from([[2, 3], [4, 5]]);
        assert_eq!(mat3.mul_mat(&mat4), Matrix::from([[2, 3], [4, 5]]));

        let mut mat5 = Matrix::from([[1, 1], [1, 1]]);
        let mat6 = Matrix::from([[1, 2], [3, 4]]);
        assert_eq!(mat5.mul_mat(&mat6), Matrix::from([[4, 6], [4, 6]]));

        let mut mat7 = Matrix::from([[2, 0], [0, 2]]);
        let mat8 = Matrix::from([[1, 2], [3, 4]]);
        assert_eq!(mat7.mul_mat(&mat8), Matrix::from([[2, 4], [6, 8]]));
    }

    #[test]
    fn test_matrix_matrix_mul() {
        let mut u = Matrix::from([[1., 0.], [0., 1.]]);
        let v = Matrix::from([[1., 0.], [0., 1.]]);

        assert_eq!(u.mul_mat(&v).store, [[1., 0.], [0., 1.]]); // [1., 0.] [0., 1.]

        let mut u = Matrix::from([[1., 0.], [0., 1.]]);
        let v = Matrix::from([[2., 1.], [4., 2.]]);
        assert_eq!(u.mul_mat(&v).store, [[2., 1.], [4., 2.]]); // [2., 1.] [4., 2.]

        let mut u = Matrix::from([[3., -5.], [6., 8.]]);
        let v = Matrix::from([[2., 1.], [4., 2.]]);
        assert_eq!(u.mul_mat(&v).store, [[-14., -7.], [44., 22.]]); // [2., 1.] [44., 22.]
    }

    #[test]
    fn test_trace() {
        let m = Matrix::from([[2., 3.], [5., 7.]]);
        assert_eq!(m.trace(), 9.);

        let u = Matrix::from([[1., 0.], [0., 1.]]);
        assert_eq!(u.trace(), 2.); // 2.0

        let u = Matrix::from([[2., -5., 0.], [4., 3., 7.], [-2., 3., 4.]]);
        assert_eq!(u.trace(), 9.); // 9.0

        let u = Matrix::from([[-2., -8., 4.], [1., -23., 4.], [0., 6., 4.]]);
        assert_eq!(u.trace(), -21.); // -21.0
    }

    #[test]
    fn test_trace_subject() {
        let mat1 = Matrix::from([[0, 0], [0, 0]]);
        assert_eq!(mat1.trace(), 0);

        let mat2 = Matrix::from([[1, 0], [0, 1]]);
        assert_eq!(mat2.trace(), 2);

        let mat3 = Matrix::from([[1, 2], [3, 4]]);
        assert_eq!(mat3.trace(), 5);

        let mat4 = Matrix::from([[8, -7], [4, 2]]);
        assert_eq!(mat4.trace(), 10);

        let mat5 = Matrix::from([[1, 0, 0], [0, 1, 0], [0, 0, 1]]);
        assert_eq!(mat5.trace(), 3);

        let mat6 = Matrix::from([[5, -3, 2], [0, 7, 1], [4, -1, 6]]);
        assert_eq!(mat6.trace(), 18);
    }

    #[test]
    fn test_transpose() {
        let mut m = Matrix::from([[1., 2., 3.], [4., 5., 6.], [7., 8., 9.]]);
        let result = m.transpose();
        let expected = Matrix::from([[1., 4., 7.], [2., 5., 8.], [3., 6., 9.]]);
        assert_eq!(result.store, expected.store);

        let mut u = Matrix::from([[1., 0.], [0., 1.]]);
        let result = u.transpose();
        let expected = Matrix::from([[1., 0.], [0., 1.]]);
        assert_eq!(result.store, expected.store);

        let mut u = Matrix::from([[2., -5., 0.], [4., 3., 7.], [-2., 3., 4.]]);
        let result = u.transpose();
        let expected = Matrix::from([[2., 4., -2.], [-5., 3., 3.], [0., 7., 4.]]);
        assert_eq!(result.store, expected.store);

        let mut u = Matrix::from([[-2., -8., 4.], [1., -23., 4.], [0., 6., 4.]]);
        let result = u.transpose();
        let expected = Matrix::from([[-2., 1., 0.], [-8., -23., 6.], [4., 4., 4.]]);
        assert_eq!(result.store, expected.store);
    }

    // #[test]
    // fn test_determinant() {
    //     let mut m = Matrix::from([[1., 2.], [3., 4.]]);
    //     assert_eq!(m.determinant(), -2.);

    //     let mut u = Matrix::from([[1., 0.], [0., 1.]]);
    //     assert_eq!(u.determinant(), 1.);

    //     let mut u = Matrix::from([[2., -5., 0.], [4., 3., 7.], [-2., 3., 4.]]);
    //     assert_eq!(u.determinant(), 117.);

    //     let mut u = Matrix::from([[-2., -8., 4.], [1., -23., 4.], [0., 6., 4.]]);
    //     assert_eq!(u.determinant(), 0.);
    // }
    #[test]
    fn test_row_echelon_from() {
        let u = Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
        let result = u.row_echelon();
        let expected = Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
        assert_eq!(result.store, expected.store);
        // [1.0, 0.0, 0.0]
        // [0.0, 1.0, 0.0]
        // [0.0, 0.0, 1.0]

        let u = Matrix::from([[1., 2.], [3., 4.]]);
        let result = u.row_echelon();
        let expected = Matrix::from([[1., 0.], [0., 1.]]);
        assert_eq!(result.store, expected.store);
        // [1.0, 0.0]
        // [0.0, 1.0]

        let u = Matrix::from([[1., 2.], [2., 4.]]);
        let result = u.row_echelon();
        let expected = Matrix::from([[1., 2.], [0., 0.]]);
        assert_eq!(result.store, expected.store);
        // [1.0, 2.0]
        // [0.0, 0.0]

        // let u = Matrix::from([
        //     [8., 5., -2., 4., 28.],
        //     [4., 2.5, 20., 4., -4.],
        //     [8., 5., 1., 4., 17.],
        // ]);
        // let result: f32 = u.row_echelon().store.as_flattened().iter().sum();
        // let expected: f32 = Matrix::from([
        //     [1.0, 0.625, 0.0, 0.0, -12.1666667],
        //     [0.0, 0.0, 1.0, 0.0, -3.6666667],
        //     [0.0, 0.0, 0.0, 1.0, 29.5],
        // ] as [[f32; 5]; 3])
        // .store
        // .as_flattened()
        // .iter()
        // .sum();
        // println!("-------------------> failing test {:?}", (result - expected).abs());
        // assert!((result - expected).abs() < 0.000001);
        // [1.0, 0.625, 0.0, 0.0, -12.1666667]
        // [0.0, 0.0, 1.0, 0.0, -3.6666667]
        // [0.0, 0.0, 0.0, 1.0, 29.5 ]
    }

    #[test]
    fn test_row_echelon_subject() {
        // Test for a 2x2 zero matrix
        let mat1 = Matrix::from([[0, 0], [0, 0]]);
        assert_eq!(mat1.row_echelon().store, [[0, 0], [0, 0]]);

        // Test for a 2x2 identity matrix
        let mat2 = Matrix::from([[1, 0], [0, 1]]);
        assert_eq!(mat2.row_echelon().store, [[1, 0], [0, 1]]);

        // Test for a 2x2 matrix with different values
        let mat3 = Matrix::from([[4., 2.], [2., 1.]]);
        assert_eq!(mat3.row_echelon().store, [[1., 0.5], [0., 0.]]);

        // Test for a 2x2 matrix with more complex values
        let mat4 = Matrix::from([[-7, 2], [4, 8]]);
        assert_eq!(mat4.row_echelon().store, [[1, 0], [0, 1]]);

        // Test for a 2x2 matrix with another set of values
        let mat5 = Matrix::from([[1, 2], [4, 8]]);
        assert_eq!(mat5.row_echelon().store, [[1, 2], [0, 0]]);
    }

    #[test]
    fn test_determinant() {
        let m = Matrix::from([[1., 2.], [3., 4.]]);
        assert_eq!(m.determinant(), -2.);

        let u = Matrix::from([[1., 0.], [0., 1.]]);
        assert_eq!(u.determinant(), 1.);

        let u = Matrix::from([[2., -5., 0.], [4., 3., 7.], [-2., 3., 4.]]);
        assert_eq!(u.determinant(), 132.);

        let u = Matrix::from([[-2., -8., 4.], [1., -23., 4.], [0., 6., 4.]]);
        assert_eq!(u.determinant(), 288.);

        let u = Matrix::from([[1., -1.], [-1., 1.]]);
        assert_eq!(u.determinant(), 0.); // 0.0

        let u = Matrix::from([[2., 0., 0.], [0., 2., 0.], [0., 0., 2.]]);
        assert_eq!(u.determinant(), 8.); // 8.0

        let u = Matrix::from([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.]]);
        assert_eq!(u.determinant(), -174.); // -174.0

        let u = Matrix::from([
            [8., 5., -2., 4.],
            [4., 2.5, 20., 4.],
            [8., 5., 1., 4.],
            [28., -4., 17., 1.],
        ]);
        assert_eq!(u.determinant(), 1032.); // 1032
    }

    #[test]
    fn test_determinant_subject() {
        // Test for a 2x2 zero matrix
        let mat1 = Matrix::from([[0, 0], [0, 0]]);
        assert_eq!(mat1.determinant(), 0);

        // Test for a 2x2 identity matrix
        let mat2 = Matrix::from([[1, 0], [0, 1]]);
        assert_eq!(mat2.determinant(), 1);

        // Test for a 2x2 diagonal matrix
        let mat3 = Matrix::from([[2, 0], [0, 2]]);
        assert_eq!(mat3.determinant(), 4);

        // Test for a 2x2 matrix with identical rows
        let mat4 = Matrix::from([[1, 1], [1, 1]]);
        assert_eq!(mat4.determinant(), 0);

        // Test for a 2x2 matrix with specific values
        let mat5 = Matrix::from([[0, 1], [1, 0]]);
        assert_eq!(mat5.determinant(), -1);

        // Test for a 3x3 matrix
        let mat6 = Matrix::from([[1, 2], [3, 4]]);
        assert_eq!(mat6.determinant(), -2);

        // Test for a 2x2 matrix with negative values
        let mat7 = Matrix::from([[-7, 5], [4, 6]]);
        assert_eq!(mat7.determinant(), -62);

        // Test for a 3x3 identity matrix
        let mat8 = Matrix::from([[1, 0, 0], [0, 1, 0], [0, 0, 1]]);
        assert_eq!(mat8.determinant(), 1);
    }

    #[test]
    fn test_inverse_identity_2x2() {
        let matrix = Matrix::from([[1.0, 0.0], [0.0, 1.0]]);
        let expected = Matrix::from([[1.0, 0.0], [0.0, 1.0]]);
        assert_eq!(matrix.inverse().unwrap(), expected);
    }

    #[test]
    fn test_inverse_scalar_2x2() {
        let matrix = Matrix::from([[2.0, 0.0], [0.0, 2.0]]);
        let expected = Matrix::from([[0.5, 0.0], [0.0, 0.5]]);
        assert_eq!(matrix.inverse().unwrap(), expected);
    }

    #[test]
    fn test_inverse_scaled_identity_2x2() {
        let matrix = Matrix::from([[0.5, 0.0], [0.0, 0.5]]);
        let expected = Matrix::from([[2.0, 0.0], [0.0, 2.0]]);
        assert_eq!(matrix.inverse().unwrap(), expected);
    }

    #[test]
    fn test_inverse_swap_2x2() {
        let matrix = Matrix::from([[0.0, 1.0], [1.0, 0.0]]);
        let expected = Matrix::from([[0.0, 1.0], [1.0, 0.0]]);
        assert_eq!(matrix.inverse().unwrap(), expected);
    }

    #[test]
    fn test_inverse_non_trivial_2x2() {
        let matrix = Matrix::from([[1.0, 2.0], [3.0, 4.0]]);
        let expected = Matrix::from([[-2.0, 1.0], [1.5, -0.5]]);
        assert_eq!(matrix.inverse().unwrap(), expected);
    }

    #[test]
    fn test_inverse_identity_3x3() {
        let matrix = Matrix::from([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]);
        let expected = Matrix::from([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]);
        assert_eq!(matrix.inverse().unwrap(), expected);
    }

    // #[test]
    // fn test_rank_zeros() {
    //     let matrix: Matrix<f64, 2, 2> = Matrix::from([[0.0, 0.0], [0.0, 0.0]]);
    //     assert_eq!(matrix.rank(), 0);
    // }

    // #[test]
    // fn test_rank_identity() {
    //     let matrix: Matrix<f64, 2, 2> = Matrix::from([[1.0, 0.0], [0.0, 1.0]]);
    //     assert_eq!(matrix.rank(), 2);
    // }

    // #[test]
    // fn test_rank_scaled_identity() {
    //     let matrix: Matrix<f64, 2, 2> = Matrix::from([[2.0, 0.0], [0.0, 2.0]]);
    //     assert_eq!(matrix.rank(), 2);
    // }

    // #[test]
    // fn test_rank_identical_rows() {
    //     let matrix: Matrix<f64, 2, 2> = Matrix::from([[1.0, 1.0], [1.0, 1.0]]);
    //     assert_eq!(matrix.rank(), 1);
    // }

    // #[test]
    // fn test_rank_permutation() {
    //     let matrix: Matrix<f64, 2, 2> = Matrix::from([[0.0, 1.0], [1.0, 0.0]]);
    //     assert_eq!(matrix.rank(), 2);
    // }

    // #[test]
    // fn test_rank_generic() {
    //     let matrix: Matrix<f64, 2, 2> = Matrix::from([[1.0, 2.0], [3.0, 4.0]]);
    //     assert_eq!(matrix.rank(), 2);
    // }

    // #[test]
    // fn test_rank_custom() {
    //     let matrix: Matrix<f64, 2, 2> = Matrix::from([[-7.0, 5.0], [4.0, 6.0]]);
    //     assert_eq!(matrix.rank(), 2);
    // }

    // #[test]
    // fn test_rank_identity_3x3() {
    //     let matrix: Matrix<f64, 3, 3> =
    //         Matrix::from([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]);
    //     assert_eq!(matrix.rank(), 3);
    // }
}
