use std::iter;

use ahash::AHashSet;
use itertools::Itertools;
use shared::{
    grid::{self, Direction, Grid, Position},
    vector::{RotationInTwoDimensions, Vector},
};

type Map = Grid<char>;
type State = (Position, Direction);

const STARTING_DIRECTION: Direction = grid::NORTH;

pub fn first_answer(input: &str) -> String {
    let map = Map::from(input);
    let starting_position = starting_position(&map);
    visited_positions(&map, (starting_position, STARTING_DIRECTION))
        .len()
        .to_string()
}

pub fn second_answer(input: &str) -> String {
    let mut map = Map::from(input);
    let starting_position = starting_position(&map);
    obstruction_positions(&mut map, (starting_position, STARTING_DIRECTION))
        .len()
        .to_string()
}

fn visited_positions(map: &Map, state: State) -> AHashSet<Position> {
    path(map, state).map(|(position, _)| position).collect()
}

fn obstruction_positions(map: &mut Map, state: State) -> Vec<Position> {
    let mut obstruction_positions = vec![];
    let mut previous_positions = AHashSet::new();
    for state @ (position, direction) in path(map, state).collect_vec() {
        let next_position = position.add(direction);
        let is_free = map.get(next_position).copied() == Some('.');
        let already_walked = previous_positions.contains(&next_position);
        if is_free && !already_walked {
            map[next_position] = '#';
            if loops(map, state) {
                obstruction_positions.push(next_position);
            }
            map[next_position] = '.';
        }
        previous_positions.insert(position);
    }
    obstruction_positions
}

fn path(map: &Map, state: State) -> impl Iterator<Item = State> + use<'_> {
    iter::successors(Some(state), |&(position, direction)| {
        let next_position = position.add(direction);
        match map.get(next_position) {
            Some('#') => Some((position, direction.right())),
            Some(_) => Some((next_position, direction)),
            None => None,
        }
    })
}

fn loops(map: &Map, state: State) -> bool {
    let mut cycle = AHashSet::new();
    for state in path(map, state) {
        if !cycle.insert(state) {
            return true;
        }
    }
    false
}

fn starting_position(map: &Map) -> Position {
    let (starting_position, _) = map
        .find(|_, &element| element == '^')
        .expect("starting position should be present");
    starting_position
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 6;

    #[test]
    fn first_answer_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 41);
    }

    #[test]
    fn first_answer_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 5516);
    }

    #[test]
    fn second_answer_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 6);
    }

    #[test]
    fn second_answer_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 2008);
    }
}
