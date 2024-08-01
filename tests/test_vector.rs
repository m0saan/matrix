#[cfg(test)]
mod vector_tests {
    use matrix::Vector;

    #[test]
    fn test_from() {
        let u = Vector::from([2., 3.]);
        assert_eq!(u.data, [2., 3.]);

        let v = Vector::from([5., 7., 9., 11.]);
        assert_eq!(v.data, [5., 7., 9., 11.]);
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
        assert_eq!(u.data, [0., 0., 0.]);

        let v = Vector::<i32, 5>::zero();
        assert_eq!(v.data, [0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_add() {
        let mut u = Vector::from([2., 3.]);
        let v = Vector::from([5., 7.]);
        u.add(&v);
        assert_eq!(u.data, [7., 10.]);
    }

    #[test]
    fn test_sub() {
        let mut u = Vector::from([2., 3.]);
        let v = Vector::from([5., 7.]);
        u.sub(&v);
        assert_eq!(u.data, [-3., -4.]);
    }

    #[test]
    fn test_scl() {
        let mut u = Vector::from([2., 3.]);
        u.scl(3.);
        assert_eq!(u.data, [6., 9.]);
    }

    #[test]
    fn test_neg() {
        let u = Vector::from([2., 3.]);
        let v = -u;
        assert_eq!(v.data, [-2., -3.]);
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
    fn test_dot() {
        let u = Vector::from([2., 3.]);
        let v = Vector::from([5., 7.]);
        assert_eq!(u.dot(v), 31.);
    }

    #[test]
    fn test_add_trait() {
        let u = Vector::from([2., 3.]);
        let v = Vector::from([5., 7.]);
        let w = u + v;
        assert_eq!(w.data, [7., 10.]);
    }

    #[test]
    fn test_add_assign_trait() {
        let mut u = Vector::from([2., 3.]);
        let v = Vector::from([5., 7.]);
        u += v;
        assert_eq!(u.data, [7., 10.]);
    }

    #[test]
    fn test_sub_trait() {
        let u = Vector::from([2., 3.]);
        let v = Vector::from([5., 7.]);
        let w = u - v;
        assert_eq!(w.data, [-3., -4.]);
    }

    #[test]
    fn test_sub_assign_trait() {
        let mut u = Vector::from([2., 3.]);
        let v = Vector::from([5., 7.]);
        u -= v;
        assert_eq!(u.data, [-3., -4.]);
    }

    #[test]
    fn test_mul_with_number_trait() {
        let u = Vector::from([2., 3.]);
        let r = u * 3.;
        assert_eq!(r.data, [6., 9.]);
    }

    #[test]
    fn test_mul_with_vector_trait() {
        let u = Vector::from([2., 3.]);
        let v = Vector::from([5., 7.]);
        assert_eq!(u*v, 31.);
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
        assert_eq!(u.data, [5., 7.]);
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
        assert_eq!(u.data, [5., 7.]);
    }
}
