use shared::{
    grid::{Coordinate, Position, EAST},
    matrix::{self, Matrix},
    vector::{RotationInTwoDimensions, Vector},
};

pub fn first_answer(input: &str) -> String {
    let [x, y] = destination_without_waypoint(input);
    (x.abs() + y.abs()).to_string()
}

pub fn second_answer(input: &str) -> String {
    let [x, y] = destination_with_waypoint(input);
    (x.abs() + y.abs()).to_string()
}

fn destination_without_waypoint(input: &str) -> Position {
    let mut position = [0, 0];
    let mut direction = EAST;
    for instruction in input.lines() {
        let value = instruction[1..]
            .parse()
            .expect("instruction value should be numeric");
        match &instruction[0..1] {
            "N" => position[0] -= value,
            "S" => position[0] += value,
            "E" => position[1] += value,
            "W" => position[1] -= value,
            "L" => {
                for _ in 0..value / 90 {
                    direction = direction.left();
                }
            }
            "R" => {
                for _ in 0..value / 90 {
                    direction = direction.right();
                }
            }
            "F" => position = position.add(direction.mul(value)),
            _ => panic!("instruction should be 'N', 'S', 'E', 'W', 'L', 'R', or 'F'"),
        }
    }
    position
}

const LEFT_ROTATION: Matrix<Coordinate, 2, 2> = [[0, -1], [1, 0]];
const RIGHT_ROTATION: Matrix<Coordinate, 2, 2> = [[0, 1], [-1, 0]];

fn destination_with_waypoint(input: &str) -> Position {
    let mut ship = [0, 0];
    let mut waypoint = [-1, 10]; // relative to ship
    for instruction in input.lines() {
        let value = instruction[1..]
            .parse()
            .expect("instruction value should be numeric");
        match &instruction[0..1] {
            "N" => waypoint[0] -= value,
            "S" => waypoint[0] += value,
            "E" => waypoint[1] += value,
            "W" => waypoint[1] -= value,
            "L" => {
                for _ in 0..value / 90 {
                    waypoint = matrix::vector_mul(LEFT_ROTATION, waypoint);
                }
            }
            "R" => {
                for _ in 0..value / 90 {
                    waypoint = matrix::vector_mul(RIGHT_ROTATION, waypoint);
                }
            }
            "F" => {
                ship = ship.add(waypoint.mul(value));
            }
            _ => panic!("instruction should be 'N', 'S', 'E', 'W', 'L', 'R', or 'F'"),
        }
    }
    ship
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 12;

    #[test]
    fn first_answer_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 25);
    }

    #[test]
    fn first_answer_puzzle_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 1177);
    }

    #[test]
    fn second_answer_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 286);
    }

    #[test]
    fn second_answer_puzzle_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 46530);
    }
}
