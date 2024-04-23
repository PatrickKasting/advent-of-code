use std::collections::BTreeSet;

use itertools::Itertools;

type Summary = usize;
type Pattern = Vec<Vec<Tile>>;
type Symmetry = usize;
type Tile = u8;

pub fn first(input: &str) -> String {
    sum_of_pattern_summaries(input, pattern_summary).to_string()
}

pub fn second(input: &str) -> String {
    sum_of_pattern_summaries(input, correct_pattern_summary).to_string()
}

fn sum_of_pattern_summaries(
    input: &str,
    mut pattern_summary: impl FnMut(&mut Pattern) -> Summary,
) -> Summary {
    input
        .split("\n\n")
        .map(pattern)
        .map(|mut pattern| pattern_summary(&mut pattern))
        .sum()
}

fn pattern_summary(pattern: &mut Pattern) -> Summary {
    let [horizontal_reflection, vertical_reflection] = reflections(pattern).map(|reflections| {
        reflections
            .into_iter()
            .at_most_one()
            .expect("at most one reflection in each direction should exist")
    });
    summary_from_reflections(horizontal_reflection, vertical_reflection)
        .expect("pattern should have a horizontal reflection or a vertical reflection")
}

fn correct_pattern_summary(pattern: &mut Pattern) -> Summary {
    let original_reflections = reflections(pattern);
    for row_index in 0..pattern.len() {
        for column_index in 0..pattern[row_index].len() {
            pattern[row_index][column_index] = opposite(pattern[row_index][column_index]);
            let new_reflections = reflections(pattern);
            let [horizontal_reflection, vertical_reflection] = [0, 1].map(|index| {
                new_reflections[index]
                    .difference(&original_reflections[index])
                    .exactly_one()
                    .ok()
                    .copied()
            });
            if let Some(summary) =
                summary_from_reflections(horizontal_reflection, vertical_reflection)
            {
                return summary;
            }
            pattern[row_index][column_index] = opposite(pattern[row_index][column_index]);
        }
    }
    panic!("pattern should be corrected by exactly one change");
}

fn reflections(pattern: &Pattern) -> [BTreeSet<Symmetry>; 2] {
    [symmetries(pattern), nested_symmetries(pattern)]
}

fn symmetries<T: Eq>(sequence: &[T]) -> BTreeSet<Symmetry> {
    let mut symmetries = BTreeSet::new();
    for possible_symmetry in 1..sequence.len() {
        let (left, right) = sequence.split_at(possible_symmetry);
        let is_symmetry = left
            .iter()
            .rev()
            .zip(right)
            .all(|(left_element, right_element)| left_element == right_element);
        if is_symmetry {
            symmetries.insert(possible_symmetry);
        }
    }
    symmetries
}

fn nested_symmetries<T: Eq>(sequence: &[Vec<T>]) -> BTreeSet<Symmetry> {
    sequence
        .iter()
        .map(|nested_sequence| symmetries(nested_sequence))
        .reduce(|possible_symmetries, nested_sequence_symmetries| {
            possible_symmetries
                .intersection(&nested_sequence_symmetries)
                .copied()
                .collect::<BTreeSet<Symmetry>>()
        })
        .expect("sequence should not be empty")
}

fn summary_from_reflections(
    horizontal_reflection: Option<Symmetry>,
    vertical_reflection: Option<Symmetry>,
) -> Option<Summary> {
    match (horizontal_reflection, vertical_reflection) {
        (None, None) => None,
        (Some(horizontal_reflection), None) => Some(100 * horizontal_reflection),
        (None, Some(vertical_reflection)) => Some(vertical_reflection),
        _ => {
            panic!("two reflections should not exist in one pattern");
        }
    }
}

fn opposite(tile: Tile) -> Tile {
    if tile == b'#' {
        b'.'
    } else {
        b'#'
    }
}

fn pattern(pattern: &str) -> Vec<Vec<Tile>> {
    pattern
        .lines()
        .map(str::as_bytes)
        .map(Vec::from)
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use super::super::tests::test_on_input;
    use crate::{Input, Puzzle};

    const DAY: usize = 13;

    #[test]
    fn first_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 405);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 29130);
    }

    #[test]
    fn second_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 400);
    }

    #[test]
    fn second_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 33438);
    }
}
