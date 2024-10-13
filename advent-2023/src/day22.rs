use std::array;

use itertools::Itertools;

use shared::{grid::Grid, string::isizes};

type Brick = [RangeInclusive; 3];
type RangeInclusive = [Coordinate; 2];
type Coordinate = isize;

const HEIGHTMAP_SIZE: usize = 10;

pub fn first(input: &str) -> String {
    let mut bricks = sorted_bricks(input);
    number_of_falls(&mut bricks);
    bricks.sort_unstable_by_key(|&[_, _, [z_min, _]]| z_min);
    number_of_falls_for_each_disintegrated_brick(&bricks)
        .filter(|&number_of_falls| number_of_falls == 0)
        .count()
        .to_string()
}

pub fn second(input: &str) -> String {
    let mut bricks = sorted_bricks(input);
    number_of_falls(&mut bricks);
    bricks.sort_unstable_by_key(|&[_, _, [z_min, _]]| z_min);
    number_of_falls_for_each_disintegrated_brick(&bricks)
        .sum::<usize>()
        .to_string()
}

fn number_of_falls<'brick>(sorted_bricks: impl IntoIterator<Item = &'brick mut Brick>) -> usize {
    let mut number_of_falls = 0;
    let mut heightmap = Grid::new(HEIGHTMAP_SIZE, HEIGHTMAP_SIZE, |_| 0);
    for brick in sorted_bricks {
        let fall_distance = fall_distance(&heightmap, *brick);
        if fall_distance != 0 {
            number_of_falls += 1;
            brick[2][0] -= fall_distance;
            brick[2][1] -= fall_distance;
        }
        for (x, y) in brick_shadow(*brick) {
            *heightmap
                .get_mut([x, y])
                .expect("brick shadow should be within heightmap") = brick[2][1];
        }
    }
    number_of_falls
}

fn number_of_falls_for_each_disintegrated_brick(
    sorted_settled_bricks: &[Brick],
) -> impl Iterator<Item = usize> + '_ {
    (0..sorted_settled_bricks.len()).map(move |index| {
        let mut bricks = Vec::from(sorted_settled_bricks);
        bricks.remove(index);
        number_of_falls(&mut bricks)
    })
}

fn fall_distance(heightmap: &Grid<Coordinate>, brick @ [_, _, [z_min, _]]: Brick) -> Coordinate {
    let resting_height = brick_shadow(brick)
        .map(|(x, y)| heightmap[[x, y]])
        .max()
        .expect("brick shadow should be at least 1x1");
    z_min - resting_height - 1
}

fn brick_shadow(
    [[x_min, x_max], [y_min, y_max], _]: Brick,
) -> impl Iterator<Item = (Coordinate, Coordinate)> {
    (x_min..=x_max).cartesian_product(y_min..=y_max)
}

fn sorted_bricks(input: &str) -> Vec<Brick> {
    input
        .lines()
        .map(brick)
        .sorted_unstable_by_key(|&[_, _, [z_min, _]]| z_min)
        .collect_vec()
}

fn brick(line: &str) -> Brick {
    let coordinates = isizes(line);
    array::from_fn(|index| [coordinates[index], coordinates[index + 3]])
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use super::*;
    use crate::tests::{input, test_on_input};

    const DAY: usize = 22;

    #[test]
    fn first_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 5);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 488);
    }

    #[test]
    fn second_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 7);
    }

    #[test]
    fn second_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 79465);
    }

    #[test]
    fn number_of_falls() {
        let mut bricks = sorted_bricks(&input(DAY, Input::Example(0)));
        let actual = super::number_of_falls(&mut bricks);
        let expected = vec![
            [[1, 1], [0, 2], [1, 1]],
            [[0, 2], [0, 0], [2, 2]],
            [[0, 2], [2, 2], [2, 2]],
            [[0, 0], [0, 2], [3, 3]],
            [[2, 2], [0, 2], [3, 3]],
            [[0, 2], [1, 1], [4, 4]],
            [[1, 1], [1, 1], [5, 6]],
        ];
        assert_eq!(actual, 5);
        assert_eq!(bricks, expected);
    }
}
