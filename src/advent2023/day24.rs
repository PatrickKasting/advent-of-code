use std::ops::RangeInclusive;

use itertools::Itertools;

use crate::matrix::solution;
use crate::strings::parse;

type Coordinate = f64;
type Position = [Coordinate; 3];
type Velocity = [Coordinate; 3];
type Hailstone = (Position, Velocity);

fn coordinates(str: &str) -> [Coordinate; 3] {
    str.split(',')
        .map(str::trim)
        .map(parse)
        .collect_vec()
        .try_into()
        .expect("coordinates should appear in triplets")
}

fn hailstone(line: &str) -> Hailstone {
    let (position, velocity) = line
        .split_once('@')
        .expect("every line should contain a '@'");
    (coordinates(position), coordinates(velocity))
}

fn hailstones(input: &str) -> Vec<Hailstone> {
    input.lines().map(hailstone).collect_vec()
}

fn path_intersection(first: Hailstone, second: Hailstone) -> Option<[Coordinate; 2]> {
    let augmented_matrix = [
        [first.1[0], -second.1[0], second.0[0] - first.0[0]],
        [first.1[1], -second.1[1], second.0[1] - first.0[1]],
    ];
    match &solution(augmented_matrix)[..] {
        [] => None,
        [intersection] => {
            let [first_time, second_time] = [intersection[0], intersection[1]];
            (first_time >= 0.0 && second_time >= 0.0).then(|| {
                [
                    first.0[0] + first.1[0] * first_time,
                    first.0[1] + first.1[1] * first_time,
                ]
            })
        }
        _ => panic!("paths should not have identical trajectories"),
    }
}

fn is_inside_area(area: RangeInclusive<Coordinate>, point: [Coordinate; 2]) -> bool {
    point
        .into_iter()
        .all(|coordinate| area.contains(&coordinate))
}

fn number_of_intersections_in_test_area(
    area: RangeInclusive<Coordinate>,
    hailstones: &[Hailstone],
) -> usize {
    hailstones
        .iter()
        .combinations(2)
        .filter_map(|pair| path_intersection(*pair[0], *pair[1]))
        .filter(|&intersection| is_inside_area(area.clone(), intersection))
        .count()
}

pub fn first(input: &str) -> String {
    let hailstones = hailstones(input);
    let area = 200_000_000_000_000.0..=400_000_000_000_000.0;
    number_of_intersections_in_test_area(area, &hailstones).to_string()
}

pub fn second(_input: &str) -> String {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::super::tests::{test_on_input, YEAR};
    use crate::{input, Input, Puzzle};

    use super::*;

    const DAY: usize = 24;

    #[test]
    fn first_example() {
        let hailstones = hailstones(&input(YEAR, DAY, Input::Example(0)));
        let number_of_intersections = number_of_intersections_in_test_area(7.0..=27.0, &hailstones);
        assert_eq!(number_of_intersections, 2);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 17776);
    }

    // #[test]
    // fn second_example() {
    //     test_on_input(DAY, Puzzle::Second, Input::Example(1), 281);
    // }
}
