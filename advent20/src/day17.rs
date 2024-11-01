use std::array;

use ahash::AHashSet;
use easy_cast::Cast;
use itertools::Itertools;

type Grid<const DIMENSIONS: usize> = AHashSet<Position<DIMENSIONS>>;
type Position<const DIMENSIONS: usize> = [Coordinate; DIMENSIONS];
type Coordinate = i8;

pub fn first_answer(input: &str) -> String {
    let initial_grid = grid::<3>(input);
    let final_grid = grid_after(initial_grid, 6);
    final_grid.into_iter().count().to_string()
}

pub fn second_answer(input: &str) -> String {
    let initial_grid = grid::<4>(input);
    let final_grid = grid_after(initial_grid, 6);
    final_grid.into_iter().count().to_string()
}

fn grid_after<const DIMENSIONS: usize>(
    mut grid: Grid<DIMENSIONS>,
    number_of_cycles: usize,
) -> Grid<DIMENSIONS> {
    for _ in 0..number_of_cycles {
        grid = grid_after_cycle(&grid);
    }
    grid
}

fn grid_after_cycle<const DIMENSIONS: usize>(grid: &Grid<DIMENSIONS>) -> Grid<DIMENSIONS> {
    let all_neighborhoods: AHashSet<Position<DIMENSIONS>> = grid
        .iter()
        .flat_map(|&active| neighborhood(active))
        .collect();
    let mut next_grid = Grid::new();
    for position in all_neighborhoods {
        let active = grid.contains(&position);
        let number_of_active_neighbors = number_of_active_neighbors(grid, position);
        let active_with_two_or_three_neighbors =
            active && (2..=3).contains(&number_of_active_neighbors);
        let inactive_with_three_neighbors = !active && number_of_active_neighbors == 3;
        if active_with_two_or_three_neighbors || inactive_with_three_neighbors {
            next_grid.insert(position);
        }
    }
    next_grid
}

fn number_of_active_neighbors<const DIMENSIONS: usize>(
    grid: &Grid<DIMENSIONS>,
    position: Position<DIMENSIONS>,
) -> usize {
    let all = neighborhood(position)
        .filter(|position| grid.contains(position))
        .count();
    let this: usize = grid.contains(&position).into();
    all - this
}

fn neighborhood<const DIMENSIONS: usize>(
    position: Position<DIMENSIONS>,
) -> impl Iterator<Item = Position<DIMENSIONS>> {
    position
        .into_iter()
        .map(|coordinate| coordinate - 1..=coordinate + 1)
        .multi_cartesian_product()
        .map(|position| {
            position
                .try_into()
                .expect("position should have the correct number of coordinates")
        })
}

fn grid<const DIMENSIONS: usize>(input: &str) -> Grid<DIMENSIONS> {
    debug_assert!(DIMENSIONS >= 2, "at least two dimensions should be present");
    let actives = input.lines().enumerate().flat_map(|(row_index, row)| {
        row.chars()
            .enumerate()
            .filter(|&(_, char)| char == '#')
            .map(move |(column_index, _)| [row_index, column_index].cast())
    });
    let actives = actives.map(|active: [Coordinate; 2]| {
        array::from_fn(|index| active.get(index).copied().unwrap_or(0))
    });
    actives.collect()
}

#[cfg(test)]
mod tests {
    use ahash::AHashSet;
    use infrastructure::{Input, Puzzle};

    use super::*;
    use crate::tests::{input, test_on_input};

    const DAY: usize = 17;

    #[test]
    fn first_answer_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 112);
    }

    #[test]
    fn first_answer_puzzle_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 380);
    }

    #[test]
    fn second_answer_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 848);
    }

    #[test]
    fn second_answer_puzzle_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 2332);
    }

    #[test]
    fn grid_after_cycle() {
        let grid = grid(&input(DAY, Input::Example(0)));
        let actual = super::grid_after_cycle(&grid);
        let expected = AHashSet::from([
            [1, 0, -1],
            [2, 2, -1],
            [3, 1, -1],
            [1, 0, 0],
            [1, 2, 0],
            [2, 1, 0],
            [2, 2, 0],
            [3, 1, 0],
            [1, 0, 1],
            [2, 2, 1],
            [3, 1, 1],
        ]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn neighborhood() {
        let actual = super::neighborhood([0, 0, 0]).count();
        let expected = 27;
        assert_eq!(actual, expected);
    }
}
