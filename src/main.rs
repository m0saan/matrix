use matrix::Vector;

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
}
