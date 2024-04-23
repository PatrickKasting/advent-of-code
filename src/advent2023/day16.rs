use std::collections::BTreeSet;

use itertools::Itertools;

use crate::data_structures::grid::{Coordinate, Direction, Grid, Position};

type Beam = (Direction, Position);

fn number_of_energized_tiles(grid: &Grid<char>, initial_beam: Beam) -> usize {
    let mut energized: BTreeSet<Beam> = BTreeSet::new();
    let mut beams: Vec<Beam> = Vec::from([initial_beam]);
    while let Some(beam @ (direction, position)) = beams.pop() {
        let next_directions = match grid.get(position) {
            None => vec![],
            Some(_) if !energized.insert(beam) => vec![],
            Some('/') => vec![direction.reflection_north_east_diagonal()],
            Some('\\') => vec![direction.reflection_north_west_diagonal()],
            Some('|') if [Direction::West, Direction::East].contains(&direction) => {
                vec![Direction::North, Direction::South]
            }
            Some('-') if [Direction::North, Direction::South].contains(&direction) => {
                vec![Direction::West, Direction::East]
            }
            _ => vec![direction],
        };
        for next_direction in next_directions {
            let next_position = position.neighbor(next_direction);
            beams.push((next_direction, next_position));
        }
    }

    energized
        .into_iter()
        .map(|(_, position)| position)
        .counts()
        .len()
}

fn beams_from_edges(grid: &Grid<char>) -> Vec<Beam> {
    #[allow(clippy::cast_possible_wrap)]
    let [grid_height, grid_width] =
        [Grid::height, Grid::width].map(|dimension| dimension(grid) as Coordinate);
    let mut beams = Vec::new();
    for row_index in 0..grid_height {
        let from_left = (Direction::East, Position::new(row_index, 0));
        let from_right = (Direction::West, Position::new(row_index, grid_width - 1));
        beams.extend([from_left, from_right]);
    }
    for column_index in 0..grid_width as Coordinate {
        let from_top = (Direction::South, Position::new(0, column_index));
        let from_bottom = (
            Direction::North,
            Position::new(grid_height - 1, column_index),
        );
        beams.extend([from_top, from_bottom]);
    }
    beams
}

fn maximum_number_of_energized_tiles(grid: &Grid<char>) -> usize {
    beams_from_edges(grid)
        .into_iter()
        .map(|beam| number_of_energized_tiles(grid, beam))
        .max()
        .expect("many possible beams from the edges of the grid should exist")
}

pub fn first(input: &str) -> String {
    let grid = Grid::from(input);
    let beam = (Direction::East, Position::new(0, 0));
    number_of_energized_tiles(&grid, beam).to_string()
}

pub fn second(input: &str) -> String {
    let grid = Grid::from(input);
    maximum_number_of_energized_tiles(&grid).to_string()
}

#[cfg(test)]
mod tests {
    use super::super::tests::test_on_input;
    use crate::{Input, Puzzle};

    const DAY: usize = 16;

    #[test]
    fn first_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 46);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 8551);
    }

    #[test]
    fn second_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 51);
    }

    #[test]
    fn second_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 8754);
    }
}
