use std::cmp::Ordering;

use ahash::AHashSet;
use itertools::Itertools;

use shared::{
    string::isizes,
    vector::{CrossProduct, Vector},
};

type Scanner = Vec<Beacon>;
type Beacon = Position;
type Position = [Coordinate; 3];
type Direction = [Coordinate; 3];
type Coordinate = isize;

pub fn first(input: &str) -> String {
    let scanners = scanners(input);
    beacon_positions(scanners).len().to_string()
}

pub fn second(_input: &str) -> String {
    todo!()
}

fn beacon_positions(mut scanners: Vec<Scanner>) -> AHashSet<Position> {
    scanners[0].sort_unstable();
    let mut not_connected_indices = (1..scanners.len()).collect_vec();
    let mut connected_indices = vec![0];
    while !not_connected_indices.is_empty() {
        'connect_one: for &connected_index in &connected_indices {
            for &not_connected_index in &not_connected_indices {
                let (connected, other) =
                    get_two_mut(&mut scanners, connected_index, not_connected_index);
                if connect(connected, other) {
                    scanners[not_connected_index].sort_unstable();
                    not_connected_indices.retain(|&index| index != not_connected_index);
                    connected_indices.push(not_connected_index);
                    break 'connect_one;
                }
            }
        }
    }
    scanners.into_iter().flatten().collect()
}

fn connect(sorted_fixed_scanner: &[Beacon], scanner: &mut [Beacon]) -> bool {
    for _ in 0..4 {
        for _ in 0..4 {
            for _ in 0..4 {
                if let Some(translation) = position(sorted_fixed_scanner, scanner) {
                    translate(scanner, translation);
                    return true;
                }
                rotate(scanner, [0, 0, 1]);
            }
            rotate(scanner, [0, 1, 0]);
        }
        rotate(scanner, [1, 0, 0]);
    }
    false
}

fn position(sorted_fixed_scanner: &[Beacon], scanner: &[Beacon]) -> Option<Position> {
    for &fixed in sorted_fixed_scanner {
        for &other in scanner {
            let translation = other.sub(fixed);
            let number_of_matches = scanner
                .iter()
                .filter(|&other| {
                    let matching = other.sub(translation);
                    sorted_fixed_scanner.binary_search(&matching).is_ok()
                })
                .count();
            if number_of_matches >= 12 {
                return Some(translation);
            }
        }
    }
    None
}

fn translate(scanner: &mut [Beacon], translation: Position) {
    for beacon in scanner {
        *beacon = beacon.add(translation);
    }
}

fn rotate(scanner: &mut [Beacon], around: Direction) {
    debug_assert_eq!(
        around.norm(),
        1,
        "direction should be a cardinal unit vector"
    );
    for beacon in scanner {
        let component_parallel_to_direction = beacon.dot(around);
        *beacon = beacon.cross(around);
        *beacon = beacon.add(around.mul(component_parallel_to_direction));
    }
}

// fn maximum_manhattan_distance(scanner_positions: Vec<Beacon>) -> Coordinate {
//     scanner_positions
//         .iter()
//         .flat_map(|&left| {
//             scanner_positions
//                 .iter()
//                 .map(move |&right| manhattan_distance(left, right))
//         })
//         .max()
//         .expect("there should be at least one beacon")
// }

// fn manhattan_distance(left: Beacon, right: Beacon) -> Coordinate {
//     left.into_iter()
//         .zip(right)
//         .map(|(left, right)| (left - right).abs())
//         .sum()
// }

fn scanners(input: &str) -> Vec<Scanner> {
    input.split("\n\n").map(scanner).collect_vec()
}

fn scanner(str: &str) -> Scanner {
    str.lines().skip(1).map(beacon).collect_vec()
}

fn beacon(line: &str) -> Beacon {
    isizes(line)
        .try_into()
        .expect("beacon position should consist of three coordinates")
}

fn get_two_mut<T>(slice: &mut [T], one: usize, other: usize) -> (&mut T, &mut T) {
    match one.cmp(&other) {
        Ordering::Less => {
            let split = one + 1;
            let (left, right) = slice.split_at_mut(split);
            (&mut left[one], &mut right[other - split])
        }
        Ordering::Equal => panic!("indices should be different"),
        Ordering::Greater => {
            let split = other + 1;
            let (left, right) = slice.split_at_mut(split);
            (&mut right[one - split], &mut left[other])
        }
    }
}

// type Scanner = HashSet<Beacon>;
// type Beacon = Position;
// type Position = [Coordinate; 3];
// type Coordinate = isize;

// const NUM_REQUIRED_OVERLAPS: usize = 12;

// pub fn first(input: String) -> String {
//     let mut scanners = parse_scanners(&input);
//     let scanner_positions = scanner_positions(&mut scanners);
//     beacon_positions(&scanner_positions, &scanners)
//         .len()
//         .to_string()
// }

// pub fn second(input: String) -> String {
//     let mut scanners = parse_scanners(&input);
//     let scanner_positions = scanner_positions(&mut scanners);
//     maximum_manhattan_distance(scanner_positions).to_string()
// }

// fn parse_scanner<'input>(lines: &mut impl Iterator<Item = &'input str>) -> Scanner {
//     let header = lines.next().expect("a scanner should have a header");
//     debug_assert!(
//         header.contains("scanner"),
//         "scanner header does not contain the word 'scanner'"
//     );
//     let mut beacons = HashSet::new();
//     for line in lines {
//         if line.is_empty() {
//             return beacons;
//         }
//         let coordinates = line
//             .split(',')
//             .map(|coordinate| {
//                 coordinate
//                     .parse()
//                     .expect("every coordinate should be a parsable integer")
//             })
//             .collect::<Vec<Coordinate>>()
//             .try_into()
//             .expect("every point should contain three coordinates");
//         beacons.insert(coordinates);
//     }
//     beacons
// }

// fn parse_scanners(input: &str) -> Vec<Scanner> {
//     let mut scanners = Vec::new();
//     let mut lines = input.lines().peekable();
//     while lines.peek().is_some() {
//         scanners.push(parse_scanner(&mut lines));
//     }
//     scanners
// }

// fn all_rotations() -> HashSet<Matrix<Coordinate>> {
//     todo!()
// }

// fn translation(fixed: &Scanner, other: &Scanner) -> Option<Beacon> {
//     for &this_beacon in fixed.iter() {
//         for &other_beacon in other.iter() {
//             let translation = this_beacon.sub(other_beacon);
//             let num_overlaps = other
//                 .iter()
//                 .filter(|&&beacon| fixed.contains(&(translation.add(beacon))))
//                 .count();
//             if num_overlaps >= NUM_REQUIRED_OVERLAPS {
//                 return Some(translation);
//             }
//         }
//     }
//     None
// }

// fn aligned(fixed: &Scanner, other: &Scanner) -> Option<(Position, Scanner)> {
//     for &rotation in all_rotations {
//         let rotated = other.iter().map(|&beacon| rotation * beacon).collect();
//         if let Some(translation) = translation(fixed, &rotated) {
//             return Some((translation, rotated));
//         }
//     }
//     None
// }

// fn scanner_positions(scanners: &mut [Scanner]) -> Vec<Beacon> {
//     let mut scanner_positions = vec![[0, 0, 0]; scanners.len()];
//     let mut connected = vec![false; scanners.len()];
//     connected[0] = true;
//     let num_scanners = scanners.len();
//     let mut indices = (0..num_scanners)
//         .flat_map(|left| (0..num_scanners).map(move |right| (left, right)))
//         .filter(|(left, right)| left != right)
//         .cycle();
//     let (mut old_index, mut new_index);
//     loop {
//         if connected.iter().all(ToOwned::to_owned) {
//             break;
//         }
//         while {
//             (old_index, new_index) = indices.next().expect("indices should cycle");
//             !connected[old_index] || connected[new_index]
//         } {}
//         if let Some((translation, aligned)) =
//             aligned(&all_rotations, &scanners[old_index], &scanners[new_index])
//         {
//             scanners[new_index] = aligned;
//             scanner_positions[new_index] = scanner_positions[old_index].add(translation);
//             connected[new_index] = true;
//         }
//     }
//     scanner_positions
// }

// fn beacon_positions(scanner_positions: &[Beacon], scanners: &[Scanner]) -> HashSet<Beacon> {
//     scanner_positions
//         .iter()
//         .zip(scanners.iter())
//         .flat_map(|(&position, scanner)| scanner.iter().map(move |&beacon| position.add(beacon)))
//         .collect()
// }

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use super::*;
    use crate::tests::test_on_input;

    const DAY: usize = 19;

    #[test]
    fn first_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 79);
    }

    // #[test]
    // fn first_input() {
    //     test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 449);
    // }

    #[test]
    fn second_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 3621);
    }

    // #[test]
    // fn second_input() {
    //     test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 13128);
    // }

    // #[test]
    // fn overlap() {
    //     let input = input(YEAR, DAY, Input::Example(0));
    //     let scanners = parse_scanners(&input);
    //     let aligned = aligned(&all_rotations(), &scanners[0], &scanners[1]);
    //     let (actual_position, _) = aligned.expect("there should be an overlap");
    //     assert_eq!(actual_position, [68, -1246, -43]);
    // }

    // #[test]
    // fn positions() {
    //     let input = input(YEAR, DAY, Input::Example(0));
    //     let mut scanners = parse_scanners(&input);
    //     let actual_positions = scanner_positions(&mut scanners);
    //     let expected_positions = [
    //         [0, 0, 0],
    //         [68, -1246, -43],
    //         [1105, -1205, 1229],
    //         [-92, -2380, -20],
    //         [-20, -1133, 1061],
    //     ];
    //     assert_eq!(&actual_positions, &expected_positions);
    // }

    // #[test]
    // fn rotations() {
    //     assert_eq!(all_rotations().len(), 24);
    // }

    #[test]
    fn rotate() {
        let mut scanner = vec![[2, 1, 1], [-2, -1, 2]];
        super::rotate(&mut scanner, [0, 0, 1]);
        assert_eq!(scanner, vec![[1, -2, 1], [-1, 2, 2]]);
    }

    #[test]
    fn get_two_mut() {
        let mut elements = (0..10).collect_vec();
        assert_eq!(super::get_two_mut(&mut elements, 2, 9), (&mut 2, &mut 9));
        assert_eq!(super::get_two_mut(&mut elements, 4, 5), (&mut 4, &mut 5));
        assert_eq!(super::get_two_mut(&mut elements, 6, 0), (&mut 6, &mut 0));
        assert_eq!(super::get_two_mut(&mut elements, 7, 6), (&mut 7, &mut 6));
    }
}
