use easy_cast::Cast;
use itertools::Itertools;

use shared::{
    grid::{self, Coordinate, Grid, Position},
    string::isizes,
    vector::Vector,
};

type Cave = Grid<u8>;
type Path = Vec<Position>;

pub fn first_answer(input: &str) -> String {
    let (mut cave, sand_source) = cave(input);
    let cave_height: isize = cave.height().cast();
    let stop = |[rest_row, _]: Position| rest_row == cave_height - 2;
    produce_sand(&mut cave, sand_source, stop);
    (number_of_units_of_sand(&cave) - 1).to_string()
}

pub fn second_answer(input: &str) -> String {
    let (mut cave, sand_source) = cave(input);
    let stop = |rest_position: Position| rest_position == sand_source;
    produce_sand(&mut cave, sand_source, stop);
    number_of_units_of_sand(&cave).to_string()
}

fn number_of_units_of_sand(cave: &Cave) -> usize {
    cave.iter_row_major()
        .filter(|(_, &element)| element == b'o')
        .count()
}

fn produce_sand(cave: &mut Cave, sand_source: Position, mut stop: impl FnMut(Position) -> bool) {
    loop {
        let rest_position = rest_position(cave, sand_source);
        cave[rest_position] = b'o';
        if stop(rest_position) {
            return;
        }
    }
}

fn rest_position(cave: &Cave, mut sand: Position) -> Position {
    loop {
        let below = sand.add(grid::SOUTH);
        let [below_left, below_right] =
            [grid::WEST, grid::EAST].map(|direction| below.add(direction));
        sand = if cave[below] == b'.' {
            below
        } else if cave[below_left] == b'.' {
            below_left
        } else if cave[below_right] == b'.' {
            below_right
        } else {
            return sand;
        }
    }
}

fn cave(input: &str) -> (Cave, Position) {
    let paths = input.lines().map(path).collect_vec();
    let (height, width, sand_source) = needed_cave_dimensions(&paths);
    let empty_cave_element = |[row, _]: Position| {
        if row == height - 1 {
            b'#'
        } else {
            b'.'
        }
    };
    let mut cave = Grid::new(height.cast(), width.cast(), empty_cave_element);
    for path in paths {
        add_path(&mut cave, &path, sand_source);
    }
    (cave, sand_source)
}

fn needed_cave_dimensions(paths: &[Path]) -> (Coordinate, Coordinate, Position) {
    let lowest = paths
        .iter()
        .flat_map(|path| path.iter())
        .map(|&[row, _]| row)
        .max()
        .expect("paths should contain at least one position");
    let height = lowest + 2 + 1;
    let width = (height - 1) * 2 + 1;
    let sand_source = [0, width / 2];
    (height, width, sand_source)
}

fn add_path(cave: &mut Cave, path: &[Position], [_, sand_source_column]: Position) {
    let shift = [0, 500 - sand_source_column];
    let mut path = path.iter().map(|position| position.sub(shift));

    let mut position = path.next().expect("path should be non-empty");
    if let Some(element) = cave.get_mut(position) {
        *element = b'#';
    }
    for joint in path {
        let direction = joint.sub(position).unit();
        while position != joint {
            position = position.add(direction);
            if let Some(element) = cave.get_mut(position) {
                *element = b'#';
            }
        }
    }
}

fn path(line: &str) -> Vec<Position> {
    line.split(" -> ")
        .map(isizes)
        .map(|coordinates| [coordinates[1], coordinates[0]])
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 14;

    #[test]
    fn first_answer_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 24);
    }

    #[test]
    fn first_answer_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 817);
    }

    #[test]
    fn second_answer_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 93);
    }

    #[test]
    fn second_answer_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 23416);
    }
}
