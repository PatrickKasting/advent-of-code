use std::ops::RangeInclusive;

use itertools::Itertools;

use crate::{matrix::solution, vector::Vector};

type Hailstone = (Position, Velocity);
type Position = [Coordinate; 3];
type Velocity = [Coordinate; 3];
type Coordinate = f64;

pub fn first(input: &str) -> String {
    let hailstones = hailstones(input);
    let area = 200_000_000_000_000.0..=400_000_000_000_000.0;
    number_of_intersections_in_test_area(area, &hailstones).to_string()
}

pub fn second(input: &str) -> String {
    let hailstones = hailstones(input);
    let (position, _) = initial_position_and_velocity(&hailstones);
    position.into_iter().sum::<Coordinate>().round().to_string()
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

fn path_intersection((s0, v0): Hailstone, (s1, v1): Hailstone) -> Option<[Coordinate; 2]> {
    const X: usize = 0;
    const Y: usize = 1;
    let augmented_matrix: [[Coordinate; 3]; 2] = [
        [v0[X], -v1[X], s1[X] - s0[X]],
        [v0[Y], -v1[Y], s1[Y] - s0[Y]],
    ];
    match &solution(augmented_matrix)[..] {
        [] => None,
        [time] => (time[0] >= 0.0 && time[1] >= 0.0)
            .then(|| [s0[X] + v0[X] * time[0], s0[Y] + v0[Y] * time[0]]),
        _ => panic!("hailstones should not have identical paths"),
    }
}

fn is_inside_area(area: RangeInclusive<Coordinate>, point: [Coordinate; 2]) -> bool {
    point
        .into_iter()
        .all(|coordinate| area.contains(&coordinate))
}

/// Let `s0` and `v0` be the initial position and velocity of the rock. These are the
/// values, we want to determine. We do this by setting up a system of linear equations and solving
/// using Gaussian elimination.
///
/// Let `s[i]` and `v[i]` be the initial position and velocity of the `i`'th hailstone. Let
/// `t[i]` denote the time of collision between the rock and the `i`'th hailstone. For every `i`, we
/// must have `s0 + v0 * t[i] = s[i] + v[i] * t[i]`. Rearranging gives
/// `s0 + -s[i] + (v0 + -v[i]) * t[i] = 0`. Since `t[i]` is scalar, `s0 + -s[i]` and `v0 + -v[i]`
/// must be parallel for the vector sum to equal `0`. Thus, `(s0 + -s[i]) x (v0 + -v[i]) = 0`.
///
/// Distributivity of the cross product yields `((s0 + -s[i]) x v0) + ((s0 + -s[i]) x -v[i]) = 0`.
/// Anticommutativity of the cross product gives `(v0 x (s0 + -s[i])) + (-v[i] x (s0 + -s[i])) = 0`.
/// Invoking distributivity once again results in
/// `(v0 x s0) + (v0 x -s[i]) + (-v[i] x s0) + (-v[i] x -s[i]) = 0`.
///
/// Note that the term `v0 x s0` is not linear in the unknowns, so it is seemingly preventing us
/// from having a system of linear equations. However, `v0 x s0` does not depend on `i`, so this
/// term can be equated for any pair of `i`s.
///
/// Let us equate the above equation for `i = 1` and `i = 2`:
/// `(v0 x -s[1]) + (-v[1] x s0) + (-v[1] x -s[1]) = (v0 x -s[2]) + (-v[2] x s0) + (-v[2] x -s[2])`.
/// Coordinate by coordinate:
/// `(v0[y] * -s[1][z] - v0[z] * -s[1][y]) + (-v[1][y] * s0[z] - -v[1][z] * s0[y])`
/// `(v0[z] * -s[1][x] - v0[x] * -s[1][z]) + (-v[1][z] * s0[x] - -v[1][x] * s0[z])`
/// `(v0[x] * -s[1][y] - v0[y] * -s[1][x]) + (-v[1][x] * s0[y] - -v[1][y] * s0[x])`
fn initial_position_and_velocity(hailstones: &[Hailstone]) -> (Position, Velocity) {
    let mut augmented_matrix: [[Coordinate; 7]; 6] = Default::default();
    augmented_matrix[0..3].copy_from_slice(&equations(hailstones[0], hailstones[1]));
    augmented_matrix[3..6].copy_from_slice(&equations(hailstones[0], hailstones[2]));
    match &solution(augmented_matrix)[..] {
        [solution] => (
            [solution[0], solution[1], solution[2]],
            [solution[3], solution[4], solution[5]],
        ),
        _ => panic!("linear equations should have exactly one solution"),
    }
}

fn equations(left: Hailstone, right: Hailstone) -> [[Coordinate; 7]; 3] {
    let [lhs, rhs] = [coefficients(left), coefficients(right)];
    let mut equations: [[Coordinate; 7]; 3] = lhs
        .into_iter()
        .zip(rhs)
        .map(|(left_equation, right_equation)| left_equation.sub(right_equation))
        .collect_vec()
        .try_into()
        .expect("equation should have the right shape");
    for equation in &mut equations {
        equation[6] = -equation[6];
    }
    equations
}

fn coefficients(([sx, sy, sz], [vx, vy, vz]): Hailstone) -> [[Coordinate; 7]; 3] {
    [
        [0.0, vz, -vy, 0.0, -sz, sy, vy * sz - vz * sy],
        [-vz, 0.0, vx, sz, 0.0, -sx, vz * sx - vx * sz],
        [vy, -vx, 0.0, -sy, sx, 0.0, vx * sy - vy * sx],
    ]
}

fn hailstones(input: &str) -> Vec<Hailstone> {
    input.lines().map(hailstone).collect_vec()
}

fn hailstone(line: &str) -> Hailstone {
    let (position, velocity) = line
        .split_once('@')
        .expect("every line should contain a '@'");
    (coordinates(position), coordinates(velocity))
}

fn coordinates(str: &str) -> [Coordinate; 3] {
    let coordinate = |str: &str| str.parse().expect("coordinate should be numerical");
    str.split(',')
        .map(str::trim)
        .map(coordinate)
        .collect_vec()
        .try_into()
        .expect("coordinates should appear in triplets")
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

    #[test]
    fn second_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 47);
    }

    #[test]
    fn second_input() {
        test_on_input(
            DAY,
            Puzzle::Second,
            Input::PuzzleInput,
            948_978_092_202_212_usize,
        );
    }

    #[test]
    fn coefficients() {
        let hailstone = ([20.0, 19.0, 15.0], [1.0, -5.0, -3.0]);
        let actual = super::coefficients(hailstone);
        let expected = [
            [0.0, -3.0, 5.0, 0.0, -15.0, 19.0, -18.0],
            [3.0, 0.0, 1.0, 15.0, 0.0, -20.0, -75.0],
            [-5.0, -1.0, 0.0, -19.0, 20.0, 0.0, 119.0],
        ];
        assert_eq!(actual, expected);
    }
}
