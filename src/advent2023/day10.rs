use std::{collections::HashMap, sync::OnceLock};

use itertools::Itertools;
use strum::IntoEnumIterator;

use crate::{
    data_structures::grid::{Direction, Grid, Position, RelativeDirection},
    search::Exploration,
};

type Tile = char;
type Cycle = Vec<Position>;

pub fn first(input: &str) -> String {
    (longest_cycle(&Grid::from(input)).len() / 2).to_string()
}

pub fn second(input: &str) -> String {
    area(&mut longest_cycle(&Grid::from(input))).to_string()
}

pub fn area(cycle: &mut [Position]) -> usize {
    if !is_clockwise(cycle) {
        cycle.reverse();
    }

    let mut area = Exploration::new(cycle.iter().copied());
    for (&first, &second, &third) in cycle.iter().circular_tuple_windows() {
        let [toward, away] = [[first, second], [second, third]].map(|[from, to]| {
            from.direction_to(to)
                .expect("direction from position to position should be cardinal")
        });
        let directions_toward_inside = match toward.relative_direction_to(away) {
            RelativeDirection::Right => vec![],
            RelativeDirection::Forward => vec![away.right()],
            RelativeDirection::Left => vec![away.right(), away.backward()],
            RelativeDirection::Backward => panic!("cycle should not bend back on itself"),
        };
        for direction in directions_toward_inside {
            area.explore(second.neighbor(direction), Position::neighbors);
        }
    }
    area.explored().len() - cycle.len()
}

fn is_clockwise(cycle: &mut [Position]) -> bool {
    let turns = cycle
        .iter()
        .circular_tuple_windows()
        .map(relative_direction)
        .counts();
    #[allow(clippy::cast_possible_wrap)]
    let [left, right] = [RelativeDirection::Left, RelativeDirection::Right]
        .map(|direction| turns[&direction] as isize);
    let turn_difference = right - left;

    debug_assert!(
        !turns.contains_key(&RelativeDirection::Backward),
        "cycle should not bend back on itself"
    );
    debug_assert_eq!(
        turn_difference.abs(),
        4,
        "difference in left and right turns should be four for a cycle"
    );

    turn_difference.is_positive()
}

fn relative_direction(
    (&first, &second, &third): (&Position, &Position, &Position),
) -> RelativeDirection {
    let [towards, away] = [[first, second], [second, third]].map(|[from, to]| {
        from.direction_to(to)
            .expect("direction from position to position should be cardinal")
    });
    towards.relative_direction_to(away)
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

fn cycle(grid: &Grid<Tile>, from: Position, mut toward: Direction) -> Option<Cycle> {
    let mut cycle = Vec::from([from]);
    loop {
        let &position = cycle.last().expect("cycle should be non-empty");
        let next_position = position.neighbor(toward);
        let &next_tile = grid.get(next_position)?;
        if next_tile == 'S' {
            return Some(cycle);
        }
        toward = out_port(next_tile, toward.backward())?;
        cycle.push(next_position);
    }
}

fn out_port(tile: Tile, in_port: Direction) -> Option<Direction> {
    fn out_ports() -> HashMap<(Tile, Direction), Direction> {
        [
            ('|', [Direction::North, Direction::South]),
            ('-', [Direction::East, Direction::West]),
            ('L', [Direction::North, Direction::East]),
            ('J', [Direction::North, Direction::West]),
            ('7', [Direction::South, Direction::West]),
            ('F', [Direction::East, Direction::South]),
        ]
        .into_iter()
        .flat_map(|(tile, ports)| {
            [[0, 1], [1, 0]].map(|[from, to]| ((tile, ports[from]), ports[to]))
        })
        .collect()
    }

    static OUT_PORTS: OnceLock<HashMap<(Tile, Direction), Direction>> = OnceLock::new();
    OUT_PORTS
        .get_or_init(out_ports)
        .get(&(tile, in_port))
        .copied()
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
