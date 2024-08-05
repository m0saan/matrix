#[cfg(test)]
mod vector_tests {
    use mini_matrix::Vector;

    #[test]
    fn test_from() {
        let u = Vector::from([2., 3.]);
        assert_eq!(u.store, [2., 3.]);

        let v = Vector::from([5., 7., 9., 11.]);
        assert_eq!(v.store, [5., 7., 9., 11.]);
    }

    #[test]
    fn test_size() {
        let u = Vector::from([2., 3.]);
        assert_eq!(u.size(), 2);

        let v = Vector::from([5., 7., 9., 11.]);
        assert_eq!(v.size(), 4);
    }

    #[test]
    fn test_zero() {
        let u = Vector::<f32, 3>::zero();
        assert_eq!(u.store, [0., 0., 0.]);

        let v = Vector::<i32, 5>::zero();
        assert_eq!(v.store, [0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_add() {
        let mut u = Vector::from([2., 3.]);
        let v = Vector::from([5., 7.]);
        u.add(&v);
        assert_eq!(u.store, [7., 10.]);
    }

    #[test]
    fn test_add_subject() {
        let v1 = Vector::from([0, 0]);
        let v2 = Vector::from([0, 0]);
        assert_eq!(v1 + v2, Vector::from([0, 0]));

        let v1 = Vector::from([1, 0]);
        let v2 = Vector::from([0, 1]);
        assert_eq!(v1 + v2, Vector::from([1, 1]));

        let v1 = Vector::from([1, 1]);
        let v2 = Vector::from([1, 1]);
        assert_eq!(v1 + v2, Vector::from([2, 2]));

        let v1 = Vector::from([21, 21]);
        let v2 = Vector::from([21, 21]);
        assert_eq!(v1 + v2, Vector::from([42, 42]));

        let v1 = Vector::from([-21, 21]);
        let v2 = Vector::from([21, -21]);
        assert_eq!(v1 + v2, Vector::from([0, 0]));

        let v1 = Vector::from([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        let v2 = Vector::from([9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
        assert_eq!(v1 + v2, Vector::from([9, 9, 9, 9, 9, 9, 9, 9, 9, 9]));
    }

    #[test]
    fn test_sub() {
        let mut u = Vector::from([2., 3.]);
        let v = Vector::from([5., 7.]);
        u.sub(&v);
        assert_eq!(u.store, [-3., -4.]);
    }

    #[test]
    fn test_sub_subject() {
        assert_eq!(
            Vector::from([0, 0]) - Vector::from([0, 0]),
            Vector::from([0, 0])
        );

        assert_eq!(
            Vector::from([1, 0]) - Vector::from([0, 1]),
            Vector::from([1, -1])
        );

        assert_eq!(
            Vector::from([1, 1]) - Vector::from([1, 1]),
            Vector::from([0, 0])
        );

        assert_eq!(
            Vector::from([21, 21]) - Vector::from([21, 21]),
            Vector::from([0, 0])
        );

        assert_eq!(
            Vector::from([-21, 21]) - Vector::from([21, -21]),
            Vector::from([-42, 42])
        );

        assert_eq!(
            Vector::from([0, 1, 2, 3, 4, 5, 6, 7, 8, 9])
                - Vector::from([9, 8, 7, 6, 5, 4, 3, 2, 1, 0]),
            Vector::from([-9, -7, -5, -3, -1, 1, 3, 5, 7, 9])
        );
    }

    #[test]
    fn test_scl() {
        let mut u = Vector::from([2., 3.]);
        u.scl(3.);
        assert_eq!(u.store, [6., 9.]);
    }

    #[test]
    fn test_scl_subject() {
        let mut v0 = Vector::from([0.0, 0.0]);
        let mut v1 = Vector::from([1.0, 0.0]);
        let mut v2 = Vector::from([1.0, 1.0]);
        let mut v3 = Vector::from([21.0, 21.0]);
        let mut v4 = Vector::from([42.0, 42.0]);

        v0.scl(1.0);
        v1.scl(1.0);
        v2.scl(2.0);
        v3.scl(2.0);
        v4.scl(0.5);
        assert_eq!(v0, Vector::from([0.0, 0.0]));
        assert_eq!(v1, Vector::from([1.0, 0.0]));
        assert_eq!(v2, Vector::from([2.0, 2.0]));
        assert_eq!(v3, Vector::from([42.0, 42.0]));
        assert_eq!(v4, Vector::from([21.0, 21.0]));
    }

    #[test]
    fn test_dot() {
        let u = Vector::from([2., 3.]);
        let v = Vector::from([5., 7.]);
        assert_eq!(u.dot(&v), 31.);
    }

    #[test]
    fn test_dot_product_subject() {
        let v1 = Vector::from([0, 0]);
        let v2 = Vector::from([0, 0]);
        assert_eq!(v1.dot(&v2), 0);

        let v3 = Vector::from([1, 0]);
        let v4 = Vector::from([0, 0]);
        assert_eq!(v3.dot(&v4), 0);

        let v5 = Vector::from([1, 0]);
        let v6 = Vector::from([1, 0]);
        assert_eq!(v5.dot(&v6), 1);

        let v7 = Vector::from([1, 0]);
        let v8 = Vector::from([0, 1]);
        assert_eq!(v7.dot(&v8), 0);

        let v9 = Vector::from([1, 1]);
        let v10 = Vector::from([1, 1]);
        assert_eq!(v9.dot(&v10), 2);

        let v11 = Vector::from([4, 2]);
        let v12 = Vector::from([2, 1]);
        assert_eq!(v11.dot(&v12), 10);
    }

    #[test]
    fn test_norm1() {
        let u = Vector::from([2., 3.]);
        let v = Vector::from([5., 7.]);
        assert_eq!(u.norm_1(), 5.);
        assert_eq!(v.norm_1(), 12.);
    }

    #[test]
    fn test_norm() {
        let u = Vector::from([2., 3.]);
        let v = Vector::from([5., 7.]);
        assert_eq!(u.norm(), 13_f32.sqrt());
        assert_eq!(v.norm(), 74_f32.sqrt());
    }

    #[test]
    fn test_norm_inf() {
        let u = Vector::from([2., 3.]);
        let v = Vector::from([5., 7.]);
        assert_eq!(u.norm_inf(), 3.);
        assert_eq!(v.norm_inf(), 7.);
    }

    #[test]
    fn test_norm1_subject() {
        let v1 = Vector::from([0.0]);
        assert_eq!(v1.norm_1(), 0.0);

        let v2 = Vector::from([1.0]);
        assert_eq!(v2.norm_1(), 1.0);

        let v3 = Vector::from([0.0, 0.0]);
        assert_eq!(v3.norm_1(), 0.0);

        let v4 = Vector::from([1.0, 0.0]);
        assert_eq!(v4.norm_1(), 1.0);

        let v5 = Vector::from([2.0, 1.0]);
        assert_eq!(v5.norm_1(), 3.0);

        let v6 = Vector::from([4.0, 2.0]);
        assert_eq!(v6.norm_1(), 6.0);

        let v7 = Vector::from([-4.0, -2.0]);
        assert_eq!(v7.norm_1(), 6.0);
    }

    #[test]
    fn test_norm_subject() {
        let v1 = Vector::from([0.0]);
        assert_eq!(v1.norm(), 0.0);

        let v2 = Vector::from([1.0]);
        assert_eq!(v2.norm(), 1.0);

        let v3 = Vector::from([0.0, 0.0]);
        assert_eq!(v3.norm(), 0.0);

        let v4 = Vector::from([1.0, 0.0]);
        assert_eq!(v4.norm(), 1.0);

        let v5 = Vector::from([2.0, 1.0]);
        assert_eq!(v5.norm(), 2.23606797749979);

        let v6 = Vector::from([4.0, 2.0]);
        assert_eq!(v6.norm(), 4.47213595499958);

        let v7 = Vector::from([-4.0, -2.0]);
        assert_eq!(v7.norm(), 4.47213595499958);
    }

    #[test]
    fn test_norm_inf_subject() {
        let v1 = Vector::from([0.0]);
        assert_eq!(v1.norm_inf(), 0.0);

        let v2 = Vector::from([1.0]);
        assert_eq!(v2.norm_inf(), 1.0);

        let v3 = Vector::from([0.0, 0.0]);
        assert_eq!(v3.norm_inf(), 0.0);

        let v4 = Vector::from([1.0, 0.0]);
        assert_eq!(v4.norm_inf(), 1.0);

        let v5 = Vector::from([2.0, 1.0]);
        assert_eq!(v5.norm_inf(), 2.0);

        let v6 = Vector::from([4.0, 2.0]);
        assert_eq!(v6.norm_inf(), 4.0);

        let v7 = Vector::from([-4.0, -2.0]);
        assert_eq!(v7.norm_inf(), 4.0);
    }

    #[test]
    fn test_neg_trait() {
        let u = Vector::from([2., 3.]);
        let v = -u;
        assert_eq!(v.store, [-2., -3.]);
    }

    #[test]
    fn test_add_trait() {
        let u = Vector::from([2., 3.]);
        let v = Vector::from([5., 7.]);
        let w = u + v;
        assert_eq!(w.store, [7., 10.]);
    }

    #[test]
    fn test_add_assign_trait() {
        let mut u = Vector::from([2., 3.]);
        let v = Vector::from([5., 7.]);
        u += v;
        assert_eq!(u.store, [7., 10.]);
    }

    #[test]
    fn test_sub_trait() {
        let u = Vector::from([2., 3.]);
        let v = Vector::from([5., 7.]);
        let w = u - v;
        assert_eq!(w.store, [-3., -4.]);
    }

    #[test]
    fn test_sub_assign_trait() {
        let mut u = Vector::from([2., 3.]);
        let v = Vector::from([5., 7.]);
        u -= v;
        assert_eq!(u.store, [-3., -4.]);
    }

    #[test]
    fn test_mul_with_number_trait() {
        let u = Vector::from([2., 3.]);
        let r = u * 3.;
        assert_eq!(r.store, [6., 9.]);
    }

    #[test]
    fn test_mul_with_vector_trait() {
        let u = Vector::from([2., 3.]);
        let v = Vector::from([5., 7.]);
        assert_eq!(u * v, 31.);
    }

    #[test]
    fn test_index() {
        let u = Vector::from([2., 3.]);
        assert_eq!(u[0], 2.);
        assert_eq!(u[1], 3.);
    }

    #[test]
    #[should_panic]
    fn test_index_out_of_bounds() {
        let u = Vector::from([2., 3.]);
        let _ = u[2];
    }

    #[test]
    fn test_index_mut() {
        let mut u = Vector::from([2., 3.]);
        u[0] = 5.;
        u[1] = 7.;
        assert_eq!(u.store, [5., 7.]);
    }

    #[test]
    #[should_panic]
    fn test_index_mut_out_of_bounds() {
        let mut u = Vector::from([2., 3.]);
        u[2] = 5.;
    }

    #[test]
    fn test_display() {
        let u = Vector::from([2., 3.]);
        assert_eq!(format!("{}", u), "//-> [2.0, 3.0]\n");
    }

    #[test]
    fn test_deref() {
        let u = Vector::from([2., 3.]);
        assert_eq!(*u, [2., 3.]);
    }

    #[test]
    fn test_deref_mut() {
        let mut u = Vector::from([2., 3.]);
        *u = [5., 7.];
        assert_eq!(u.store, [5., 7.]);
    }
}
