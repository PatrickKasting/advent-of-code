use itertools::Itertools;

use crate::grid::Grid;

type Image = Grid<char>;

fn is_galaxy(char: &char) -> bool {
    *char == '#'
}

fn one_dimensinoal_distance(
    expensive_steps: &[isize],
    expensive_step_cost: isize,
    mut points: [isize; 2],
) -> isize {
    points.sort();
    let [first, second] = points;
    let number_of_expensive_steps = expensive_steps
        .iter()
        .filter(|expensive_step| (first..second).contains(expensive_step))
        .count();
    (second - first) + number_of_expensive_steps as isize * (expensive_step_cost - 1)
}

fn sum_of_distances(input: &str, expansion_factor: isize) -> isize {
    let image = Image::from(input);
    let galaxies = image.positions(is_galaxy);
    let empty_rows = image.row_indices(|position| !is_galaxy(position));
    let empty_columns = image.column_indices(|position| !is_galaxy(position));

    let distances = galaxies.iter().combinations(2).map(|pair| {
        let vertical_distance = one_dimensinoal_distance(
            &empty_rows,
            expansion_factor,
            [pair[0].row(), pair[1].row()],
        );
        let horizontal_distance = one_dimensinoal_distance(
            &empty_columns,
            expansion_factor,
            [pair[0].column(), pair[1].column()],
        );
        vertical_distance + horizontal_distance
    });
    distances.sum()
}

pub fn first(input: String) -> String {
    sum_of_distances(&input, 2).to_string()
}

pub fn second(input: String) -> String {
    sum_of_distances(&input, 1_000_000).to_string()
}

#[cfg(test)]
mod tests {
    use crate::{tests::*, Input, Puzzle};

    const DAY: usize = 11;

    #[test]
    fn first_examples() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 374);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::Real, 9521776);
    }

    #[test]
    fn second_input() {
        test_on_input(DAY, Puzzle::Second, Input::Real, 553224415344isize);
    }
}
