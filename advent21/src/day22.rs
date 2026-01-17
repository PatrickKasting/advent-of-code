use std::{array, ops::Range};

use itertools::Itertools;
use shared::string::isizes;

type RebootStep = (Switch, Cuboid);
type Switch = bool;
type Cuboid = [Range<Coordinate>; 3];
type Coordinate = isize;

type Direction = usize;
const X: Direction = 0;
const Y: Direction = 1;
const Z: Direction = 2;

pub fn first_answer(input: &str) -> String {
    let initialization_procedure = initialization_procedure(input);
    let cuboids = reboot(initialization_procedure);
    number_of_cubes(&cuboids).to_string()
}

pub fn second_answer(input: &str) -> String {
    let reboot_steps = reboot_steps(input);
    let cuboids = reboot(reboot_steps);
    number_of_cubes(&cuboids).to_string()
}

fn reboot(reboot_steps: impl Iterator<Item = RebootStep>) -> Vec<Cuboid> {
    let mut cuboids = vec![];
    for (switch, cuboid) in reboot_steps {
        cuboids = cuboids
            .into_iter()
            .flat_map(|existing| cuboid_difference(&existing, &cuboid))
            .collect_vec();
        if switch {
            cuboids.push(cuboid);
        }
    }
    cuboids
}

fn cuboid_difference(lhs: &Cuboid, rhs: &Cuboid) -> Vec<Cuboid> {
    if !cuboids_intersect(lhs, rhs) {
        return vec![lhs.clone()];
    }

    let mut lhs = lhs.clone();
    let mut difference = vec![];
    for direction in [X, Y, Z] {
        let (range_difference, range_intersection) =
            range_difference(&lhs[direction], &rhs[direction]);
        debug_assert!(!range_intersection.is_empty(), "cuboids should intersect");

        let differences = range_difference
            .into_iter()
            .map(|range| cuboid_with(&lhs, &range, direction));
        difference.extend(differences);

        lhs = cuboid_with(&lhs, &range_intersection, direction);
    }
    difference
}

fn cuboids_intersect(left: &Cuboid, right: &Cuboid) -> bool {
    let x_ranges_intersect = ranges_intersect(&left[X], &right[X]);
    let y_ranges_intersect = ranges_intersect(&left[Y], &right[Y]);
    let z_ranges_intersect = ranges_intersect(&left[Z], &right[Z]);
    x_ranges_intersect && y_ranges_intersect && z_ranges_intersect
}

fn ranges_intersect(left: &Range<Coordinate>, right: &Range<Coordinate>) -> bool {
    left.start < right.end && right.start < left.end
}

fn range_difference(
    lhs: &Range<Coordinate>,
    rhs: &Range<Coordinate>,
) -> (Vec<Range<Coordinate>>, Range<Coordinate>) {
    if lhs.start < rhs.start && rhs.end < lhs.end {
        let remaining = vec![lhs.start..rhs.start, rhs.end..lhs.end];
        let subtracted = rhs.clone();
        (remaining, subtracted)
    } else if lhs.start < rhs.start && lhs.end <= rhs.end {
        #[expect(
            clippy::single_range_in_vec_init,
            reason = "difference is one contiguous range"
        )]
        (vec![lhs.start..rhs.start], rhs.start..lhs.end)
    } else if rhs.start <= lhs.start && rhs.end < lhs.end {
        #[expect(
            clippy::single_range_in_vec_init,
            reason = "difference is one contiguous range"
        )]
        (vec![rhs.end..lhs.end], lhs.start..rhs.end)
    } else if rhs.start <= lhs.start && lhs.end <= rhs.end {
        (vec![], lhs.clone())
    } else {
        panic!("ranges should intersect");
    }
}

fn cuboid_with(cuboid: &Cuboid, range: &Range<Coordinate>, direction: Direction) -> Cuboid {
    array::from_fn(|index| {
        if index == direction {
            range.clone()
        } else {
            cuboid[index].clone()
        }
    })
}

fn number_of_cubes(cuboids: &[Cuboid]) -> usize {
    cuboids.iter().map(volume).sum()
}

fn volume(cuboid: &Cuboid) -> usize {
    cuboid.iter().map(ExactSizeIterator::len).product()
}

fn initialization_procedure(input: &str) -> impl Iterator<Item = RebootStep> + '_ {
    reboot_steps(input).take_while(|(_, cuboid)| is_within_initialization_area(cuboid))
}

const INITIALIZATION_AREA_LIMIT: Coordinate = 50;

fn is_within_initialization_area(cuboid: &Cuboid) -> bool {
    cuboid.iter().all(|range| {
        -INITIALIZATION_AREA_LIMIT <= range.start && range.end - 1 <= INITIALIZATION_AREA_LIMIT
    })
}

fn reboot_steps(input: &str) -> impl Iterator<Item = RebootStep> + '_ {
    input.lines().map(reboot_step)
}

fn reboot_step(line: &str) -> RebootStep {
    let (switch, cuboid) = line
        .split_once(' ')
        .expect("switch and cuboid should be separated by a space");
    let switch = match switch {
        "on" => true,
        "off" => false,
        _ => panic!("switch should be 'on' or 'off'"),
    };
    (switch, self::cuboid(cuboid))
}

fn cuboid(str: &str) -> Cuboid {
    let coordinates = isizes(str);
    debug_assert_eq!(
        coordinates.len(),
        6,
        "cuboid should be defined by six coordinates"
    );
    [
        coordinates[0]..coordinates[1] + 1,
        coordinates[2]..coordinates[3] + 1,
        coordinates[4]..coordinates[5] + 1,
    ]
}

#[cfg(test)]
mod tests {
    use std::ops::Range;

    use infrastructure::{test, Input, Puzzle};

    use super::*;
    use crate::tests::test_on_input;

    const DAY: usize = 22;

    #[test]
    fn first_examples() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 39);
        test_on_input(DAY, Puzzle::First, Input::Example(1), 590_784);
    }

    #[test]
    fn first_answer_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 570_915);
    }

    #[test]
    fn second_answer_example() {
        test_on_input(
            DAY,
            Puzzle::Second,
            Input::Example(2),
            2_758_514_936_282_235_usize,
        );
    }

    #[test]
    fn second_answer_input() {
        test_on_input(
            DAY,
            Puzzle::Second,
            Input::PuzzleInput,
            1_268_313_839_428_137_usize,
        );
    }

    #[test]
    fn cuboid_difference() {
        let lhs = [5..15, 5..15, 5..15];
        let rhs = [0..20, 7..11, 1..8];
        let difference = super::cuboid_difference(&lhs, &rhs);
        let actual_number_of_cubes = number_of_cubes(&difference);
        let expected_number_of_cubes = 1000 - 10 * 4 * 3;
        assert_eq!(actual_number_of_cubes, expected_number_of_cubes);
    }

    #[test]
    fn range_difference() {
        let sample = 5..15;
        let function = |rhs| super::range_difference(&sample, &rhs);
        #[expect(
            clippy::single_range_in_vec_init,
            reason = "a list of ranges is desired"
        )]
        let cases = [
            (6..14, (vec![5..6, 14..15], 6..14)),
            (1..14, (vec![14..15], 5..14)),
            (14..22, (vec![5..14], 14..15)),
            (4..16, (vec![], 5..15)),
            (5..15, (vec![], 5..15)),
        ];
        test::cases(function, cases);
    }

    #[test]
    fn cuboid_with() {
        let sample = [5..15, 5..15, 5..15];
        let function = |(range, direction)| super::cuboid_with(&sample, &range, direction);
        let cases = [
            ((7..9, X), [7..9, 5..15, 5..15]),
            ((11..15, Y), [5..15, 11..15, 5..15]),
            ((5..15, Z), [5..15, 5..15, 5..15]),
        ];
        test::cases(function, cases);
    }

    #[test]
    fn cuboids_intersect() {
        let function = |[left, right]: [Cuboid; 2]| super::cuboids_intersect(&left, &right);
        let cases = [
            ([[2..4, 2..4, 2..4], [2..4, 2..4, 2..4]], true),
            ([[1..3, 1..3, 1..3], [2..4, 2..4, 2..4]], true),
            ([[2..4, 2..4, 2..4], [4..6, 2..4, 2..4]], false),
            ([[2..4, 2..4, 2..4], [2..4, 4..6, 2..4]], false),
            ([[2..4, 2..4, 2..4], [2..4, 2..4, 4..6]], false),
        ];
        test::cases(function, cases);
    }

    #[test]
    fn ranges_intersect() {
        let function =
            |[left, right]: [Range<Coordinate>; 2]| super::ranges_intersect(&left, &right);
        let cases = [
            ([2..4, 4..6], false),
            ([5..6, 2..5], false),
            ([2..4, 3..7], true),
            ([2..4, 0..3], true),
            ([7..13, 7..13], true),
            ([7..18, 7..13], true),
            ([7..13, 7..14], true),
        ];
        test::cases(function, cases);
    }

    #[test]
    fn is_within_initialization_area() {
        let function = |cuboid| super::is_within_initialization_area(&cuboid);
        let cases = [
            ([-20..26, -36..17, -47..7], true),
            ([-20..33, -21..23, -26..28], true),
            ([-22..28, -29..23, -38..16], true),
            ([-46..7, -6..46, -50..-1], true),
            ([-49..1, -3..46, -24..28], true),
            ([-57795..-6158, 29564..72030, 20435..90618], false),
            ([36731..105_352, -21140..28532, 16094..90401], false),
            ([30999..107_136, -53464..15513, 8553..71215], false),
            ([13528..83982, -99403..-27377, -24141..23996], false),
            ([-72682..-12347, 18159..111_354, 7391..80950], false),
        ];
        test::cases(function, cases);
    }
}
