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
    simulation(&mut elves, 10);
    number_of_free_tiles(&elves).to_string()
}

pub fn second(_input: &str) -> String {
    todo!();
}

fn simulation(elves: &mut HashSet<Position>, number_of_rounds: usize) {
    let mut directions = INITIAL_DIRECTIONS;
    for _ in 0..number_of_rounds {
        round(elves, directions);
        directions.rotate_left(1);
    }
}

fn round(elves: &mut HashSet<Position>, directions: [Direction; 4]) {
    let proposals: HashMap<Position, Position> = elves
        .iter()
        .filter_map(|&elf| proposal(elves, elf, directions).map(|proposal| (elf, proposal)))
        .collect();
    let proposal_counts = proposals.iter().counts_by(|(_, &proposal)| proposal);
    let accepted = proposals
        .into_iter()
        .filter(|(_, proposal)| proposal_counts[proposal] == 1);
    for (old, new) in accepted {
        elves.remove(&old);
        let inserted = elves.insert(new);
        debug_assert!(inserted, "elf should not be moved to occupied tile");
    }
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

    // #[test]
    // fn second_example() {
    //     test_on_input(DAY, Puzzle::Second, Input::Example(0), 20);
    // }

    // #[test]
    // fn second_input() {
    //     test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 404_395);
    // }

    #[test]
    fn ten_rounds_large_example() {
        let input = input(YEAR, DAY, Input::Example(0));
        let mut elves = super::elves(&input);

        simulation(&mut elves, 10);
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

        simulation(&mut elves, 3);
        let expected = [(0, 2), (1, 4), (2, 0), (3, 4), (5, 2)]
            .map(|(row, column)| Position::new(row, column));
        let expected = HashSet::from(expected);
        assert_eq!(elves, expected);
    }

    #[test]
    fn one_round_small_example() {
        let input = input(YEAR, DAY, Input::Example(1));
        let mut elves = elves(&input);

        round(&mut elves, INITIAL_DIRECTIONS);
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
        let cases = [(1, 2), (1, 3), (2, 2), (4, 2), (4, 3)]
            .map(|(row, column)| Position::new(row, column));
        let expected = [(0, 2), (0, 3), (3, 2), (3, 2), (3, 3)]
            .map(|(row, column)| Some(Position::new(row, column)));
        test_cases(function, cases, expected);
    }
}
