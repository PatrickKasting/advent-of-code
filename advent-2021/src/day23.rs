use std::fmt::{Display, Write};

use shared::search;

type HomeIndex = usize;
type HallwayPosition = usize;
type Energy = usize;

pub fn first(input: &str) -> String {
    let initial = parse_input(input);
    search(initial).to_string()
}

pub fn second(_input: &str) -> String {
    unimplemented!()
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Amphipod {
    Amber = 0,
    Bronze = 1,
    Copper = 2,
    Desert = 3,
}

impl From<Amphipod> for char {
    fn from(value: Amphipod) -> Self {
        match value {
            Amphipod::Amber => 'A',
            Amphipod::Bronze => 'B',
            Amphipod::Copper => 'C',
            Amphipod::Desert => 'D',
        }
    }
}

impl Amphipod {
    fn home_index(self) -> HomeIndex {
        self as usize
    }

    fn energy_per_step(self) -> Energy {
        10usize.pow(self as u32)
    }

    fn amphipod_in(home_index: HomeIndex) -> Self {
        match home_index {
            0 => Self::Amber,
            1 => Self::Bronze,
            2 => Self::Copper,
            3 => Self::Desert,
            _ => panic!("home index should be no greater than three"),
        }
    }
}

impl Display for Amphipod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char((*self).into())
    }
}

const NUM_HALLWAY_POSITIONS: usize = 11;
const HALLWAY_STOP_POSITIONS: [usize; 7] = [0, 1, 3, 5, 7, 9, 10];
const REVERSE_HALLWAY_STOP_POSITIONS: [usize; 7] = [10, 9, 7, 5, 3, 1, 0];
const NUM_HOMES: usize = 4;
const HOME_SIZE: usize = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct State {
    hallway: [Option<Amphipod>; NUM_HALLWAY_POSITIONS],
    homes: [[Option<Amphipod>; HOME_SIZE]; NUM_HOMES],
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn optional_amphipod_to_char(amphipod: Option<Amphipod>) -> char {
            amphipod.map(Into::into).unwrap_or('.')
        }
        let home = |home_index: HomeIndex, level: usize| {
            optional_amphipod_to_char(self.homes[home_index][level])
        };
        let home_level = |level: usize| {
            format!(
                "#{}#{}#{}#{}#",
                home(0, level),
                home(1, level),
                home(2, level),
                home(3, level)
            )
        };
        let hallway = String::from_iter(self.hallway.map(optional_amphipod_to_char));

        writeln!(f, "#############")?;
        writeln!(f, "#{}#", hallway)?;
        writeln!(f, "##{}##", home_level(0))?;
        for level in 1..HOME_SIZE {
            writeln!(f, "..{}..", home_level(level))?;
        }
        writeln!(f, "..#########..")
    }
}

fn parse_optional_amphipod(char: char) -> Option<Amphipod> {
    match char {
        'A' => Some(Amphipod::Amber),
        'B' => Some(Amphipod::Bronze),
        'C' => Some(Amphipod::Copper),
        'D' => Some(Amphipod::Desert),
        '.' => None,
        _ => panic!("burrow positions should be represented by 'A', 'B', 'C', 'D', or '.'"),
    }
}

fn parse_home_line(
    home_line: &str,
    homes: &mut [[Option<Amphipod>; HOME_SIZE]; NUM_HOMES],
    level: usize,
) {
    for (index, home) in homes.iter_mut().enumerate() {
        let amphipod = home_line
            .chars()
            .nth(2 * index + 3)
            .expect("burrow should contain four homes");
        home[level] = parse_optional_amphipod(amphipod);
    }
}

fn parse_input(input: &str) -> State {
    let mut lines = input.lines();
    lines.next();

    let hallway = lines
        .next()
        .expect("every burrow should have a hallway")
        .chars()
        .skip(1)
        .take(NUM_HALLWAY_POSITIONS)
        .map(parse_optional_amphipod)
        .collect::<Vec<Option<Amphipod>>>()
        .try_into()
        .expect("hallway should have the correct length");

    let mut homes = [[None; HOME_SIZE]; NUM_HOMES];
    for (level, home_line) in lines.take(HOME_SIZE).enumerate() {
        parse_home_line(home_line, &mut homes, level);
    }

    State { hallway, homes }
}

fn home_entrance(home_index: HomeIndex) -> HallwayPosition {
    2 * (home_index + 1)
}

impl State {
    fn go_home(self, from: HallwayPosition) -> Option<(Self, Energy)> {
        let amphipod =
            self.hallway[from].expect("the given hallway position should contain an amphipod");
        let home_index = amphipod.home_index();
        let first_non_roommate_position = self.homes[home_index]
            .iter()
            .position(|amphipod| {
                !amphipod.is_some_and(|resident| resident.home_index() == home_index)
            })
            .expect(
                "room should not be full of correct amphipods when correct amphipod is in hallway",
            );
        if self.homes[home_index][first_non_roommate_position].is_some() {
            return None;
        }

        let to = home_entrance(home_index);
        let hallway_range = if from < to {
            from + 1..=to
        } else {
            to..=from - 1
        };
        let hallway_empty = self.hallway[hallway_range.clone()]
            .iter()
            .all(Option::is_none);
        if !hallway_empty {
            return None;
        }

        let mut successor = self;
        successor.hallway[from] = None;
        successor.homes[home_index][first_non_roommate_position] = Some(amphipod);

        let num_steps = hallway_range.count() + HOME_SIZE - first_non_roommate_position;
        Some((successor, num_steps * amphipod.energy_per_step()))
    }

    fn go_out(self, home_index: HomeIndex) -> Vec<(Self, Energy)> {
        let Some((home_position, &Some(amphipod))) = self.homes[home_index]
            .iter()
            .enumerate()
            .rev()
            .find(|(_, amphipod)| amphipod.is_some())
        else {
            return Vec::new();
        };

        let home_entrance = home_entrance(home_index);
        let split_index = HALLWAY_STOP_POSITIONS
            .iter()
            .position(|&stop_position| home_entrance < stop_position)
            .expect("every home entrance should be between two stop positions");
        let reverse_split_index = HALLWAY_STOP_POSITIONS.len() - split_index;
        let reverse_left = &REVERSE_HALLWAY_STOP_POSITIONS[reverse_split_index..];
        let right = &HALLWAY_STOP_POSITIONS[split_index..];

        let mut successors = Vec::new();
        let mut hallway_walk = |stop_positions: &[HallwayPosition]| {
            for &stop_position in stop_positions {
                if self.hallway[stop_position].is_some() {
                    break;
                }

                let mut successor = self;
                successor.homes[home_index][home_position] = None;
                successor.hallway[stop_position] = Some(amphipod);

                let num_steps = stop_position.abs_diff(home_entrance) + HOME_SIZE - home_position;
                successors.push((successor, num_steps * amphipod.energy_per_step()))
            }
        };
        hallway_walk(reverse_left);
        hallway_walk(right);
        successors
    }

    fn successors(self) -> Vec<(Self, Energy)> {
        let mut successors: Vec<(Self, Energy)> = (0..NUM_HOMES)
            .flat_map(|home_index| self.go_out(home_index))
            .collect();
        successors.extend(self.hallway.iter().enumerate().filter_map(
            |(hallway_index, amphipod)| {
                amphipod
                    .is_some()
                    .then(|| self.go_home(hallway_index))
                    .flatten()
            },
        ));
        successors
    }

    fn is_home_organized(self, home_index: usize) -> bool {
        let supposed_resident = Some(Amphipod::amphipod_in(home_index));
        self.homes[home_index]
            .iter()
            .all(|&resident| resident == supposed_resident)
    }

    fn is_burrow_organized(self) -> bool {
        (0..NUM_HOMES).all(|home_index| self.is_home_organized(home_index))
    }
}

fn search(initial: State) -> Energy {
    search::cheapest_path_cost(initial, State::successors, State::is_burrow_organized)
        .expect("a sequence of moves organizing the burrow should exist")
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use infrastructure::{Input, Puzzle};

    use super::*;
    use crate::tests::{input, test_on_input};

    const DAY: usize = 23;

    #[test]
    fn first_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 12521);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 17120);
    }

    #[test]
    fn second_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 44169);
    }

    // #[test]
    // fn second_input() {
    //     test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 150004);
    // }

    #[test]
    fn parse() {
        let mut input = input(DAY, Input::Example(0));
        input.retain(|char| char != '\r');
        let reconstructed = parse_input(&input).to_string();
        assert_eq!(reconstructed, input);
    }

    #[test]
    fn entrance() {
        let actual: Vec<HallwayPosition> = (0..NUM_HOMES).map(home_entrance).collect();
        let expected = vec![2, 4, 6, 8];
        assert_eq!(actual, expected);
    }

    #[test]
    fn homes() {
        use Amphipod::*;
        let actual = [Amber, Bronze, Copper, Desert].map(Amphipod::home_index);
        let expected = [0, 1, 2, 3];
        assert_eq!(actual, expected);
    }

    #[test]
    fn energies() {
        use Amphipod::*;
        let actual = [Amber, Bronze, Copper, Desert].map(Amphipod::energy_per_step);
        let expected = [1, 10, 100, 1000];
        assert_eq!(actual, expected);
    }

    #[test]
    fn reverse_hallway_stop_positions() {
        let mut hallway_stop_positions = HALLWAY_STOP_POSITIONS;
        hallway_stop_positions.reverse();
        assert_eq!(hallway_stop_positions, REVERSE_HALLWAY_STOP_POSITIONS);
    }

    fn assert_can_go_home(
        state: &str,
        from: HallwayPosition,
        expected_successor: &str,
        expected_energy: Energy,
    ) {
        let (successor, energy) = parse_input(state)
            .go_home(from)
            .expect("amber amphipod should be able to move to its home");
        assert_eq!(successor.to_string(), expected_successor);
        assert_eq!(energy, expected_energy);
    }

    #[test]
    fn can_go_home_to_empty() {
        let state = "\
            #############\n\
            #A........A.#\n\
            ###.#B#C#D###\n\
            ..#.#B#C#D#..\n\
            ..#########..\n\
        ";
        let expected_successor = "\
            #############\n\
            #.........A.#\n\
            ###.#B#C#D###\n\
            ..#A#B#C#D#..\n\
            ..#########..\n\
        ";
        assert_can_go_home(state, 0, expected_successor, 4);
    }

    #[test]
    fn can_go_home_to_roommate() {
        let state = "\
            #############\n\
            #.........A.#\n\
            ###.#B#C#D###\n\
            ..#A#B#C#D#..\n\
            ..#########..\n\
        ";
        let expected_successor = "\
            #############\n\
            #...........#\n\
            ###A#B#C#D###\n\
            ..#A#B#C#D#..\n\
            ..#########..\n\
        ";
        assert_can_go_home(state, 9, expected_successor, 8);
    }

    fn assert_cannot_go_home(state: &str, from: HallwayPosition) {
        assert!(parse_input(state).go_home(from).is_none());
    }

    #[test]
    fn cannot_go_home_because_blocked() {
        let state = "\
            #############\n\
            #...B...A...#\n\
            ###.#.#C#D###\n\
            ..#A#B#C#D#..\n\
            ..#########..\n\
        ";
        assert_cannot_go_home(state, 7);
    }

    #[test]
    fn cannot_go_home_because_bad_roommate() {
        let state = "\
            #############\n\
            #...B.......#\n\
            ###B#.#C#D###\n\
            ..#A#A#C#D#..\n\
            ..#########..\n\
        ";
        assert_cannot_go_home(state, 3);
    }

    fn assert_successors<const N: usize>(
        successors: impl FnOnce(State) -> Vec<(State, Energy)>,
        state: &str,
        expected_successors: [&str; N],
        expected_energies: [Energy; N],
    ) {
        let actual: BTreeSet<(String, Energy)> = successors(parse_input(state))
            .into_iter()
            .map(|(successor, energy)| (successor.to_string(), energy))
            .collect();
        let expected = BTreeSet::from_iter(
            expected_successors
                .into_iter()
                .map(ToString::to_string)
                .zip(expected_energies),
        );
        assert_eq!(actual, expected);
    }

    #[test]
    fn can_go_out_blocked_left() {
        let state = "\
            #############\n\
            #...B...A...#\n\
            ###.#.#C#D###\n\
            ..#A#B#C#D#..\n\
            ..#########..\n\
        ";
        let expected_successors = [
            "\
                #############\n\
                #...B...A.D.#\n\
                ###.#.#C#.###\n\
                ..#A#B#C#D#..\n\
                ..#########..\n\
            ",
            "\
                #############\n\
                #...B...A..D#\n\
                ###.#.#C#.###\n\
                ..#A#B#C#D#..\n\
                ..#########..\n\
            ",
        ];
        let expected_energies = [2000, 3000];
        assert_successors(
            |state| state.go_out(3),
            state,
            expected_successors,
            expected_energies,
        );
    }

    #[test]
    fn can_go_out_blocked_right() {
        let state = "\
            #############\n\
            #.........A.#\n\
            ###.#B#C#D###\n\
            ..#A#B#C#D#..\n\
            ..#########..\n\
        ";
        let expected_successors = [
            "\
                #############\n\
                #C........A.#\n\
                ###.#B#.#D###\n\
                ..#A#B#C#D#..\n\
                ..#########..\n\
            ",
            "\
                #############\n\
                #.C.......A.#\n\
                ###.#B#.#D###\n\
                ..#A#B#C#D#..\n\
                ..#########..\n\
            ",
            "\
                #############\n\
                #...C.....A.#\n\
                ###.#B#.#D###\n\
                ..#A#B#C#D#..\n\
                ..#########..\n\
            ",
            "\
                #############\n\
                #.....C...A.#\n\
                ###.#B#.#D###\n\
                ..#A#B#C#D#..\n\
                ..#########..\n\
            ",
            "\
                #############\n\
                #.......C.A.#\n\
                ###.#B#.#D###\n\
                ..#A#B#C#D#..\n\
                ..#########..\n\
            ",
        ];
        let expected_energies: [Energy; 5] = [700, 600, 400, 200, 200];
        assert_successors(
            |state| state.go_out(2),
            state,
            expected_successors,
            expected_energies,
        );
    }

    #[test]
    fn successors() {
        let state = "\
            #############\n\
            #...B...A...#\n\
            ###.#.#C#D###\n\
            ..#A#B#C#D#..\n\
            ..#########..\n\
        ";
        let expected_successors = [
            "\
                #############\n\
                #A..B...A...#\n\
                ###.#.#C#D###\n\
                ..#.#B#C#D#..\n\
                ..#########..\n\
            ",
            "\
                #############\n\
                #.A.B...A...#\n\
                ###.#.#C#D###\n\
                ..#.#B#C#D#..\n\
                ..#########..\n\
            ",
            "\
                #############\n\
                #.......A...#\n\
                ###.#B#C#D###\n\
                ..#A#B#C#D#..\n\
                ..#########..\n\
            ",
            "\
                #############\n\
                #...B.B.A...#\n\
                ###.#.#C#D###\n\
                ..#A#.#C#D#..\n\
                ..#########..\n\
            ",
            "\
                #############\n\
                #...B.C.A...#\n\
                ###.#.#.#D###\n\
                ..#A#B#C#D#..\n\
                ..#########..\n\
            ",
            "\
                #############\n\
                #...B...A.D.#\n\
                ###.#.#C#.###\n\
                ..#A#B#C#D#..\n\
                ..#########..\n\
            ",
            "\
                #############\n\
                #...B...A..D#\n\
                ###.#.#C#.###\n\
                ..#A#B#C#D#..\n\
                ..#########..\n\
            ",
        ];
        let expected_energies = [4, 3, 20, 30, 200, 2000, 3000];
        assert_successors(
            |state| state.successors(),
            state,
            expected_successors,
            expected_energies,
        );
    }

    #[test]
    fn homes_organized() {
        let state = "\
            #############\n\
            #...B.......#\n\
            ###B#.#C#D###\n\
            ..#A#A#C#D#..\n\
            ..#########..\n\
        ";
        let state = parse_input(state);
        let actual = (0..NUM_HOMES).map(|home_index| state.is_home_organized(home_index));
        let expected = [false, false, true, true].into_iter();
        assert!(actual.eq(expected));
    }

    #[test]
    fn burrow_organized() {
        let states = [
            "\
                #############\n\
                #.........A.#\n\
                ###.#B#C#D###\n\
                ..#A#B#C#D#..\n\
                ..#########..\n\
            ",
            "\
                #############\n\
                #...........#\n\
                ###A#B#C#D###\n\
                ..#A#B#C#D#..\n\
                ..#########..\n\
            ",
        ];
        let actual = states.map(parse_input).map(State::is_burrow_organized);
        let expected = [false, true];
        assert_eq!(actual, expected);
    }
}
