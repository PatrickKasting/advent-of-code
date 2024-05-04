use std::{collections::HashMap, f64, sync::OnceLock};

use itertools::Itertools;

use crate::{
    data_structures::grid::{self, Direction, Grid, Position},
    floating::ApproxEq,
    search::Exploration,
    vector::{Addition, Angle, Negation, RotationInTwoDimensions, Subtraction},
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
        let [toward, away] = [second.sub(first), third.sub(second)];
        let angle = toward.angle(away);
        let directions_toward_inside = if angle.approx_eq(-f64::consts::FRAC_PI_2) {
            vec![]
        } else if angle.approx_eq(0.0) {
            vec![away.right()]
        } else if angle.approx_eq(f64::consts::FRAC_PI_2) {
            vec![away.right(), away.neg()]
        } else {
            panic!("cycle should only curve left or right")
        };
        for direction in directions_toward_inside {
            area.explore(second.add(direction), grid::neighbors);
        }
    }
    area.explored().len() - cycle.len()
}

fn is_clockwise(cycle: &mut [Position]) -> bool {
    let sum_of_interior_angles: f64 = cycle
        .iter()
        .circular_tuple_windows()
        .map(|(&first, &second, &third)| {
            let [toward, away] = [second.sub(first), third.sub(second)];
            toward.angle(away)
        })
        .sum();
    debug_assert!(
        sum_of_interior_angles
            .abs()
            .approx_eq(2.0 * f64::consts::PI),
        "sum of interior angles should be 2.0π or -2.0π",
    );
    sum_of_interior_angles.is_sign_negative()
}

fn longest_cycle(grid: &Grid<Tile>) -> Cycle {
    let starting_position = grid
        .iter_row_major()
        .find_map(|(position, &tile)| (tile == 'S').then_some(position))
        .expect("there should be exactly one starting position");
    grid::DIRECTIONS
        .into_iter()
        .filter_map(|direction| cycle(grid, starting_position, direction))
        .max_by_key(Vec::len)
        .expect("at least one loop should exist")
}

fn cycle(grid: &Grid<Tile>, from: Position, mut toward: Direction) -> Option<Cycle> {
    let mut cycle = Vec::from([from]);
    loop {
        let &position = cycle.last().expect("cycle should be non-empty");
        let next_position = position.add(toward);
        let &next_tile = grid.get(next_position)?;
        if next_tile == 'S' {
            return Some(cycle);
        }
        toward = out_port(next_tile, toward.neg())?;
        cycle.push(next_position);
    }
}

fn out_port(tile: Tile, in_port: Direction) -> Option<Direction> {
    fn out_ports() -> HashMap<(Tile, Direction), Direction> {
        [
            ('|', [grid::NORTH, grid::SOUTH]),
            ('-', [grid::EAST, grid::WEST]),
            ('L', [grid::NORTH, grid::EAST]),
            ('J', [grid::NORTH, grid::WEST]),
            ('7', [grid::SOUTH, grid::WEST]),
            ('F', [grid::EAST, grid::SOUTH]),
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
