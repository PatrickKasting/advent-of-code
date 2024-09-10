use std::sync::LazyLock;

use ahash::AHashSet;
use itertools::Itertools;
use shared::{
    matrix::{
        self, quarter_rotation_around_x_axis, quarter_rotation_around_y_axis,
        quarter_rotation_around_z_axis, Matrix,
    },
    string::isizes,
    vector::{ManhattanDistance, Vector},
};

type Scanner = AHashSet<Beacon>;
type Beacon = Position;
type Position = [Coordinate; 3];
type Coordinate = isize;

type Rotation = Matrix<Coordinate, 3, 3>;

const NUMBER_OF_BEACONS_IN_OVERLAP: usize = 12;

pub fn first(input: &str) -> String {
    let mut scanners = scanners(input);
    scanner_positions(&mut scanners);
    let beacons: AHashSet<Beacon> = scanners.into_iter().flatten().collect();
    beacons.len().to_string()
}

pub fn second(input: &str) -> String {
    let mut scanners = scanners(input);
    let scanner_positions = scanner_positions(&mut scanners);
    maximum_manhattan_distance(&scanner_positions).to_string()
}

fn scanner_positions(scanners: &mut [Scanner]) -> Vec<Position> {
    let mut scanner_positions = vec![None; scanners.len()];

    scanner_positions[0] = Some([0, 0, 0]);
    while pin_scanners(scanners, &mut scanner_positions) {}

    let scanner_positions: Option<Vec<_>> = scanner_positions.into_iter().collect();
    scanner_positions.expect("all scanners should have connected positions")
}

fn pin_scanners(scanners: &mut [Scanner], scanner_positions: &mut [Option<Position>]) -> bool {
    let mut pinned_at_least_one = false;
    for (pinned, unpinned) in (0..scanners.len()).cartesian_product(0..scanners.len()) {
        if scanner_positions[pinned].is_none() || scanner_positions[unpinned].is_some() {
            continue;
        }

        if let Some((rotation, position)) = overlap(&scanners[pinned], &scanners[unpinned]) {
            let now_pinned_scanner = scanners[unpinned]
                .iter()
                .map(|&right_beacon| matrix::vector_mul(rotation, right_beacon).add(position))
                .collect();

            scanners[unpinned] = now_pinned_scanner;
            scanner_positions[unpinned] = Some(position);
            pinned_at_least_one = true;
        }
    }
    pinned_at_least_one
}

fn overlap(left_scanner: &Scanner, right_scanner: &Scanner) -> Option<(Rotation, Position)> {
    for rotation in all_rotations() {
        let right_scanner_rotated = right_scanner
            .iter()
            .map(|&right_beacon| matrix::vector_mul(rotation, right_beacon))
            .collect_vec();

        let number_of_duplicates_if_overlap = NUMBER_OF_BEACONS_IN_OVERLAP - 1;
        let translations = left_scanner
            .iter()
            .skip(number_of_duplicates_if_overlap)
            .cartesian_product(&right_scanner_rotated[number_of_duplicates_if_overlap..])
            .map(|(&left, &right)| left.sub(right));
        for translation in translations {
            let right_scanner_rotated_translated = right_scanner_rotated
                .iter()
                .map(|&beacon| beacon.add(translation));

            let mut number_of_matches = 0;
            for right_beacon_rotated_translated in right_scanner_rotated_translated {
                if left_scanner.contains(&right_beacon_rotated_translated) {
                    number_of_matches += 1;
                    if number_of_matches == NUMBER_OF_BEACONS_IN_OVERLAP {
                        return Some((rotation, translation));
                    }
                }
            }
        }
    }
    None
}

fn all_rotations() -> [Rotation; 24] {
    static ALL_ROTATIONS: LazyLock<[Matrix<Coordinate, 3, 3>; 24]> = LazyLock::new(|| {
        let mut all_rotations = AHashSet::new();
        let mut rotation = matrix::identity();
        for _ in 0..4 {
            for _ in 0..4 {
                for _ in 0..4 {
                    all_rotations.insert(rotation);
                    rotation = matrix::matrix_mul(rotation, quarter_rotation_around_z_axis());
                }
                rotation = matrix::matrix_mul(rotation, quarter_rotation_around_y_axis());
            }
            rotation = matrix::matrix_mul(rotation, quarter_rotation_around_x_axis());
        }
        all_rotations
            .into_iter()
            .collect_vec()
            .try_into()
            .expect("exactly 24 rotations should exist")
    });
    *ALL_ROTATIONS
}

fn maximum_manhattan_distance(positions: &[Position]) -> Coordinate {
    positions
        .iter()
        .combinations(2)
        .map(|pair| pair[0].manhattan(*pair[1]))
        .max()
        .expect("at least one position should exist")
}

fn scanners(input: &str) -> Vec<Scanner> {
    input.split("\n\n").map(scanner).collect_vec()
}

fn scanner(str: &str) -> Scanner {
    str.lines().skip(1).map(beacon).collect()
}

fn beacon(line: &str) -> Beacon {
    isizes(line)
        .try_into()
        .expect("each position should contain three coordinates")
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use super::*;
    use crate::tests::{input, test_on_input};

    const DAY: usize = 19;

    #[test]
    fn first_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 79);
    }

    #[test]
    #[ignore]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 449);
    }

    #[test]
    fn second_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 3621);
    }

    #[test]
    #[ignore]
    fn second_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 13128);
    }

    #[test]
    fn scanner_positions() {
        let mut scanners = scanners(&input(DAY, Input::Example(0)));
        let actual = super::scanner_positions(&mut scanners);
        let expected = vec![
            [0, 0, 0],
            [68, -1246, -43],
            [1105, -1205, 1229],
            [-92, -2380, -20],
            [-20, -1133, 1061],
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn overlap() {
        let scanners = scanners(&input(DAY, Input::Example(0)));
        let actual = super::overlap(&scanners[0], &scanners[1]);
        let expected = Some(([[-1, 0, 0], [0, 1, 0], [0, 0, -1]], [68, -1246, -43]));
        assert_eq!(actual, expected)
    }

    #[test]
    fn rotations() {
        let scanner = ORIENTATIONS[0];
        let all_orientations = all_rotations()
            .map(|rotation| scanner.map(|beacon| matrix::vector_mul(rotation, beacon)));
        for orientation in ORIENTATIONS {
            assert!(all_orientations.contains(&orientation));
        }
    }

    const ORIENTATIONS: [[[Coordinate; 3]; 6]; 5] = [
        [
            [-1, -1, 1],
            [-2, -2, 2],
            [-3, -3, 3],
            [-2, -3, 1],
            [5, 6, -4],
            [8, 0, 7],
        ],
        [
            [1, -1, 1],
            [2, -2, 2],
            [3, -3, 3],
            [2, -1, 3],
            [-5, 4, -6],
            [-8, -7, 0],
        ],
        [
            [-1, -1, -1],
            [-2, -2, -2],
            [-3, -3, -3],
            [-1, -3, -2],
            [4, 6, 5],
            [-7, 0, 8],
        ],
        [
            [1, 1, -1],
            [2, 2, -2],
            [3, 3, -3],
            [1, 3, -2],
            [-4, -6, 5],
            [7, 0, 8],
        ],
        [
            [1, 1, 1],
            [2, 2, 2],
            [3, 3, 3],
            [3, 1, 2],
            [-6, -4, -5],
            [0, 7, -8],
        ],
    ];
}
