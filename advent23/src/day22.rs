use std::array;

use easy_cast::Cast;
use itertools::Itertools;

use shared::{grid::Grid, string::isizes};

type Brick = [RangeInclusive; 3];
type RangeInclusive = [Coordinate; 2];
type Coordinate = isize;

pub fn first_answer(input: &str) -> String {
    let mut bricks = sorted_bricks(input);
    number_of_falls(&mut bricks);
    bricks.sort_unstable_by_key(|&[_, _, [z_min, _]]| z_min);
    number_of_falls_for_each_disintegrated_brick(&bricks)
        .filter(|&number_of_falls| number_of_falls == 0)
        .count()
        .to_string()
}

pub fn second_answer(input: &str) -> String {
    let mut bricks = sorted_bricks(input);
    number_of_falls(&mut bricks);
    bricks.sort_unstable_by_key(|&[_, _, [z_min, _]]| z_min);
    number_of_falls_for_each_disintegrated_brick(&bricks)
        .sum::<usize>()
        .to_string()
}

fn number_of_falls(sorted_bricks: &mut [Brick]) -> usize {
    let [height, width] = bounding_rectangle(sorted_bricks);

    let mut number_of_falls = 0;
    let mut heightmap = Grid::new(height, width, |_| 0);
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

fn bounding_rectangle(bricks: &mut [Brick]) -> [usize; 2] {
    let [x_mins, x_maxes, y_mins, y_maxes] = [
        |&[[x_min, _], _, _]: &Brick| x_min,
        |&[[_, x_max], _, _]: &Brick| x_max,
        |&[_, [y_min, _], _]: &Brick| y_min,
        |&[_, [_, y_max], _]: &Brick| y_max,
    ]
    .map(|extractor| bricks.iter().map(extractor));
    let x_range = [x_mins.min(), x_maxes.max()]
        .map(|extremum| extremum.expect("at least one brick should exist"));
    let y_range = [y_mins.min(), y_maxes.max()]
        .map(|extremum| extremum.expect("at least one brick should exist"));

    for [[x_min, x_max], [y_min, y_max], _] in bricks {
        *x_min -= x_range[0];
        *x_max -= x_range[0];
        *y_min -= y_range[0];
        *y_max -= y_range[0];
    }
    [x_range[1] - x_range[0] + 1, y_range[1] - y_range[0] + 1].cast()
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
    fn first_answer_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 5);
    }

    #[test]
    fn first_answer_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 488);
    }

    #[test]
    fn second_answer_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 7);
    }

    #[test]
    fn second_answer_input() {
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
