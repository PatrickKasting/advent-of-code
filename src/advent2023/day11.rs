use itertools::Itertools;

use crate::data_structures::grid::Grid;

type Image = Grid<char>;

pub fn first(input: &str) -> String {
    sum_of_distances(input, 2).to_string()
}

pub fn second(input: &str) -> String {
    sum_of_distances(input, 1_000_000).to_string()
}

#[allow(clippy::cast_sign_loss)]
fn sum_of_distances(input: &str, expansion_factor: usize) -> usize {
    let image = Image::from(input);
    let empty_rows = indices_of_empty(image.rows());
    let empty_columns = indices_of_empty(image.columns());

    let galaxies = image
        .iter_row_major()
        .filter_map(|(position, &element)| is_galaxy(element).then_some(position));
    let distances = galaxies.combinations(2).map(|pair| {
        let vertical_distance = one_dimensional_distance(
            [pair[0].row() as usize, pair[1].row() as usize],
            &empty_rows[..],
            expansion_factor,
        );
        let horizontal_distance = one_dimensional_distance(
            [pair[0].column() as usize, pair[1].column() as usize],
            &empty_columns[..],
            expansion_factor,
        );
        vertical_distance + horizontal_distance
    });
    distances.sum()
}

fn indices_of_empty<'image>(
    lines: impl Iterator<Item = impl Iterator<Item = &'image char>>,
) -> Vec<usize> {
    lines
        .enumerate()
        .filter_map(|(line_index, mut line)| {
            line.all(|&element| !is_galaxy(element))
                .then_some(line_index)
        })
        .collect_vec()
}

fn one_dimensional_distance(
    mut points: [usize; 2],
    expensive_steps: &[usize],
    expensive_step_cost: usize,
) -> usize {
    points.sort_unstable();
    let [first, second] = points;
    let number_of_expensive_steps = expensive_steps
        .iter()
        .filter(|expensive_step| (first..second).contains(expensive_step))
        .count();
    (second - first) + number_of_expensive_steps * (expensive_step_cost - 1)
}

fn is_galaxy(element: char) -> bool {
    element == '#'
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
