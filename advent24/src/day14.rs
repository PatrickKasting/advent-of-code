use std::cmp::Ordering;

use easy_cast::Cast;
use itertools::Itertools;
use shared::{string::isizes, vector::Vector};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Robot {
    position: Position,
    velocity: Velocity,
}
type Position = [Coordinate; 2];
type Velocity = [Coordinate; 2];
type Coordinate = isize;

const DIMENSIONS: [Coordinate; 2] = [101, 103];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Quadrant {
    TopRight,
    BottomRight,
    BottomLeft,
    TopLeft,
}

const BOUNDARIES: [Coordinate; 2] = [DIMENSIONS[0] / 2, DIMENSIONS[1] / 2];

pub fn first_answer(input: &str) -> String {
    safety_factor(robots(input), 100).to_string()
}

pub fn second_answer(input: &str) -> String {
    todo!()
}

fn safety_factor(robots: impl Iterator<Item = Robot>, seconds: usize) -> usize {
    let counts = robots.filter_map(|robot| quadrant(robot, seconds)).counts();
    counts.values().copied().product()
}

fn quadrant(robot: Robot, seconds: usize) -> Option<Quadrant> {
    let [x, y] = final_position(robot, seconds);
    match [x.cmp(&BOUNDARIES[0]), y.cmp(&BOUNDARIES[1])] {
        [Ordering::Less, Ordering::Less] => Some(Quadrant::TopLeft),
        [Ordering::Less, Ordering::Greater] => Some(Quadrant::BottomLeft),
        [Ordering::Greater, Ordering::Less] => Some(Quadrant::TopRight),
        [Ordering::Greater, Ordering::Greater] => Some(Quadrant::BottomRight),
        _ => None,
    }
}

fn final_position(Robot { position, velocity }: Robot, seconds: usize) -> Position {
    let [x, y] = position.add(velocity.mul(seconds.cast()));
    [x.rem_euclid(DIMENSIONS[0]), y.rem_euclid(DIMENSIONS[1])]
}

fn robots(input: &str) -> impl Iterator<Item = Robot> + use<'_> {
    input.lines().map(robot)
}

fn robot(line: &str) -> Robot {
    let numbers = isizes(line);
    Robot {
        position: [numbers[0], numbers[1]],
        velocity: [numbers[2], numbers[3]],
    }
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 14;

    #[test]
    fn first_answer_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 211_692_000);
    }

    // #[test]
    // fn second_answer_example() {
    //     test_on_input(DAY, Puzzle::Second, Input::Example(0), 34);
    // }

    // #[test]
    // fn second_answer_input() {
    //     test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 1200);
    // }

    // #[test]
    // fn final_position() {
    //     let robot = robot("p=2,4 v=2,-3");
    //     let actual = super::final_position(robot, 5);
    //     let expected = [1, 3];
    //     assert_eq!(actual, expected);
    // }
}
