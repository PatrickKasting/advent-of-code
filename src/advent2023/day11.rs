use itertools::Itertools;

use crate::{grid::Grid, utilities::as_isize};

type Image = Grid<char>;

fn is_galaxy(element: char) -> bool {
    element == '#'
}

fn one_dimensinoal_distance(
    expensive_steps: &[isize],
    expensive_step_cost: isize,
    mut points: [isize; 2],
) -> isize {
    points.sort_unstable();
    let [first, second] = points;
    let number_of_expensive_steps = expensive_steps
        .iter()
        .filter(|expensive_step| (first..second).contains(expensive_step))
        .count();
    (second - first) + as_isize(number_of_expensive_steps) * (expensive_step_cost - 1)
}

fn sum_of_distances(input: &str, expansion_factor: isize) -> isize {
    let image = Image::from(input);
    let empty_rows = image
        .rows()
        .enumerate()
        .filter_map(|(row_index, mut row)| {
            row.all(|&position| !is_galaxy(position))
                .then_some(as_isize(row_index))
        })
        .collect_vec();
    let empty_columns = image
        .columns()
        .enumerate()
        .filter_map(|(column_index, mut column)| {
            column
                .all(|&position| !is_galaxy(position))
                .then_some(as_isize(column_index))
        })
        .collect_vec();

    let galaxies = image
        .iter_row_major()
        .filter_map(|(position, &element)| is_galaxy(element).then_some(position));
    let distances = galaxies.combinations(2).map(|pair| {
        let vertical_distance = one_dimensinoal_distance(
            &empty_rows[..],
            expansion_factor,
            [pair[0].row(), pair[1].row()],
        );
        let horizontal_distance = one_dimensinoal_distance(
            &empty_columns[..],
            expansion_factor,
            [pair[0].column(), pair[1].column()],
        );
        vertical_distance + horizontal_distance
    });
    distances.sum()
}

pub fn first(input: &str) -> String {
    sum_of_distances(input, 2).to_string()
}

pub fn second(input: &str) -> String {
    sum_of_distances(input, 1_000_000).to_string()
}

#[cfg(test)]
mod tests {
    use super::super::tests::test_on_input;
    use crate::{Input, Puzzle};

    const DAY: usize = 11;

    #[test]
    fn first_examples() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 374);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 9_521_776);
    }

    #[test]
    fn second_input() {
        test_on_input(
            DAY,
            Puzzle::Second,
            Input::PuzzleInput,
            553_224_415_344_isize,
        );
    }
}
