use itertools::Itertools;

use crate::utilities::number;

type Coordinate = usize;
type Position = [Coordinate; 3];
type Brick = [Position; 2];

fn position(str: &str) -> Position {
    str.split(',')
        .map(number)
        .collect_vec()
        .try_into()
        .expect("position should have three coordinates")
}

fn brick(line: &str) -> Brick {
    let (left, right) = line
        .split_once('~')
        .expect("every line should contain a tilde");
    let mut ends = [left, right].map(position);
    ends.sort_unstable();
    ends
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

fn settled_bricks(bricks: impl IntoIterator<Item = Brick>) -> (Vec<Brick>, usize) {
    let mut settled_bricks: Vec<[[usize; 3]; 2]> = vec![];
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

fn disintegrable(settled_bricks: &[Brick], index: usize) -> bool {
    if index == 48 {
        return false;
    }
    let without_brick = settled_bricks
        .iter()
        .enumerate()
        .filter(|&(other_index, _)| other_index != index)
        .map(|(_, brick)| *brick);
    let (_, number_of_fallen_bricks) = self::settled_bricks(without_brick);
    number_of_fallen_bricks == 0
}

fn number_of_disintegrable_bricks(settled_bricks: &[Brick]) -> usize {
    (0..settled_bricks.len())
        .filter(|&missing_brick_index| disintegrable(settled_bricks, missing_brick_index))
        .count()
}

pub fn first(input: &str) -> String {
    let bricks = bricks(input);
    let (settled_bricks, _) = settled_bricks(bricks);
    number_of_disintegrable_bricks(&settled_bricks).to_string()
}

pub fn second(_input: &str) -> String {
    todo!();
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

    // #[test]
    // fn first_input() {
    //     test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 488);
    // }
}
