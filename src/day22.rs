use std::ops::Range;

use itertools::Itertools;

use crate::utilities::number;

type Coordinate = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Position {
    z: Coordinate,
    y: Coordinate,
    x: Coordinate,
}

impl Position {
    fn new(x: Coordinate, y: Coordinate, z: Coordinate) -> Self {
        Position { z, y, x }
    }
}

impl From<&str> for Position {
    fn from(str: &str) -> Self {
        let mut coordinates = str.split(',');
        let mut coordinate = || {
            let coordinate = coordinates
                .next()
                .expect("position should contain three coordinates");
            number(coordinate)
        };
        Position::new(coordinate(), coordinate(), coordinate())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Brick {
    lower_end: Position,
    upper_end: Position,
}

impl Brick {
    fn new(one_end: Position, other_end: Position) -> Self {
        let mut ends = [one_end, other_end];
        ends.sort();
        Self {
            lower_end: ends[0],
            upper_end: ends[1],
        }
    }

    fn cross_section_intersects(self, other: Self) -> bool {
        let not_intersecting = self.upper_end.y < other.lower_end.y
            || self.lower_end.y > other.upper_end.y
            || self.upper_end.x < other.lower_end.x
            || self.lower_end.x > other.upper_end.x;
        !not_intersecting
    }

    fn fall_to(&mut self, z: usize) {
        let height = self.upper_end.z - self.lower_end.z;
        self.lower_end.z = z;
        self.upper_end.z = z + height;
    }
}

impl From<&str> for Brick {
    fn from(line: &str) -> Self {
        let (left, right) = line
            .split_once('~')
            .expect("every line should contain a tilde");
        Brick::new(Position::from(left), Position::from(right))
    }
}

fn bricks(input: &str) -> Vec<Brick> {
    input.lines().map(Brick::from).collect_vec()
}

fn brick_below_index(sorted_bricks: &[Brick], brick_index: usize) -> Option<usize> {
    let brick = sorted_bricks[brick_index];
    for (index, &brick_below_candidate) in sorted_bricks[0..brick_index].iter().enumerate().rev() {
        if brick_below_candidate.cross_section_intersects(brick) {
            return Some(index);
        }
    }
    None
}

fn apply_gravity(sorted_bricks: &mut [Brick]) {
    for brick_index in 0..sorted_bricks.len() {
        let (new_brick_index_lower_bound, settled_height) =
            if let Some(brick_below_index) = brick_below_index(sorted_bricks, brick_index) {
                let settled_height = sorted_bricks[brick_below_index].upper_end.z + 1;
                (brick_below_index + 1, settled_height)
            } else {
                (0, 1)
            };
        sorted_bricks[brick_index].fall_to(settled_height);
        sorted_bricks[new_brick_index_lower_bound..=brick_index].sort_unstable();
    }
}

fn bricks_at_height_indices(bricks: &[Brick], z: Coordinate) -> Range<usize> {
    let zero_this_height = Position::new(0, 0, z);
    let lower_bound = Brick::new(zero_this_height, zero_this_height);
    let zero_next_height = Position::new(0, 0, z + 1);
    let upper_bound = Brick::new(zero_next_height, zero_next_height);
    let lower_bound = match bricks.binary_search(&lower_bound) {
        Ok(lower_bound) | Err(lower_bound) => lower_bound,
    };
    let upper_bound = match bricks.binary_search(&upper_bound) {
        Ok(upper_bound) | Err(upper_bound) => upper_bound,
    };
    lower_bound..upper_bound
}

fn supported_brick_indices(settled_bricks: &[Brick], brick: Brick) -> Vec<usize> {
    let supported_bricks_height = brick.upper_end.z + 1;
    bricks_at_height_indices(settled_bricks, supported_bricks_height)
        .filter(|&above_brick_index| {
            settled_bricks[above_brick_index].cross_section_intersects(brick)
        })
        .collect_vec()
}

fn all_supported_brick_indices(settled_bricks: &[Brick]) -> Vec<Vec<usize>> {
    settled_bricks
        .iter()
        .map(|&brick| supported_brick_indices(settled_bricks, brick))
        .collect_vec()
}

fn all_supporting_brick_indices(all_supported_brick_indices: &[Vec<usize>]) -> Vec<Vec<usize>> {
    let mut all_supporting_brick_indices = all_supported_brick_indices
        .iter()
        .map(|_| vec![])
        .collect_vec();
    for (below_brick_index, above_brick_indices) in all_supported_brick_indices.iter().enumerate() {
        for &above_brick_index in above_brick_indices {
            all_supporting_brick_indices[above_brick_index].push(below_brick_index);
        }
    }
    all_supporting_brick_indices
}

fn disintegrable(
    all_supported_brick_indices: &[Vec<usize>],
    all_supporting_brick_indices: &[Vec<usize>],
    brick_index: usize,
) -> bool {
    all_supported_brick_indices[brick_index]
        .iter()
        .all(|&supported_brick_index| all_supporting_brick_indices[supported_brick_index].len() > 1)
}

fn disintegrable_brick_indices(settled_bricks: &[Brick]) -> impl Iterator<Item = usize> {
    let all_supported_brick_indices = all_supported_brick_indices(settled_bricks);
    let all_supporting_brick_indices = all_supporting_brick_indices(&all_supported_brick_indices);
    (0..settled_bricks.len()).filter(move |&brick_index| {
        disintegrable(
            &all_supported_brick_indices,
            &all_supporting_brick_indices,
            brick_index,
        )
    })
}

pub fn first(input: &str) -> String {
    let mut bricks = bricks(input);
    bricks.sort_unstable();
    apply_gravity(&mut bricks);
    disintegrable_brick_indices(&bricks).count().to_string()
}

pub fn second(_input: &str) -> String {
    todo!();
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::{input, tests::*, Input, Puzzle};

    use super::*;

    const DAY: usize = 22;

    fn settled_bricks_from_input(input: Input) -> Vec<Brick> {
        let mut bricks = bricks(&crate::input(DAY, input));
        bricks.sort_unstable();
        apply_gravity(&mut bricks);
        bricks
    }

    #[test]
    fn cross_section_intersects() {
        let bricks = bricks(&input(DAY, Input::Example(0)));
        let bricks_with_indices = bricks.iter().enumerate();

        let actual: HashSet<[usize; 2]> = bricks_with_indices
            .clone()
            .cartesian_product(bricks_with_indices)
            .filter(|((_, left), (_, right))| left.cross_section_intersects(**right))
            .map(|((left_index, _), (right_index, _))| [left_index, right_index])
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
        let expected = expected
            .into_iter()
            .chain(symmetry)
            .chain(reflexivity)
            .collect();
        assert_eq!(actual, expected);
    }

    #[test]
    fn settled_bricks() {
        let bricks = settled_bricks_from_input(Input::Example(0));
        let expected = "\
            1,0,1~1,2,1\n\
            0,0,2~2,0,2\n\
            0,2,2~2,2,2\n\
            0,0,3~0,2,3\n\
            2,0,3~2,2,3\n\
            0,1,4~2,1,4\n\
            1,1,5~1,1,6\n\
        ";
        assert_eq!(bricks, super::bricks(expected));
    }

    #[test]
    fn bricks_at_height_indices() {
        let bricks = settled_bricks_from_input(Input::Example(0));
        test_cases(
            |z| super::bricks_at_height_indices(&bricks, z),
            [2, 3, 4],
            [1..3, 3..5, 5..6],
        );
    }

    #[test]
    fn supported_brick_indices() {
        let bricks = settled_bricks_from_input(Input::Example(0));
        let expected = [
            vec![1, 2],
            vec![3, 4],
            vec![3, 4],
            vec![5],
            vec![5],
            vec![6],
            vec![],
        ];
        test_cases(
            |&brick| super::supported_brick_indices(&bricks, brick),
            bricks.iter(),
            expected,
        );
    }

    #[test]
    fn disintegrable_brick_indices() {
        let bricks = settled_bricks_from_input(Input::Example(0));
        let actual = super::disintegrable_brick_indices(&bricks).collect_vec();
        let expected = vec![1, 2, 3, 4, 6];
        assert_eq!(actual, expected);
    }

    #[test]
    fn first_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 5);
    }
}
