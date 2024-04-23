use std::{char, str};

use easy_cast::{Cast, Conv};
use itertools::Itertools;

use crate::data_structures::grid::{Coordinate, Direction, Grid, Position};

type Board = Grid<char>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Instruction {
    Left,
    Right,
    Forward(usize),
}

pub fn first(input: &str) -> String {
    let (board, path) = board_and_path(input);
    let (final_position, final_direction) = final_position_and_direction(wrap_plane, &board, path);
    final_password(final_position, final_direction).to_string()
}

pub fn second(input: &str) -> String {
    let (board, path) = board_and_path(input);
    let (final_position, final_direction) = final_position_and_direction(wrap_cube, &board, path);
    final_password(final_position, final_direction).to_string()
}

fn final_password(final_position: Position, final_direction: Direction) -> isize {
    let [row, column] =
        [Position::row, Position::column].map(|coordinate| coordinate(final_position) + 1);
    let direction_as_value = match final_direction {
        Direction::North => 3,
        Direction::East => 0,
        Direction::South => 1,
        Direction::West => 2,
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
    let mut direction = Direction::East;
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
        let neighbor = position.neighbor(direction);
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
    position: Position,
    direction: Direction,
) -> (Position, Direction) {
    let [row, column] = [position.row(), position.column()];
    let mut wrap_position = match direction {
        Direction::North => Position::new(Coordinate::conv(board.height()) - 1, column),
        Direction::East => Position::new(row, 0),
        Direction::South => Position::new(0, column),
        Direction::West => Position::new(row, Coordinate::conv(board.width()) - 1),
    };
    while board[wrap_position] == ' ' {
        wrap_position = wrap_position.neighbor(direction);
    }
    (wrap_position, direction)
}

type Vector = [Coordinate; 3];

fn wrap_cube(
    board: &Grid<char>,
    position: Position,
    direction: Direction,
) -> (Position, Direction) {
    let face_size = face_size(board);

    let mut plane_face = Position::new(position.row() / face_size, position.column() / face_size);
    let mut plane_direction = direction.right();
    let mut cube_face = [0, 0, -1];
    let mut cube_direction = unit_vector(plane_direction);
    while !(cube_face == unit_vector(direction)
        && cross_product(cube_direction, cube_face) == [0, 0, -1])
    {
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
    cube_face: Vector,
    cube_direction: Vector,
) -> (Position, Direction, Vector, Vector) {
    let plane_face_forward = plane_face.neighbor(plane_direction);
    let plane_face_forward_left = plane_face_forward.neighbor(plane_direction.left());
    if face_exists(board, face_size, plane_face_forward_left) {
        (
            plane_face_forward_left,
            plane_direction.left(),
            cross_product(cube_direction, cube_face),
            negation(cube_direction),
        )
    } else if face_exists(board, face_size, plane_face_forward) {
        (
            plane_face_forward,
            plane_direction,
            cube_direction,
            negation(cube_face),
        )
    } else {
        (
            plane_face,
            plane_direction.right(),
            cube_face,
            cross_product(cube_face, cube_direction),
        )
    }
}

fn cube_wrap_position(
    face_size: Coordinate,
    source_position: Position,
    source_direction: Direction,
    destination_face: Position,
    destination_direction: Direction,
) -> Position {
    let source_distance_to_face_border = match source_direction {
        Direction::North => face_size - source_position.column() % face_size - 1,
        Direction::East => face_size - source_position.row() % face_size - 1,
        Direction::South => source_position.column() % face_size,
        Direction::West => source_position.row() % face_size,
    };

    let top_left_position_of_face = [
        destination_face.row() * face_size,
        destination_face.column() * face_size,
    ];
    let wrap_position = match destination_direction {
        Direction::North => [
            top_left_position_of_face[0] + face_size - 1,
            top_left_position_of_face[1] + face_size - 1 - source_distance_to_face_border,
        ],
        Direction::East => [
            top_left_position_of_face[0] + face_size - 1 - source_distance_to_face_border,
            top_left_position_of_face[1],
        ],
        Direction::South => [
            top_left_position_of_face[0],
            top_left_position_of_face[1] + source_distance_to_face_border,
        ],
        Direction::West => [
            top_left_position_of_face[0] + source_distance_to_face_border,
            top_left_position_of_face[1] + face_size - 1,
        ],
    };
    Position::new(wrap_position[0], wrap_position[1])
}

fn face_size(board: &Board) -> Coordinate {
    let mut board_sides: [Coordinate; 2] = [board.height().cast(), board.width().cast()];
    board_sides.sort_unstable();
    if board_sides[0] * 5 == board_sides[1] * 2 {
        board_sides[0] / 2
    } else {
        board_sides[0] / 3
    }
}

fn face_exists(board: &Board, face_size: Coordinate, face: Position) -> bool {
    let top_left_position_of_face =
        Position::new(face.row() * face_size, face.column() * face_size);
    match board.get(top_left_position_of_face) {
        Some(' ') | None => false,
        Some('.' | '#') => true,
        _ => panic!("tile on the board should be '.', '#', or ' '"),
    }
}

fn unit_vector(direction: Direction) -> Vector {
    match direction {
        Direction::North => [0, 1, 0],
        Direction::East => [1, 0, 0],
        Direction::South => [0, -1, 0],
        Direction::West => [-1, 0, 0],
    }
}

fn negation(direction: Vector) -> Vector {
    direction.map(|coordinate| -coordinate)
}

fn cross_product([l1, l2, l3]: Vector, [r1, r2, r3]: Vector) -> Vector {
    [l2 * r3 - l3 * r2, l3 * r1 - l1 * r3, l1 * r2 - l2 * r1]
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

    use super::{
        super::tests::{test_on_input, YEAR},
        *,
    };
    use crate::{advent2022::day22::board_and_path, input, tests::test_cases, Input, Puzzle};

    const DAY: usize = 22;

    #[test]
    fn first_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 6032);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 164_014);
    }

    #[test]
    fn second_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 5031);
    }

    #[test]
    fn second_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 47525);
    }

    #[test]
    fn wrap_cube() {
        let board = board(Input::Example(0));
        let function = |(position, direction)| super::wrap_cube(&board, position, direction);
        let cases = [
            ((A, Direction::East), (B, Direction::South)),
            ((B, Direction::North), (A, Direction::West)),
            ((C, Direction::South), (D, Direction::North)),
            ((D, Direction::South), (C, Direction::North)),
            ((E, Direction::North), (F, Direction::East)),
            ((F, Direction::West), (E, Direction::South)),
        ];
        test_cases(function, cases);
    }

    #[test]
    fn cube_wrap_position() {
        let face_size = super::face_size(&board(Input::Example(0)));
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
            (
                (A, Direction::East, Position::new(2, 3), Direction::South),
                B,
            ),
            (
                (B, Direction::North, Position::new(1, 2), Direction::West),
                A,
            ),
            (
                (C, Direction::South, Position::new(1, 0), Direction::North),
                D,
            ),
            (
                (D, Direction::South, Position::new(2, 2), Direction::North),
                C,
            ),
            (
                (E, Direction::North, Position::new(0, 2), Direction::East),
                F,
            ),
            (
                (F, Direction::West, Position::new(1, 1), Direction::South),
                E,
            ),
        ];
        test_cases(function, cases);
    }

    const A: Position = Position::new(5, 11);
    const B: Position = Position::new(8, 14);
    const C: Position = Position::new(11, 10);
    const D: Position = Position::new(7, 1);
    const E: Position = Position::new(4, 6);
    const F: Position = Position::new(2, 8);

    #[test]
    fn face_size() {
        let function = |input| super::face_size(&board(input));
        let cases = [(Input::Example(0), 4), (Input::Example(1), 1)];
        test_cases(function, cases);
    }

    #[test]
    fn cross_product() {
        let [left, right] = [[3, -3, 1], [4, 9, 2]];
        let actual = super::cross_product(left, right);
        let expected = [-15, -2, 39];
        assert_eq!(actual, expected);
    }

    fn board(input: Input) -> Board {
        let input = crate::input(YEAR, DAY, input);
        let (board, _) = board_and_path(&input);
        board
    }

    #[test]
    fn path() {
        let input = input(YEAR, DAY, Input::Example(0));
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
