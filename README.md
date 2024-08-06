# Mini Matrix Library

This project is a Rust library for basic linear algebra operations, including vector and matrix manipulations.
It provides functionalities to perform various exercises related to vectors and matrices.

## Features

- Vector and Matrix Operations
- Linear Combination
- Linear Interpolation (Lerp)
- Dot Product
- Norms (1-norm, 2-norm, ∞-norm)
- Angle Cosine
- Cross Product
- Matrix Multiplication
- Trace
- Transpose
- Reduced Row Echelon Form (RREF)
- Determinant
- Inverse
- Rank

## Usage

The `main` function allows you to run different exercises based on a command-line argument specifying the exercise number.

### Running Exercises

To run a specific exercise, use the following command:

```bash
cargo run <exercise_number>


The crate is currently on [version 0.1.0](https://crates.io/crates/mini_matrix).

---


## Summary

`mini_matrix` is a lightweight linear algebra library written in Rust, designed to provide fundamental matrix and vector operations without heavy external dependencies. This project serves as an introduction to both Rust programming and linear algebra concepts, specifically tailored as a learning exercise from the 42 cursus.

#### Contributing

Contributions are welcome! Please see the [contribution guidelines](CONTRIBUTING.md) for more details.

---

## Implementation

This project is implemented using [Rust](https://www.rust-lang.org/), a systems programming language known for its performance and safety features.

---

### Installation

The library is most easily used with [cargo](http://doc.crates.io/guide.html). Simply include the following in your `Cargo.toml` file:

```toml
[dependencies]
mini_matrix = "0.1.0"

```
or use the following command:

```bash
cargo add mini_matrix
```

And then import the library using:

```rust
#[macro_use]
extern crate mini_matrix;
```

Then import the modules and you're done!

```rust
use mini_matrix::Matrix;

// Create a 2x2 matrix:
let a = Matrix::from([
    1.0, 2.0,
    3.0, 4.0,
]);

// Create a 2x3 matrix:
let b = Matrix::([
    1.0, 2.0, 3.0,
    4.0, 5.0, 6.0,
]);

let c = a * b; // Matrix product of a and b

// Construct the product of `a` and `b` using the `matrix!` macro:
let expected = matrix![9.0, 12.0, 15.0;
                       19.0, 26.0, 33.0];

assert_eq!(c, expected);
```
