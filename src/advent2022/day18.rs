use std::collections::HashSet;

use crate::strings::isizes;

type Cube = [Coordinate; 3];
type Coordinate = isize;

pub fn first(input: &str) -> String {
    let cubes = cubes(input);
    surface_area(&cubes).to_string()
}

pub fn second(_input: &str) -> String {
    todo!();
}

fn surface_area(cubes: &HashSet<Cube>) -> usize {
    cubes
        .iter()
        .copied()
        .flat_map(neighbors)
        .filter(|neighbor| !cubes.contains(neighbor))
        .count()
}

fn neighbors([x, y, z]: Cube) -> impl Iterator<Item = Cube> {
    [[1, 0, 0], [0, 1, 0], [0, 0, 1]]
        .into_iter()
        .flat_map(|[dx, dy, dz]| [[dx, dy, dz], [-dx, -dy, -dz]])
        .map(move |[dx, dy, dz]| [x + dx, y + dy, z + dz])
}

fn cubes(input: &str) -> HashSet<Cube> {
    input.lines().map(cube).collect()
}

fn cube(line: &str) -> Cube {
    isizes(line)
        .try_into()
        .expect("line should contain three coordinates")
}

#[cfg(test)]
mod tests {
    use super::super::tests::test_on_input;
    use crate::{Input, Puzzle};

    const DAY: usize = 18;

    #[test]
    fn first_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 64);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 4400);
    }

    // #[test]
    // fn second_example() {
    //     test_on_input(DAY, Puzzle::Second, Input::Example(0), 24_933_642);
    // }

    // #[test]
    // fn second_input() {
    //     test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 404_395);
    // }
}
