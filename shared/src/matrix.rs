use std::{array, ops::Neg};

use num_traits::{NumCast, NumOps, One, Zero};

use crate::vector::Vector;

pub type Matrix<T, const NUM_ROWS: usize, const NUM_COLUMNS: usize> = [[T; NUM_COLUMNS]; NUM_ROWS];

pub fn vector_mul<
    T: Copy + Zero + NumOps + NumCast,
    const NUM_ROWS: usize,
    const NUM_COLUMNS: usize,
>(
    lhs: Matrix<T, NUM_ROWS, NUM_COLUMNS>,
    rhs: [T; NUM_COLUMNS],
) -> [T; NUM_ROWS] {
    lhs.map(|row| row.dot(rhs))
}

#[expect(
    clippy::module_name_repetitions,
    reason = "module name indicates lhs of matrix multiplication"
)]
pub fn matrix_mul<
    T: Copy + Zero + NumOps + NumCast,
    const LHS_NUM_ROWS: usize,
    const LHS_NUM_COLUMNS: usize,
    const RHS_NUM_COLUMNS: usize,
>(
    lhs: Matrix<T, LHS_NUM_ROWS, LHS_NUM_COLUMNS>,
    rhs: Matrix<T, LHS_NUM_COLUMNS, RHS_NUM_COLUMNS>,
) -> Matrix<T, LHS_NUM_ROWS, RHS_NUM_COLUMNS> {
    lhs.map(|row| array::from_fn(|index| row.dot(column(rhs, index))))
}

pub fn column<T: Copy, const NUM_ROWS: usize, const NUM_COLUMNS: usize>(
    matrix: Matrix<T, NUM_ROWS, NUM_COLUMNS>,
    index: usize,
) -> [T; NUM_ROWS] {
    array::from_fn(|row| matrix[row][index])
}

#[must_use]
pub fn identity<T: Zero + One + From<bool>, const SIZE: usize>() -> Matrix<T, SIZE, SIZE> {
    array::from_fn(|row| array::from_fn(|column| (row == column).into()))
}

#[must_use]
pub fn quarter_rotation_around_x_axis<T: Zero + One + Neg<Output = T>>() -> Matrix<T, 3, 3> {
    [
        [T::one(), T::zero(), T::zero()],
        [T::zero(), T::zero(), -T::one()],
        [T::zero(), T::one(), T::zero()],
    ]
}

#[must_use]
pub fn quarter_rotation_around_y_axis<T: Zero + One + Neg<Output = T>>() -> Matrix<T, 3, 3> {
    [
        [T::zero(), T::zero(), T::one()],
        [T::zero(), T::one(), T::zero()],
        [-T::one(), T::zero(), T::zero()],
    ]
}

#[must_use]
pub fn quarter_rotation_around_z_axis<T: Zero + One + Neg<Output = T>>() -> Matrix<T, 3, 3> {
    [
        [T::zero(), -T::one(), T::zero()],
        [T::one(), T::zero(), T::zero()],
        [T::zero(), T::zero(), T::one()],
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_mul() {
        let matrix = [[1, -1, 2], [0, -3, 1]];
        let vector = [2, 1, 0];
        let actual = super::vector_mul(matrix, vector);
        let expected = [1, -3];
        assert_eq!(actual, expected);
    }

    #[test]
    fn matrix_mul() {
        let lhs = [[1, 2, 2], [2, 3, 4]];
        let rhs = [[2, 5], [6, 1], [4, 3]];
        let actual = super::matrix_mul(lhs, rhs);
        let expected = [[22, 13], [38, 25]];
        assert_eq!(actual, expected);
    }

    #[test]
    fn column() {
        let matrix = [[3, 7], [4, 9]];
        let actual = super::column(matrix, 1);
        let expected = [7, 9];
        assert_eq!(actual, expected);
    }

    #[test]
    fn identity() {
        let actual = super::identity::<isize, 3>();
        let expected = [[1, 0, 0], [0, 1, 0], [0, 0, 1]];
        assert_eq!(actual, expected);
    }

    #[test]
    fn rotation_x_axis() {
        let vector = [1, 2, 3];
        let actual = super::vector_mul(quarter_rotation_around_x_axis(), vector);
        let expected = [1, -3, 2];
        assert_eq!(actual, expected);
    }

    #[test]
    fn rotation_y_axis() {
        let vector = [1, 2, 3];
        let actual = super::vector_mul(quarter_rotation_around_y_axis(), vector);
        let expected = [3, 2, -1];
        assert_eq!(actual, expected);
    }

    #[test]
    fn rotation_z_axis() {
        let vector = [1, 2, 3];
        let actual = super::vector_mul(quarter_rotation_around_z_axis(), vector);
        let expected = [-2, 1, 3];
        assert_eq!(actual, expected);
    }
}
