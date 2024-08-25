use std::ops::RangeInclusive;

use regex::Regex;

const FOUR_COORDINATES: &str = "target area should be described by four coordinates";

type Coordinate = isize;
type Range = RangeInclusive<Coordinate>;

pub fn first(input: &str) -> String {
    let (x_target, y_target) = parse_target_area(input);
    let greatest_y_velocity = hits(&x_target, &y_target)
        .map(|(_, y_velocity)| y_velocity)
        .max()
        .expect("at least one set of velocities should hit the target");
    sum_of_first_naturals(greatest_y_velocity).to_string()
}

pub fn second(input: &str) -> String {
    let (x_target, y_target) = parse_target_area(input);
    hits(&x_target, &y_target).count().to_string()
}

fn parse_target_area(input: &str) -> (Range, Range) {
    let coordinate = Regex::new(r"-?\d+").expect("regex should be correct");
    let mut coordinates = coordinate.find_iter(input).map(|coordinate| {
        coordinate
            .as_str()
            .parse()
            .expect("every regex match should be parsed into an integer")
    });
    (
        coordinates.next().expect(FOUR_COORDINATES)..=coordinates.next().expect(FOUR_COORDINATES),
        coordinates.next().expect(FOUR_COORDINATES)..=coordinates.next().expect(FOUR_COORDINATES),
    )
}

fn num_first_naturals_to_sum(sum: isize) -> isize {
    let num_first_naturals = 1.0 / 2.0 * (-1.0 + (1.0 + 8.0 * sum as f64).sqrt());
    num_first_naturals.ceil() as Coordinate
}

fn possible_x_velocities(x_target: &Range) -> Range {
    num_first_naturals_to_sum(*x_target.start())..=*x_target.end()
}

fn possible_y_velocities(y_target: &Range) -> Range {
    *y_target.start()..=-*y_target.start()
}

fn probe_hits_target(
    x_target: &Range,
    y_target: &Range,
    mut x_velocity: Coordinate,
    mut y_velocity: Coordinate,
) -> bool {
    let mut x_position = 0;
    let mut y_position = 0;
    loop {
        if x_target.contains(&x_position) && y_target.contains(&y_position) {
            return true;
        }
        if y_position < *y_target.start() || x_position > *x_target.end() {
            return false;
        }
        x_position += x_velocity;
        y_position += y_velocity;
        x_velocity -= x_velocity.signum();
        y_velocity -= 1;
    }
}

fn hits<'target>(
    x_target: &'target Range,
    y_target: &'target Range,
) -> impl Iterator<Item = (Coordinate, Coordinate)> + 'target {
    possible_x_velocities(x_target)
        .flat_map(|x_velocity| {
            possible_y_velocities(y_target).map(move |y_velocity| (x_velocity, y_velocity))
        })
        .filter(|&(x_velocity, y_velocity)| {
            probe_hits_target(x_target, y_target, x_velocity, y_velocity)
        })
}

fn sum_of_first_naturals(num_naturals: isize) -> isize {
    num_naturals * (num_naturals + 1) / 2
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use super::*;
    use crate::tests::{input, test_on_input};

    const DAY: usize = 17;

    #[test]
    fn first_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 45);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 3655);
    }

    #[test]
    fn second_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 112);
    }

    #[test]
    fn second_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 1447);
    }

    #[test]
    fn parse() {
        let input = input(DAY, Input::Example(0));
        let (x_target, y_target) = parse_target_area(&input);
        assert_eq!(*x_target.start(), 20);
        assert_eq!(*x_target.end(), 30);
        assert_eq!(*y_target.start(), -10);
        assert_eq!(*y_target.end(), -5);
    }

    #[test]
    fn naturals_to_sum() {
        let cases = [(0, 0), (1, 1), (3, 2), (6, 3), (10, 4), (15, 5), (21, 6)];
        for (input, output) in cases {
            assert_eq!(num_first_naturals_to_sum(input), output);
            assert_eq!(num_first_naturals_to_sum(input + 1), output + 1);
        }
    }

    #[test]
    fn x_velocities() {
        let x_velocities = possible_x_velocities(&(20..=30));
        assert_eq!(*x_velocities.start(), 6);
        assert_eq!(*x_velocities.end(), 30);
    }

    #[test]
    fn y_velocities() {
        let y_velocities = possible_y_velocities(&(-10..=-5));
        assert_eq!(*y_velocities.start(), -10);
        assert_eq!(*y_velocities.end(), 10);
    }

    #[test]
    fn probe_hits() {
        let input = input(DAY, Input::Example(0));
        let (x_target, y_target) = parse_target_area(&input);
        assert!(probe_hits_target(&x_target, &y_target, 7, 2));
        assert!(probe_hits_target(&x_target, &y_target, 6, 3));
        assert!(probe_hits_target(&x_target, &y_target, 9, 0));
        assert!(!probe_hits_target(&x_target, &y_target, 17, -4));
    }
}
