use std::{
    cmp::{max, min},
    collections::HashMap,
};

use shared::{
    grid::{self, Position},
    string::isizes,
    vector::Vector,
};

type Grid = HashMap<Position, usize>;
type Line = (Position, Position);

pub fn first(input: &str) -> String {
    number_of_dangerous_areas(true, input).to_string()
}

pub fn second(input: &str) -> String {
    number_of_dangerous_areas(false, input).to_string()
}

fn number_of_dangerous_areas(ignore_diagonals: bool, input: &str) -> usize {
    let mut grid: Grid = HashMap::new();
    for line in parse_input(input) {
        add_line(ignore_diagonals, &mut grid, line)
    }
    grid.values().filter(|&&count| count >= 2).count()
}

fn parse_input(input: &str) -> Vec<Line> {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> Line {
    let coordinates: Vec<isize> = isizes(line);
    debug_assert!(
        coordinates.len() == 4,
        "number of coordinates on a single line should be four"
    );
    (
        [coordinates[0], coordinates[1]],
        [coordinates[2], coordinates[3]],
    )
}

fn add_line(ignore_diagonal: bool, grid: &mut Grid, (start, end): Line) {
    let (mut start, end) = (min(start, end), max(start, end));
    let direction = if start[0] == end[0] {
        grid::EAST
    } else if start[1] == end[1] {
        grid::SOUTH
    } else if !ignore_diagonal && start[1] < end[1] {
        grid::SOUTH_EAST
    } else if !ignore_diagonal {
        grid::SOUTH_WEST
    } else {
        return;
    };
    loop {
        *grid.entry(start).or_default() += 1;
        if start == end {
            break;
        }
        start = start.add(direction);
    }
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 5;

    #[test]
    fn first_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 5);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 8622);
    }

    #[test]
    fn second_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 12);
    }

    #[test]
    fn second_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 22037);
    }
}
