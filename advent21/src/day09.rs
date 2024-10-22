use itertools::Itertools;
use shared::{
    grid::{neighbors, Grid, Position},
    search::Exploration,
};

type BasinSize = usize;
type RiskLevel = isize;
type Heightmap = Grid<Height>;
type Height = isize;

pub fn first(input: &str) -> String {
    let heightmap = Heightmap::from(input);
    low_points(&heightmap)
        .map(|low_point| risk_level(&heightmap, low_point))
        .sum::<RiskLevel>()
        .to_string()
}

pub fn second(input: &str) -> String {
    let heightmap = Heightmap::from(input);
    descending_basin_sizes(&heightmap)[0..3]
        .iter()
        .product::<BasinSize>()
        .to_string()
}

fn descending_basin_sizes(heightmap: &Heightmap) -> Vec<BasinSize> {
    let mut basin_sizes = low_points(heightmap)
        .map(|low_point| basin_size(heightmap, low_point))
        .collect_vec();
    basin_sizes.sort_unstable_by_key(|&basin_size| BasinSize::MAX - basin_size);
    basin_sizes
}

fn basin_size(heightmap: &Heightmap, low_point: Position) -> BasinSize {
    let mut exploration = Exploration::new([]);
    let successors = |point| {
        neighbors(point)
            .into_iter()
            .filter(|&neighbor| heightmap.get(neighbor).is_some())
            .filter(|&point| heightmap[point] != 9)
    };
    exploration.explore(low_point, successors);
    exploration.explored().len()
}

fn risk_level(heightmap: &Heightmap, low_point: Position) -> RiskLevel {
    heightmap[low_point] + 1
}

fn low_points(heightmap: &Heightmap) -> impl Iterator<Item = Position> + '_ {
    heightmap
        .iter_row_major()
        .filter(|&(point, _)| is_low_point(heightmap, point))
        .map(|(point, _)| point)
}

fn is_low_point(heightmap: &Heightmap, point: Position) -> bool {
    let height = heightmap[point];
    let mut neighbor_heights = neighbors(point)
        .into_iter()
        .filter_map(|neighbor| heightmap.get(neighbor))
        .copied();
    neighbor_heights.all(|neighbor_height| neighbor_height > height)
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 9;

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
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 931_200);
    }
}
