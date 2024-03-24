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
    pub fn right(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::West => Direction::North,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
        }
    }

    pub fn backward(self) -> Self {
        self.right().right()
    }

    pub fn left(self) -> Direction {
        self.backward().right()
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

impl TryFrom<[Position; 2]> for Direction {
    type Error = &'static str;

    fn try_from([from, to]: [Position; 2]) -> Result<Self, Self::Error> {
        if from == to {
            return Err("positions should not be identical");
        }

        if from.row() == to.row() {
            if from.column() < to.column() {
                Ok(Direction::East)
            } else {
                Ok(Direction::West)
            }
        } else if from.column() == to.column() {
            if from.row() < to.row() {
                Ok(Direction::South)
            } else {
                Ok(Direction::North)
            }
        } else {
            Err("positions should share a row or a column")
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

impl From<[Direction; 2]> for RelativeDirection {
    fn from([nose, other]: [Direction; 2]) -> Self {
        if other == nose {
            RelativeDirection::Forward
        } else if other == nose.right() {
            RelativeDirection::Right
        } else if other == nose.backward() {
            RelativeDirection::Backward
        } else {
            RelativeDirection::Left
        }
    }
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
    pub fn new<R, C>(row: R, column: C) -> Self
    where
        Coordinate: TryFrom<R> + TryFrom<C>,
        <Coordinate as TryFrom<R>>::Error: Debug,
        <Coordinate as TryFrom<C>>::Error: Debug,
    {
        Position {
            row: row.try_into().expect("row should convert to 'Coordinate'"),
            column: column
                .try_into()
                .expect("column should convert to 'Coordinate'"),
        }
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
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Grid<T> {
    elements: Vec<T>,
    width: usize,
}

impl<T> Grid<T> {
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

    pub fn get(&self, position: Position) -> Option<&T> {
        self.within_grid(position)
            .then(|| &self.elements[self.index_of_position(position)])
    }

    pub fn get_mut(&mut self, position: Position) -> Option<&mut T> {
        self.within_grid(position).then(|| {
            let index = self.index_of_position(position);
            &mut self.elements[index]
        })
    }

    pub fn iter_row_major(&self) -> impl Iterator<Item = (Position, &T)> {
        self.rows().enumerate().flat_map(|(row_index, row)| {
            row.enumerate().map(move |(column_index, element)| {
                (Position::new(row_index, column_index), element)
            })
        })
    }

    pub fn iter_column_major(&self) -> impl Iterator<Item = (Position, &T)> {
        self.columns()
            .enumerate()
            .flat_map(|(column_index, column)| {
                column.enumerate().map(move |(row_index, element)| {
                    (Position::new(row_index, column_index), element)
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

    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_possible_wrap)]
    fn within_grid(&self, position: Position) -> bool {
        0 <= position.row()
            && position.row() < self.height() as Coordinate
            && 0 <= position.column()
            && position.column() < self.width() as Coordinate
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

impl Display for Grid<char> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.elements.chunks(self.width) {
            for &element in row {
                f.write_char(element)?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}
