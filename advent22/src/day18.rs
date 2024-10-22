use std::cmp;

use ahash::AHashSet;
use easy_cast::Cast;
use itertools::Itertools;

use shared::{search::Exploration, string::isizes};

type BoundingBox = [Position; 2];
type Position = [Coordinate; 3];
type Coordinate = isize;

pub fn first(input: &str) -> String {
    let cubes = cubes(input);
    area(&cubes).to_string()
}

pub fn second(input: &str) -> String {
    let cubes = cubes(input);
    exterior_area(&cubes).to_string()
}

fn exterior_area(cubes: &AHashSet<Position>) -> Coordinate {
    let bounding_box = bounding_box(cubes);
    let bounding_box_area = bounding_box_area(bounding_box);
    let exterior_volume = exterior_volume(cubes, bounding_box);
    let exterior_volume_area = area(&exterior_volume);
    exterior_volume_area - bounding_box_area
}

fn bounding_box(cubes: &AHashSet<Position>) -> [Position; 2] {
    let [[mut min_x, mut min_y, mut min_z], [mut max_x, mut max_y, mut max_z]] =
        [[Coordinate::MAX; 3], [Coordinate::MIN; 3]];
    for &[x, y, z] in cubes {
        min_x = cmp::min(min_x, x);
        min_y = cmp::min(min_y, y);
        min_z = cmp::min(min_z, z);
        max_x = cmp::max(max_x, x);
        max_y = cmp::max(max_y, y);
        max_z = cmp::max(max_z, z);
    }
    [
        [min_x - 1, min_y - 1, min_z - 1],
        [max_x + 1, max_y + 1, max_z + 1],
    ]
}

fn bounding_box_area([min, max]: BoundingBox) -> Coordinate {
    let edge_lengths = min.into_iter().zip(max).map(|(min, max)| max - min + 1);
    edge_lengths
        .combinations(2)
        .map(|pair| 2 * pair.into_iter().product::<Coordinate>())
        .sum()
}

fn exterior_volume(
    cubes: &AHashSet<Position>,
    [[min_x, min_y, min_z], [max_x, max_y, max_z]]: BoundingBox,
) -> AHashSet<Position> {
    let mut exploration = Exploration::new([]);
    let successors = |position| {
        neighbors(position).filter(|&neighbor @ [x, y, z]| {
            let is_inside_bounding_box =
                min_x <= x && x <= max_x && min_y <= y && y <= max_y && min_z <= z && z <= max_z;
            let is_free = !cubes.contains(&neighbor);
            is_inside_bounding_box && is_free
        })
    };
    exploration.explore([min_x, min_y, min_z], successors);
    exploration.explored()
}

fn area(cubes: &AHashSet<Position>) -> isize {
    cubes
        .iter()
        .copied()
        .flat_map(neighbors)
        .filter(|neighbor| !cubes.contains(neighbor))
        .count()
        .cast()
}

fn neighbors([x, y, z]: Position) -> impl Iterator<Item = Position> {
    [[1, 0, 0], [0, 1, 0], [0, 0, 1]]
        .into_iter()
        .flat_map(|[dx, dy, dz]| [[dx, dy, dz], [-dx, -dy, -dz]])
        .map(move |[dx, dy, dz]| [x + dx, y + dy, z + dz])
}

fn cubes(input: &str) -> AHashSet<Position> {
    input.lines().map(cube).collect()
}

fn cube(line: &str) -> Position {
    isizes(line)
        .try_into()
        .expect("line should contain three coordinates")
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 18;

    #[test]
    fn first_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 64);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 4400);
    }

    #[test]
    fn second_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 58);
    }

    #[test]
    fn second_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 2522);
    }
}
