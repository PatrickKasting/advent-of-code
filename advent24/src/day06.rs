use std::iter;

use ahash::AHashSet;
use shared::{
    grid::{self, Direction, Grid, Position},
    vector::{RotationInTwoDimensions, Vector},
};

type Map = Grid<char>;

pub fn first_answer(input: &str) -> String {
    let (map, starting_position, starting_direction) = map_and_start(input);
    visited_positions(&map, starting_position, starting_direction)
        .len()
        .to_string()
}

pub fn second_answer(input: &str) -> String {
    todo!()
}

fn visited_positions(
    map: &Map,
    starting_position: Position,
    starting_direction: Direction,
) -> AHashSet<Position> {
    path(map, starting_position, starting_direction)
        .map(|(position, _)| position)
        .collect()
}

fn path(
    map: &Map,
    starting_position: Position,
    starting_direction: Direction,
) -> impl Iterator<Item = (Position, Direction)> + use<'_> {
    iter::successors(
        Some((starting_position, starting_direction)),
        |&(position, direction)| {
            let next_position = position.add(direction);
            match map.get(next_position) {
                Some(&'#') => Some((position, direction.right())),
                Some(_) => Some((next_position, direction)),
                None => None,
            }
        },
    )
}

fn map_and_start(input: &str) -> (Map, Position, Direction) {
    let map = Map::from(input);
    let (starting_position, starting_direction) = map
        .find_map(|_, &element| grid::direction(element))
        .expect("starting direction should be '^', '>', 'v', or '<'");
    (map, starting_position, starting_direction)
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

    // #[test]
    // fn second_answer_example() {
    //     test_on_input(DAY, Puzzle::Second, Input::Example(0), 6);
    // }

    // #[test]
    // fn second_answer_input() {
    //     test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 26_800_609);
    // }
}
