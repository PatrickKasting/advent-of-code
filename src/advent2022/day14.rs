use itertools::Itertools;

use crate::{
    data_structures::grid::{Direction, Grid, Position},
    strings::isizes,
};

type Cave = Grid<u8>;
type Path = Vec<Position>;

pub fn first(input: &str) -> String {
    let (mut cave, sand_source) = cave(input);
    let cave_height: isize = (cave.height())
        .try_into()
        .expect("cave height should be less than 'isize::MAX'");
    let stop = |rest_position: Position| rest_position.row() == cave_height - 2;
    produce_sand(&mut cave, sand_source, stop);
    (number_of_units_of_sand(&cave) - 1).to_string()
}

pub fn second(input: &str) -> String {
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
        let below = sand.neighbor(Direction::South);
        let [below_left, below_right] =
            [Direction::West, Direction::East].map(|direction| below.neighbor(direction));
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
    let empty_cave_element = |position: Position| {
        let height: isize = height
            .try_into()
            .expect("cave height should be less than 'isize::MAX'");
        if position.row() == height - 1 {
            b'#'
        } else {
            b'.'
        }
    };
    let mut cave = Grid::new(height, width, empty_cave_element);
    for path in paths {
        add_path(&mut cave, &path, sand_source);
    }
    (cave, sand_source)
}

fn needed_cave_dimensions(paths: &[Path]) -> (usize, usize, Position) {
    let lowest: usize = paths
        .iter()
        .flat_map(|path| path.iter())
        .map(|position| position.row())
        .max()
        .expect("paths should contain at least one position")
        .try_into()
        .expect("lowest rock should be less than 'isize::MAX'");
    let height = lowest + 2 + 1;
    let width = (height - 1) * 2 + 1;
    let sand_source = Position::new(0, width / 2);
    (height, width, sand_source)
}

fn add_path(cave: &mut Cave, path: &[Position], sand_source: Position) {
    let column_shift = 500 - sand_source.column();
    let mut path = path
        .iter()
        .map(|position| Position::new(position.row(), position.column() - column_shift));

    let mut position = path.next().expect("path should be non-empty");
    if let Some(element) = cave.get_mut(position) {
        *element = b'#';
    }
    for joint in path {
        let direction = Direction::try_from([position, joint])
            .expect("direction to next path joint should be cardinal");
        while position != joint {
            position = position.neighbor(direction);
            if let Some(element) = cave.get_mut(position) {
                *element = b'#';
            }
        }
    }
}

fn path(line: &str) -> Vec<Position> {
    line.split(" -> ")
        .map(isizes)
        .map(|coordinates| Position::new(coordinates[1], coordinates[0]))
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use super::super::tests::test_on_input;
    use crate::{Input, Puzzle};

    const DAY: usize = 14;

    #[test]
    fn first_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 24);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 817);
    }

    #[test]
    fn second_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 93);
    }

    #[test]
    fn second_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 23416);
    }
}
