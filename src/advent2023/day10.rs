use std::{collections::HashMap, sync::OnceLock};

use strum::IntoEnumIterator;

use crate::grid::{area, Direction, Grid, Position};

type Tile = char;
type Cycle = Vec<Position>;

fn connections(tile: Tile) -> &'static [Direction] {
    const CONNECTIONS: &[(Tile, &[Direction])] = &[
        ('|', &[Direction::North, Direction::South]),
        ('-', &[Direction::East, Direction::West]),
        ('L', &[Direction::North, Direction::East]),
        ('J', &[Direction::North, Direction::West]),
        ('7', &[Direction::South, Direction::West]),
        ('F', &[Direction::East, Direction::South]),
        ('.', &[]),
        (
            'S',
            &[
                Direction::North,
                Direction::East,
                Direction::South,
                Direction::West,
            ],
        ),
    ];

    static CONNECTIONS_LOCK: OnceLock<HashMap<char, &'static [Direction]>> = OnceLock::new();
    CONNECTIONS_LOCK.get_or_init(|| CONNECTIONS.iter().copied().collect())[&tile]
}

fn out_direction(tile: Tile, in_direction: Direction) -> Option<Direction> {
    let connections = connections(tile);
    let in_direction_index = connections
        .iter()
        .position(|&direction| direction == in_direction)?;
    let out_direction_index = (in_direction_index + 1) % connections.len();
    Some(connections[out_direction_index])
}

fn cycle(grid: &Grid<Tile>, from: Position, mut towards: Direction) -> Option<Cycle> {
    let mut cycle = Vec::from([from]);
    loop {
        let &position = cycle.last().expect("cycle should be non-empty");
        let next_position = position.neighbor(towards);
        let &next_tile = grid.get(next_position)?;
        if next_tile == 'S' {
            return Some(cycle);
        }
        towards = out_direction(next_tile, towards.opposite())?;
        cycle.push(next_position);
    }
}

fn longest_cycle(grid: &Grid<Tile>) -> Cycle {
    let starting_position = grid
        .iter_row_major()
        .find_map(|(position, &tile)| (tile == 'S').then_some(position))
        .expect("there should be exactly one starting position");
    Direction::iter()
        .filter_map(|direction| cycle(grid, starting_position, direction))
        .max_by_key(Vec::len)
        .expect("at least one loop should exist")
}

pub fn first(input: &str) -> String {
    (longest_cycle(&Grid::from(input)).len() / 2).to_string()
}

pub fn second(input: &str) -> String {
    area(&mut longest_cycle(&Grid::from(input)))
        .len()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::super::tests::test_on_input;
    use crate::{Input, Puzzle};

    const DAY: usize = 10;

    #[test]
    fn first_examples() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 4);
        test_on_input(DAY, Puzzle::First, Input::Example(1), 8);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 6690);
    }

    #[test]
    fn second_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(2), 4);
        test_on_input(DAY, Puzzle::Second, Input::Example(3), 8);
        test_on_input(DAY, Puzzle::Second, Input::Example(4), 10);
    }

    #[test]
    fn second_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 525);
    }
}
