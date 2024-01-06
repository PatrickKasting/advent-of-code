use std::collections::HashSet;

use itertools::Itertools;
use strum::IntoEnumIterator;

use crate::grid::{Direction, Grid, Position};

type Tile = char;
type Cycle = Vec<(Position, Direction)>;
type CycleSlice<'cycle> = &'cycle [(Position, Direction)];

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

fn connections(tile: Tile) -> &'static [Direction] {
    CONNECTIONS
        .iter()
        .find(|(other, _)| *other == tile)
        .expect("tile should be known")
        .1
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
        .filter_map(|(position, &tile)| (tile == 'S').then_some(position))
        .exactly_one()
        .ok()
        .expect("there should be exactly one starting position");
    Direction::iter()
        .filter_map(|direction| cycle(grid, starting_position, direction))
        .max_by_key(|cycle| cycle.len())
        .expect("at least one loop should exist")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Bend {
    LeftTurn,
    Straight,
    RightTurn,
}

impl From<(Direction, Direction)> for Bend {
    fn from((towards, away): (Direction, Direction)) -> Self {
        if away.next_clockwise() == towards {
            Self::LeftTurn
        } else if away == towards.next_clockwise() {
            Self::RightTurn
        } else {
            Self::Straight
        }
    }
}

fn area(clockwise_cycle: CycleSlice) -> usize {
    let cycle_positions: HashSet<Position> = clockwise_cycle
        .iter()
        .map(|(position, _)| *position)
        .collect();
    let mut area = HashSet::new();
    let mut frontier = Vec::new();
    for (&(_, towards), &(position, away)) in clockwise_cycle.iter().circular_tuple_windows() {
        match Bend::from((towards, away)) {
            Bend::LeftTurn => {
                frontier.push(position.neighbor(away.next_clockwise()));
                frontier.push(position.neighbor(away.opposite()));
            }
            Bend::Straight => frontier.push(position.neighbor(away.next_clockwise())),
            Bend::RightTurn => (),
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
    let bend_counts = cycle
        .iter()
        .map(|&(_, direction)| direction)
        .circular_tuple_windows::<(Direction, Direction)>()
        .map(Bend::from)
        .counts();
    let difference = bend_counts[&Bend::RightTurn] as isize - bend_counts[&Bend::LeftTurn] as isize;
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

pub fn first(input: String) -> String {
    (longest_cycle(&Grid::from(input.as_str())).len() / 2).to_string()
}

pub fn second(input: String) -> String {
    cycle_area(longest_cycle(&Grid::from(input.as_str()))).to_string()
}

#[cfg(test)]
mod tests {
    use crate::{tests::*, Input, Puzzle};

    const DAY: usize = 10;

    #[test]
    fn first_examples() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 4);
        test_on_input(DAY, Puzzle::First, Input::Example(1), 8);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::Real, 6690);
    }

    #[test]
    fn second_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(2), 4);
        test_on_input(DAY, Puzzle::Second, Input::Example(3), 8);
        test_on_input(DAY, Puzzle::Second, Input::Example(4), 10);
    }

    #[test]
    fn second_input() {
        test_on_input(DAY, Puzzle::Second, Input::Real, 525);
    }
}
