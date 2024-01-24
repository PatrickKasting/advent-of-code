use std::{
    collections::{HashMap, HashSet},
    sync::OnceLock,
};

use itertools::Itertools;
use strum::IntoEnumIterator;

use crate::{
    grid::{Curvature, Direction, Grid, Position},
    utilities::as_isize,
};

type Tile = char;
type Cycle = Vec<(Position, Direction)>;
type CycleSlice<'cycle> = &'cycle [(Position, Direction)];

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

fn cycle(grid: &Grid<Tile>, from: Position, towards: Direction) -> Option<Cycle> {
    let mut cycle = Vec::from([(from, towards)]);
    loop {
        let &(position, direction) = cycle.last().expect("cycle should be non-empty");
        let next_position = position.neighbor(direction);
        let &next_tile = grid.get(next_position)?;
        if next_tile == 'S' {
            return Some(cycle);
        }
        let next_direction = out_direction(next_tile, direction.opposite())?;
        cycle.push((next_position, next_direction));
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

fn area(clockwise_cycle: CycleSlice) -> usize {
    let cycle_positions: HashSet<Position> = clockwise_cycle
        .iter()
        .map(|&(position, _)| position)
        .collect();
    let mut area = HashSet::new();
    let mut frontier = Vec::new();
    for (&(_, towards), &(position, away)) in clockwise_cycle.iter().circular_tuple_windows() {
        match Curvature::from((towards, away)) {
            Curvature::LeftTurn => {
                frontier.push(position.neighbor(away.next_clockwise()));
                frontier.push(position.neighbor(away.opposite()));
            }
            Curvature::Straight => frontier.push(position.neighbor(away.next_clockwise())),
            Curvature::RightTurn => (),
            Curvature::UTurn => panic!("cycle should turn no more than 90 degrees at a time"),
        }
        while let Some(position) = frontier.pop() {
            if !cycle_positions.contains(&position) && area.insert(position) {
                frontier.extend(position.neighbors());
            }
        }
    }
    area.len()
}

fn is_clockwise(cycle: CycleSlice) -> bool {
    let curvature_counts = cycle
        .iter()
        .map(|&(_, direction)| direction)
        .circular_tuple_windows::<(Direction, Direction)>()
        .map(Curvature::from)
        .counts();
    let difference = as_isize(curvature_counts[&Curvature::RightTurn])
        - as_isize(curvature_counts[&Curvature::LeftTurn]);
    debug_assert_eq!(
        difference.abs(),
        4,
        "turn count difference should be four or negative four",
    );
    difference.is_positive()
}

fn reverse(cycle: CycleSlice) -> Cycle {
    let positions = cycle.iter().map(|&(position, _)| position);
    let directions = cycle.iter().map(|&(_, direction)| direction);
    positions
        .rev()
        .zip(directions.map(Direction::opposite).rev().cycle().skip(1))
        .collect_vec()
}

fn cycle_area(mut cycle: Cycle) -> usize {
    if !is_clockwise(&cycle) {
        cycle = reverse(&cycle);
    }
    area(&cycle)
}

pub fn first(input: &str) -> String {
    (longest_cycle(&Grid::from(input)).len() / 2).to_string()
}

pub fn second(input: &str) -> String {
    cycle_area(longest_cycle(&Grid::from(input))).to_string()
}

#[cfg(test)]
mod tests {
    use crate::{tests::*, InputType, Puzzle};

    const DAY: usize = 10;

    #[test]
    fn first_examples() {
        test_on_input(DAY, Puzzle::First, InputType::Example(0), 4);
        test_on_input(DAY, Puzzle::First, InputType::Example(1), 8);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, InputType::PuzzleInput, 6690);
    }

    #[test]
    fn second_example() {
        test_on_input(DAY, Puzzle::Second, InputType::Example(2), 4);
        test_on_input(DAY, Puzzle::Second, InputType::Example(3), 8);
        test_on_input(DAY, Puzzle::Second, InputType::Example(4), 10);
    }

    #[test]
    fn second_input() {
        test_on_input(DAY, Puzzle::Second, InputType::PuzzleInput, 525);
    }
}
