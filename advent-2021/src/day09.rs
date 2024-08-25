use std::collections::HashSet;

use shared::grid::{self, Grid, Position};

pub fn first(input: &str) -> String {
    let heightmap = Grid::from(input);
    let low_points = low_points(&heightmap);
    low_points
        .map(|pos| 1 + heightmap[pos])
        .sum::<usize>()
        .to_string()
}

pub fn second(input: &str) -> String {
    let heightmap = Grid::from(input);
    let low_points = low_points(&heightmap);
    let mut basin_sizes: Vec<usize> = low_points
        .map(|low_point| basin_size(&heightmap, low_point))
        .collect();
    basin_sizes.sort_by_key(|basin_size| usize::MAX - basin_size);
    basin_sizes[0..3].iter().product::<usize>().to_string()
}

fn low_points(heightmap: &Grid<usize>) -> impl Iterator<Item = Position> + '_ {
    heightmap
        .iter_row_major()
        .filter(|&(pos, &height)| {
            grid::neighbors(pos)
                .into_iter()
                .all(|neighbor| heightmap[neighbor] > height)
        })
        .map(|(pos, _)| pos)
}

fn basin_size(heightmap: &Grid<usize>, low_point: Position) -> usize {
    let mut explored = HashSet::from([low_point]);
    let mut frontier = Vec::from([low_point]);
    while let Some(position) = frontier.pop() {
        for neighbor in grid::neighbors(position) {
            if heightmap[neighbor] == 9 {
                continue;
            }
            if explored.insert(neighbor) {
                frontier.push(neighbor)
            }
        }
    }
    explored.len()
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 7;

    #[test]
    fn first_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 15);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 506);
    }

    #[test]
    fn second_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 1134);
    }

    #[test]
    fn second_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 931200);
    }
}
