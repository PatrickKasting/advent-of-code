use std::iter;

use ahash::{AHashMap, AHashSet};
use easy_cast::Cast;
use itertools::Itertools;
use shared::{grid::Position, vector::Vector};

pub fn first_answer(input: &str) -> String {
    let ([height, width], antennas) = map(input);
    unique_antinodes(height, width, &antennas, antinode)
        .len()
        .to_string()
}

pub fn second_answer(input: &str) -> String {
    let ([height, width], antennas) = map(input);
    unique_antinodes(height, width, &antennas, antinodes)
        .len()
        .to_string()
}

fn unique_antinodes(
    height: usize,
    width: usize,
    antennas: &AHashMap<char, Vec<Position>>,
    antinodes: fn(usize, usize, Position, Position) -> Vec<Position>,
) -> AHashSet<Position> {
    antennas
        .values()
        .flat_map(|antennas| {
            antennas
                .iter()
                .permutations(2)
                .flat_map(|pair| antinodes(height, width, *pair[0], *pair[1]))
        })
        .filter(|&antinode| is_within_map(height, width, antinode))
        .collect()
}

fn antinode(_: usize, _: usize, lhs: Position, rhs: Position) -> Vec<Position> {
    vec![lhs.add(lhs.sub(rhs))]
}

fn antinodes(height: usize, width: usize, lhs: Position, rhs: Position) -> Vec<Position> {
    let step = lhs.sub(rhs);
    iter::successors(Some(lhs), |&position| {
        is_within_map(height, width, position).then(|| position.add(step))
    })
    .collect_vec()
}

fn is_within_map(height: usize, width: usize, [row, column]: Position) -> bool {
    0 <= row && 0 <= column && row < height.cast() && column < width.cast()
}

fn map(input: &str) -> ([usize; 2], AHashMap<char, Vec<Position>>) {
    let height = input.lines().count();
    let width = input
        .lines()
        .next()
        .expect("input should contain at least one line")
        .len();
    let mut antennas: AHashMap<char, Vec<Position>> = AHashMap::new();
    for (row, line) in input.lines().enumerate() {
        for (column, char) in line.chars().enumerate() {
            if char != '.' {
                antennas.entry(char).or_default().push([row, column].cast());
            }
        }
    }
    ([height, width], antennas)
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 8;

    #[test]
    fn first_answer_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 14);
    }

    #[test]
    fn first_answer_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 396);
    }

    #[test]
    fn second_answer_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 34);
    }

    #[test]
    fn second_answer_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 1200);
    }
}
