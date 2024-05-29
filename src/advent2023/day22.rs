use itertools::Itertools;

use crate::string::usizes;

type Brick = [Position; 2];
type Position = [Coordinate; 3];
type Coordinate = usize;

pub fn first(input: &str) -> String {
    let bricks = bricks(input);
    let (settled_bricks, _) = settled_bricks(bricks);
    number_of_disintegrable_bricks(&settled_bricks).to_string()
}

pub fn second(_input: &str) -> String {
    unimplemented!();
}

fn number_of_disintegrable_bricks(settled_bricks: &[Brick]) -> usize {
    (0..settled_bricks.len())
        .filter(|&missing_brick_index| disintegrable(settled_bricks, missing_brick_index))
        .count()
}

fn disintegrable(settled_bricks: &[Brick], index: usize) -> bool {
    let without_brick = settled_bricks
        .iter()
        .enumerate()
        .filter(|&(other_index, _)| other_index != index)
        .map(|(_, brick)| *brick);
    let (_, number_of_fallen_bricks) = self::settled_bricks(without_brick);
    number_of_fallen_bricks == 0
}

fn settled_bricks(bricks: impl IntoIterator<Item = Brick>) -> (Vec<Brick>, usize) {
    let mut settled_bricks = vec![];
    let mut number_of_fallen_bricks = 0;
    for mut brick in bricks {
        let mut distance = brick[0][2] - 1;
        for &settled_brick in settled_bricks.iter().rev() {
            if shadows_intersect(brick, settled_brick) {
                distance -= settled_brick[1][2];
                break;
            }
        }
        brick[0][2] -= distance;
        brick[1][2] -= distance;
        settled_bricks.push(brick);
        if distance > 0 {
            number_of_fallen_bricks += 1;
        }
    }
    (settled_bricks, number_of_fallen_bricks)
}

fn shadows_intersect(left: Brick, right: Brick) -> bool {
    let not_intersecting = left[1][1] < right[0][1]
        || left[0][1] > right[1][1]
        || left[1][0] < right[0][0]
        || left[0][0] > right[1][0];
    !not_intersecting
}

fn bricks(input: &str) -> Vec<Brick> {
    let mut bricks = input.lines().map(brick).collect_vec();
    bricks.sort_unstable_by(|left, right| left[0][2].cmp(&right[0][2]));
    bricks
}

fn brick(line: &str) -> Brick {
    let coordinates = usizes(line);
    let mut ends = [
        [0, 1, 2].map(|index| coordinates[index]),
        [3, 4, 5].map(|index| coordinates[index]),
    ];
    ends.sort_unstable();
    ends
}

#[cfg(test)]
mod tests {
    use super::super::tests::test_on_input;
    use crate::{Input, Puzzle};

    const DAY: usize = 22;

    #[test]
    fn first_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 5);
        test_on_input(DAY, Puzzle::First, Input::Example(1), 3);
        test_on_input(DAY, Puzzle::First, Input::Example(2), 2);
    }
}
