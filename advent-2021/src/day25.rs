use shared::grid::{self, Direction, Grid, Position};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum SeaCucumber {
    None,
    East,
    South,
}

type Seafloor = Grid<SeaCucumber>;
type Moved = bool;

pub fn first(input: &str) -> String {
    let seafloor = parse_input(input);
    movement(seafloor).to_string()
}

pub fn second(_input: &str) -> String {
    todo!()
}

fn parse_sea_cucumber(char: char) -> SeaCucumber {
    match char {
        '.' => SeaCucumber::None,
        '>' => SeaCucumber::East,
        'v' => SeaCucumber::South,
        _ => panic!("locations should be only '.', '>', or 'v'"),
    }
}

fn parse_input(input: &str) -> Seafloor {
    let width = input
        .lines()
        .next()
        .expect("input should not be empty")
        .chars()
        .count();
    let elements = input
        .chars()
        .filter(|char| !char.is_ascii_whitespace())
        .map(parse_sea_cucumber)
        .collect();
    Grid::from_elements(elements, width)
}

fn move_one_direction(
    seafloor: Seafloor,
    kind: SeaCucumber,
    direction: Direction,
) -> (Seafloor, Moved) {
    let mut next = seafloor.clone();
    let mut moved = false;
    for (position, &sea_cucumber) in seafloor.iter_row_major() {
        if sea_cucumber != kind {
            continue;
        }
        let neighbor = wrapping_neighbor(&seafloor, position, direction);
        if seafloor[neighbor] == SeaCucumber::None {
            next[position] = SeaCucumber::None;
            next[neighbor] = kind;
            moved |= true;
        }
    }
    (next, moved)
}

fn wrapping_neighbor(seafloor: &Seafloor, position: Position, direction: Direction) -> Position {
    let row = (position[0] + direction[0]).rem_euclid(seafloor.height() as isize);
    let column = (position[1] + direction[1]).rem_euclid(seafloor.width() as isize);
    [row, column]
}

fn move_both_directions(seafloor: Seafloor) -> (Seafloor, Moved) {
    let (after_east, moved_east) = move_one_direction(seafloor, SeaCucumber::East, grid::EAST);
    let (after_south, moved_south) =
        move_one_direction(after_east, SeaCucumber::South, grid::SOUTH);
    (after_south, moved_east | moved_south)
}

fn movement(mut seafloor: Seafloor) -> usize {
    let mut moved;
    for step in 0.. {
        (seafloor, moved) = move_both_directions(seafloor);
        if !moved {
            return step + 1;
        }
    }
    panic!("sea cucumber movement should stop at one point");
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use super::*;
    use crate::tests::{input, test_on_input};

    const DAY: usize = 25;

    impl SeaCucumber {
        fn to_char(self) -> char {
            match self {
                SeaCucumber::None => '.',
                SeaCucumber::East => '>',
                SeaCucumber::South => 'v',
            }
        }
    }

    #[test]
    fn first_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 58);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 560);
    }

    fn assert_seafloor(actual: Seafloor, expected: &str) {
        let actual = actual.map(|_, cucumber| cucumber.to_char()).to_string();
        assert_eq!(actual, expected);
    }

    #[test]
    fn parse() {
        let actual = parse_input(&input(DAY, Input::Example(0)));
        let expected = "\
            v...>>.vv>\n\
            .vv>>.vv..\n\
            >>.>v>...v\n\
            >>v>>.>.v.\n\
            v>v.vv.v..\n\
            >.>>..v...\n\
            .vv..>.>v.\n\
            v.v..>>v.v\n\
            ....v..v.>\n\
        ";
        assert_seafloor(actual, expected);
    }

    fn assert_example_seafloor_after_steps(num_steps: usize, expected: &str) {
        let mut seafloor = parse_input(&input(DAY, Input::Example(0)));
        for _ in 0..num_steps {
            (seafloor, _) = move_both_directions(seafloor);
        }
        assert_seafloor(seafloor, expected);
    }

    #[test]
    fn one_step() {
        let expected = "\
            ....>.>v.>\n\
            v.v>.>v.v.\n\
            >v>>..>v..\n\
            >>v>v>.>.v\n\
            .>v.v...v.\n\
            v>>.>vvv..\n\
            ..v...>>..\n\
            vv...>>vv.\n\
            >.v.v..v.v\n\
        ";
        assert_example_seafloor_after_steps(1, expected);
    }

    #[test]
    fn ten_steps() {
        let expected = "\
            ..>..>>vv.\n\
            v.....>>.v\n\
            ..v.v>>>v>\n\
            v>.>v.>>>.\n\
            ..v>v.vv.v\n\
            .v.>>>.v..\n\
            v.v..>v>..\n\
            ..v...>v.>\n\
            .vv..v>vv.\n\
        ";
        assert_example_seafloor_after_steps(10, expected);
    }

    #[test]
    fn fifty_eight_steps() {
        let expected = "\
            ..>>v>vv..\n\
            ..v.>>vv..\n\
            ..>>v>>vv.\n\
            ..>>>>>vv.\n\
            v......>vv\n\
            v>v....>>v\n\
            vvv.....>>\n\
            >vv......>\n\
            .>v.vv.v..\n\
        ";
        assert_example_seafloor_after_steps(58, expected);
    }
}
