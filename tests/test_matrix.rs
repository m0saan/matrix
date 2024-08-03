#[cfg(test)]
mod matrix_tests {
    use matrix::{Matrix, Vector};

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
    fn test_sub() {
        let mut m = Matrix::from([[2., 3.], [5., 7.]]);
        let n = Matrix::from([[11., 13.], [17., 19.]]);
        m.sub(&n);
        assert_eq!(m.store, [[-9., -10.], [-12., -12.]]);
    }

    #[test]
    fn test_scl() {
        let mut m = Matrix::from([[2., 3.], [5., 7.]]);
        m.scl(2.);
        assert_eq!(m.store, [[4., 6.], [10., 14.]]);
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
        assert_eq!(u.mul_vec(v).store, [4., 2.]); // [4.] [2.]

        let mut u = Matrix::from([[2., 0.], [0., 2.]]);
        let v = Vector::from([4., 2.]);
        assert_eq!(u.mul_vec(v).store, [8., 4.]); // [8.] [4.]

        let mut u = Matrix::from([[2., -2.], [-2., 2.]]);
        let v = Vector::from([4., 2.]);
        assert_eq!(u.mul_vec(v).store, [4., -4.]); // [4.] [-4.]
    }

    #[test]
    fn test_matrix_matrix_mul() {
        let mut u = Matrix::from([[1., 0.], [0., 1.]]);
        let v = Matrix::from([[1., 0.], [0., 1.]]);

        assert_eq!(u.mul_mat(v).store, [[1., 0.], [0., 1.]]); // [1., 0.] [0., 1.]

        let mut u = Matrix::from([[1., 0.], [0., 1.]]);
        let v = Matrix::from([[2., 1.], [4., 2.]]);
        assert_eq!(u.mul_mat(v).store, [[2., 1.], [4., 2.]]); // [2., 1.] [4., 2.]

        let mut u = Matrix::from([[3., -5.], [6., 8.]]);
        let v = Matrix::from([[2., 1.], [4., 2.]]);
        assert_eq!(u.mul_mat(v).store, [[-14., -7.], [44., 22.]]); // [2., 1.] [44., 22.]
    }

    #[test]
    fn test_trace() {
        let mut m = Matrix::from([[2., 3.], [5., 7.]]);
        assert_eq!(m.trace(), 9.);

        let mut u = Matrix::from([[1., 0.], [0., 1.]]);
        assert_eq!(u.trace(), 2.); // 2.0

        let mut u = Matrix::from([[2., -5., 0.], [4., 3., 7.], [-2., 3., 4.]]);
        assert_eq!(u.trace(), 9.); // 9.0

        let mut u = Matrix::from([[-2., -8., 4.], [1., -23., 4.], [0., 6., 4.]]);
        assert_eq!(u.trace(), -21.); // -21.0
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

        let u = Matrix::from([
            [8., 5., -2., 4., 28.],
            [4., 2.5, 20., 4., -4.],
            [8., 5., 1., 4., 17.],
        ]);
        let result: f32 = u.row_echelon().store.as_flattened().iter().sum();
        let expected: f32 = Matrix::from([
            [1.0, 0.625, 0.0, 0.0, -12.1666667],
            [0.0, 0.0, 1.0, 0.0, -3.6666667],
            [0.0, 0.0, 0.0, 1.0, 29.5],
        ] as [[f32; 5]; 3])
        .store
        .as_flattened()
        .iter()
        .sum();
        assert!((result - expected).abs() < 0.0001);
        // [1.0, 0.625, 0.0, 0.0, -12.1666667]
        // [0.0, 0.0, 1.0, 0.0, -3.6666667]
        // [0.0, 0.0, 0.0, 1.0, 29.5 ]
    }
}
