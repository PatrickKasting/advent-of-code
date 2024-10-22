use std::collections::BTreeSet;

use easy_cast::Cast;
use itertools::Itertools;

use shared::{
    grid::{self, Direction, Grid, Position},
    vector::Vector,
};

type Beam = (Direction, Position);

pub fn first_answer(input: &str) -> String {
    let grid = Grid::from(input);
    let beam = (grid::EAST, [0, 0]);
    number_of_energized_tiles(&grid, beam).to_string()
}

pub fn second_answer(input: &str) -> String {
    let grid = Grid::from(input);
    maximum_number_of_energized_tiles(&grid).to_string()
}

fn number_of_energized_tiles(grid: &Grid<char>, initial_beam: Beam) -> usize {
    let mut energized: BTreeSet<Beam> = BTreeSet::new();
    let mut beams: Vec<Beam> = Vec::from([initial_beam]);
    while let Some(beam @ (direction, position)) = beams.pop() {
        let next_directions = match grid.get(position) {
            None => vec![],
            Some(_) if !energized.insert(beam) => vec![],
            Some('/') => vec![reflection_north_east_diagonal(direction)],
            Some('\\') => vec![reflection_north_west_diagonal(direction)],
            Some('|') if [grid::WEST, grid::EAST].contains(&direction) => {
                vec![grid::NORTH, grid::SOUTH]
            }
            Some('-') if [grid::NORTH, grid::SOUTH].contains(&direction) => {
                vec![grid::WEST, grid::EAST]
            }
            _ => vec![direction],
        };
        for next_direction in next_directions {
            let next_position = position.add(next_direction);
            beams.push((next_direction, next_position));
        }
    }

    energized
        .into_iter()
        .map(|(_, position)| position)
        .counts()
        .len()
}

fn reflection_north_east_diagonal([row, column]: Direction) -> Direction {
    [-column, -row]
}

fn reflection_north_west_diagonal([row, column]: Direction) -> Direction {
    [column, row]
}

fn maximum_number_of_energized_tiles(grid: &Grid<char>) -> usize {
    beams_from_edges(grid)
        .into_iter()
        .map(|beam| number_of_energized_tiles(grid, beam))
        .max()
        .expect("many possible beams from the edges of the grid should exist")
}

fn beams_from_edges(grid: &Grid<char>) -> Vec<Beam> {
    let [grid_height, grid_width] =
        [Grid::height, Grid::width].map(|dimension| dimension(grid).cast());
    let mut beams = Vec::new();
    for row_index in 0..grid_height {
        let from_left = (grid::EAST, [row_index, 0]);
        let from_right = (grid::WEST, [row_index, grid_width - 1]);
        beams.extend([from_left, from_right]);
    }
    for column_index in 0..grid_width.cast() {
        let from_top = (grid::SOUTH, [0, column_index]);
        let from_bottom = (grid::NORTH, [grid_height - 1, column_index]);
        beams.extend([from_top, from_bottom]);
    }
    beams
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 16;

    #[test]
    fn first_answer_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 46);
    }

    #[test]
    fn first_answer_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 8551);
    }

    #[test]
    fn second_answer_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 51);
    }

    #[test]
    fn second_answer_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 8754);
    }
}
