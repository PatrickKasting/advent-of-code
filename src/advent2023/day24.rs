use std::ops::RangeInclusive;

use itertools::Itertools;

use crate::{linear_equations::solution_set, vector::Vector};

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
    match solution_set(augmented_matrix) {
        None => None,
        Some((origin, directions)) if directions.is_empty() => {
            let time = origin;
            let collision_in_future_for_both_hailstones = time[0] >= 0.0 && time[1] >= 0.0;
            collision_in_future_for_both_hailstones
                .then(|| [s0[X] + v0[X] * time[0], s0[Y] + v0[Y] * time[0]])
        }
        _ => panic!("hailstones should not have identical paths"),
    }
}

fn is_inside_area(range: RangeInclusive<Coordinate>, point: [Coordinate; 2]) -> bool {
    point
        .into_iter()
        .all(|coordinate| range.contains(&coordinate))
}

/// Returns the initial position and velocity needed for the rock to hit every hailstone.
///
/// # Correctness
///
/// The overall strategy is to set up system of linear equations that we can solve using our solver.
///
/// Let `s0` and `v0` be the initial position and velocity of the rock. Let `s[i]` and `v[i]` be the
/// initial position and velocity of the `i`'th hailstone. Let `t[i]` denote the time of collision
/// between the rock and the `i`'th hailstone. The unknowns are `s0`, `v0`, and the `t[i]`s.
///
/// For every `i`, we have a collision: `s0 + v0 * t[i] == s[i] + v[i] * t[i]`. The term `v0 * t[i]`
/// is not linear in the unknowns, so this equation does not fit in our system of linear equations.
/// Rearranging gives `s[i] + -s0 + (v[i] + -v0) * t[i] == 0` and since `t[i]` is scalar,
/// `s[i] + -s0` and `v[i] + -v0` must be parallel for the vector sum to equal `0`. Thus,
/// `(s[i] + -s0) x (v[i] + -v0) == 0`.
///
/// Distributivity of the cross product yields `((s[i] + -s0) x v[i]) + ((s[i] + -s0) x -v0) == 0`.
/// Anticommutativity of the cross product gives
/// `-(v[i] x (s[i] + -s0)) + -(-v0 x (s[i] + -s0)) == 0`. Multiplying both sides by `-1` gives
/// `(v[i] x (s[i] + -s0)) + (-v0 x (s[i] + -s0)) == 0`. Invoking distributivity once again results
/// in `(v[i] x s[i]) + (v[i] x -s0) + (-v0 x s[i]) + (-v0 x -s0) == 0`.
///
/// Note that the term `-v0 x -s0` is not linear in the unknowns, so it is seemingly preventing us
/// from having a system of linear equations. However, `-v0 x -s0` does not depend on `i`, so this
/// term can be equated for any pair of `i`s, leaving us with linear equations! The other three
/// terms `(v[i] x s[i]) + (v[i] x -s0) + (-v0 x s[i])` can be written as a product of the following
/// matrix `A[i]` and the vector `v == [s0[x], s0[y], s0[z], v0[x], v0[y], v0[z], 1]`:
///
/// ```ignore
/// |        0,  v[i][z], -v[i][y],        0, -s[i][z],  s[i][y], v[i][y]*s[i][z]-v[i][z]*s[i][y] |
/// | -v[i][z],        0,  v[i][x],  s[i][z],        0, -s[i][x], v[i][z]*s[i][x]-v[i][x]*s[i][z] |
/// |  v[i][y], -v[i][x],        0, -s[i][y],  s[i][x],        0, v[i][x]*s[i][y]-v[i][y]*s[i][x] |
/// ```
///
/// Picking the pairs of `i`s `(0, 1)` and `(0, 2)` yields `A[0] . v == A[1] . v` and
/// `A[0] . v == A[2] . v`, which are six linear equations. This is enough to determine our six
/// unknowns `s0[x]`, `s0[y]`, `s0[z]`, `v0[x]`, `v0[y]`, and `v0[z]`.
///
/// To bring these equations into standard matrix-equation form, we subtract the right-hand sides,
/// which gives `(A[0] - A[1]) . v == 0` and `(A[0] - A[2]) . v == 0`. Finally, moving the constants
/// to the now-empty right-hand sides yields a system of linear equations that can be solved.
fn initial_position_and_velocity(hailstones: &[Hailstone]) -> (Position, Velocity) {
    let mut augmented_matrix: [[Coordinate; 7]; 6] = Default::default();
    augmented_matrix[0..3].copy_from_slice(&equations(hailstones[0], hailstones[1]));
    augmented_matrix[3..6].copy_from_slice(&equations(hailstones[0], hailstones[2]));
    match solution_set(augmented_matrix) {
        Some((origin, directions)) if directions.is_empty() => (
            [origin[0], origin[1], origin[2]],
            [origin[3], origin[4], origin[5]],
        ),
        _ => panic!("linear equations should have exactly one solution"),
    }
}

/// Let `s[i]` and `v[i]` be the initial position and velocity of the `i`'th hailstone. Then, let
/// `A[i]` be the following matrix (notice the signs of the last column):
///
/// ```ignore
/// |        0,  v[i][z], -v[i][y],        0, -s[i][z],  s[i][y], -v[i][y]*s[i][z]+v[i][z]*s[i][y] |
/// | -v[i][z],        0,  v[i][x],  s[i][z],        0, -s[i][x], -v[i][z]*s[i][x]+v[i][x]*s[i][z] |
/// |  v[i][y], -v[i][x],        0, -s[i][y],  s[i][x],        0, -v[i][x]*s[i][y]+v[i][y]*s[i][x] |
/// ```
///
/// Given the hailstones `j` and `k`, this function returns `A[j] - A[k]`.
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

/// Given the `i`th hailstone `(s[i], v[i])`, return the following matrix:
///
/// ```ignore
/// |        0,  v[i][z], -v[i][y],        0, -s[i][z],  s[i][y], v[i][y]*s[i][z]-v[i][z]*s[i][y] |
/// | -v[i][z],        0,  v[i][x],  s[i][z],        0, -s[i][x], v[i][z]*s[i][x]-v[i][x]*s[i][z] |
/// |  v[i][y], -v[i][x],        0, -s[i][y],  s[i][x],        0, v[i][x]*s[i][y]-v[i][y]*s[i][x] |
/// ```
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
