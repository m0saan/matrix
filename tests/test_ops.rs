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
    fn test_li_combination_subject() {
        let v1 = Vector::from([-42.0, 42.0]);
        let scalars1 = [-1.0];
        assert_eq!(
            linear_combination(&mut [v1], &scalars1),
            Vector::from([42.0, -42.0])
        );

        let v2 = Vector::from([-42.0]);
        let scalars2 = [-1.0, 1.0, 0.0];
        assert_eq!(
            linear_combination(&mut [v2, v2, v2], &scalars2),
            Vector::from([0.0])
        );

        let v3 = Vector::from([-42.0, 42.0]);
        let v4 = Vector::from([1.0, 3.0]);
        let v5 = Vector::from([10.0, 20.0]);
        let scalars3 = [1.0, -10.0, -1.0];
        assert_eq!(
            linear_combination(&mut [v3, v4, v5], &scalars3),
            Vector::from([-62.0, -8.0])
        );

        let v6 = Vector::from([-42.0, 100.0, -69.5]);
        let v7 = Vector::from([1.0, 3.0, 5.0]);
        let scalars4 = [1.0, -10.0];
        assert_eq!(
            linear_combination(&mut [v6, v7], &scalars4),
            Vector::from([-52.0, 70.0, -119.5])
        );
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
    fn test_angle_cos_subject() {
        let u1 = Vector::from([1.0, 0.0]);
        let v1 = Vector::from([0.0, 1.0]);
        assert_eq!(angle_cos(&u1, &v1), 0.0);

        let u2 = Vector::from([8.0, 7.0]);
        let v2 = Vector::from([3.0, 2.0]);
        assert!(((angle_cos(&u2, &v2) - 0.9914542955425437) as f64).abs() < f64::EPSILON);

        let u3 = Vector::from([1.0, 1.0]);
        let v3 = Vector::from([1.0, 1.0]);
        assert_eq!((angle_cos(&u3, &v3) - 1.0) < f64::EPSILON, true);

        let u4 = Vector::from([4.0, 2.0]);
        let v4 = Vector::from([1.0, 1.0]);
        assert!(((angle_cos(&u4, &v4) - 0.9486832980505138) as f64).abs() < f64::EPSILON);

        let u5 = Vector::from([-7.0, 3.0]);
        let v5 = Vector::from([6.0, 4.0]);
        assert!(((angle_cos(&u5, &v5) + 0.5462677805469223) as f64).abs() < f64::EPSILON);

        // Test commutativity
        assert_eq!(angle_cos(&u2, &v2), angle_cos(&v2, &u2));
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

    #[test]
    fn test_cross_product_subject() {
        let u1 = Vector::from([0.0, 0.0, 0.0]);
        let v1 = Vector::from([0.0, 0.0, 0.0]);
        assert_eq!(cross_product(&u1, &v1), Vector::from([0.0, 0.0, 0.0]));

        let u2 = Vector::from([1.0, 0.0, 0.0]);
        let v2 = Vector::from([0.0, 0.0, 0.0]);
        assert_eq!(cross_product(&u2, &v2), Vector::from([0.0, 0.0, 0.0]));

        let u3 = Vector::from([1.0, 0.0, 0.0]);
        let v3 = Vector::from([0.0, 1.0, 0.0]);
        assert_eq!(cross_product(&u3, &v3), Vector::from([0.0, 0.0, 1.0]));

        let u4 = Vector::from([8.0, 7.0, -4.0]);
        let v4 = Vector::from([3.0, 2.0, 1.0]);
        assert_eq!(cross_product(&u4, &v4), Vector::from([15.0, -20.0, -5.0]));

        let u5 = Vector::from([1.0, 1.0, 1.0]);
        let v5 = Vector::from([0.0, 0.0, 0.0]);
        assert_eq!(cross_product(&u5, &v5), Vector::from([0.0, 0.0, 0.0]));

        let u6 = Vector::from([1.0, 1.0, 1.0]);
        let v6 = Vector::from([1.0, 1.0, 1.0]);
        assert_eq!(cross_product(&u6, &v6), Vector::from([0.0, 0.0, 0.0]));

        // Additional test: Vectors in a plane
        let u7 = Vector::from([1.0, 2.0, 3.0]);
        let v7 = Vector::from([4.0, 5.0, 6.0]);
        assert_eq!(cross_product(&u7, &v7), Vector::from([-3.0, 6.0, -3.0]));
    }
}
