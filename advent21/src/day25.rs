use easy_cast::Conv;
use shared::{
    grid::{Coordinate, Direction, Grid, Position},
    vector::Vector,
};

type Seafloor = Grid<Cucumber>;
type Cucumber = u8;

pub fn first_answer(input: &str) -> String {
    let seafloor = Seafloor::from(input);
    number_of_steps_before_no_cucumber_moves(seafloor).to_string()
}

pub fn second_answer(_input: &str) -> String {
    panic!("there is no second part on the 25th");
}

fn number_of_steps_before_no_cucumber_moves(mut seafloor: Seafloor) -> usize {
    for number_of_steps in 1.. {
        let at_least_one_cucumber_moves;
        (seafloor, at_least_one_cucumber_moves) = step_all(seafloor);
        if !at_least_one_cucumber_moves {
            return number_of_steps;
        }
    }
    unreachable!("loop should only break because no cucumber moves");
}

fn step_all(mut seafloor: Seafloor) -> (Seafloor, bool) {
    let [east_facing_cucumber_moves, south_facing_cucumber_moves];
    (seafloor, east_facing_cucumber_moves) = step_kind(&seafloor, b'>');
    (seafloor, south_facing_cucumber_moves) = step_kind(&seafloor, b'v');
    let at_least_one_cucumber_moves = east_facing_cucumber_moves || south_facing_cucumber_moves;
    (seafloor, at_least_one_cucumber_moves)
}

fn step_kind(seafloor: &Seafloor, kind: Cucumber) -> (Seafloor, bool) {
    let direction = direction(kind);
    let mut new = seafloor.clone();
    let mut at_least_one_cucumber_moves = false;
    for (position, &cucumber) in seafloor.iter_row_major() {
        if cucumber != kind {
            continue;
        }

        let next_position = next_position(seafloor, position, direction);
        if seafloor[next_position] == b'.' {
            new[position] = b'.';
            new[next_position] = kind;
            at_least_one_cucumber_moves = true;
        }
    }
    (new, at_least_one_cucumber_moves)
}

fn next_position(seafloor: &Seafloor, position: Position, direction: Direction) -> Position {
    let mut next_position = position.add(direction);
    let is_outside_grid = seafloor.get(next_position).is_none();
    if is_outside_grid {
        next_position = [
            next_position[0] % Coordinate::conv(seafloor.height()),
            next_position[1] % Coordinate::conv(seafloor.width()),
        ];
    }
    next_position
}

fn direction(cucumber: u8) -> Direction {
    match cucumber {
        b'>' => [0, 1],
        b'v' => [1, 0],
        _ => panic!("cucumber should be '>' or 'v'"),
    }
}

#[cfg(test)]
mod tests {
    use infrastructure::{test, Input, Puzzle};

    use super::*;
    use crate::tests::{input, test_on_input};

    const DAY: usize = 25;

    #[test]
    fn first_answer_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 58);
    }

    #[test]
    fn first_answer_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 560);
    }

    #[test]
    fn step_all() {
        let seafloor = Seafloor::from(&input(DAY, Input::Example(0)));
        let (actual, at_least_one_cucumber_moves) = super::step_all(seafloor);
        let expected = "\
            ....>.>v.>\n\
            v.v>.>v.v.\n\
            >v>>..>v..\n\
            >>v>v>.>.v\n\
            .>v.v...v.\n\
            v>>.>vvv..\n\
            ..v...>>..\n\
            vv...>>vv.\n\
            >.v.v..v.v\n\
        ";
        let expected = Seafloor::from(expected);
        assert_eq!(actual, expected);
        assert!(at_least_one_cucumber_moves);
    }

    #[test]
    fn next_position() {
        let seafloor = Seafloor::from(&input(DAY, Input::Example(0)));
        let function =
            |(position, kind)| super::next_position(&seafloor, position, direction(kind));
        let cases = [
            (([7, 2], b'v'), [8, 2]),
            (([3, 8], b'>'), [3, 9]),
            (([8, 4], b'v'), [0, 4]),
            (([5, 9], b'>'), [5, 0]),
        ];
        test::cases(function, cases);
    }
}
