use itertools::Itertools;
use strum::IntoEnumIterator;

use crate::data_structures::grid::{Direction, Grid, Position};

type Height = isize;
type ScenicScore = usize;

pub fn first(input: &str) -> String {
    number_of_visible_trees_from_edges(&Grid::from(input)).to_string()
}

pub fn second(input: &str) -> String {
    max_scenic_score(&Grid::from(input)).to_string()
}

fn number_of_visible_trees_from_edges(grid: &Grid<Height>) -> usize {
    let mut visibility = grid.map(|_, _| false);
    for (from_corner, to_corner) in grid
        .corners_clockwise()
        .into_iter()
        .circular_tuple_windows()
    {
        visibility_from_edge(grid, &mut visibility, from_corner, to_corner);
    }
    visibility
        .iter_row_major()
        .filter(|(_, &visible)| visible)
        .count()
}

fn visibility_from_edge(
    grid: &Grid<Height>,
    visibility: &mut Grid<bool>,
    from_corner: Position,
    to_corner: Position,
) {
    let edge_direction = from_corner
        .direction_to(to_corner)
        .expect("positions should be on the same edge");
    let mut position = from_corner;
    while position != to_corner {
        visibility_along_line(grid, visibility, position, edge_direction.right());
        position = position.neighbor(edge_direction);
    }
}

fn visibility_along_line(
    grid: &Grid<Height>,
    visibility: &mut Grid<bool>,
    mut position: Position,
    direction: Direction,
) {
    let mut tallest = -1;
    while let Some(&height) = grid.get(position) {
        if height > tallest {
            tallest = height;
            visibility[position] = true;
        }
        position = position.neighbor(direction);
    }
}

fn max_scenic_score(grid: &Grid<isize>) -> ScenicScore {
    grid.iter_row_major()
        .map(|(position, _)| scenic_score(grid, position))
        .max()
        .expect("grid should contain at least one tree")
}

fn scenic_score(grid: &Grid<Height>, position: Position) -> ScenicScore {
    Direction::iter()
        .map(|direction| number_of_visible_trees_along_line(grid, position, direction))
        .product()
}

fn number_of_visible_trees_along_line(
    grid: &Grid<Height>,
    mut position: Position,
    direction: Direction,
) -> ScenicScore {
    let view_height = grid[position];
    position = position.neighbor(direction);
    let mut number_of_visible_trees = 0;
    while let Some(&height) = grid.get(position) {
        number_of_visible_trees += 1;
        if height >= view_height {
            break;
        }
        position = position.neighbor(direction);
    }
    number_of_visible_trees
}

#[cfg(test)]
mod tests {
    use super::super::tests::test_on_input;
    use crate::{Input, Puzzle};

    const DAY: usize = 8;

    #[test]
    fn first_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 21);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 1695);
    }

    #[test]
    fn second_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 8);
    }

    #[test]
    fn second_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 287_040);
    }
}
