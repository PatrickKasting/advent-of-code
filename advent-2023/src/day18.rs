use itertools::Itertools;

use shared::vector::Vector;

type Area = isize;
type Trench = Vec<Corner>;
type DigStep = (Direction, Coordinate);
type Corner = [Coordinate; 2];
type Direction = [Coordinate; 2];
type Coordinate = isize;

pub fn first(input: &str) -> String {
    let dig_plan = dig_plan(input, dig_plan_step_from_directions_and_distances);
    let trench = trench(dig_plan);
    area(&trench).to_string()
}

pub fn second(input: &str) -> String {
    let dig_plan = dig_plan(input, dig_plan_step_from_color_codes);
    let trench = trench(dig_plan);
    area(&trench).to_string()
}

fn area(trench: &[Corner]) -> Area {
    let perimeter: Area = trench
        .iter()
        .circular_tuple_windows()
        .map(|(&c0, &c1)| c1.sub(c0).norm())
        .sum();
    let areas = trench
        .iter()
        .circular_tuple_windows()
        .map(|(&[x0, y0], &[x1, y1])| (y0 + y1) * (x0 - x1));
    let area = (areas.sum::<Area>() / 2).abs();
    area + perimeter / 2 + 1
}

fn trench(dig_plan: impl Iterator<Item = DigStep>) -> Trench {
    let mut position = [0, 0];
    let mut corners = vec![];
    for (direction, distance) in dig_plan {
        position = position.add(direction.mul(distance));
        corners.push(position);
    }
    corners
}

const UP: Direction = [-1, 0];
const DOWN: Direction = [1, 0];
const LEFT: Direction = [0, -1];
const RIGHT: Direction = [0, 1];

fn dig_plan(input: &str, parser: fn(&str) -> DigStep) -> impl Iterator<Item = DigStep> + '_ {
    input.lines().map(parser)
}

fn dig_plan_step_from_directions_and_distances(line: &str) -> DigStep {
    let direction = match &line[0..1] {
        "U" => UP,
        "D" => DOWN,
        "L" => LEFT,
        "R" => RIGHT,
        _ => panic!("direction should be 'U', 'D', 'L', or 'R'"),
    };
    let (distance, _) = line[2..]
        .split_once(' ')
        .expect("line should contain distance and color code");
    let distance = distance.parse().expect("distance should be numeric");
    (direction, distance)
}

fn dig_plan_step_from_color_codes(line: &str) -> DigStep {
    let (_, color_code) = line
        .split_once('#')
        .expect("line should contain a color code starting with '#'");
    let direction = match &color_code[5..6] {
        "0" => RIGHT,
        "1" => DOWN,
        "2" => LEFT,
        "3" => UP,
        _ => panic!("direction should be '0', '1', '2', or '3'"),
    };
    let distance = Coordinate::from_str_radix(&color_code[0..5], 16)
        .expect("first five digits of color code should be parsable");
    (direction, distance)
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use super::*;
    use crate::tests::{input, test_on_input};

    const DAY: usize = 18;

    #[test]
    fn first_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 62);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 70253);
    }

    #[test]
    fn second_example() {
        test_on_input(
            DAY,
            Puzzle::Second,
            Input::Example(0),
            952_408_144_115_usize,
        );
    }

    #[test]
    fn second_input() {
        test_on_input(
            DAY,
            Puzzle::Second,
            Input::PuzzleInput,
            131_265_059_885_080_usize,
        );
    }

    #[test]
    fn trench() {
        let input = input(DAY, Input::Example(0));
        let dig_plan = dig_plan(&input, dig_plan_step_from_directions_and_distances);
        let actual = super::trench(dig_plan);
        let expected = vec![
            [0, 6],
            [5, 6],
            [5, 4],
            [7, 4],
            [7, 6],
            [9, 6],
            [9, 1],
            [7, 1],
            [7, 0],
            [5, 0],
            [5, 2],
            [2, 2],
            [2, 0],
            [0, 0],
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn dig_plan_step_from_color_codes() {
        let actual = super::dig_plan_step_from_color_codes("R 6 (#70c710)");
        let expected = (RIGHT, 461_937);
        assert_eq!(actual, expected);
    }
}
