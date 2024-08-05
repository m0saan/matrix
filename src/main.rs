#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use num_complex::Complex;
use std::env;

use matrix::{angle_cos, cross_product, linear_combination, Matrix, Vector};

fn main() {
    // Collect command-line arguments into a vector
    let args: Vec<String> = env::args().collect();

    // Check if there is at least one argument (the first is the program name)
    if args.len() < 2 {
        eprintln!("Usage: {} <exercise_number>", args[0]);
        std::process::exit(1);
    }

    // Parse the exercise number from the command line argument
    let exercise_number: usize = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Error: Exercise number must be a positive integer.");
            std::process::exit(1);
        }
    };

    // Execute the corresponding exercise
    match exercise_number {
        1 => ex01(),
        2 => ex02(),
        3 => ex03(),
        4 => ex04(),
        5 => ex05(),
        6 => ex06(),
        7 => ex07(),
        8 => ex08(),
        9 => ex09(),
        10 => ex10(),
        11 => ex11(),
        12 => ex12(),
        13 => ex13(),
        _ => {
            eprintln!("Error: Exercise number must be between 1 and 13.");
            std::process::exit(1);
        }
    }
}

// Define the exercises as separate functions
fn ex01() {
    println!("Running Exercise 1: Vector and Matrix Operations...");

    let mut u = Vector::from([2., 3.]);
    let v = Vector::from([5., 7.]);
    u.add(&v);
    println!("{}", u);
    // [7.0]
    // [10.0]
    let mut u = Vector::from([2., 3.]);
    let v = Vector::from([5., 7.]);
    u.sub(&v);
    println!("{}", u);
    // [-3.0]
    // [-4.0]
    let mut u = Vector::from([2., 3.]);
    u.scl(2.);
    println!("{}", u);
    // [4.0]
    // [6.0]
    let mut u = Matrix::from([[1., 2.], [3., 4.]]);
    let v = Matrix::from([[7., 4.], [-2., 2.]]);
    u.add(&v);

    println!("{}", u);
    // [8.0, 6.0]
    // [1.0, 6.0]
    let mut u = Matrix::from([[1., 2.], [3., 4.]]);
    let v = Matrix::from([[7., 4.], [-2., 2.]]);
    u.sub(&v);
    println!("{}", u);
    // [-6.0, -2.0]
    // [5.0, 2.0]
    let mut u = Matrix::from([[1., 2.], [3., 4.]]);
    u.scl(2.);
    println!("{}", u);
    // [2.0, 4.0]
    // [6.0, 8.0]
}

fn ex02() {
    println!("Running Exercise 2: Linear Combination...");
    let e1 = Vector::from([1., 0., 0.]);
    let e2 = Vector::from([0., 1., 0.]);
    let e3 = Vector::from([0., 0., 1.]);
    let v1 = Vector::from([1., 2., 3.]);
    let v2 = Vector::from([0., 10., -100.]);
    println!(
        "{}",
        linear_combination(&mut [e1, e2, e3], &[10., -2., 0.5])
    );
    // [10.]
    // [-2.]
    // [0.5]
    println!("{}", linear_combination(&mut [v1, v2], &[10., -2.]));
    // [10.]
    // [0.]
    // [230.]
}

fn ex03() {
    println!("Running Exercise 3: Dot Product...");
    let u = Vector::from([0., 0.]);
    let v = Vector::from([1., 1.]);
    println!("{}", u.dot(&v));
    // 0.0
    let u = Vector::from([1., 1.]);
    let v = Vector::from([1., 1.]);
    println!("{}", u.dot(&v));
    // 2.0
    let u = Vector::from([-1., 6.]);
    let v = Vector::from([3., 2.]);
    println!("{}", u.dot(&v));
    // 9.0
}

fn ex04() {
    println!("Running Exercise 4: Norms...");
    let u = Vector::from([0., 0., 0.]);
    println!("{}, {}, {}", u.norm_1(), u.norm(), u.norm_inf());
    // 0.0, 0.0, 0.0
    let u = Vector::from([1., 2., 3.]);
    println!("{}, {}, {}", u.norm_1(), u.norm(), u.norm_inf());
    // 6.0, 3.74165738, 3.0
    let u = Vector::from([-1., -2.]);
    println!("{}, {}, {}", u.norm_1(), u.norm(), u.norm_inf());
    // 3.0, 2.236067977, 2.0
}

fn ex05() {
    println!("Running Exercise 5: Angle Cos...");
    let u = Vector::from([1., 0.]);
    let v = Vector::from([1., 0.]);
    println!("{}", angle_cos(&u, &v));
    // 1.0
    let u = Vector::from([1., 0.]);
    let v = Vector::from([0., 1.]);
    println!("{}", angle_cos(&u, &v));
    // 0.0
    let u = Vector::from([-1., 1.]);
    let v = Vector::from([1., -1.]);
    println!("{}", angle_cos(&u, &v));
    // -1.0
    let u = Vector::from([2., 1.]);
    let v = Vector::from([4., 2.]);
    println!("{}", angle_cos(&u, &v));
    // 1.0
    let u = Vector::from([1., 2., 3.]);
    let v = Vector::from([4., 5., 6.]);
    println!("{}", angle_cos(&u, &v));
    // 0.974631846
}

fn ex06() {
    println!("Running Exercise 6: Cross Product...");
    let u = Vector::from([0., 0., 1.]);
    let v = Vector::from([1., 0., 0.]);
    println!("{}", cross_product(&u, &v));
    // [0.]
    // [1.]
    // [0.]
    let u = Vector::from([1., 2., 3.]);
    let v = Vector::from([4., 5., 6.]);
    println!("{}", cross_product(&u, &v));
    // [-3.]
    // [6.]
    // [-3.]
    let u = Vector::from([4., 2., -3.]);
    let v = Vector::from([-2., -5., 16.]);
    println!("{}", cross_product(&u, &v));
    // [17.]
    // [-58.]
    // [-16.]
}

fn ex07() {
    println!("Running Exercise 7: Matrix Multiplication...");
    let mut u = Matrix::from([[1., 0.], [0., 1.]]);
    let v = Vector::from([4., 2.]);
    println!("{}", u.mul_vec(&v));
    // [4.]
    // [2.]
    let mut u = Matrix::from([[2., 0.], [0., 2.]]);
    let v = Vector::from([4., 2.]);
    println!("{}", u.mul_vec(&v));
    // [8.]
    // [4.]
    let mut u = Matrix::from([[2., -2.], [-2., 2.]]);
    let v = Vector::from([4., 2.]);
    println!("{}", u.mul_vec(&v));
    // [4.]
    // [-4.]
    let mut u = Matrix::from([[1., 0.], [0., 1.]]);
    let v = Matrix::from([[1., 0.], [0., 1.]]);

    println!("{}", u.mul_mat(&v));
    // [1., 0.]
    // [0., 1.]
    let mut u = Matrix::from([[1., 0.], [0., 1.]]);
    let v = Matrix::from([[2., 1.], [4., 2.]]);
    println!("{}", u.mul_mat(&v));
    // [2., 1.]
    // [4., 2.]
    let mut u = Matrix::from([[3., -5.], [6., 8.]]);
    let v = Matrix::from([[2., 1.], [4., 2.]]);
    println!("{}", u.mul_mat(&v));
    // [-14., -7.]
    // [44., 22.]
}

fn ex08() {
    println!("Running Exercise 8: Trace");
    let u = Matrix::from([[1., 0.], [0., 1.]]);
    println!("{}", u.trace());
    // 2.0
    let u = Matrix::from([[2., -5., 0.], [4., 3., 7.], [-2., 3., 4.]]);
    println!("{}", u.trace());
    // 9.0
    let u = Matrix::from([[-2., -8., 4.], [1., -23., 4.], [0., 6., 4.]]);
    println!("{}", u.trace());
    // -21.0
}

fn ex09() {
    println!("Running Exercise 9: Transpose");
    let mut m = Matrix::from([[1., 2., 3.], [4., 5., 6.], [7., 8., 9.]]);
    println!("{}", m.transpose());
    // [[1., 4., 7.]
    // [2., 5., 8.]
    // [3., 6., 9.]]

    let mut u = Matrix::from([[1., 0.], [0., 1.]]);
    println!("{}", u.transpose());
    // [[1., 0.]
    // [0., 1.]]

    let mut u = Matrix::from([[2., -5., 0.], [4., 3., 7.], [-2., 3., 4.]]);
    println!("{}", u.transpose());
    // [[2., 4., -2.]
    // [-5., 3., 3.]
    // [0., 7., 4.]]
}

fn ex10() {
    println!("Running Exercise 10: Reduced Row Echelon Form...");
    let u = Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
    println!("{}", u.row_echelon());
    // [1.0, 0.0, 0.0]
    // [0.0, 1.0, 0.0]
    // [0.0, 0.0, 1.0]
    let u = Matrix::from([[1., 2.], [3., 4.]]);
    println!("{}", u.row_echelon());
    // [1.0, 0.0]
    // [0.0, 1.0]
    let u = Matrix::from([[1., 2.], [2., 4.]]);
    println!("{}", u.row_echelon());
    // [1.0, 2.0]
    // [0.0, 0.0]
    let u = Matrix::from([
        [8., 5., -2., 4., 28.],
        [4., 2.5, 20., 4., -4.],
        [8., 5., 1., 4., 17.],
    ]);
    println!("{}", u.row_echelon());
    // [1.0, 0.625, 0.0, 0.0, -12.1666667]
    // [0.0, 0.0, 1.0, 0.0, -3.6666667]
    // [0.0, 0.0, 0.0, 1.0, 29.5 ]
}

fn ex11() {
    println!("Running Exercise 11: Determinant...");
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

fn ex12() {
    println!("Running Exercise 12: Inverse...");
    let u = Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
    println!("{}", u.inverse().unwrap());
    // [1.0, 0.0, 0.0]
    // [0.0, 1.0, 0.0]
    // [0.0, 0.0, 1.0]
    let u = Matrix::from([[2., 0., 0.], [0., 2., 0.], [0., 0., 2.]]);
    println!("{}", u.inverse().unwrap());
    // [0.5, 0.0, 0.0]
    // [0.0, 0.5, 0.0]
    // [0.0, 0.0, 0.5]
    let u = Matrix::from([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.]]);
    println!("{}", u.inverse().unwrap());
    // [0.649425287, 0.097701149, -0.655172414]
    // [-0.781609195, -0.126436782, 0.965517241]
    // [0.143678161, 0.074712644, -0.20ss6896552]
}

fn ex13() {
    println!("Running Exercise 13: Rank");
    let u = Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
    println!("{}", u.rank());
    // 3
    let u = Matrix::from([[1., 2., 0., 0.], [2., 4., 0., 0.], [-1., 2., 1., 1.]]);
    println!("{}", u.rank());
    // 2
    let u = Matrix::from([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.], [21., 18., 7.]]);
    println!("{}", u.rank());
    // 3
}
