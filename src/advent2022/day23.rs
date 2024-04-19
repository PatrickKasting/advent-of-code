use std::{
    collections::{HashMap, HashSet},
    iter,
};

use itertools::Itertools;

use crate::data_structures::grid::{Direction, Position};

const INITIAL_DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::South,
    Direction::West,
    Direction::East,
];

pub fn first(input: &str) -> String {
    let mut elves = elves(input);
    simulation(&mut elves, Some(10));
    number_of_free_tiles(&elves).to_string()
}

pub fn second(input: &str) -> String {
    let mut elves = elves(input);
    let number_of_rounds_before_steady_state =
        simulation(&mut elves, None).expect("steady state should be reached");
    number_of_rounds_before_steady_state.to_string()
}

fn simulation(
    elves: &mut HashSet<Position>,
    maximum_number_of_rounds: Option<usize>,
) -> Option<usize> {
    let mut directions = INITIAL_DIRECTIONS;
    for number_of_rounds in 1..=maximum_number_of_rounds.unwrap_or(usize::MAX) {
        let is_steady_state = round(elves, directions);
        if is_steady_state {
            return Some(number_of_rounds);
        }
        directions.rotate_left(1);
    }
    None
}

fn round(elves: &mut HashSet<Position>, directions: [Direction; 4]) -> bool {
    let proposals: HashMap<Position, Position> = elves
        .iter()
        .filter_map(|&elf| proposal(elves, elf, directions).map(|proposal| (elf, proposal)))
        .collect();
    let proposal_counts = proposals.iter().counts_by(|(_, &proposal)| proposal);
    let accepted = proposals
        .into_iter()
        .filter(|(_, proposal)| proposal_counts[proposal] == 1);
    let mut is_steady_state = true;
    for (old, new) in accepted {
        elves.remove(&old);
        let inserted = elves.insert(new);
        debug_assert!(inserted, "elf should not be moved to occupied tile");
        is_steady_state = false;
    }
    is_steady_state
}

fn proposal(
    elves: &HashSet<Position>,
    elf: Position,
    directions: [Direction; 4],
) -> Option<Position> {
    let is_free_in_direction = directions.map(|direction| {
        let neighbor = elf.neighbor(direction);
        let [left_corner, right_corner] =
            [Direction::left, Direction::right].map(|turn| neighbor.neighbor(turn(direction)));
        let is_free = [neighbor, left_corner, right_corner]
            .into_iter()
            .all(|neighbor| !elves.contains(&neighbor));
        (direction, is_free)
    });
    let no_neighbors = is_free_in_direction.into_iter().all(|(_, is_free)| is_free);
    if no_neighbors {
        None
    } else {
        is_free_in_direction
            .into_iter()
            .find_map(|(direction, is_free)| is_free.then(|| elf.neighbor(direction)))
    }
}

fn number_of_free_tiles(elves: &HashSet<Position>) -> isize {
    let [height, width] = bounding_rectangle(elves);
    let number_of_elves: isize = elves
        .len()
        .try_into()
        .expect("number of elves should be less then 'isize::MAX'");
    height * width - number_of_elves
}

fn bounding_rectangle(elves: &HashSet<Position>) -> [isize; 2] {
    [Position::row, Position::column].map(|coordinate| {
        let (min, max) = elves
            .iter()
            .map(|&elf| coordinate(elf))
            .minmax()
            .into_option()
            .expect("at least one elf should be present");
        max - min + 1
    })
}

fn elves(input: &str) -> HashSet<Position> {
    input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| iter::repeat(row).zip(line.chars().enumerate()))
        .filter(|&(_, (_, char))| char == '#')
        .map(|(row, (column, _))| Position::new(row, column))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{
        super::tests::{test_on_input, YEAR},
        *,
    };
    use crate::{input, tests::test_cases};
    use crate::{Input, Puzzle};

    const DAY: usize = 23;

    #[test]
    fn first_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 110);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 4241);
    }

    #[test]
    fn second_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 20);
    }

    #[test]
    fn second_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 1079);
    }

    #[test]
    fn ten_rounds_large_example() {
        let input = input(YEAR, DAY, Input::Example(0));
        let mut elves = super::elves(&input);

        let steady_state = simulation(&mut elves, Some(10));
        assert_eq!(steady_state, None);

        let expected = "\
            .......#......\n\
            ...........#..\n\
            ..#.#..#......\n\
            ......#.......\n\
            ...#.....#..#.\n\
            .#......##....\n\
            .....##.......\n\
            ..#........#..\n\
            ....#.#..#....\n\
            ..............\n\
            ....#..#..#...\n\
            ..............\n\
        ";
        let expected = super::elves(expected)
            .into_iter()
            .map(|elf| Position::new(elf.row() - 2, elf.column() - 3))
            .collect();
        assert_eq!(elves, expected);
    }

    #[test]
    fn several_rounds_small_example() {
        let input = input(YEAR, DAY, Input::Example(1));
        let mut elves = elves(&input);

        let steady_state = simulation(&mut elves, Some(3));
        assert_eq!(steady_state, None);

        let expected = [(0, 2), (1, 4), (2, 0), (3, 4), (5, 2)]
            .map(|(row, column)| Position::new(row, column));
        let expected = HashSet::from(expected);
        assert_eq!(elves, expected);
    }

    #[test]
    fn one_round_small_example() {
        let input = input(YEAR, DAY, Input::Example(1));
        let mut elves = elves(&input);

        let is_steady_state = round(&mut elves, INITIAL_DIRECTIONS);
        assert!(!is_steady_state);

        let expected = [(0, 2), (0, 3), (2, 2), (4, 2), (3, 3)]
            .map(|(row, column)| Position::new(row, column));
        let expected = HashSet::from(expected);
        assert_eq!(elves, expected);
    }

    #[test]
    fn proposals() {
        let input = input(YEAR, DAY, Input::Example(1));
        let elves = elves(&input);

        let function = |elf| proposal(&elves, elf, INITIAL_DIRECTIONS);
        let cases = [
            ((1, 2), (0, 2)),
            ((1, 3), (0, 3)),
            ((2, 2), (3, 2)),
            ((4, 2), (3, 2)),
            ((4, 3), (3, 3)),
        ]
        .map(|(input, expected)| {
            (
                Position::new(input.0, input.1),
                Some(Position::new(expected.0, expected.1)),
            )
        });
        test_cases(function, cases);
    }
}
