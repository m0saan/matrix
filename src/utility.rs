use crate::Matrix;
use num::Num;
use std::ops::{AddAssign, Mul, Neg};


impl<T, const M: usize, const N: usize> Matrix<T, M, N>
where
    T: Copy + Default + Mul + Num + Neg<Output = T> + AddAssign + PartialEq,
{
    #[allow(dead_code)]
    fn cofactor3x3(&self, row: usize, col: usize) -> Matrix<T, 3, 3> {
        let mut cofactor_matrix = Matrix::<T, 3, 3>::zero();
        let mut row_index = 0;

        for r in 0..row {
            if r == row {
                continue;
            }
            let mut col_index = 0;
            for c in 0..col {
                if c == col {
                    continue;
                }
                cofactor_matrix[(row_index, col_index)] = self[(r, c)];
                col_index += 1;
            }
            row_index += 1;
        }
        cofactor_matrix
    }

    pub fn cofactor2x2(&self, row: usize, col: usize) -> Matrix<T, 2, 2> {
        let mut cofactor_matrix = Matrix::<T, 2, 2>::zero();
        let mut row_index = 0;

        for r in 0..M {
            if r == row {
                continue;
            }
            let mut col_index = 0;
            for c in 0..N {
                if c == col {
                    continue;
                }
                cofactor_matrix[(row_index, col_index)] = self[(r, c)];
                col_index += 1;
            }
            row_index += 1;
        }
        cofactor_matrix
    }

    pub fn cofactor1x1(&self, row: usize, col: usize) -> Matrix<T, 1, 1> {
        let mut cofactor_matrix = Matrix::<T, 1, 1>::zero();
        let mut row_index = 0;

        for r in 0..M {
            if r == row {
                continue;
            }
            let mut col_index = 0;
            for c in 0..N {
                if c == col {
                    continue;
                }
                cofactor_matrix[(row_index, col_index)] = self[(r, c)];
                col_index += 1;
            }
            row_index += 1;
        }
        cofactor_matrix
    }
}
