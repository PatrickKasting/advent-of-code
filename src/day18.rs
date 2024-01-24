use std::sync::OnceLock;

use regex::Regex;

use crate::{
    grid::{area, Direction, Position},
    utilities::number,
};

fn dig_plan_step(line: &str) -> (Direction, usize) {
    static REGEX: OnceLock<Regex> = OnceLock::new();
    let regex = REGEX.get_or_init(|| {
        Regex::new(r"([ULDR]) (\d+) \(#([0-9a-f]{6})\)").expect("regex should be valid")
    });

    let (_, [direction, meters, _]) = regex
        .captures(line)
        .expect("line should match regex")
        .extract();
    let direction = match direction {
        "U" => Direction::North,
        "L" => Direction::West,
        "D" => Direction::South,
        "R" => Direction::East,
        _ => panic!("direction should be 'U', 'L', 'D', or 'R'"),
    };
    (direction, number(meters))
}

fn dig_plan(input: &str) -> impl Iterator<Item = (Direction, usize)> + '_ {
    input.lines().map(dig_plan_step)
}

fn trench(dig_plan: impl Iterator<Item = (Direction, usize)>) -> Vec<Position> {
    let mut trench = vec![];
    let mut current_position = Position::new(0, 0);
    for (direction, meters) in dig_plan {
        for _ in 0..meters {
            current_position = current_position.neighbor(direction);
            trench.push(current_position);
        }
    }
    trench
}

pub fn first(input: &str) -> String {
    let dig_plan = dig_plan(input);
    let mut trench = trench(dig_plan);
    let area = area(&mut trench);
    (trench.len() + area.len()).to_string()
}

pub fn second(_input: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::{tests::*, InputType, Puzzle};

    const DAY: usize = 18;

    #[test]
    fn first_example() {
        test_on_input(DAY, Puzzle::First, InputType::Example(0), 62);
    }

    // #[test]
    // fn first_input() {
    //     test_on_input(DAY, Puzzle::First, InputType::PuzzleInput, 513158);
    // }

    // #[test]
    // fn second_example() {
    //     test_on_input(DAY, Puzzle::Second, InputType::Example(0), 145);
    // }

    // #[test]
    // fn second_input() {
    //     test_on_input(DAY, Puzzle::Second, InputType::PuzzleInput, 200277);
    // }
}
