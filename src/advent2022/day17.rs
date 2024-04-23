use std::{cmp, iter::Peekable};

use easy_cast::{Cast, Conv};

type Chamber = Vec<[bool; CHAMBER_WIDTH_INCLUDING_WALLS]>;
type Position = [Coordinate; 2];
type Coordinate = usize;
type Rock = &'static [&'static [bool]];
type Jet = char;

const ROCKS: [Rock; 5] = [
    &[&[true, true, true, true]],
    &[
        &[false, true, false],
        &[true, true, true],
        &[false, true, false],
    ],
    &[
        &[false, false, true],
        &[false, false, true],
        &[true, true, true],
    ],
    &[&[true], &[true], &[true], &[true]],
    &[&[true, true], &[true, true]],
];

const CHAMBER_WIDTH: Coordinate = 7;
const CHAMBER_WIDTH_INCLUDING_WALLS: Coordinate = 1 + CHAMBER_WIDTH + 1;
const APPEARING_ROCK_LEFT_MARGIN: Coordinate = 2;
const APPEARING_ROCK_BOTTOM_MARGIN: Coordinate = 3;

type Surface = Vec<Direction>;
type Direction = [isize; 2];

pub fn first(input: &str) -> String {
    tower_height(input.trim(), 2022).to_string()
}

pub fn second(input: &str) -> String {
    tower_height(input.trim(), 1_000_000_000_000_usize).to_string()
}

fn tower_height(jets: &str, mut number_of_rocks: usize) -> Coordinate {
    let mut jets = jets.chars().enumerate().cycle().peekable();
    let mut chamber = empty_chamber();
    let mut tower_height = 0;
    let mut past_states_and_tower_heights =
        vec![(state(&mut jets, &chamber, tower_height), tower_height)];
    for rock in ROCKS.into_iter().cycle() {
        if number_of_rocks == 0 {
            return tower_height;
        }

        let rock_top = drop_rock(&mut jets, &mut chamber, tower_height, rock);
        tower_height = cmp::max(tower_height, rock_top);
        number_of_rocks -= 1;

        let state = state(&mut jets, &chamber, tower_height);
        if let Some((cycle_length, tower_height_cycle_start)) =
            cycle_length_and_tower_height_cycle_start(&past_states_and_tower_heights, &state)
        {
            let remaining_cycles = number_of_rocks / cycle_length;
            let tower_height_complete_cycles =
                (tower_height - tower_height_cycle_start) * remaining_cycles;
            number_of_rocks %= cycle_length;
            let incomplete_cycle_end_index =
                past_states_and_tower_heights.len() - cycle_length + number_of_rocks;
            let tower_height_incomplete_cycle =
                past_states_and_tower_heights[incomplete_cycle_end_index].1
                    - tower_height_cycle_start;
            return tower_height + tower_height_complete_cycles + tower_height_incomplete_cycle;
        }
        past_states_and_tower_heights.push((state, tower_height));
    }
    unreachable!("rocks should cycle indefinitely");
}

fn drop_rock(
    jets: &mut impl Iterator<Item = (usize, Jet)>,
    chamber: &mut Chamber,
    tower_height: Coordinate,
    rock: Rock,
) -> Coordinate {
    let initial_position @ [top, _] = initial_position(tower_height, rock);
    ensure_needed_height(chamber, top);
    let landing_position @ [top, _] = landing_position(jets, &*chamber, rock, initial_position);
    stop_rock(chamber, rock, landing_position);
    top
}

fn initial_position(tower_height: Coordinate, rock: Rock) -> Position {
    let row = tower_height + APPEARING_ROCK_BOTTOM_MARGIN + rock.len();
    let column = 1 + APPEARING_ROCK_LEFT_MARGIN;
    [row, column]
}

fn landing_position(
    jets: &mut impl Iterator<Item = (usize, Jet)>,
    chamber: &Chamber,
    rock: Rock,
    [mut top, mut left]: Position,
) -> Position {
    for (_, jet) in jets {
        let next_left = match jet {
            '<' => left - 1,
            '>' => left + 1,
            _ => panic!("jet should be '<' or '>'"),
        };
        if !collision(chamber, rock, [top, next_left]) {
            left = next_left;
        }

        let next_top = top - 1;
        if collision(chamber, rock, [next_top, left]) {
            return [top, left];
        }
        top = next_top;
    }
    panic!("jets should repeat indefinitely");
}

fn collision(chamber: &Chamber, rock: Rock, [row, column]: Position) -> bool {
    for (row_index, &rock_row) in rock.iter().enumerate() {
        for (column_index, &is_rock) in rock_row.iter().enumerate() {
            if is_rock && chamber[row - row_index][column + column_index] {
                return true;
            }
        }
    }
    false
}

fn stop_rock(chamber: &mut Chamber, rock: Rock, [row, column]: Position) {
    for (row_index, &rock_row) in rock.iter().enumerate() {
        for (column_index, &is_rock) in rock_row.iter().enumerate() {
            if is_rock {
                let position = &mut chamber[row - row_index][column + column_index];
                debug_assert!(!*position, "position should not already contain rock");
                *position = true;
            }
        }
    }
}

type State = (Coordinate, Surface);

fn state(
    jets: &mut Peekable<impl Iterator<Item = (usize, Jet)>>,
    chamber: &Chamber,
    tower_height: Coordinate,
) -> State {
    let &(jet_index, _) = jets.peek().expect("jets should repeat indefinitely");
    let surface = surface(chamber, tower_height);
    (jet_index, surface)
}

fn cycle_length_and_tower_height_cycle_start(
    past_states_and_tower_heights: &[(State, Coordinate)],
    (jet_index, surface): &State,
) -> Option<(usize, Coordinate)> {
    let mut index = past_states_and_tower_heights.len();
    while ROCKS.len() <= index {
        index -= ROCKS.len();
        let ((past_jet_index, past_surface), past_tower_height) =
            &past_states_and_tower_heights[index];
        if jet_index == past_jet_index && surface == past_surface {
            return Some((
                past_states_and_tower_heights.len() - index,
                *past_tower_height,
            ));
        }
    }
    None
}

fn surface(chamber: &Chamber, tower_height: Coordinate) -> Surface {
    let left = |[row, column]: Direction| [column, -row];
    let right = |[row, column]: Direction| [-column, row];
    let backward = |[row, column]: Direction| [-row, -column];
    let add = |left: [isize; 2], right: [isize; 2]| [left[0] + right[0], left[1] + right[1]];
    let convert = |[row, column]: [isize; 2]| [usize::conv(row), column.cast()];

    let mut row = tower_height;
    while !chamber[row][1] {
        row -= 1;
    }

    let mut position = [row.cast(), 1];
    let mut direction = [0, 1];
    let mut surface = vec![];
    loop {
        let next_directions = [
            left(direction),
            direction,
            right(direction),
            backward(direction),
        ];
        for next_direction in next_directions {
            let next_position = add(position, next_direction);
            let [row, column] = convert(next_position);
            if chamber[row][column] {
                if column == CHAMBER_WIDTH_INCLUDING_WALLS - 1 {
                    return surface;
                }
                direction = next_direction;
                position = next_position;
                surface.push(direction);
                break;
            }
        }
    }
}

fn empty_chamber() -> Chamber {
    let mut chamber = vec![[true; CHAMBER_WIDTH_INCLUDING_WALLS]];
    add_empty_row(&mut chamber);
    chamber
}

fn ensure_needed_height(chamber: &mut Chamber, rock_top: Coordinate) {
    let needed_height = 1 + rock_top;
    for _ in chamber.len()..needed_height {
        add_empty_row(chamber);
    }
}

fn add_empty_row(chamber: &mut Chamber) {
    chamber.push([true, false, false, false, false, false, false, false, true]);
}

#[cfg(test)]
mod tests {
    use super::{
        super::tests::{test_on_input, YEAR},
        *,
    };
    use crate::{input, Input, Puzzle};

    const DAY: usize = 17;

    #[test]
    fn first_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 3068);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 3081);
    }

    #[test]
    fn second_example() {
        test_on_input(
            DAY,
            Puzzle::Second,
            Input::Example(0),
            1_514_285_714_288_usize,
        );
    }

    #[test]
    fn second_input() {
        test_on_input(
            DAY,
            Puzzle::Second,
            Input::PuzzleInput,
            1_524_637_681_145_usize,
        );
    }

    #[test]
    fn some_cycle() {
        let flat = vec![RIGHT, RIGHT, RIGHT, RIGHT, RIGHT, RIGHT];
        let bumpy = vec![RIGHT, RIGHT, UP, RIGHT, RIGHT, DOWN, RIGHT, RIGHT];
        let past_states = [
            ((0, flat.clone()), 0),
            ((1, flat.clone()), 1),
            ((0, bumpy.clone()), 2),
            ((1, bumpy.clone()), 3),
            ((0, flat.clone()), 4),
            ((1, flat.clone()), 5),
            ((0, flat.clone()), 6),
            ((1, flat.clone()), 7),
            ((0, flat.clone()), 8),
            ((1, bumpy.clone()), 9),
        ];
        let state = (0, flat);
        let actual = cycle_length_and_tower_height_cycle_start(&past_states, &state);
        let expected = Some((10, 0));
        assert_eq!(actual, expected);
    }

    #[test]
    fn no_cycle() {
        let flat = vec![RIGHT, RIGHT, RIGHT, RIGHT, RIGHT, RIGHT];
        let bumpy = vec![RIGHT, RIGHT, UP, RIGHT, RIGHT, DOWN, RIGHT, RIGHT];
        let past_states = [
            ((0, flat.clone()), 0),
            ((1, flat.clone()), 1),
            ((2, bumpy.clone()), 2),
            ((0, bumpy.clone()), 3),
            ((1, flat.clone()), 4),
            ((2, bumpy.clone()), 5),
            ((0, bumpy.clone()), 6),
            ((1, flat.clone()), 7),
            ((2, flat.clone()), 8),
            ((0, bumpy.clone()), 9),
        ];
        let state = (0, bumpy);
        let actual = cycle_length_and_tower_height_cycle_start(&past_states, &state);
        let expected = None;
        assert_eq!(actual, expected);
    }

    #[test]
    fn surface_ten_rocks_in_chamber() {
        let expected = vec![
            RIGHT, DOWN, RIGHT, RIGHT, RIGHT, UP, UP, UP, UP, DOWN, DOWN, RIGHT, DOWN, DOWN, LEFT,
            LEFT, DOWN, LEFT, DOWN, DOWN, RIGHT, RIGHT, DOWN, RIGHT, DOWN, LEFT, DOWN, DOWN, DOWN,
            DOWN, DOWN, LEFT, DOWN, DOWN, RIGHT, RIGHT, DOWN, RIGHT,
        ];
        assert_surface_equals(Input::Example(0), 10, &expected);
    }

    #[test]
    fn surface_two_rocks_in_chamber() {
        let expected = vec![
            RIGHT, RIGHT, UP, RIGHT, UP, UP, LEFT, RIGHT, UP, DOWN, RIGHT, LEFT, DOWN, DOWN, RIGHT,
            RIGHT, DOWN, RIGHT,
        ];
        assert_surface_equals(Input::Example(0), 2, &expected);
    }

    #[test]
    fn surface_empty_chamber() {
        let expected = vec![RIGHT; 6];
        assert_surface_equals(Input::Example(0), 0, &expected);
    }

    const RIGHT: Direction = [0, 1];
    const UP: Direction = [1, 0];
    const LEFT: Direction = [0, -1];
    const DOWN: Direction = [-1, 0];

    fn assert_surface_equals(input: Input, number_of_rocks: usize, expected: &Surface) {
        let (chamber, tower_height) = chamber(input, number_of_rocks);
        let actual = surface(&chamber, tower_height);
        assert_eq!(
            &actual, expected,
            "surface after {number_of_rocks} rock(s) should equal expected"
        );
    }

    fn chamber(input: Input, number_of_rocks: usize) -> (Chamber, usize) {
        let input = self::input(YEAR, DAY, input);
        let mut jets = input.trim().chars().enumerate().cycle();
        let mut chamber = empty_chamber();
        let mut tower_height = 0;
        for rock in ROCKS.into_iter().cycle().take(number_of_rocks) {
            let rock_top = drop_rock(&mut jets, &mut chamber, tower_height, rock);
            tower_height = cmp::max(tower_height, rock_top);
        }
        (chamber, tower_height)
    }
}
