use std::ops::RangeInclusive;

use itertools::Itertools;
use strum::IntoEnumIterator;

use crate::{
    data_structures::grid::{Direction, Grid, Position},
    search::uniform_cost,
};

type Move = (Option<Direction>, Position);

fn possible_next_moves(
    map: &Grid<usize>,
    possible_number_of_steps: RangeInclusive<usize>,
    (previous_direction, position): Move,
) -> impl IntoIterator<Item = (Move, usize)> {
    let next_directions = match previous_direction {
        Some(previous_direction) => vec![previous_direction.right(), previous_direction.left()],
        None => Direction::iter().collect_vec(),
    };

    let mut next_moves = Vec::new();
    for next_direction in next_directions {
        let mut next_position = position;
        let mut heat_loss = 0;
        for current_number_of_steps in 1..=*possible_number_of_steps.end() {
            next_position = next_position.neighbor(next_direction);
            let Some(&heat_loss_at_next_position) = map.get(next_position) else {
                break;
            };
            heat_loss += heat_loss_at_next_position;
            if possible_number_of_steps.contains(&current_number_of_steps) {
                next_moves.push(((Some(next_direction), next_position), heat_loss));
            }
        }
    }
    next_moves
}

fn minimum_heat_loss(map: &Grid<usize>, possible_number_of_steps: RangeInclusive<usize>) -> usize {
    let starting_point = (None, Position::new(0, 0));
    let machine_parts_factory = Position::new(map.height() - 1, map.width() - 1);
    let is_machine_parts_factory = |(_, position)| position == machine_parts_factory;
    uniform_cost(
        starting_point,
        |mov| possible_next_moves(map, possible_number_of_steps.clone(), mov),
        is_machine_parts_factory,
    )
    .expect("search should reach the machine parts factory")
}

pub fn first(input: &str) -> String {
    let map: Grid<usize> = Grid::from(input);
    minimum_heat_loss(&map, 1..=3).to_string()
}

pub fn second(input: &str) -> String {
    let map: Grid<usize> = Grid::from(input);
    minimum_heat_loss(&map, 4..=10).to_string()
}

#[cfg(test)]
mod tests {
    use super::super::tests::test_on_input;
    use crate::{Input, Puzzle};

    const DAY: usize = 17;

    #[test]
    fn first_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 102);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 953);
    }

    #[test]
    fn second_examples() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 94);
        test_on_input(DAY, Puzzle::Second, Input::Example(1), 71);
    }

    #[test]
    fn second_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 1180);
    }
}
