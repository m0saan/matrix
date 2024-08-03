#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use matrix::{angle_cos, cross_product, Matrix, Vector};

fn main() {
    // let mut u = Vector::from([2., 3.]);
    // let v = Vector::from([5., 7.]);
    // u.add(v);
    // println!("{}", u);
    // // [7.0]
    // // [10.0]

    // // println!("{}", u+v);

    // let mut u = Vector::from([2., 3.]);
    // let v = Vector::from([5., 7.]);
    // u.sub(&v);
    // println!("{}", u);
    // // [-3.0]
    // // [-4.0]

    // // println!("{}", u + v);

    // let mut u = Vector::from([2., 3.]);
    // u.scl(2.);
    // println!("{}", u);
    // // [4.0]
    // // [6.0]

    // println!("{}", u * 2.0);

    // // dot product
    // let mut u = Vector::from([0., 0.]);
    // let v = Vector::from([1., 1.]);
    // println!("{}", u.dot(v));
    // // 0.0
    // let mut u = Vector::from([1., 1.]);
    // let v = Vector::from([1., 1.]);
    // println!("{}", u.dot(v));
    // // 2.0
    // let mut u = Vector::from([-1., 6.]);
    // let v = Vector::from([3., 2.]);
    // println!("{}", u.dot(v));
    // 9.0

    // // Matrix addition
    // let mut u = Matrix::from([[1., 2.], [3., 4.]]);
    // let v = Matrix::from([[7., 4.], [-2., 2.]]);

    // // u.add(&v);
    // println!("{}", u+v);

    // // println!("{}", u);
    // // [8.0, 6.0]
    // // [1.0, 6.0]

    // let mut u = Matrix::from([[1., 2.], [3., 4.]]);

    // let v = Matrix::from([[7., 4.], [-2., 2.]]);

    // u.sub(&v);
    // // println!("{}", u);
    // println!("{}", u+v);
    // // [-6.0, -2.0]
    // // [5.0, 2.0]

    // let mut u = Matrix::from([[1., 2.], [3., 4.]]);

    // // u.scl(2.);
    // // println!("{}", u);
    // println!("{}", u*2.);
    // // [2.0, 4.0]
    // // [6.0, 8.0]

    // println!("{}", Vector::zero());

    // // Linear Combination
    // let e1 = Vector::from([1., 0., 0.]);
    // let e2 = Vector::from([0., 1., 0.]);
    // let e3 = Vector::from([0., 0., 1.]);

    // let v1 = Vector::from([1., 2., 3.]);
    // let v2 = Vector::from([0., 10., -100.]);

    // println!(
    //     "{}",
    //     linear_combination(&mut [e1, e2, e3], &[10., -2., 0.5])
    // );
    // // [10.]
    // // [-2.]
    // // [0.5]

    // println!("{}", linear_combination(&mut [v1, v2], &[10., -2.]));
    // // [10.][0.][230.]

    // let  u = Vector::from([0., 0., 0.]);
    // println!("{}, {}, {}", u.norm_1(), u.norm(), u.norm_inf());
    // // 0.0, 0.0, 0.0
    // let  u = Vector::from([1., 2., 3.]);
    // println!("{}, {}, {}", u.norm_1(), u.norm(), u.norm_inf());
    // // 6.0, 3.74165738, 3.0
    // let  u = Vector::from([-1., -2.]);
    // println!("{}, {}, {}", u.norm_1(), u.norm(), u.norm_inf());
    // // 3.0, 2.236067977, 2.0

    // let u = Vector::from([1., 0.]);
    // let v = Vector::from([1., 0.]);
    // println!("{}", angle_cos(&u, &v));
    // // 1.0
    // let u = Vector::from([1., 0.]);
    // let v = Vector::from([0., 1.]);
    // println!("{}", angle_cos(&u, &v));
    // // 0.0
    // let u = Vector::from([-1., 1.]);
    // let v = Vector::from([ 1., -1.]);
    // println!("{}", angle_cos(&u, &v));
    // // -1.0
    // let u = Vector::from([2., 1.]);
    // let v = Vector::from([4., 2.]);
    // println!("{}", angle_cos(&u, &v));
    // // 1.0
    // let u = Vector::from([1., 2., 3.]);
    // let v = Vector::from([4., 5., 6.]);
    // println!("{}", angle_cos(&u, &v));
    // // 0.974631846

    // let u = Vector::from([0., 0., 1.]);
    // let v = Vector::from([1., 0., 0.]);
    // println!("{}", cross_product(&u, &v));
    // // [0.]
    // // [1.]
    // // [0.]
    // let u = Vector::from([1., 2., 3.]);
    // let v = Vector::from([4., 5., 6.]);
    // println!("{}", cross_product(&u, &v));
    // // [-3.]
    // // [6.]
    // // [-3.]
    // let u = Vector::from([4., 2., -3.]);
    // let v = Vector::from([-2., -5., 16.]);
    // println!("{}", cross_product(&u, &v));
    // // [17.]
    // // [-58.]
    // // [-16.]

    // let mut u = Matrix::from([[1., 0.], [0., 1.]]);
    // let v = Vector::from([4., 2.]);
    // println!("{}", u.mul_vec(v));
    // // [4.]
    // // [2.]
    // let mut u = Matrix::from([[2., 0.], [0., 2.]]);
    // let v = Vector::from([4., 2.]);
    // println!("{}", u.mul_vec(v));
    // // [8.]
    // // [4.]
    // let mut u = Matrix::from([[2., -2.], [-2., 2.]]);
    // let v = Vector::from([4., 2.]);
    // println!("{}", u.mul_vec(v));
    // // [4.]
    // // [-4.]

    // let mut u = Matrix::from([
    //     [1., 0.],
    // [0., 1.], ]);
    // let v = Matrix::from([
    // [1., 0.],
    // [0., 1.],]);

    // println!("{}", u.mul_mat(v));
    // // [1., 0.]
    // // [0., 1.]
    // let mut u = Matrix::from([
    //     [1., 0.],
    // [0., 1.],
    // ]);
    // let v = Matrix::from([
    // [2., 1.],
    // [4., 2.], ]);
    // println!("{}", u.mul_mat(v));
    // // [2., 1.]
    // // [4., 2.]

    // let mut u = Matrix::from([
    //     [3., -5.],
    // [6., 8.], ]);
    // let v = Matrix::from([
    //     [2., 1.],
    // [4., 2.],
    // ]);
    // println!("{}", u.mul_mat(v));
    // // [-14., -7.]
    // // [44., 22.]

    // let u = Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
    // println!("{}", u.row_echelon());
    // // [1.0, 0.0, 0.0]
    // // [0.0, 1.0, 0.0]
    // // [0.0, 0.0, 1.0]
    // let u = Matrix::from([[1., 2.], [3., 4.]]);
    // println!("{}", u.row_echelon());
    // // [1.0, 0.0]
    // // [0.0, 1.0]

    // let u = Matrix::from([
    //     [1., 2.],
    // [2., 4.], ]);
    // println!("{}", u.row_echelon());
    // // [1.0, 2.0]
    // // [0.0, 0.0]
    // let u = Matrix::from([
    //     [8., 5., -2., 4., 28.],
    //     [4., 2.5, 20., 4., -4.],
    //     [8., 5., 1., 4., 17.],
    // ]);
    // println!("{}", u.row_echelon());
    // // [1.0, 0.625, 0.0, 0.0, -12.1666667]
    // // [0.0, 0.0, 1.0, 0.0, -3.6666667]
    // // [0.0, 0.0, 0.0, 1.0, 29.5 ]
    //

    let u = Matrix::from([[1., -1.], [-1., 1.]]);
    println!("{}", u.determinant());
    // 0.0
    let u = Matrix::from([[2., 0., 0.], [0., 2., 0.], [0., 0., 2.]]);
    println!("{}", u.determinant());
    // 8.0
    let u = Matrix::from([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.]]);
    println!("{}", u.determinant());
    // -174.0
    let u = Matrix::from([
        [8., 5., -2., 4.],
        [4., 2.5, 20., 4.],
        [8., 5., 1., 4.],
        [28., -4., 17., 1.],
    ]);
    println!("{}", u.determinant());
    // 1032
}
