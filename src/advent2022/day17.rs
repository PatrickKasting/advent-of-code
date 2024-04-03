use std::cmp;

type Chamber = Vec<bool>;
type Position = [usize; 2];
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

const CHAMBER_WIDTH: usize = 7;
const CHAMBER_WIDTH_INCLUDING_WALLS: usize = 1 + CHAMBER_WIDTH + 1;
const APPEARING_ROCK_LEFT_MARGIN: usize = 2;
const APPEARING_ROCK_BOTTOM_MARGIN: usize = 3;

pub fn first(input: &str) -> String {
    simulation(input, 2022).to_string()
}

pub fn second(_input: &str) -> String {
    todo!();
}

fn simulation(jets: &str, number_of_rocks: usize) -> usize {
    let mut jets = jets.trim().chars().cycle();
    let mut chamber = empty_chamber();
    let mut tower_height = 0;
    for rock in ROCKS.into_iter().cycle().take(number_of_rocks) {
        let initial_position @ [top, _] = initial_position(tower_height, rock);
        ensure_needed_height(&mut chamber, top);
        let rock_height = drop_rock(&mut jets, &mut chamber, rock, initial_position);
        tower_height = cmp::max(tower_height, rock_height);
    }
    tower_height
}

fn initial_position(tower_height: usize, rock: Rock) -> Position {
    let row = tower_height + APPEARING_ROCK_BOTTOM_MARGIN + rock.len();
    let column = 1 + APPEARING_ROCK_LEFT_MARGIN;
    [row, column]
}

fn drop_rock(
    jets: &mut impl Iterator<Item = Jet>,
    chamber: &mut Chamber,
    rock: Rock,
    [mut top, mut left]: Position,
) -> usize {
    for jet in jets {
        let next_left = match jet {
            '<' => left - 1,
            '>' => left + 1,
            _ => panic!("jet should be '<' or '>'"),
        };
        if !collision(&*chamber, rock, [top, next_left]) {
            left = next_left;
        }

        let next_top = top - 1;
        if collision(&*chamber, rock, [next_top, left]) {
            freeze_rock(chamber, rock, [top, left]);
            return top;
        }
        top = next_top;
    }
    panic!("jets should repeat indefinitely");
}

fn collision(chamber: &Chamber, rock: Rock, [row, column]: Position) -> bool {
    for (row_index, &rock_row) in rock.iter().enumerate() {
        for (column_index, &is_rock) in rock_row.iter().enumerate() {
            if is_rock && chamber[index([row - row_index, column + column_index])] {
                return true;
            }
        }
    }
    false
}

fn index([row, column]: Position) -> usize {
    row * CHAMBER_WIDTH_INCLUDING_WALLS + column
}

fn freeze_rock(chamber: &mut Chamber, rock: Rock, [row, column]: Position) {
    for (row_index, &rock_row) in rock.iter().enumerate() {
        for (column_index, &is_rock) in rock_row.iter().enumerate() {
            if is_rock {
                let position = &mut chamber[index([row - row_index, column + column_index])];
                debug_assert!(!*position, "position, should not already contain rock");
                *position = true;
            }
        }
    }
}

fn empty_chamber() -> Chamber {
    vec![true; CHAMBER_WIDTH_INCLUDING_WALLS]
}

fn ensure_needed_height(chamber: &mut Chamber, rock_top: usize) {
    debug_assert!(
        chamber.len() % CHAMBER_WIDTH_INCLUDING_WALLS == 0,
        "chamber should consist of entire rows only"
    );
    let current_height = chamber.len() / CHAMBER_WIDTH_INCLUDING_WALLS;
    let needed_height = 1 + rock_top;
    for _ in current_height..needed_height {
        chamber.extend([true, false, false, false, false, false, false, false, true]);
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::test_on_input;
    use crate::{Input, Puzzle};

    const DAY: usize = 17;

    #[test]
    fn first_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 3068);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 3081);
    }

    // #[test]
    // fn second_example() {
    //     test_on_input(DAY, Puzzle::Second, Input::Example(0), 1514285714288);
    // }

    // #[test]
    // fn second_input() {
    //     test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 404_395);
    // }
}
