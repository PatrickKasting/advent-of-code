use itertools::Itertools;

use crate::floating::ApproxEq;

type Matrix<const NUM_ROWS: usize, const NUM_COLUMNS: usize> = [[Real; NUM_COLUMNS]; NUM_ROWS];
type Real = f64;

#[must_use]
pub fn solution<const NUM_ROWS: usize, const NUM_COLUMNS: usize>(
    augmented_matrix: Matrix<NUM_ROWS, NUM_COLUMNS>,
) -> Solution {
    let reduced_row_echelon_form = reduced_row_echelon_form(augmented_matrix);
    solution_from_reduced_row_echelon_form(reduced_row_echelon_form)
}

fn reduced_row_echelon_form<const NUM_ROWS: usize, const NUM_COLUMNS: usize>(
    mut matrix: Matrix<NUM_ROWS, NUM_COLUMNS>,
) -> Matrix<NUM_ROWS, NUM_COLUMNS> {
    let mut pivot_row = 0;
    let mut pivot_column = 0;
    while pivot_row < NUM_ROWS && pivot_column < NUM_COLUMNS {
        let (row_with_max_absolute_value, _) = matrix
            .into_iter()
            .map(|row| row[pivot_column].abs())
            .enumerate()
            .skip(pivot_row)
            .max_by(|(_, first), (_, second)| first.total_cmp(second))
            .expect("slice should not be empty because the pivot row is within the matrix");
        let pivot = matrix[row_with_max_absolute_value][pivot_column];
        if pivot.approx_eq(0.0) {
            pivot_column += 1;
            continue;
        }

        [matrix[row_with_max_absolute_value], matrix[pivot_row]] =
            [matrix[pivot_row], matrix[row_with_max_absolute_value]];
        for row in (0..NUM_ROWS).filter(|&row| row != pivot_row) {
            let ratio = matrix[row][pivot_column] / pivot;
            matrix[row][pivot_column] = 0.0;
            for column in pivot_column + 1..NUM_COLUMNS {
                matrix[row][column] -= matrix[pivot_row][column] * ratio;
            }
        }
        for column in pivot_column..NUM_COLUMNS {
            matrix[pivot_row][column] /= pivot;
        }
        (pivot_row, pivot_column) = (pivot_row + 1, pivot_column + 1);
    }
    matrix
}

type Solution = Vec<Vec<Real>>;

fn solution_from_reduced_row_echelon_form<const NUM_ROWS: usize, const NUM_COLUMNS: usize>(
    matrix: Matrix<NUM_ROWS, NUM_COLUMNS>,
) -> Solution {
    let negated_column = |column: usize, num_rows| {
        matrix[0..num_rows]
            .iter()
            .map(|row| -row[column])
            .collect_vec()
    };

    let (mut pivot_row, mut pivot_column) = (0, 0);
    let mut free_parameters = vec![];
    let mut solution = vec![vec![]];
    while pivot_row < NUM_ROWS && pivot_column < NUM_COLUMNS - 1 {
        if matrix[pivot_row][pivot_column].approx_eq(0.0) {
            solution[0].push(0.0);

            let mut vector = negated_column(pivot_column, pivot_row);
            for &parameter in &free_parameters {
                vector.insert(parameter, 0.0);
            }
            vector.push(1.0);
            vector.resize(NUM_COLUMNS - 1, 0.0);
            solution.push(vector);

            free_parameters.push(pivot_column);
            pivot_column += 1;
        } else {
            solution[0].push(matrix[pivot_row][NUM_COLUMNS - 1]);

            (pivot_row, pivot_column) = (pivot_row + 1, pivot_column + 1);
        }
    }
    if pivot_row < NUM_ROWS && matrix[pivot_row][NUM_COLUMNS - 1].approx_eq(1.0) {
        solution.clear();
    }
    solution
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exactly_one_solution() {
        let matrix = [
            [2.0, 1.0, -1.0, 8.0],
            [-3.0, -1.0, 2.0, -11.0],
            [-2.0, 1.0, 2.0, -3.0],
        ];
        let solution = solution(matrix);
        let expected = vec![vec![2.0, 3.0, -1.0]];
        are_solutions_almost_equal(solution, expected);
    }

    #[test]
    fn no_solutions() {
        let matrix = [
            [3.0, 1.0, 7.0, -2.0],
            [-1.0, -3.0, 3.0, 1.0],
            [2.0, 3.0, 0.0, -3.0],
        ];
        let solution = solution(matrix);
        let expected: Solution = vec![];
        are_solutions_almost_equal(solution, expected);
    }

    #[test]
    fn one_free_parameter() {
        let matrix = [
            [1.0, 3.0, 1.0, 9.0],
            [1.0, 1.0, -1.0, 1.0],
            [3.0, 11.0, 5.0, 35.0],
        ];
        let solution = solution(matrix);
        let expected = vec![vec![-3.0, 4.0, 0.0], vec![2.0, -1.0, 1.0]];
        are_solutions_almost_equal(solution, expected);
    }

    #[test]
    fn two_free_parameters() {
        let matrix = [
            [1.0, 3.0, 2.0, 4.0, 5.0, 9.0],
            [2.0, 6.0, 4.0, 3.0, 5.0, 3.0],
            [3.0, 8.0, 6.0, 7.0, 6.0, 5.0],
            [4.0, 14.0, 8.0, 10.0, 22.0, 32.0],
        ];
        let solution = solution(matrix);
        let expected = vec![
            vec![-24.0, 7.0, 0.0, 3.0, 0.0],
            vec![-2.0, 0.0, 1.0, 0.0, 0.0],
            vec![11.0, -4.0, 0.0, -1.0, 1.0],
        ];
        are_solutions_almost_equal(solution, expected);
    }

    fn are_solutions_almost_equal(first: Solution, second: Solution) -> bool {
        for (left_vector, right_vector) in first.into_iter().zip_eq(second) {
            for (left_number, right_number) in left_vector.into_iter().zip_eq(right_vector) {
                if !left_number.approx_eq(right_number) {
                    return false;
                }
            }
        }
        true
    }
}
