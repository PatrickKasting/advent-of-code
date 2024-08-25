use std::collections::BTreeSet;

use shared::grid::Position;

type Transparent = BTreeSet<Position>;

const COORDINATE_NOT_A_NUMBER: &str = "coordinates should be positive integers";
const AT_LEAST_ONE_DOT: &str = "the transparent should contain at least one dot.";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Fold {
    Horizontal(isize),
    Vertical(isize),
}

pub fn first(input: &str) -> String {
    let (transparent, folds) = parse_input(input);
    let [height, width] = transparent_size(&transparent);
    let (transparent, _, _) = fold_transparent(transparent, folds[0], height, width);
    transparent.len().to_string()
}

pub fn second(input: &str) -> String {
    let (mut transparent, folds) = parse_input(input);
    let [mut height, mut width] = transparent_size(&transparent);
    for fold in folds {
        (transparent, height, width) = fold_transparent(transparent, fold, height, width);
    }
    display_transparent(&transparent, height, width)
}

fn parse_dot(coordinates: &str) -> Position {
    let (col, row) = coordinates
        .split_once(',')
        .expect("dot coordinates should be split by a comma");
    [
        row.parse().expect(COORDINATE_NOT_A_NUMBER),
        col.parse().expect(COORDINATE_NOT_A_NUMBER),
    ]
}

fn parse_fold(fold: &str) -> Fold {
    let equals_sign = fold
        .chars()
        .position(|char| char == '=')
        .expect("every fold should contain an equals sign");
    let coordinate: isize = fold[equals_sign + 1..]
        .parse()
        .expect(COORDINATE_NOT_A_NUMBER);
    match fold
        .chars()
        .nth(equals_sign - 1)
        .expect("equals sign should not be the first character of a fold")
    {
        'x' => Fold::Vertical(coordinate),
        'y' => Fold::Horizontal(coordinate),
        _ => panic!("folds should be along 'x' or 'y'"),
    }
}

fn parse_input(input: &str) -> (Transparent, Vec<Fold>) {
    let mut lines = input.lines();
    let mut transparent = BTreeSet::new();
    loop {
        let line = lines
            .next()
            .expect("input should contain an empty line separating dots from folds");
        if line.is_empty() {
            break;
        }
        transparent.insert(parse_dot(line));
    }
    (transparent, lines.map(parse_fold).collect())
}

fn transparent_size(transparent: &Transparent) -> [isize; 2] {
    let size = |index| {
        transparent
            .iter()
            .map(|dot| dot[index])
            .max()
            .expect(AT_LEAST_ONE_DOT)
    };
    [0, 1].map(size).map(|size| size + 1)
}

fn mirror(mirror: isize, number: isize) -> isize {
    2 * mirror - number
}

fn fold_horizontal(fold_row: isize, [row, column]: Position) -> Position {
    let maybe_mirrored_dot_row = if fold_row < row {
        mirror(fold_row, row)
    } else {
        row
    };
    [maybe_mirrored_dot_row, column]
}

fn fold_vertical(fold_col: isize, [row, column]: Position) -> Position {
    let maybe_mirrored_dot_col = if fold_col < column {
        mirror(fold_col, column)
    } else {
        column
    };
    [row, maybe_mirrored_dot_col]
}

fn fold_transparent(
    transparent: Transparent,
    fold: Fold,
    mut height: isize,
    mut width: isize,
) -> (Transparent, isize, isize) {
    let folder: Box<dyn Fn(Position) -> Position> = match fold {
        Fold::Horizontal(fold_row) => {
            height /= 2;
            Box::new(move |dot| fold_horizontal(fold_row, dot))
        }
        Fold::Vertical(fold_col) => {
            width /= 2;
            Box::new(move |dot| fold_vertical(fold_col, dot))
        }
    };
    let transparent = transparent.into_iter().map(folder).collect();
    (transparent, height, width)
}

pub fn display_transparent(transparent: &Transparent, height: isize, width: isize) -> String {
    let mut display = String::new();
    for row in 0..height {
        for col in 0..width {
            let char = if transparent.contains(&[row, col]) {
                '#'
            } else {
                '.'
            };
            display.push(char);
        }
        display.push('\n');
    }
    display
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 13;

    #[test]
    fn first_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 17);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 755);
    }

    #[test]
    fn second_example() {
        let answer = "\
            #####\n\
            #...#\n\
            #...#\n\
            #...#\n\
            #####\n\
            .....\n\
            .....\n\
        ";
        test_on_input(DAY, Puzzle::Second, Input::Example(0), answer);
    }

    #[test]
    fn second_input() {
        let answer = "\
            ###..#....#..#...##.###..###...##...##..\n\
            #..#.#....#.#.....#.#..#.#..#.#..#.#..#.\n\
            ###..#....##......#.#..#.###..#..#.#....\n\
            #..#.#....#.#.....#.###..#..#.####.#.##.\n\
            #..#.#....#.#..#..#.#.#..#..#.#..#.#..#.\n\
            ###..####.#..#..##..#..#.###..#..#..###.\n\
        ";
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, answer);
    }
}
