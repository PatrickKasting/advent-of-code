use std::{
    convert::identity,
    fmt::{Debug, Display, Write},
    ops::{Index, IndexMut},
};

use easy_cast::Cast;
use itertools::Itertools;

use crate::vector::Vector;

pub type Position = [Coordinate; 2];
pub type Coordinate = isize;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Grid<T> {
    elements: Vec<T>,
    width: usize,
}

impl<T> Grid<T> {
    #[must_use]
    pub fn new(height: usize, width: usize, mut element: impl FnMut(Position) -> T) -> Self {
        let mut elements = vec![];
        for row in 0..height {
            for column in 0..width {
                elements.push(element([row.cast(), column.cast()]));
            }
        }
        Self { elements, width }
    }

    #[must_use]
    pub fn from_elements(elements: Vec<T>, width: usize) -> Self {
        debug_assert!(
            elements.len() % width == 0,
            "width should divide number of elements for rectangular grid"
        );
        Self { elements, width }
    }

    #[must_use]
    pub fn get(&self, [row, column]: Position) -> Option<&T> {
        let position = [row.try_into().ok()?, column.try_into().ok()?];
        self.is_within_grid(position)
            .then(|| &self.elements[self.index(position)])
    }

    #[must_use]
    pub fn get_mut(&mut self, [row, column]: Position) -> Option<&mut T> {
        let position = [row.try_into().ok()?, column.try_into().ok()?];
        self.is_within_grid(position).then(|| {
            let index = self.index(position);
            &mut self.elements[index]
        })
    }

    pub fn iter_row_major(&self) -> impl Iterator<Item = (Position, &T)> {
        self.rows().enumerate().flat_map(|(row_index, row)| {
            row.enumerate().map(move |(column_index, element)| {
                ([row_index.cast(), column_index.cast()], element)
            })
        })
    }

    pub fn iter_column_major(&self) -> impl Iterator<Item = (Position, &T)> {
        self.columns()
            .enumerate()
            .flat_map(|(column_index, column)| {
                column.enumerate().map(move |(row_index, element)| {
                    ([row_index.cast(), column_index.cast()], element)
                })
            })
    }

    #[must_use]
    pub fn rows(
        &self,
    ) -> impl ExactSizeIterator<Item = impl Iterator<Item = &T>> + DoubleEndedIterator {
        self.elements.chunks(self.width()).map(|row| row.iter())
    }

    #[must_use]
    pub fn columns(
        &self,
    ) -> impl ExactSizeIterator<Item = impl Iterator<Item = &T>> + DoubleEndedIterator {
        (0..self.width()).map(move |column_index| {
            (0..self.height())
                .map(move |row_index| &self.elements[self.index([row_index, column_index])])
        })
    }

    #[must_use]
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

    pub fn position(&self, mut predicate: impl FnMut(&T) -> bool) -> Option<Position> {
        self.iter_row_major()
            .find_map(|(position, element)| predicate(element).then_some(position))
    }

    #[must_use]
    pub fn corners_clockwise(&self) -> [Position; 4] {
        let corners = [
            [0, 0],
            [0, self.width() - 1],
            [self.height() - 1, self.width() - 1],
            [self.height() - 1, 0],
        ];
        corners.map(|[row, column]| [row.cast(), column.cast()])
    }

    #[must_use]
    pub fn height(&self) -> usize {
        self.elements.len() / self.width()
    }

    #[must_use]
    pub fn width(&self) -> usize {
        self.width
    }

    fn is_within_grid(&self, [row, column]: [usize; 2]) -> bool {
        row < self.height() && column < self.width()
    }

    fn index(&self, [row, column]: [usize; 2]) -> usize {
        row * self.width() + column
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

pub type Direction = [Coordinate; 2];

pub const NORTH: Direction = [-1, 0];
pub const NORTH_EAST: Direction = [-1, 1];
pub const EAST: Direction = [0, 1];
pub const SOUTH_EAST: Direction = [1, 1];
pub const SOUTH: Direction = [1, 0];
pub const SOUTH_WEST: Direction = [1, -1];
pub const WEST: Direction = [0, -1];
pub const NORTH_WEST: Direction = [-1, -1];

pub const DIRECTIONS: [Direction; 4] = [NORTH, EAST, SOUTH, WEST];
pub const DIRECTIONS_INCLUDING_DIAGONAL: [Direction; 8] = [
    NORTH, NORTH_EAST, EAST, SOUTH_EAST, SOUTH, SOUTH_WEST, WEST, NORTH_WEST,
];

#[must_use]
pub fn neighbors(position: Position) -> [Position; 4] {
    DIRECTIONS.map(|direction| position.add(direction))
}

#[must_use]
pub fn neighbors_including_diagonal(position: Position) -> [Position; 8] {
    DIRECTIONS_INCLUDING_DIAGONAL.map(|direction| position.add(direction))
}
