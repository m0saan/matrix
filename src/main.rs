use matrix::{Vector, linear_combination};

fn main() {
    let mut u = Vector::from(&[2., 3.]);
    let v = Vector::from(&[5., 7.]);
    u.add(&v);
    println!("{}", u);
    // [7.0]
    // [10.0]
    //

    let mut u = Vector::from(&[2., 3.]);
    let v = Vector::from(&[5., 7.]);
    u.sub(&v);
    println!("{}", u);
    // [-3.0]
    // [-4.0]

    let mut u = Vector::from(&[2., 3.]);
    u.scl(2.);
    println!("{}", u);
    // [4.0]
    // [6.0]

    // let mut u = Matrix::from(&[&[1, 2], &[3, 4]]);

    // u.scl(2);

    // println!("{}", u);
    // // [2.0, 4.0]
    // // [6.0, 8.0]

    // Linear Combination
    let e1 = Vector::from(&[1., 0., 0.]);
    let e2 = Vector::from(&[0., 1., 0.]);
    let e3 = Vector::from(&[0., 0., 1.]);

    let v1 = Vector::from(&[1., 2., 3.]);
    let v2 = Vector::from(&[0., 10., -100.]);

    println!(
        "{}",
        linear_combination(&mut [e1, e2, e3], &[10., -2., 0.5])
    );
    // [10.]
    // [-2.]
    // [0.5]

    println!("{}", linear_combination(&mut [v1, v2], &[10., -2.]));
    // [10.][0.][230.]


    // println!("{}", lerp::<f32>(0., 1., 0.));
    // // 0.0
    // println!("{}", lerp(0., 1., 1.));
    // // 1.0
    // println!("{}", lerp(0., 1., 0.5));
    // // 0.5
    // println!("{}", lerp(21., 42., 0.3));
    // // 27.3
    // println!(
    //     "{}",
    //     lerp(Vector::from([2., 1.]), Vector::from(4., 2.), 0.3)
    // );
    // [2.6]
    // [1.3]
    // println!("{}", lerp(Matrix::from([[2., 1.], [3., 4.]]), Matrix::from([[20.,
    // 10.], [30., 40.]]), 0.5));
    // [[11., 5.5]
    // [16.5, 22.]]
}
