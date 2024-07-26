use matrix::Vector;

fn main() {
    let mut u = Vector::from(&[2., 3.]);
    let v = Vector::from(&[5., 7.]);
    u.add(&v);
    println!("{:?}", u);
    // [7.0]
    // [10.0]
}
