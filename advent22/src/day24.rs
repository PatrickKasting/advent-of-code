use ahash::AHashSet;
use easy_cast::{Cast, Conv};
use itertools::Itertools;

use shared::{
    grid::{self, Coordinate, Direction, Position},
    vector::Vector,
};

type Blizzards = Vec<Blizzard>;
type Blizzard = (Position, Direction);
type Minutes = usize;

pub fn first_answer(input: &str) -> String {
    let (dimensions @ [height, width], mut blizzards) = valley(input);
    let [start, end] = [[0, 1], [height - 1, width - 2]];
    fastest_journey(dimensions, &mut blizzards, &[start, end]).to_string()
}

pub fn second_answer(input: &str) -> String {
    let (dimensions @ [height, width], mut blizzards) = valley(input);
    let [start, end] = [[0, 1], [height - 1, width - 2]];
    let journey = [start, end, start, end];
    fastest_journey(dimensions, &mut blizzards, &journey).to_string()
}

fn fastest_journey(
    valley_dimensions: [Coordinate; 2],
    blizzards: &mut Blizzards,
    journey: &[Position],
) -> Minutes {
    journey
        .windows(2)
        .map(|pair| fastest_path(valley_dimensions, blizzards, pair[0], pair[1]))
        .sum()
}

fn fastest_path(
    valley_dimensions: [Coordinate; 2],
    blizzards: &mut Blizzards,
    start: Position,
    end: Position,
) -> Minutes {
    let mut positions = AHashSet::from([start]);
    let mut time = 0;
    loop {
        time += 1;
        move_blizzards(valley_dimensions, blizzards);
        let mut valid_neighbors = vec![];
        for &position in &positions {
            for neighbor in grid::neighbors(position) {
                if neighbor == end {
                    return time;
                }
                let is_on_boundary = is_outside_valley(valley_dimensions, neighbor);
                let is_in_blizzard = blizzards
                    .binary_search_by_key(&neighbor, |&(position, _)| position)
                    .is_ok();
                if !is_on_boundary && !is_in_blizzard {
                    valid_neighbors.push(neighbor);
                }
            }
        }
        positions.retain(|position| {
            blizzards
                .binary_search_by_key(position, |&(position, _)| position)
                .is_err()
        });
        positions.extend(valid_neighbors);
    }
}

fn move_blizzards(valley_dimensions: [Coordinate; 2], blizzards: &mut Blizzards) {
    for (position, direction) in blizzards.iter_mut() {
        *position = position.add(*direction);
        if is_outside_valley(valley_dimensions, *position) {
            *position = opposite_side(valley_dimensions, *position, *direction);
        }
    }
    blizzards.sort_unstable();
}

fn is_outside_valley([height, width]: [Coordinate; 2], [row, column]: Position) -> bool {
    row <= 0
        || row >= Coordinate::conv(height) - 1
        || column == 0
        || column == Coordinate::conv(width) - 1
}

fn opposite_side(
    [height, width]: [Coordinate; 2],
    [row, column]: Position,
    direction: Direction,
) -> Position {
    match direction {
        grid::NORTH => [height - 2, column],
        grid::EAST => [row, 1],
        grid::SOUTH => [1, column],
        grid::WEST => [row, width - 2],
        _ => panic!("direction should be one of four unit vectors"),
    }
}

fn valley(input: &str) -> ([Coordinate; 2], Blizzards) {
    let lines = input.lines().collect_vec();
    let dimensions = [lines.len().cast(), lines[0].len().cast()];
    let blizzards = lines
        .into_iter()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars().enumerate().filter_map(move |(column, char)| {
                let blizzard_direction = match char {
                    '^' => Some(grid::NORTH),
                    '>' => Some(grid::EAST),
                    'v' => Some(grid::SOUTH),
                    '<' => Some(grid::WEST),
                    _ => None,
                }?;
                let position = [row.cast(), column.cast()];
                Some((position, blizzard_direction))
            })
        })
        .collect_vec();
    debug_assert!(
        blizzards.windows(2).all(|pair| pair[0] <= pair[1]),
        "blizzards should always be sorted"
    );
    (dimensions, blizzards)
}

#[cfg(test)]
mod tests {
    use infrastructure::{test, Input, Puzzle};

    use super::*;
    use crate::tests::test_on_input;

    const DAY: usize = 24;

    #[test]
    fn first_answer_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(1), 18);
    }

    #[test]
    fn first_answer_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 240);
    }

    #[test]
    fn second_answer_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(1), 54);
    }

    #[test]
    fn second_answer_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 717);
    }

    #[test]
    fn two_irrelevant_blizzards() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 10);
    }

    #[test]
    fn opposite_side() {
        let function = |(position, direction)| super::opposite_side([6, 7], position, direction);
        let cases = [
            (([0, 5], grid::NORTH), [4, 5]),
            (([3, 7], grid::EAST), [3, 1]),
            (([5, 3], grid::SOUTH), [1, 3]),
            (([4, 0], grid::WEST), [4, 5]),
        ];
        test::cases(function, cases);
    }
}
