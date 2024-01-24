#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_sign_loss)]

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
    West,
    South,
    East,
}

impl Direction {
    pub fn next_clockwise(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::West => Direction::North,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
        }
    }

    pub fn opposite(self) -> Self {
        self.next_clockwise().next_clockwise()
    }

    pub fn next_counterclockwise(self) -> Direction {
        self.opposite().next_clockwise()
    }

    pub fn reflection_north_west_diagonal(self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::North,
            Direction::South => Direction::East,
            Direction::East => Direction::South,
        }
    }

    pub fn reflection_north_east_diagonal(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::West => Direction::South,
            Direction::South => Direction::West,
            Direction::East => Direction::North,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Curvature {
    Straight,
    LeftTurn,
    UTurn,
    RightTurn,
}

impl From<(Direction, Direction)> for Curvature {
    fn from((towards, away): (Direction, Direction)) -> Self {
        if towards == away {
            Self::Straight
        } else if towards == away.next_clockwise() {
            Self::LeftTurn
        } else if towards.opposite() == away {
            Self::UTurn
        } else {
            Self::RightTurn
        }
    }
}

type Coordinate = isize;

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
    pub fn new(row: Coordinate, column: Coordinate) -> Self {
        Position { row, column }
    }

    pub fn row(self) -> Coordinate {
        self.row
    }

    pub fn column(self) -> Coordinate {
        self.column
    }

    pub fn neighbor(mut self, direction: Direction) -> Position {
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

    fn coordinates_as_usize(self) -> [Option<usize>; 2] {
        [self.row, self.column]
            .map(|coordinate| (!coordinate.is_negative()).then_some(coordinate as usize))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Grid<T>(Vec<Vec<T>>);

impl<T> Grid<T> {
    fn from_str(grid: &str, mut element_from_char: impl FnMut(char) -> T) -> Self {
        let elements = grid
            .lines()
            .map(|line| line.chars().map(&mut element_from_char).collect_vec())
            .collect_vec();
        Self(elements)
    }

    pub fn get(&self, position: Position) -> Option<&T> {
        let [row, column] = position.coordinates_as_usize();
        self.0.get(row?)?.get(column?)
    }

    pub fn get_mut(&mut self, position: Position) -> Option<&mut T> {
        let [row, column] = position.coordinates_as_usize();
        self.0.get_mut(row?)?.get_mut(column?)
    }

    pub fn iter_row_major(&self) -> impl Iterator<Item = (Position, &T)> {
        self.rows().enumerate().flat_map(|(row_index, row)| {
            row.enumerate().map(move |(column_index, element)| {
                (
                    Position::new(row_index as isize, column_index as isize),
                    element,
                )
            })
        })
    }

    pub fn iter_column_major(&self) -> impl Iterator<Item = (Position, &T)> {
        self.columns()
            .enumerate()
            .flat_map(|(column_index, column)| {
                column.enumerate().map(move |(row_index, element)| {
                    (
                        Position::new(row_index as isize, column_index as isize),
                        element,
                    )
                })
            })
    }

    pub fn rows(
        &self,
    ) -> impl ExactSizeIterator<Item = impl Iterator<Item = &T>> + DoubleEndedIterator {
        self.0.iter().map(|row| row.iter())
    }

    pub fn columns(
        &self,
    ) -> impl ExactSizeIterator<Item = impl Iterator<Item = &T>> + DoubleEndedIterator {
        (0..self.width())
            .map(|column_index| self.0.iter().map(move |row| &row[column_index as usize]))
    }

    pub fn height(&self) -> Coordinate {
        self.0.len() as Coordinate
    }

    pub fn width(&self) -> Coordinate {
        self.0
            .first()
            .map(|row| row.len() as Coordinate)
            .unwrap_or_default()
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

impl Display for Grid<char> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.0 {
            for &element in row {
                f.write_char(element)?;
            }
            f.write_char('\n')?;
        }
        Ok(())
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
