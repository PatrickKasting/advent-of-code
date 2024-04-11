use std::collections::HashSet;

use itertools::Itertools;

use crate::data_structures::grid::{Direction, Position};

type Blizzard = (Position, Direction);
type Blizzards = Vec<Blizzard>;

pub fn first(input: &str) -> String {
    let (dimensions @ [height, width], mut blizzards) = valley(input);
    fastest_path(
        dimensions,
        &mut blizzards,
        Position::new(0, 1),
        Position::new(height - 1, width - 2),
    )
    .to_string()
}

pub fn second(_input: &str) -> String {
    todo!();
}

fn fastest_path(
    valley_dimensions: [usize; 2],
    blizzards: &mut Blizzards,
    start: Position,
    end: Position,
) -> usize {
    let mut minutes = vec![HashSet::from([start])];
    loop {
        move_blizzards(valley_dimensions, blizzards);
        minutes.push(HashSet::new());
        for minute in (0..minutes.len() - 1).rev() {
            let mut next_minute = vec![];
            for position in &minutes[minute] {
                for neighbor in position.neighbors() {
                    if neighbor == end {
                        return minutes.len() - 1;
                    }
                    let is_on_boundary = is_outside_valley(valley_dimensions, neighbor);
                    let is_blizzard = blizzards
                        .binary_search_by_key(&neighbor, |&(position, _)| position)
                        .is_ok();
                    if !is_on_boundary && !is_blizzard {
                        next_minute.push(neighbor);
                    }
                }
            }
            minutes[minute + 1].extend(next_minute);

            minutes[minute].retain(|&position| {
                blizzards
                    .binary_search_by_key(&position, |&(position, _)| position)
                    .is_err()
            });
        }
    }
}

fn move_blizzards(valley_dimensions: [usize; 2], blizzards: &mut Blizzards) {
    for (position, direction) in blizzards.iter_mut() {
        *position = position.neighbor(*direction);
        if is_outside_valley(valley_dimensions, *position) {
            *position = opposite_side(valley_dimensions, *position, *direction);
        }
    }
    blizzards.sort_unstable();
}

#[allow(clippy::cast_possible_wrap)]
fn is_outside_valley(valley_dimensions: [usize; 2], position: Position) -> bool {
    let [height, width] = valley_dimensions;
    position.row() <= 0
        || position.row() >= height as isize - 1
        || position.column() == 0
        || position.column() == width as isize - 1
}

fn opposite_side(
    valley_dimensions: [usize; 2],
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

fn valley(input: &str) -> ([usize; 2], Blizzards) {
    let lines = input.lines().collect_vec();
    let dimensions = [lines.len(), lines[0].len()];
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
                let position = Position::new(row, column);
                Some((position, blizzard_direction))
            })
        })
        .collect_vec();
    let is_sorted = blizzards.windows(2).all(|pair| pair[0] <= pair[1]);
    debug_assert!(is_sorted, "blizzards should always be sorted");
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

    // #[test]
    // fn second_example() {
    //     test_on_input(DAY, Puzzle::Second, Input::Example(1), 54);
    // }

    // #[test]
    // fn second_input() {
    //     test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 404_395);
    // }

    #[test]
    fn two_irrelevant_blizzards() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 10);
    }

    #[test]
    fn opposite_side() {
        let function = |(position, direction)| super::opposite_side([6, 7], position, direction);
        let cases = [
            (Position::new(0, 5), Direction::North),
            (Position::new(3, 7), Direction::East),
            (Position::new(5, 3), Direction::South),
            (Position::new(4, 0), Direction::West),
        ];
        let expected = [
            Position::new(4, 5),
            Position::new(3, 1),
            Position::new(1, 3),
            Position::new(4, 5),
        ];
        test_cases(function, cases, expected);
    }
}
