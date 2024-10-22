use std::ops::RangeInclusive;

use easy_cast::Conv;

use shared::{
    grid::{self, Coordinate, Direction, Grid, Position},
    search::minimum_path_cost,
    vector::{RotationInTwoDimensions, Vector},
};

type Move = (Option<Direction>, Position);
type Map = Grid<HeatLoss>;
type HeatLoss = usize;

pub fn first(input: &str) -> String {
    minimum_heat_loss(&Map::from(input), 1..=3).to_string()
}

pub fn second(input: &str) -> String {
    minimum_heat_loss(&Map::from(input), 4..=10).to_string()
}

fn minimum_heat_loss(map: &Map, number_of_steps: RangeInclusive<usize>) -> HeatLoss {
    let starting_point = (None, [0, 0]);
    let machine_parts_factory = [
        Coordinate::conv(map.height()) - 1,
        Coordinate::conv(map.width()) - 1,
    ];
    let is_machine_parts_factory = |(_, position)| position == machine_parts_factory;
    minimum_path_cost(
        starting_point,
        |mov| moves(map, mov, number_of_steps.clone()),
        is_machine_parts_factory,
    )
    .expect("search should reach the machine parts factory")
}

fn moves(
    map: &Map,
    (previous_direction, position): Move,
    number_of_steps: RangeInclusive<usize>,
) -> impl Iterator<Item = (Move, HeatLoss)> + '_ {
    let next_directions = match previous_direction {
        Some(previous_direction) => vec![previous_direction.left(), previous_direction.right()],
        None => grid::DIRECTIONS.into(),
    };
    next_directions.into_iter().flat_map(move |next_direction| {
        moves_in_direction(map, position, next_direction, number_of_steps.clone())
    })
}

fn moves_in_direction(
    map: &Map,
    mut position: Position,
    direction: Direction,
    number_of_steps: RangeInclusive<usize>,
) -> Vec<(Move, HeatLoss)> {
    let mut moves = vec![];
    let mut heat_loss = 0;
    for current_number_of_steps in 1..=*number_of_steps.end() {
        position = position.add(direction);
        let Some(&heat_loss_at_position) = map.get(position) else {
            break;
        };
        heat_loss += heat_loss_at_position;
        if number_of_steps.contains(&current_number_of_steps) {
            moves.push(((Some(direction), position), heat_loss));
        }
    }
    moves
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

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
