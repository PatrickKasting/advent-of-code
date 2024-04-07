use std::{char, str};

use itertools::Itertools;

use crate::data_structures::grid::{Direction, Grid, Position};

type Board = Grid<char>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Instruction {
    Left,
    Right,
    Forward(usize),
}

pub fn first(input: &str) -> String {
    let (board, path) = board_and_path(input);
    let (final_position, final_direction) = final_position_and_direction(&board, path);
    final_password(final_position, final_direction).to_string()
}

pub fn second(_input: &str) -> String {
    todo!()
}

fn final_password(final_position: Position, final_direction: Direction) -> usize {
    let [row, column] = [Position::row, Position::column].map(|coordinate| {
        let coordinate_as_usize: usize = coordinate(final_position)
            .try_into()
            .expect("coordinate should not be nagative");
        coordinate_as_usize + 1
    });
    let direction_as_value = match final_direction {
        Direction::North => 3,
        Direction::East => 0,
        Direction::South => 1,
        Direction::West => 2,
    };
    1000 * row + 4 * column + direction_as_value
}

fn final_position_and_direction(
    board: &Board,
    path: impl Iterator<Item = Instruction>,
) -> (Position, Direction) {
    let (mut position, _) = board
        .iter_row_major()
        .find(|&(_, &tile)| tile == '.')
        .expect("at least one open tile should be in the first row");
    let mut direction = Direction::East;
    for instruction in path {
        match instruction {
            Instruction::Left => direction = direction.left(),
            Instruction::Right => direction = direction.right(),
            Instruction::Forward(number_of_tiles) => {
                position = forward(board, position, direction, number_of_tiles);
            }
        }
    }
    (position, direction)
}

fn forward(
    board: &Board,
    mut position: Position,
    direction: Direction,
    number_of_tiles: usize,
) -> Position {
    for _ in 0..number_of_tiles {
        let potential_next_position = position.neighbor(direction);
        position = match board.get(potential_next_position) {
            Some('.') => potential_next_position,
            Some('#') => return position,
            Some(' ') | None => {
                let Some(warp_position) = warp(board, position, direction) else {
                    return position;
                };
                warp_position
            }
            _ => panic!("tile on the board should be '.', '#', or ' '"),
        }
    }
    position
}

fn warp(board: &Grid<char>, position: Position, direction: Direction) -> Option<Position> {
    let [row, column] = [position.row(), position.column()];
    let mut warp_position = match direction {
        Direction::North => Position::new(board.height() - 1, column),
        Direction::East => Position::new(row, 0),
        Direction::South => Position::new(0, column),
        Direction::West => Position::new(row, board.width() - 1),
    };
    while board[warp_position] == ' ' {
        warp_position = warp_position.neighbor(direction);
    }
    if board[warp_position] == '#' {
        None
    } else {
        Some(warp_position)
    }
}

fn board_and_path(input: &str) -> (Board, impl Iterator<Item = Instruction> + '_) {
    let (board, path) = input
        .trim_end()
        .split_once("\n\n")
        .expect("board and path should be separated by an empty line");
    (self::board(board), self::path(path))
}

fn board(str: &str) -> Board {
    let lines = str.lines().collect_vec();
    let board_width = lines
        .iter()
        .map(|line| line.len())
        .max()
        .expect("board should have at least one row");
    let mut tiles = String::with_capacity(lines.len() * board_width);
    for line in lines {
        let number_of_missing_empty_tiles = board_width - line.len();
        tiles.push_str(line);
        tiles.extend(itertools::repeat_n(' ', number_of_missing_empty_tiles));
        tiles.push('\n');
    }
    Board::from(&tiles)
}

fn path(line: &str) -> impl Iterator<Item = Instruction> + '_ {
    line.as_bytes()
        .chunk_by(|left, right| left.is_ascii_digit() == right.is_ascii_digit())
        .map(|instruction| match instruction[0] {
            b'L' => Instruction::Left,
            b'R' => Instruction::Right,
            digit if digit.is_ascii_digit() => {
                let number_of_steps = str::from_utf8(instruction)
                    .expect("slice should still be UTF-8")
                    .parse()
                    .expect("slice should contain only digits");
                Instruction::Forward(number_of_steps)
            }
            _ => panic!("path should consist of only 'L', 'R' and digits"),
        })
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::{super::tests::test_on_input, Instruction};
    use crate::{Input, Puzzle};

    const DAY: usize = 22;

    #[test]
    fn first_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 6032);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 164_014);
    }

    // #[test]
    // fn second_example() {
    //     test_on_input(DAY, Puzzle::Second, Input::Example(0), 5031);
    // }

    // #[test]
    // fn second_input() {
    //     test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 404_395);
    // }

    #[test]
    fn path() {
        let path = "10R5L5R10L4R5L5";
        let actual = super::path(path).collect_vec();
        let expected = vec![
            Instruction::Forward(10),
            Instruction::Right,
            Instruction::Forward(5),
            Instruction::Left,
            Instruction::Forward(5),
            Instruction::Right,
            Instruction::Forward(10),
            Instruction::Left,
            Instruction::Forward(4),
            Instruction::Right,
            Instruction::Forward(5),
            Instruction::Left,
            Instruction::Forward(5),
        ];
        assert_eq!(actual, expected);
    }
}
