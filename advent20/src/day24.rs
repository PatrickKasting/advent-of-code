use ahash::AHashSet;
use shared::{grid::Position, vector::Vector};

type Floor = AHashSet<Position>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

pub fn first_answer(input: &str) -> String {
    let walks = walks(input);
    floor(walks).len().to_string()
}

pub fn second_answer(input: &str) -> String {
    let walks = walks(input);
    let floor = floor(walks);
    floor_after(100, floor).len().to_string()
}

fn floor(
    walks: impl IntoIterator<Item = impl IntoIterator<Item = Direction>>,
) -> AHashSet<Position> {
    let mut floor = AHashSet::new();
    for walk in walks {
        let destination = destination(walk);
        if !floor.insert(destination) {
            floor.remove(&destination);
        }
    }
    floor
}

fn destination(walk: impl IntoIterator<Item = Direction>) -> Position {
    let mut position = [0, 0];
    for direction in walk {
        let step = match direction {
            Direction::East => [2, 0],
            Direction::SouthEast => [1, 1],
            Direction::SouthWest => [-1, 1],
            Direction::West => [-2, 0],
            Direction::NorthWest => [-1, -1],
            Direction::NorthEast => [1, -1],
        };
        position = position.add(step);
    }
    position
}

fn floor_after(days: usize, mut floor: Floor) -> Floor {
    for _ in 1..=days {
        floor = next_floor(&floor);
    }
    floor
}

fn next_floor(floor: &Floor) -> Floor {
    let neighborhood: AHashSet<Position> = floor
        .iter()
        .flat_map(|&position| neighbors(position))
        .chain(floor.iter().copied())
        .collect();
    neighborhood
        .into_iter()
        .filter(|&position| should_be_black(floor, position))
        .collect()
}

fn should_be_black(floor: &Floor, position: Position) -> bool {
    let is_black = floor.contains(&position);
    let number_of_black_neighbors = number_of_black_neighbors(floor, position);
    let is_black_with_one_or_two_black_neighbors =
        is_black && [1, 2].contains(&number_of_black_neighbors);
    let is_white_with_two_black_neighbors = !is_black && number_of_black_neighbors == 2;
    is_black_with_one_or_two_black_neighbors || is_white_with_two_black_neighbors
}

fn number_of_black_neighbors(floor: &Floor, position: Position) -> usize {
    neighbors(position)
        .into_iter()
        .filter(|neighbor| floor.contains(neighbor))
        .count()
}

fn neighbors(position: Position) -> [Position; 6] {
    let steps = [[2, 0], [1, 1], [-1, 1], [-2, 0], [-1, -1], [1, -1]];
    steps.map(|step| position.add(step))
}

fn walks(input: &str) -> impl Iterator<Item = Vec<Direction>> + '_ {
    input.lines().map(walk)
}

fn walk(mut str: &str) -> Vec<Direction> {
    let mut walk = vec![];
    while !str.is_empty() {
        let (remaining, direction) = direction(str);
        str = remaining;
        walk.push(direction);
    }
    walk
}

fn direction(str: &str) -> (&str, Direction) {
    match &str[0..1] {
        "e" => (&str[1..], Direction::East),
        "w" => (&str[1..], Direction::West),
        _ => match &str[0..2] {
            "se" => (&str[2..], Direction::SouthEast),
            "sw" => (&str[2..], Direction::SouthWest),
            "nw" => (&str[2..], Direction::NorthWest),
            "ne" => (&str[2..], Direction::NorthEast),
            _ => panic!("direction should be 'e', 'se', 'sw', 'w', 'nw', or 'ne'"),
        },
    }
}

#[cfg(test)]
mod tests {
    use ahash::AHashMap;
    use infrastructure::{test, Input, Puzzle};

    use super::*;
    use crate::tests::{input, test_on_input};

    const DAY: usize = 24;

    #[test]
    fn first_answer_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 10);
    }

    #[test]
    fn first_answer_puzzle_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 465);
    }

    #[test]
    fn second_answer_example() {
        let input = input(DAY, Input::Example(0));
        let mut floor = floor(walks(&input));

        let expected = AHashMap::from([
            (1, 15),
            (2, 12),
            (3, 25),
            (4, 14),
            (5, 23),
            (6, 28),
            (7, 41),
            (8, 37),
            (9, 49),
            (10, 37),
            (20, 132),
            (30, 259),
            (40, 406),
            (50, 566),
            (60, 788),
            (70, 1106),
            (80, 1373),
            (90, 1844),
            (100, 2208),
        ]);

        for day in 1..=100 {
            floor = next_floor(&floor);

            if let Some(&expected) = expected.get(&day) {
                assert_eq!(
                    floor.len(),
                    expected,
                    "number of black tiles should match expected for day {day}"
                );
            }
        }
    }

    #[test]
    fn second_answer_puzzle_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 4078);
    }

    #[test]
    fn should_be_black() {
        let function =
            |(floor, position)| super::should_be_black(&AHashSet::from_iter(floor), position);
        let cases = [
            (
                (vec![[0, 2], [0, -2], [4, 0], [3, 1], [0, 0]], [0, 0]),
                false,
            ),
            ((vec![[5, 3], [7, 3], [6, 0], [6, 4], [6, 2]], [6, 2]), true),
            (
                (vec![[4, 4], [5, 5], [2, 6], [1, 5], [2, 4]], [3, 5]),
                false,
            ),
            ((vec![[4, 4], [5, 4], [2, 7], [0, 5], [2, 4]], [3, 5]), true),
        ];
        test::cases(function, cases);
    }

    #[test]
    fn number_of_black_neighbors() {
        let function = |(floor, position)| {
            super::number_of_black_neighbors(&AHashSet::from_iter(floor), position)
        };
        let cases = [
            ((vec![[0, 2], [0, -2], [4, 0], [3, 1]], [0, 0]), 0),
            ((vec![[5, 3], [7, 3], [6, 0], [6, 4]], [6, 2]), 2),
            ((vec![[4, 4], [5, 5], [2, 6], [1, 5], [2, 4]], [3, 5]), 5),
        ];
        test::cases(function, cases);
    }

    #[test]
    fn neighbors() {
        let function = |position| AHashSet::from(super::neighbors(position));
        let cases = [
            (
                [0, 0],
                AHashSet::from([[2, 0], [1, 1], [-1, 1], [-2, 0], [-1, -1], [1, -1]]),
            ),
            (
                [3, 4],
                AHashSet::from([[5, 4], [4, 5], [2, 5], [1, 4], [2, 3], [4, 3]]),
            ),
        ];
        test::cases(function, cases);
    }
}
