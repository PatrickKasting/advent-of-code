use std::fmt::{Debug, Display, Write};

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
    pub fn next_clockwise(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    pub fn opposite(self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
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
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Grid<T> {
    elements: Vec<Vec<T>>,
}

impl<T> Grid<T> {
    pub fn get(&self, position: Position) -> Option<&T> {
        let [row, column] = [position.row(), position.column()]
            .map(|coordinate| (!coordinate.is_negative()).then_some(coordinate as usize));
        self.elements.get(row?)?.get(column?)
    }

    pub fn positions(&self, mut predicate: impl FnMut(&T) -> bool) -> Vec<Position> {
        let mut positions = Vec::new();
        for (row_index, row) in self.elements.iter().enumerate() {
            for (column_index, element) in row.iter().enumerate() {
                if predicate(element) {
                    positions.push(Position::new(row_index as isize, column_index as isize));
                }
            }
        }
        positions
    }

    pub fn row_indices(&self, mut predicate: impl FnMut(&T) -> bool) -> Vec<Coordinate> {
        self.elements
            .iter()
            .enumerate()
            .filter(|(_, row)| row.iter().all(&mut predicate))
            .map(|(row_index, _)| row_index as isize)
            .collect_vec()
    }

    pub fn column_indices(&self, mut predicate: impl FnMut(&T) -> bool) -> Vec<Coordinate> {
        (0..self.width())
            .filter(|&column_index| {
                self.elements
                    .iter()
                    .all(|row| predicate(&row[column_index as usize]))
            })
            .collect_vec()
    }

    fn width(&self) -> Coordinate {
        self.elements
            .first()
            .map(|row| row.len() as isize)
            .unwrap_or_default()
    }
}

impl From<&str> for Grid<char> {
    fn from(grid: &str) -> Self {
        let elements = grid
            .lines()
            .map(|line| line.chars().collect_vec())
            .collect_vec();
        Self { elements }
    }
}

impl Display for Grid<char> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.elements {
            for &element in row {
                f.write_char(element)?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}
