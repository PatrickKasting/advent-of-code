use itertools::Itertools;

use crate::floating::ApproxEq;

type Matrix<const NUM_ROWS: usize, const NUM_COLUMNS: usize> = [[Real; NUM_COLUMNS]; NUM_ROWS];
type SolutionSet = Option<(Origin, Vec<Direction>)>;
type Origin = Vec<Real>;
type Direction = Vec<Real>;
type Real = f64;

#[must_use]
pub fn solution_set<const NUM_ROWS: usize, const NUM_COLUMNS: usize>(
    augmented_matrix: Matrix<NUM_ROWS, NUM_COLUMNS>,
) -> SolutionSet {
    let reduced_row_echelon_form = reduced_row_echelon_form(augmented_matrix);
    solution_from_reduced_row_echelon_form(reduced_row_echelon_form)
}

fn reduced_row_echelon_form<const NUM_ROWS: usize, const NUM_COLUMNS: usize>(
    mut matrix: Matrix<NUM_ROWS, NUM_COLUMNS>,
) -> Matrix<NUM_ROWS, NUM_COLUMNS> {
    let [mut pivot_row, mut pivot_column] = [0, 0];
    while pivot_row < NUM_ROWS && pivot_column < NUM_COLUMNS {
        let row_with_max_abs_value = row_with_max_abs_value(matrix, pivot_row, pivot_column);
        let pivot = matrix[row_with_max_abs_value][pivot_column];
        if pivot.approx_eq(0.0) {
            pivot_column += 1;
            continue;
        }

        [matrix[row_with_max_abs_value], matrix[pivot_row]] =
            [matrix[pivot_row], matrix[row_with_max_abs_value]];
        subtract_pivot_row_from_other_rows(&mut matrix, pivot_row, pivot_column, pivot);
        divide_pivot_row_by_pivot(&mut matrix, pivot_row, pivot_column, pivot);
        [pivot_row, pivot_column] = [pivot_row + 1, pivot_column + 1];
    }
    matrix
}

fn row_with_max_abs_value<const NUM_ROWS: usize, const NUM_COLUMNS: usize>(
    matrix: Matrix<NUM_ROWS, NUM_COLUMNS>,
    first_row: usize,
    column: usize,
) -> usize {
    let row_with_max_abs_value = matrix[first_row..]
        .iter()
        .map(|row| row[column].abs())
        .position_max_by(|left, right| left.total_cmp(right))
        .expect("first row should be within maxtrix");
    row_with_max_abs_value + first_row
}

fn subtract_pivot_row_from_other_rows<const NUM_ROWS: usize, const NUM_COLUMNS: usize>(
    matrix: &mut Matrix<NUM_ROWS, NUM_COLUMNS>,
    pivot_row: usize,
    pivot_column: usize,
    pivot: f64,
) {
    for row in (0..NUM_ROWS).filter(|&row| row != pivot_row) {
        let ratio = matrix[row][pivot_column] / pivot;
        matrix[row][pivot_column] = 0.0;
        for column in pivot_column + 1..NUM_COLUMNS {
            matrix[row][column] -= matrix[pivot_row][column] * ratio;
        }
    }
}

fn divide_pivot_row_by_pivot<const NUM_ROWS: usize, const NUM_COLUMNS: usize>(
    matrix: &mut Matrix<NUM_ROWS, NUM_COLUMNS>,
    pivot_row: usize,
    pivot_column: usize,
    pivot: f64,
) {
    for column in pivot_column..NUM_COLUMNS {
        matrix[pivot_row][column] /= pivot;
    }
}

fn solution_from_reduced_row_echelon_form<const NUM_ROWS: usize, const NUM_COLUMNS: usize>(
    matrix: Matrix<NUM_ROWS, NUM_COLUMNS>,
) -> SolutionSet {
    let negated_column = |column: usize, num_rows| {
        matrix[0..num_rows]
            .iter()
            .map(|row| -row[column])
            .collect_vec()
    };

    let [mut pivot_row, mut pivot_column] = [0, 0];
    let mut free_parameters = vec![];
    let mut origin = vec![];
    let mut directions = vec![];
    while pivot_row < NUM_ROWS && pivot_column < NUM_COLUMNS - 1 {
        if matrix[pivot_row][pivot_column].approx_eq(0.0) {
            origin.push(0.0);

            let mut direction = negated_column(pivot_column, pivot_row);
            for &parameter in &free_parameters {
                direction.insert(parameter, 0.0);
            }
            direction.push(1.0);
            direction.resize(NUM_COLUMNS - 1, 0.0);
            directions.push(direction);

            free_parameters.push(pivot_column);
            pivot_column += 1;
        } else {
            origin.push(matrix[pivot_row][NUM_COLUMNS - 1]);

            [pivot_row, pivot_column] = [pivot_row + 1, pivot_column + 1];
        }
    }
    if pivot_row < NUM_ROWS && matrix[pivot_row][NUM_COLUMNS - 1].approx_eq(1.0) {
        None
    } else {
        Some((origin, directions))
    }
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
        let solution = solution_set(matrix);
        let expected = Some((vec![2.0, 3.0, -1.0], vec![]));
        assert_solutions_approx_eq(solution, expected);
    }

    #[test]
    fn no_solutions() {
        let matrix = [
            [3.0, 1.0, 7.0, -2.0],
            [-1.0, -3.0, 3.0, 1.0],
            [2.0, 3.0, 0.0, -3.0],
        ];
        let solution = solution_set(matrix);
        let expected = None;
        assert_solutions_approx_eq(solution, expected);
    }

    #[test]
    fn one_free_parameter() {
        let matrix = [
            [1.0, 3.0, 1.0, 9.0],
            [1.0, 1.0, -1.0, 1.0],
            [3.0, 11.0, 5.0, 35.0],
        ];
        let solution = solution_set(matrix);
        let expected = Some((vec![-3.0, 4.0, 0.0], vec![vec![2.0, -1.0, 1.0]]));
        assert_solutions_approx_eq(solution, expected);
    }

    #[test]
    fn two_free_parameters() {
        let matrix = [
            [1.0, 3.0, 2.0, 4.0, 5.0, 9.0],
            [2.0, 6.0, 4.0, 3.0, 5.0, 3.0],
            [3.0, 8.0, 6.0, 7.0, 6.0, 5.0],
            [4.0, 14.0, 8.0, 10.0, 22.0, 32.0],
        ];
        let solution = solution_set(matrix);
        let expected = Some((
            vec![-24.0, 7.0, 0.0, 3.0, 0.0],
            vec![
                vec![-2.0, 0.0, 1.0, 0.0, 0.0],
                vec![11.0, -4.0, 0.0, -1.0, 1.0],
            ],
        ));
        assert_solutions_approx_eq(solution, expected);
    }

    fn assert_solutions_approx_eq(left: SolutionSet, right: SolutionSet) {
        match [left, right] {
            [Some((left_origin, left_directions)), Some((right_origin, right_directions))] => {
                assert!(
                    vectors_approx_eq(&left_origin, &right_origin),
                    "solution set origins should be equal:\n  left: {left_origin:?}\n right: {right_origin:?}"
                );

                let direction_index = left_directions
                    .iter()
                    .zip_eq(right_directions.iter())
                    .position(|(left, right)| !vectors_approx_eq(left, right));
                if let Some(index) = direction_index {
                    let left_direction = &left_directions[index];
                    let right_direction = &right_directions[index];
                    panic!("solution set directions at index {index} should be equal:\n  left: {left_direction:?}\n right: {right_direction:?}");
                }
            }
            [None, None] => (),
            [None, Some(_)] => {
                panic!("left solution set should not be empty while right solution set is not")
            }
            [Some(_), None] => {
                panic!("right solution set should not be empty while left solution set is not")
            }
        }
    }

    fn vectors_approx_eq(left: &[Real], right: &[Real]) -> bool {
        left.iter()
            .zip_eq(right)
            .all(|(&left, &right)| left.approx_eq(right))
    }
}
