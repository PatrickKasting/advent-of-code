use std::{fmt::Debug, ops::Range};

use itertools::Itertools;

use crate::utilities::number;

type Coordinate = usize;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Position {
    z: Coordinate,
    y: Coordinate,
    x: Coordinate,
}

impl Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.z)
    }
}

impl From<&str> for Position {
    fn from(str: &str) -> Self {
        let mut coordinates = str.split(',').map(number);
        let mut coordinate = || {
            coordinates
                .next()
                .expect("position should have three coordinates")
        };
        Position {
            x: coordinate(),
            y: coordinate(),
            z: coordinate(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Brick {
    lower_end: Position,
    upper_end: Position,
}

impl Debug for Brick {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}~{:?}", self.lower_end, self.upper_end)
    }
}

impl From<&str> for Brick {
    fn from(line: &str) -> Self {
        let mut positions = line.split('~').map(Position::from).collect_vec();
        positions.sort_unstable();
        Brick {
            lower_end: positions[0],
            upper_end: positions[1],
        }
    }
}

impl Brick {
    fn fall_to(&mut self, z: Coordinate) {
        let height = self.upper_end.z - self.lower_end.z;
        self.lower_end.z = z;
        self.upper_end.z = z + height;
    }

    fn cross_section_overlaps(self, other: Brick) -> bool {
        self.upper_end.x >= other.lower_end.x
            && self.upper_end.y >= other.lower_end.y
            && self.lower_end.x <= other.upper_end.x
            && self.lower_end.y <= other.upper_end.y
    }
}

fn bricks(input: &str) -> Vec<Brick> {
    input.lines().map(Brick::from).collect_vec()
}

fn apply_gravity(sorted_bricks: &mut [Brick]) {
    for brick_index in 0..sorted_bricks.len() {
        let mut brick = sorted_bricks[brick_index];
        let mut brick_below_index = brick_index;
        loop {
            if brick_below_index == 0 {
                brick.fall_to(1);
                break;
            }

            let brick_below = sorted_bricks[brick_below_index - 1];
            if brick.cross_section_overlaps(brick_below) {
                brick.fall_to(brick_below.upper_end.z + 1);
                break;
            }
            brick_below_index -= 1;
        }
        sorted_bricks[brick_index] = brick;
        sorted_bricks[brick_below_index..=brick_index].sort_unstable();
    }
}

fn settled_bricks(input: &str) -> Vec<Brick> {
    let mut bricks = bricks(input);
    bricks.sort_unstable();
    apply_gravity(&mut bricks);
    bricks
}

fn indices_of_bricks_at_height(sorted_bricks: &[Brick], z: Coordinate) -> Range<usize> {
    let singleton = |position| Brick {
        lower_end: position,
        upper_end: position,
    };
    let binary_search = |brick| match sorted_bricks.binary_search(&brick) {
        Ok(index) | Err(index) => index,
    };
    let start = binary_search(singleton(Position {
        x: Coordinate::MIN,
        y: Coordinate::MIN,
        z,
    }));
    let end = binary_search(singleton(Position {
        x: Coordinate::MIN,
        y: Coordinate::MIN,
        z: z + 1,
    }));
    start..end
}

fn indices_of_supported_bricks(sorted_bricks: &[Brick], brick_index: usize) -> Vec<usize> {
    let brick = sorted_bricks[brick_index];
    indices_of_bricks_at_height(sorted_bricks, brick.upper_end.z + 1)
        .filter(|&brick_above_index| brick.cross_section_overlaps(sorted_bricks[brick_above_index]))
        .collect_vec()
}

fn indices_of_disintegrable_bricks(sorted_bricks: &[Brick]) -> impl Iterator<Item = usize> {
    let supported_bricks = (0..sorted_bricks.len())
        .map(|brick| indices_of_supported_bricks(sorted_bricks, brick))
        .collect_vec();
    let mut supporting_bricks = vec![vec![]; sorted_bricks.len()];
    for (supporting, supported) in supported_bricks.iter().enumerate() {
        for &supported in supported {
            supporting_bricks[supported].push(supporting);
        }
    }

    supported_bricks
        .into_iter()
        .enumerate()
        .filter(move |(_, supported)| {
            supported
                .iter()
                .all(|&supported| supporting_bricks[supported].len() > 1)
        })
        .map(|(brick, _)| brick)
}

pub fn first(input: &str) -> String {
    let bricks = settled_bricks(input);
    indices_of_disintegrable_bricks(&bricks).count().to_string()
}

pub fn second(_input: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::{input, tests::*, InputType};

    use super::*;

    const DAY: usize = 22;

    #[test]
    fn overlapping_cross_sections() {
        let bricks = bricks(&input(DAY, InputType::Example(0)));
        let bricks_with_indices = bricks.iter().enumerate();
        let actual: HashSet<[usize; 2]> = bricks_with_indices
            .clone()
            .cartesian_product(bricks_with_indices)
            .filter(|((_, left), (_, right))| left.cross_section_overlaps(**right))
            .map(|((left, _), (right, _))| [left, right])
            .collect();

        let expected = [
            [0, 1],
            [0, 2],
            [0, 5],
            [0, 6],
            [1, 3],
            [1, 4],
            [2, 3],
            [2, 4],
            [3, 5],
            [4, 5],
            [5, 6],
        ];
        let symmetry = expected.map(|[left, right]| [right, left]);
        let reflexivity = (0..bricks.len()).map(|index| [index, index]).collect_vec();
        let expected = HashSet::from_iter(expected.into_iter().chain(symmetry).chain(reflexivity));
        assert_eq!(actual, expected);
    }

    #[test]
    fn settled_stack() {
        let actual = settled_bricks(&input(DAY, InputType::Example(0)));
        let expected = "\
            1,0,1~1,2,1\n\
            0,0,2~2,0,2\n\
            0,2,2~2,2,2\n\
            0,0,3~0,2,3\n\
            2,0,3~2,2,3\n\
            0,1,4~2,1,4\n\
            1,1,5~1,1,6\n\
        ";
        assert_eq!(actual, bricks(expected));
    }

    #[test]
    fn supported_bricks() {
        let settled_bricks = settled_bricks(&input(DAY, InputType::Example(0)));
        let function = |index| indices_of_supported_bricks(&settled_bricks, index);
        let cases = 0..settled_bricks.len();
        let expected = [
            vec![1, 2],
            vec![3, 4],
            vec![3, 4],
            vec![5],
            vec![5],
            vec![6],
            vec![],
        ];
        test_cases(function, cases, expected);
    }

    #[test]
    fn disintegrable_bricks() {
        let settled_bricks = settled_bricks(&input(DAY, InputType::Example(0)));
        let actual: HashSet<usize> = indices_of_disintegrable_bricks(&settled_bricks).collect();
        assert_eq!(actual, HashSet::from([1, 2, 3, 4, 6]));
    }

    // #[test]
    // fn first_example() {
    //     test_on_input(DAY, Puzzle::First, InputType::Example(0), 5);
    // }

    // #[test]
    // fn first_input() {
    //     test_on_input(DAY, Puzzle::First, InputType::PuzzleInput, 56042);
    // }

    // #[test]
    // fn second_example() {
    //     test_on_input(DAY, Puzzle::Second, InputType::Example(1), 281);
    // }

    // #[test]
    // fn second_input() {
    //     test_on_input(DAY, Puzzle::Second, InputType::PuzzleInput, 55358);
    // }
}
