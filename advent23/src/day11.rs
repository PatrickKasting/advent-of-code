use easy_cast::Cast;
use itertools::Itertools;

use shared::grid::Grid;

type Image = Grid<char>;
type Coordinate = usize;
type Distance = usize;

pub fn first_answer(input: &str) -> String {
    sum_of_distances(input, 2).to_string()
}

pub fn second_answer(input: &str) -> String {
    sum_of_distances(input, 1_000_000).to_string()
}

fn sum_of_distances(input: &str, expansion_factor: usize) -> Distance {
    let image = Image::from(input);
    let empty_rows = indices_of_empty(image.rows());
    let empty_columns = indices_of_empty(image.columns());

    let galaxies = image
        .iter_row_major()
        .filter_map(|(position, &element)| is_galaxy(element).then_some(position));
    let distances = galaxies.combinations(2).map(|pair| {
        let vertical_distance = one_dimensional_distance(
            [pair[0][0].cast(), pair[1][0].cast()],
            &empty_rows,
            expansion_factor,
        );
        let horizontal_distance = one_dimensional_distance(
            [pair[0][1].cast(), pair[1][1].cast()],
            &empty_columns,
            expansion_factor,
        );
        vertical_distance + horizontal_distance
    });
    distances.sum()
}

fn indices_of_empty<'image>(
    lines: impl Iterator<Item = impl Iterator<Item = &'image char>>,
) -> Vec<Coordinate> {
    lines
        .enumerate()
        .filter_map(|(line_index, mut line)| {
            line.all(|&element| !is_galaxy(element))
                .then_some(line_index.cast())
        })
        .collect_vec()
}

fn one_dimensional_distance(
    mut points: [Coordinate; 2],
    expensive_steps: &[Coordinate],
    expensive_step_cost: Distance,
) -> Distance {
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
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 11;

    #[test]
    fn first_examples() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 374);
    }

    #[test]
    fn first_answer_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 9_521_776);
    }

    #[test]
    fn second_answer_input() {
        test_on_input(
            DAY,
            Puzzle::Second,
            Input::PuzzleInput,
            553_224_415_344_usize,
        );
    }
}
