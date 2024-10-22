use std::{char, str};

use easy_cast::{Cast, Conv};
use itertools::Itertools;

use shared::{
    grid::{self, Coordinate, Direction, Grid, Position},
    vector::{CrossProduct, Negation, RotationInTwoDimensions, Vector},
};

type Board = Grid<char>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Instruction {
    Left,
    Right,
    Forward(usize),
}

pub fn first_answer(input: &str) -> String {
    let (board, path) = board_and_path(input);
    let (final_position, final_direction) = final_position_and_direction(wrap_plane, &board, path);
    final_password(final_position, final_direction).to_string()
}

pub fn second_answer(input: &str) -> String {
    let (board, path) = board_and_path(input);
    let (final_position, final_direction) = final_position_and_direction(wrap_cube, &board, path);
    final_password(final_position, final_direction).to_string()
}

fn final_password(final_position: Position, final_direction: Direction) -> isize {
    let [row, column] = final_position.map(|coordinate| coordinate + 1);
    let direction_as_value = match final_direction {
        grid::NORTH => 3,
        grid::EAST => 0,
        grid::SOUTH => 1,
        grid::WEST => 2,
        _ => panic!("direction should be one of four unit vectors"),
    };
    1000 * row + 4 * column + direction_as_value
}

fn final_position_and_direction(
    wrap: Wrap,
    board: &Board,
    path: impl Iterator<Item = Instruction>,
) -> (Position, Direction) {
    let (mut position, _) = board
        .iter_row_major()
        .find(|&(_, &tile)| tile == '.')
        .expect("at least one open tile should be in the first row");
    let mut direction = grid::EAST;
    for instruction in path {
        match instruction {
            Instruction::Left => direction = direction.left(),
            Instruction::Right => direction = direction.right(),
            Instruction::Forward(number_of_tiles) => {
                (position, direction) = forward(wrap, board, position, direction, number_of_tiles);
            }
        }
    }
    (position, direction)
}

type Wrap = fn(&Board, Position, Direction) -> (Position, Direction);

fn forward(
    wrap: Wrap,
    board: &Board,
    mut position: Position,
    mut direction: Direction,
    number_of_tiles: usize,
) -> (Position, Direction) {
    for _ in 0..number_of_tiles {
        let neighbor = position.add(direction);
        (position, direction) = match board.get(neighbor) {
            Some('.') => (neighbor, direction),
            Some('#') => return (position, direction),
            Some(' ') | None => {
                let (wrap_position, wrap_direction) = wrap(board, position, direction);
                if board[wrap_position] == '#' {
                    return (position, direction);
                }
                (wrap_position, wrap_direction)
            }
            _ => panic!("tile on the board should be '.', '#', or ' '"),
        }
    }
    (position, direction)
}

fn wrap_plane(
    board: &Grid<char>,
    [row, column]: Position,
    direction: Direction,
) -> (Position, Direction) {
    let mut wrap_position = match direction {
        grid::NORTH => [Coordinate::conv(board.height()) - 1, column],
        grid::EAST => [row, 0],
        grid::SOUTH => [0, column],
        grid::WEST => [row, Coordinate::conv(board.width()) - 1],
        _ => panic!("direction should be one of four unit vectors"),
    };
    while board[wrap_position] == ' ' {
        wrap_position = wrap_position.add(direction);
    }
    (wrap_position, direction)
}

fn wrap_cube(
    board: &Grid<char>,
    position: Position,
    direction: Direction,
) -> (Position, Direction) {
    let face_size = face_size(board).cast();

    let mut plane_face = position.map(|coordinate| coordinate / face_size);
    let mut plane_direction = direction.right();
    let mut cube_face = [0, 0, -1];
    let mut cube_direction = [plane_direction[0], plane_direction[1], 0];
    while !(cube_face[0..2] == direction[0..2] && cube_direction.cross(cube_face) == [0, 0, -1]) {
        (plane_face, plane_direction, cube_face, cube_direction) = next_face(
            board,
            face_size,
            plane_face,
            plane_direction,
            cube_face,
            cube_direction,
        );
    }

    let wrap_direction = plane_direction.right();
    let wrap_position =
        cube_wrap_position(face_size, position, direction, plane_face, wrap_direction);
    (wrap_position, wrap_direction)
}

fn next_face(
    board: &Board,
    face_size: Coordinate,
    plane_face: Position,
    plane_direction: Direction,
    cube_face: [Coordinate; 3],
    cube_direction: [Coordinate; 3],
) -> (Position, Direction, [Coordinate; 3], [Coordinate; 3]) {
    let plane_face_forward = plane_face.add(plane_direction);
    let plane_face_forward_left = plane_face_forward.add(plane_direction.left());
    if face_exists(board, face_size, plane_face_forward_left) {
        (
            plane_face_forward_left,
            plane_direction.left(),
            cube_direction.cross(cube_face),
            cube_direction.neg(),
        )
    } else if face_exists(board, face_size, plane_face_forward) {
        (
            plane_face_forward,
            plane_direction,
            cube_direction,
            cube_face.neg(),
        )
    } else {
        (
            plane_face,
            plane_direction.right(),
            cube_face,
            cube_face.cross(cube_direction),
        )
    }
}

fn cube_wrap_position(
    face_size: Coordinate,
    [source_row, source_column]: Position,
    source_direction: Direction,
    destination_face: Position,
    destination_direction: Direction,
) -> Position {
    let source_distance_to_face_border = match source_direction {
        grid::NORTH => face_size - source_column % face_size - 1,
        grid::EAST => face_size - source_row % face_size - 1,
        grid::SOUTH => source_column % face_size,
        grid::WEST => source_row % face_size,
        _ => panic!("direction should be one of four unit vectors"),
    };

    let [top_of_face, left_of_face] = destination_face.mul(face_size);
    match destination_direction {
        grid::NORTH => [
            top_of_face + face_size - 1,
            left_of_face + face_size - 1 - source_distance_to_face_border,
        ],
        grid::EAST => [
            top_of_face + face_size - 1 - source_distance_to_face_border,
            left_of_face,
        ],
        grid::SOUTH => [top_of_face, left_of_face + source_distance_to_face_border],
        grid::WEST => [
            top_of_face + source_distance_to_face_border,
            left_of_face + face_size - 1,
        ],
        _ => panic!("direction should be one of four unit vectors"),
    }
}

fn face_size(board: &Board) -> usize {
    let mut board_sides = [board.height(), board.width()];
    board_sides.sort_unstable();
    if board_sides[0] * 5 == board_sides[1] * 2 {
        board_sides[0] / 2
    } else {
        board_sides[0] / 3
    }
}

fn face_exists(board: &Board, face_size: Coordinate, face_position: Position) -> bool {
    let top_left_of_face = face_position.mul(face_size);
    match board.get(top_left_of_face) {
        Some(' ') | None => false,
        Some('.' | '#') => true,
        _ => panic!("tile on the board should be '.', '#', or ' '"),
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
                    .expect("slice should still be utf8")
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

    use infrastructure::{test, Input, Puzzle};

    use super::*;
    use crate::tests::{input, test_on_input};

    const DAY: usize = 22;

    #[test]
    fn first_answer_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 6032);
    }

    #[test]
    fn first_answer_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 164_014);
    }

    #[test]
    fn second_answer_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 5031);
    }

    #[test]
    fn second_answer_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 47525);
    }

    #[test]
    fn wrap_cube() {
        let board = board(Input::Example(0));
        let function = |(position, direction)| super::wrap_cube(&board, position, direction);
        let cases = [
            ((A, grid::EAST), (B, grid::SOUTH)),
            ((B, grid::NORTH), (A, grid::WEST)),
            ((C, grid::SOUTH), (D, grid::NORTH)),
            ((D, grid::SOUTH), (C, grid::NORTH)),
            ((E, grid::NORTH), (F, grid::EAST)),
            ((F, grid::WEST), (E, grid::SOUTH)),
        ];
        test::cases(function, cases);
    }

    #[test]
    fn cube_wrap_position() {
        let face_size = super::face_size(&board(Input::Example(0))).cast();
        let function =
            |(source_position, source_direction, destination_face, destination_direction)| {
                super::cube_wrap_position(
                    face_size,
                    source_position,
                    source_direction,
                    destination_face,
                    destination_direction,
                )
            };
        let cases = [
            ((A, grid::EAST, [2, 3], grid::SOUTH), B),
            ((B, grid::NORTH, [1, 2], grid::WEST), A),
            ((C, grid::SOUTH, [1, 0], grid::NORTH), D),
            ((D, grid::SOUTH, [2, 2], grid::NORTH), C),
            ((E, grid::NORTH, [0, 2], grid::EAST), F),
            ((F, grid::WEST, [1, 1], grid::SOUTH), E),
        ];
        test::cases(function, cases);
    }

    const A: Position = [5, 11];
    const B: Position = [8, 14];
    const C: Position = [11, 10];
    const D: Position = [7, 1];
    const E: Position = [4, 6];
    const F: Position = [2, 8];

    #[test]
    fn face_size() {
        let function = |input| super::face_size(&board(input));
        let cases = [(Input::Example(0), 4), (Input::Example(1), 1)];
        test::cases(function, cases);
    }

    fn board(input: Input) -> Board {
        let input = self::input(DAY, input);
        let (board, _) = board_and_path(&input);
        board
    }

    #[test]
    fn path() {
        let input = self::input(DAY, Input::Example(0));
        let (_, actual) = board_and_path(&input);
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
        assert_eq!(actual.collect_vec(), expected);
    }
}
