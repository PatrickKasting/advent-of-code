use std::{array, ops::Range, str};

use easy_cast::{Cast, Conv};
use itertools::Itertools;
use shared::{grid::Grid, search};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Amphipod {
    Amber = 0,
    Bronze = 1,
    Copper = 2,
    Desert = 3,
}

type Burrow<const ROOM_SIZE: usize> = (Hallway, [Room<ROOM_SIZE>; NUMBER_OF_ROOMS]);
type Room<const ROOM_SIZE: usize> = [Option<Amphipod>; ROOM_SIZE];
type Hallway = [Option<Amphipod>; NUMBER_OF_HALLWAY_POSITIONS];
type Position = usize;
type Energy = usize;

const NUMBER_OF_ROOMS: usize = 4;
const HALLWAY_SPACES: [usize; NUMBER_OF_HALLWAY_POSITIONS] = [1, 2, 4, 6, 8, 10, 11];
const NUMBER_OF_HALLWAY_POSITIONS: usize = 7;

pub fn first_answer(input: &str) -> String {
    let burrow = burrow(input);
    least_total_energy_to_organize(burrow).to_string()
}

pub fn second_answer(input: &str) -> String {
    let burrow = unfold(burrow(input));
    least_total_energy_to_organize(burrow).to_string()
}

fn least_total_energy_to_organize<const ROOM_SIZE: usize>(burrow: Burrow<ROOM_SIZE>) -> Energy {
    let successors =
        |burrow| successors_move_out_all_rooms(burrow).chain(successors_move_in(burrow));
    search::minimum_path_cost(burrow, successors, is_organized)
        .expect("amphipods should be able to organize")
}

fn successors_move_out_all_rooms<const ROOM_SIZE: usize>(
    burrow: Burrow<ROOM_SIZE>,
) -> impl Iterator<Item = (Burrow<ROOM_SIZE>, Energy)> {
    (0..NUMBER_OF_ROOMS)
        .flat_map(move |room_index| successors_move_out_one_room(burrow, room_index))
}

fn successors_move_out_one_room<const ROOM_SIZE: usize>(
    burrow @ (hallway, rooms): Burrow<ROOM_SIZE>,
    room_index: usize,
) -> impl Iterator<Item = (Burrow<ROOM_SIZE>, Energy)> {
    move_out(room_index, rooms[room_index])
        .into_iter()
        .flat_map(move |(successor_room, room_space, amphipod)| {
            let left = hallway
                .into_iter()
                .enumerate()
                .take(2 + room_index)
                .rev()
                .take_while(|(_, amphipod)| amphipod.is_none());
            let right = hallway
                .into_iter()
                .enumerate()
                .skip(2 + room_index)
                .take_while(|(_, amphipod)| amphipod.is_none());
            left.chain(right).map(move |(position, _)| {
                let mut successor = burrow;
                successor.0[position] = Some(amphipod);
                successor.1[room_index] = successor_room;
                let energy =
                    number_of_steps(position, room_index, room_space) * energy_per_step(amphipod);
                (successor, energy)
            })
        })
}

fn move_out<const ROOM_SIZE: usize>(
    room_index: usize,
    mut room: Room<ROOM_SIZE>,
) -> Option<(Room<ROOM_SIZE>, usize, Amphipod)> {
    let all_inhabitants_are_home = !room
        .into_iter()
        .any(|amphipod| amphipod.is_some_and(|amphipod| amphipod as usize != room_index));
    if all_inhabitants_are_home {
        return None;
    }

    room.into_iter()
        .enumerate()
        .find_map(|(room_space, amphipod)| {
            amphipod.map(|amphipod| {
                room[room_space] = None;
                (room, room_space, amphipod)
            })
        })
}

fn successors_move_in<const ROOM_SIZE: usize>(
    burrow @ (hallway, _): Burrow<ROOM_SIZE>,
) -> impl Iterator<Item = (Burrow<ROOM_SIZE>, Energy)> {
    hallway
        .into_iter()
        .enumerate()
        .filter_map(|(position, amphipod)| amphipod.map(|amphipod| (position, amphipod)))
        .filter_map(move |(position, amphipod)| successor_move_in(burrow, position, amphipod))
}

fn successor_move_in<const ROOM_SIZE: usize>(
    burrow @ (hallway, rooms): Burrow<ROOM_SIZE>,
    position: Position,
    amphipod: Amphipod,
) -> Option<(Burrow<ROOM_SIZE>, Energy)> {
    let room_index = amphipod as usize;
    let is_path_clear = is_path_clear(hallway, position, amphipod);
    (is_path_clear && is_vacant(rooms[room_index], amphipod)).then(|| {
        let mut successor = burrow;
        successor.0[position] = None;
        let room_space;
        (successor.1[room_index], room_space) = move_in(successor.1[room_index], amphipod);
        let energy =
            number_of_steps(position, amphipod as usize, room_space) * energy_per_step(amphipod);
        (successor, energy)
    })
}

fn is_path_clear(hallway: Hallway, position: Position, amphipod: Amphipod) -> bool {
    hallway[path(position, amphipod as usize)]
        .iter()
        .all(Option::is_none)
}

fn path(position: Position, room_index: usize) -> Range<Position> {
    let room_position = 2 + room_index;
    if room_position <= position {
        room_position..position
    } else {
        position + 1..room_position
    }
}

fn is_vacant<const ROOM_SIZE: usize>(room: Room<ROOM_SIZE>, amphipod: Amphipod) -> bool {
    let has_empty_space = room[0].is_none();
    let only_similar_amphipods = !room
        .into_iter()
        .rev()
        .take_while(Option::is_some)
        .any(|inhabitant| inhabitant.expect("'take_while' should yield only 'Some'") != amphipod);
    has_empty_space && only_similar_amphipods
}

fn move_in<const ROOM_SIZE: usize>(
    mut room: Room<ROOM_SIZE>,
    amphipod: Amphipod,
) -> (Room<ROOM_SIZE>, usize) {
    let (space, element) = room
        .iter_mut()
        .enumerate()
        .rev()
        .find(|(_, amphipod)| amphipod.is_none())
        .expect("at least one side room space should be vacant");
    *element = Some(amphipod);
    (room, space)
}

fn number_of_steps(position: Position, room_index: usize, room_space: usize) -> Energy {
    let hallway_space = HALLWAY_SPACES[position];
    let room_column = 3 + 2 * room_index;
    hallway_space.abs_diff(room_column) + room_space + 1
}

fn energy_per_step(amphipod: Amphipod) -> Energy {
    10_usize.pow(amphipod as u32)
}

fn is_organized<const ROOM_SIZE: usize>((_, rooms): Burrow<ROOM_SIZE>) -> bool {
    rooms.into_iter().enumerate().all(|(index, room)| {
        room.into_iter()
            .all(|amphipod| amphipod.is_some_and(|amphipod| amphipod as usize == index))
    })
}

fn unfold((hallway, rooms): Burrow<2>) -> Burrow<4> {
    let extras = [['D', 'D'], ['C', 'B'], ['B', 'A'], ['A', 'C']].map(|room| room.map(amphipod));
    let unfolded_rooms = array::from_fn(|index| {
        [
            rooms[index][0],
            extras[index][0],
            extras[index][1],
            rooms[index][1],
        ]
    });
    (hallway, unfolded_rooms)
}

fn burrow(input: &str) -> Burrow<2> {
    let mut lines = input
        .lines()
        .map(str::as_bytes)
        .map(ToOwned::to_owned)
        .collect_vec();
    for line in &mut lines {
        line.resize(13, b' ');
    }
    let burrow =
        Grid::from(str::from_utf8(&lines.join(&b'\n')).expect("slice should still be utf8"));

    let hallway = array::from_fn(|position| amphipod(burrow[[1, HALLWAY_SPACES[position].cast()]]));
    let rooms: [Room<2>; NUMBER_OF_ROOMS] = array::from_fn(|room| {
        array::from_fn(|space| {
            amphipod(burrow[[2 + isize::conv(space), 3 + isize::conv(room) * 2]])
        })
    });

    (hallway, rooms)
}

fn amphipod(char: char) -> Option<Amphipod> {
    match char {
        'A' => Some(Amphipod::Amber),
        'B' => Some(Amphipod::Bronze),
        'C' => Some(Amphipod::Copper),
        'D' => Some(Amphipod::Desert),
        '.' => None,
        _ => panic!("amphipod should be '.', 'A', 'B', 'C', or 'D'"),
    }
}

#[cfg(test)]
mod tests {
    use std::iter;

    use ahash::AHashSet;
    use infrastructure::{test, Input, Puzzle};

    use super::*;
    use crate::tests::{input, test_on_input};

    const DAY: usize = 23;

    #[test]
    fn first_answer_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 12521);
    }

    #[test]
    fn first_answer_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 17120);
    }

    #[test]
    fn second_answer_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 44169);
    }

    #[test]
    fn second_answer_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 47234);
    }

    #[test]
    fn successors_move_out_one_room_blocked_left() {
        let (hallway, mut rooms) = super::burrow(&input(DAY, Input::Example(2)));
        let actual: AHashSet<_> = successors_move_out_one_room((hallway, rooms), 3).collect();

        rooms[3][0] = None;
        let successors = [(3, 4000), (4, 2000), (5, 2000), (6, 3000)].map(|(position, energy)| {
            let mut hallway = hallway;
            hallway[position] = Some(Amphipod::Desert);
            ((hallway, rooms), energy)
        });
        let expected = AHashSet::from(successors);
        assert_eq!(actual, expected);
    }

    #[test]
    fn successors_move_out_one_room_blocked_right() {
        let (hallway, mut rooms) = super::burrow(&input(DAY, Input::Example(2)));
        let actual: AHashSet<_> = successors_move_out_one_room((hallway, rooms), 0).collect();

        rooms[0][0] = None;
        let successors = [(0, 30), (1, 20)].map(|(position, energy)| {
            let mut hallway = hallway;
            hallway[position] = Some(Amphipod::Bronze);
            ((hallway, rooms), energy)
        });
        let expected = AHashSet::from(successors);
        assert_eq!(actual, expected);
    }

    #[test]
    fn move_out() {
        let function = |(room_index, room)| super::move_out(room_index, room);
        let cases = [
            (
                (0, [Some(Amphipod::Bronze), Some(Amphipod::Amber)]),
                Some(([None, Some(Amphipod::Amber)], 0, Amphipod::Bronze)),
            ),
            (
                (1, [None, Some(Amphipod::Desert)]),
                Some(([None, None], 1, Amphipod::Desert)),
            ),
            ((2, [None, Some(Amphipod::Copper)]), None),
            ((2, [Some(Amphipod::Copper), Some(Amphipod::Copper)]), None),
            (
                (3, [Some(Amphipod::Desert), Some(Amphipod::Amber)]),
                Some(([None, Some(Amphipod::Amber)], 0, Amphipod::Desert)),
            ),
            ((3, [None, None]), None),
        ];
        test::cases(function, cases);
    }

    #[test]
    fn successors_move_in() {
        let burrow = super::burrow(&input(DAY, Input::Example(3)));
        let actual = super::successors_move_in(burrow);
        let expected_successor = (
            [
                None,
                None,
                None,
                Some(Amphipod::Desert),
                None,
                Some(Amphipod::Amber),
                None,
            ],
            [
                [None, Some(Amphipod::Amber)],
                [Some(Amphipod::Bronze), Some(Amphipod::Bronze)],
                [Some(Amphipod::Copper), Some(Amphipod::Copper)],
                [None, Some(Amphipod::Desert)],
            ],
        );
        let expected_energy = 3000;
        itertools::assert_equal(actual, iter::once((expected_successor, expected_energy)));
    }

    #[test]
    fn move_in_blocked_by_other_amphipod_in_room() {
        let burrow = super::burrow(&input(DAY, Input::Example(2)));
        let actual = successor_move_in(burrow, 2, Amphipod::Bronze);
        let expected = None;
        assert_eq!(actual, expected);
    }

    #[test]
    fn move_in_blocked_by_other_amphipod_in_hallway() {
        let burrow = super::burrow(&input(DAY, Input::Example(3)));
        let actual = successor_move_in(burrow, 5, Amphipod::Amber);
        let expected = None;
        assert_eq!(actual, expected);
    }

    #[test]
    fn move_in_possible() {
        let burrow = super::burrow(&input(DAY, Input::Example(4)));
        let actual = successor_move_in(burrow, 5, Amphipod::Amber);
        let expected_successor = (
            [None; NUMBER_OF_HALLWAY_POSITIONS],
            [
                [Some(Amphipod::Amber), Some(Amphipod::Amber)],
                [Some(Amphipod::Bronze), Some(Amphipod::Bronze)],
                [Some(Amphipod::Copper), Some(Amphipod::Copper)],
                [Some(Amphipod::Desert), Some(Amphipod::Desert)],
            ],
        );
        let expected_energy = 8;
        assert_eq!(actual, Some((expected_successor, expected_energy)));
    }

    #[test]
    fn is_vacant() {
        let function = |(room, amphipod)| super::is_vacant(room, amphipod);
        let cases = [
            (([None, None], Amphipod::Amber), true),
            (([None, Some(Amphipod::Bronze)], Amphipod::Amber), false),
            (([None, Some(Amphipod::Amber)], Amphipod::Amber), true),
            (
                (
                    [Some(Amphipod::Amber), Some(Amphipod::Amber)],
                    Amphipod::Amber,
                ),
                false,
            ),
        ];
        test::cases(function, cases);
    }

    #[test]
    fn is_path_clear() {
        let (hallway, _) = super::burrow(&input(DAY, Input::Example(3)));
        let function = |(amphipod, position)| super::is_path_clear(hallway, amphipod, position);
        let cases = [
            ((3, Amphipod::Desert), false),
            ((4, Amphipod::Desert), true),
            ((5, Amphipod::Amber), false),
        ];
        test::cases(function, cases);
    }

    #[test]
    fn path() {
        let function = |(position, room_index)| super::path(position, room_index);
        let cases = [
            ((5, 0), 2..5),
            ((4, 0), 2..4),
            ((2, 0), 2..2),
            ((3, 3), 4..5),
            ((4, 3), 5..5),
            ((2, 1), 3..3),
            ((2, 3), 3..5),
        ];
        test::cases(function, cases);
    }

    #[test]
    fn number_of_steps() {
        let function = |(position, amphipod, room_space)| {
            super::number_of_steps(position, amphipod, room_space)
        };
        let cases = [
            ((3, 2, 0), 2),
            ((2, 1, 1), 3),
            ((4, 3, 1), 3),
            ((5, 0, 0), 8),
        ];
        test::cases(function, cases);
    }

    #[test]
    fn energy_per_step() {
        let cases = [
            (Amphipod::Amber, 1),
            (Amphipod::Bronze, 10),
            (Amphipod::Copper, 100),
            (Amphipod::Desert, 1000),
        ];
        test::cases(super::energy_per_step, cases);
    }

    #[test]
    fn is_organized() {
        let function =
            |example| super::is_organized(super::burrow(&input(DAY, Input::Example(example))));
        let cases = [(0, false), (1, true)];
        test::cases(function, cases);
    }

    #[test]
    fn unfold() {
        let actual = super::unfold(super::burrow(&input(DAY, Input::Example(0))));
        let expected = (
            [None; 7],
            [
                [
                    Some(Amphipod::Bronze),
                    Some(Amphipod::Desert),
                    Some(Amphipod::Desert),
                    Some(Amphipod::Amber),
                ],
                [
                    Some(Amphipod::Copper),
                    Some(Amphipod::Copper),
                    Some(Amphipod::Bronze),
                    Some(Amphipod::Desert),
                ],
                [
                    Some(Amphipod::Bronze),
                    Some(Amphipod::Bronze),
                    Some(Amphipod::Amber),
                    Some(Amphipod::Copper),
                ],
                [
                    Some(Amphipod::Desert),
                    Some(Amphipod::Amber),
                    Some(Amphipod::Copper),
                    Some(Amphipod::Amber),
                ],
            ],
        );
        assert_eq!(actual, expected);
    }

    #[test]
    fn burrow() {
        let actual = super::burrow(&input(DAY, Input::Example(3)));
        let expected = (
            [
                None,
                None,
                None,
                Some(Amphipod::Desert),
                Some(Amphipod::Desert),
                Some(Amphipod::Amber),
                None,
            ],
            [
                [None, Some(Amphipod::Amber)],
                [Some(Amphipod::Bronze), Some(Amphipod::Bronze)],
                [Some(Amphipod::Copper), Some(Amphipod::Copper)],
                [None, None],
            ],
        );
        assert_eq!(actual, expected);
    }
}
