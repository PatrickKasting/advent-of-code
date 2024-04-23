use std::{
    convert::identity,
    fmt::{Debug, Display, Write},
    ops::{Index, IndexMut},
};

use itertools::Itertools;
use strum::{EnumIter, IntoEnumIterator};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, EnumIter)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn right(self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }

    pub fn backward(self) -> Self {
        self.right().right()
    }

    pub fn left(self) -> Self {
        self.backward().right()
    }

    pub fn relative_direction_to(self, other: Self) -> RelativeDirection {
        if other == self {
            RelativeDirection::Forward
        } else if other == self.right() {
            RelativeDirection::Right
        } else if other == self.backward() {
            RelativeDirection::Backward
        } else {
            RelativeDirection::Left
        }
    }

    pub fn reflection_north_west_diagonal(self) -> Self {
        match self {
            Self::North => Self::West,
            Self::East => Self::South,
            Self::South => Self::East,
            Self::West => Self::North,
        }
    }

    pub fn reflection_north_east_diagonal(self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::North,
            Self::South => Self::West,
            Self::West => Self::South,
        }
    }

    pub fn from_up_down_left_or_right(char: char) -> Self {
        match char {
            'U' => Self::North,
            'D' => Self::South,
            'L' => Self::West,
            'R' => Self::East,
            _ => panic!("direction should be 'U', 'D', 'L', or 'R'"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, EnumIter)]
pub enum RelativeDirection {
    Forward,
    Left,
    Backward,
    Right,
}

pub type Coordinate = isize;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position {
    row: Coordinate,
    column: Coordinate,
}

impl Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.column)
    }
}

impl Position {
    pub const fn new(row: Coordinate, column: Coordinate) -> Self {
        Position { row, column }
    }

    pub const fn row(self) -> Coordinate {
        self.row
    }

    pub const fn column(self) -> Coordinate {
        self.column
    }

    pub const fn neighbor(mut self, direction: Direction) -> Position {
        match direction {
            Direction::North => self.row -= 1,
            Direction::East => self.column += 1,
            Direction::South => self.row += 1,
            Direction::West => self.column -= 1,
        }
        self
    }

    pub fn neighbors(self) -> impl Iterator<Item = Self> {
        Direction::iter().map(move |direction| self.neighbor(direction))
    }

    pub fn direction_to(self, other: Position) -> Option<Direction> {
        if self == other {
            return None;
        }

        if self.row() == other.row() {
            if self.column() < other.column() {
                Some(Direction::East)
            } else {
                Some(Direction::West)
            }
        } else if self.column() == other.column() {
            if self.row() < other.row() {
                Some(Direction::South)
            } else {
                Some(Direction::North)
            }
        } else {
            None
        }
    }

    pub const fn manhattan_distance(self, other: Position) -> Coordinate {
        (self.row - other.row).abs() + (self.column - other.column).abs()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Grid<T> {
    elements: Vec<T>,
    width: usize,
}

impl<T> Grid<T> {
    pub fn new(height: usize, width: usize, mut element: impl FnMut(Position) -> T) -> Self {
        let mut elements = vec![];
        for row in 0..height {
            for column in 0..width {
                #[allow(clippy::cast_possible_wrap)]
                elements.push(element(Position::new(
                    row as Coordinate,
                    column as Coordinate,
                )));
            }
        }
        Self { elements, width }
    }

    pub fn get(&self, position: Position) -> Option<&T> {
        self.is_within_grid(position)
            .then(|| &self.elements[self.index_of_position(position)])
    }

    pub fn get_mut(&mut self, position: Position) -> Option<&mut T> {
        self.is_within_grid(position).then(|| {
            let index = self.index_of_position(position);
            &mut self.elements[index]
        })
    }

    #[allow(clippy::cast_possible_wrap)]
    pub fn iter_row_major(&self) -> impl Iterator<Item = (Position, &T)> {
        self.rows().enumerate().flat_map(|(row_index, row)| {
            row.enumerate().map(move |(column_index, element)| {
                (
                    Position::new(row_index as Coordinate, column_index as Coordinate),
                    element,
                )
            })
        })
    }

    #[allow(clippy::cast_possible_wrap)]
    pub fn iter_column_major(&self) -> impl Iterator<Item = (Position, &T)> {
        self.columns()
            .enumerate()
            .flat_map(|(column_index, column)| {
                column.enumerate().map(move |(row_index, element)| {
                    (
                        Position::new(row_index as Coordinate, column_index as Coordinate),
                        element,
                    )
                })
            })
    }

    pub fn rows(
        &self,
    ) -> impl ExactSizeIterator<Item = impl Iterator<Item = &T>> + DoubleEndedIterator {
        self.elements.chunks(self.width()).map(|row| row.iter())
    }

    pub fn columns(
        &self,
    ) -> impl ExactSizeIterator<Item = impl Iterator<Item = &T>> + DoubleEndedIterator {
        (0..self.width()).map(move |column_index| {
            (0..self.height())
                .map(move |row_index| &self.elements[self.index(row_index, column_index)])
        })
    }

    pub fn map<U>(&self, mut f: impl FnMut(Position, &T) -> U) -> Grid<U> {
        let elements = self
            .iter_row_major()
            .map(|(position, element)| f(position, element))
            .collect_vec();
        Grid {
            elements,
            width: self.width(),
        }
    }

    #[allow(clippy::cast_possible_wrap)]
    pub fn corners_clockwise(&self) -> [Position; 4] {
        [
            (Position::new(0, 0)),
            (Position::new(0, self.width() as Coordinate - 1)),
            (Position::new(
                self.height() as Coordinate - 1,
                self.width() as Coordinate - 1,
            )),
            (Position::new(self.height() as Coordinate - 1, 0)),
        ]
    }

    pub fn height(&self) -> usize {
        self.elements.len() / self.width()
    }

    pub fn width(&self) -> usize {
        self.width
    }

    #[allow(clippy::cast_sign_loss)]
    fn index_of_position(&self, position: Position) -> usize {
        self.index(position.row() as usize, position.column() as usize)
    }

    fn index(&self, row: usize, column: usize) -> usize {
        row * self.width() + column
    }

    #[allow(clippy::cast_possible_wrap)]
    pub fn is_within_grid(&self, position: Position) -> bool {
        0 <= position.row()
            && position.row() < self.height() as Coordinate
            && 0 <= position.column()
            && position.column() < self.width() as Coordinate
    }

    fn from_str(str: &str, element_from_char: impl FnMut(char) -> T) -> Self {
        let width = str.lines().next().expect("grid should not be empty").len();
        debug_assert!(
            str.lines().map(str::len).all(|len| len == width),
            "every row should have the same width"
        );
        let elements = str
            .lines()
            .flat_map(|line| line.chars())
            .map(element_from_char)
            .collect_vec();
        Self { elements, width }
    }
}

impl<T: Into<char> + Clone> Grid<T> {
    fn display(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.elements.chunks(self.width) {
            for element in row {
                f.write_char(element.clone().into())?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl<S: AsRef<str>> From<S> for Grid<u8> {
    fn from(grid: S) -> Self {
        let to_byte = |char: char| -> u8 { char.try_into().expect("char should be ascii") };
        Self::from_str(grid.as_ref(), to_byte)
    }
}

impl<S: AsRef<str>> From<S> for Grid<char> {
    fn from(grid: S) -> Self {
        Self::from_str(grid.as_ref(), identity)
    }
}

impl<S: AsRef<str>> From<S> for Grid<usize> {
    fn from(grid: S) -> Self {
        Self::from_str(grid.as_ref(), |char| char as usize - '0' as usize)
    }
}

impl<S: AsRef<str>> From<S> for Grid<isize> {
    fn from(grid: S) -> Self {
        Self::from_str(grid.as_ref(), |char| char as isize - '0' as isize)
    }
}

impl<T> Index<Position> for Grid<T> {
    type Output = T;

    fn index(&self, position: Position) -> &Self::Output {
        self.get(position).expect("position should be within grid")
    }
}

impl<T> IndexMut<Position> for Grid<T> {
    fn index_mut(&mut self, position: Position) -> &mut Self::Output {
        self.get_mut(position)
            .expect("position should be within grid")
    }
}

impl Display for Grid<u8> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.display(f)
    }
}

impl Display for Grid<char> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.display(f)
    }
}
