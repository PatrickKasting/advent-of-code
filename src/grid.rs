use std::fmt::{Debug, Display, Write};

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

    pub fn next_counterclockwise(self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
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

    pub fn direction_to(self, neighbor: Position) -> Direction {
        match (neighbor.row - self.row, neighbor.column - self.column) {
            (-1, 0) => Direction::North,
            (0, 1) => Direction::East,
            (1, 0) => Direction::South,
            (0, -1) => Direction::West,
            _ => panic!("the positions should be neighbors"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Grid<T> {
    elements: Vec<T>,
    width: usize,
}

impl<T> Grid<T> {
    pub fn new(elements: Vec<T>, width: usize) -> Self {
        Self { elements, width }
    }

    pub fn get(&self, position: Position) -> Option<&T> {
        self.is_position_within(position)
            .then(|| &self.elements[self.index(position)])
    }

    pub fn position_of(&self, mut predicate: impl FnMut(&T) -> bool) -> Option<Position> {
        for (row_index, row) in self.elements.chunks(self.width).enumerate() {
            for (column_index, element) in row.iter().enumerate() {
                if predicate(element) {
                    return Some(Position::new(row_index as isize, column_index as isize));
                }
            }
        }
        None
    }

    fn height(&self) -> usize {
        self.elements.len() / self.width
    }

    pub fn is_position_within(&self, position: Position) -> bool {
        0 <= position.row()
            && position.row() < self.height() as isize
            && 0 <= position.column()
            && position.column() < self.width as isize
    }

    fn index(&self, position: Position) -> usize {
        position.row() as usize * self.width + position.column() as usize
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
