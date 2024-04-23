use std::collections::HashSet;

use itertools::Itertools;

use crate::data_structures::grid::{Coordinate, Direction, Position};

type Blizzards = Vec<Blizzard>;
type Blizzard = (Position, Direction);
type Minutes = usize;

pub fn first(input: &str) -> String {
    let (dimensions @ [height, width], mut blizzards) = valley(input);
    let [start, end] = [Position::new(0, 1), Position::new(height - 1, width - 2)];
    fastest_journey(dimensions, &mut blizzards, &[start, end]).to_string()
}

pub fn second(input: &str) -> String {
    let (dimensions @ [height, width], mut blizzards) = valley(input);
    let [start, end] = [Position::new(0, 1), Position::new(height - 1, width - 2)];
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
    let mut positions = HashSet::from([start]);
    let mut time = 0;
    loop {
        time += 1;
        move_blizzards(valley_dimensions, blizzards);
        let mut valid_neighbors = vec![];
        for position in &positions {
            for neighbor in position.neighbors() {
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
        *position = position.neighbor(*direction);
        if is_outside_valley(valley_dimensions, *position) {
            *position = opposite_side(valley_dimensions, *position, *direction);
        }
    }
    blizzards.sort_unstable();
}

#[allow(clippy::cast_possible_wrap)]
fn is_outside_valley(valley_dimensions: [Coordinate; 2], position: Position) -> bool {
    let [height, width] = valley_dimensions;
    position.row() <= 0
        || position.row() >= height as Coordinate - 1
        || position.column() == 0
        || position.column() == width as Coordinate - 1
}

fn opposite_side(
    valley_dimensions: [Coordinate; 2],
    position: Position,
    direction: Direction,
) -> Position {
    let [height, width] = valley_dimensions;
    let [row, column] = [position.row(), position.column()];
    match direction {
        Direction::North => Position::new(height - 2, column),
        Direction::East => Position::new(row, 1),
        Direction::South => Position::new(1, column),
        Direction::West => Position::new(row, width - 2),
    }
}

fn valley(input: &str) -> ([Coordinate; 2], Blizzards) {
    let lines = input.lines().collect_vec();
    #[allow(clippy::cast_possible_wrap)]
    let dimensions = [lines.len() as Coordinate, lines[0].len() as Coordinate];
    let blizzards = lines
        .into_iter()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars().enumerate().filter_map(move |(column, char)| {
                let blizzard_direction = match char {
                    '^' => Some(Direction::North),
                    '>' => Some(Direction::East),
                    'v' => Some(Direction::South),
                    '<' => Some(Direction::West),
                    _ => None,
                }?;
                #[allow(clippy::cast_possible_wrap)]
                let position = Position::new(row as Coordinate, column as Coordinate);
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
    use super::super::tests::test_on_input;
    use crate::{
        data_structures::grid::{Direction, Position},
        tests::test_cases,
        Input, Puzzle,
    };

    const DAY: usize = 24;

    #[test]
    fn first_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(1), 18);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 240);
    }

    #[test]
    fn second_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(1), 54);
    }

    #[test]
    fn second_input() {
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
            ((Position::new(0, 5), Direction::North), Position::new(4, 5)),
            ((Position::new(3, 7), Direction::East), Position::new(3, 1)),
            ((Position::new(5, 3), Direction::South), Position::new(1, 3)),
            ((Position::new(4, 0), Direction::West), Position::new(4, 5)),
        ];
        test_cases(function, cases);
    }
}
