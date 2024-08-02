mod ops_tests {
    use ::matrix::{angle_cos, cross_product, linear_combination, Vector};

    #[test]
    fn test_linear_combination() {
        // Linear Combination
        let e1 = Vector::from([1., 0., 0.]);
        let e2 = Vector::from([0., 1., 0.]);
        let e3 = Vector::from([0., 0., 1.]);

        let v1 = Vector::from([1., 2., 3.]);
        let v2 = Vector::from([0., 10., -100.]);

        let expected = Vector::from([10., -2., 0.5]);
        let result = linear_combination(&mut [e1, e2, e3], &[10., -2., 0.5]);
        assert_eq!(result.store, expected.store); // [10.] [-2.]  [0.5]

        let expected = Vector::from([10., 0., 230.]);
        let result = linear_combination(&mut [v1, v2], &[10., -2.]);
        assert_eq!(result.store, expected.store); // [10.][0.][230.]
    }

    #[test]
    fn test_angle_cos() {
        let u = Vector::from([1., 0.]);
        let v = Vector::from([1., 0.]);
        assert!((angle_cos(&u, &v) - 1.0) < f64::EPSILON); // 1.0

        let u = Vector::from([1., 0.]);
        let v = Vector::from([0., 1.]);
        assert!((angle_cos(&u, &v) - 0.0) < f64::EPSILON); // 0.0

        let u = Vector::from([-1., 1.]);
        let v = Vector::from([1., -1.]);
        assert!((angle_cos(&u, &v) - 1.0) < f64::EPSILON); // -1.0

        let u = Vector::from([2., 1.]);
        let v = Vector::from([4., 2.]);
        assert!((angle_cos(&u, &v) - 1.0) < f64::EPSILON); // 1.0

        let u = Vector::from([1., 2., 3.] as [f32; 3]);
        let v = Vector::from([4., 5., 6.] as [f32; 3]);
        println!("{:?}", angle_cos(&u, &v));
        assert!((angle_cos(&u, &v) - 0.974631846).abs() < f32::EPSILON); // 0.974631846
    }

    #[test]
    fn test_cross_product() {
        let u = Vector::from([0., 0., 1.]);
        let v = Vector::from([1., 0., 0.]);
        let result = cross_product(&u, &v);
        assert_eq!(result.store, Vector::from([0., 1., 0.]).store); // [0.] [1.] [0.]

        let u = Vector::from([1., 2., 3.]);
        let v = Vector::from([4., 5., 6.]);
        let result = cross_product(&u, &v);
        assert_eq!(result.store, Vector::from([-3., 6., -3.]).store); // [-3.] [6.] [-3.]

        let u = Vector::from([4., 2., -3.]);
        let v = Vector::from([-2., -5., 16.]);
        let result = cross_product(&u, &v);
        assert_eq!(result.store, Vector::from([17., -58., -16.]).store); // [17.] [-58.] [-16.]
    }
}
