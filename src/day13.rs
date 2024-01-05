use std::collections::BTreeSet;

use itertools::Itertools;

type Pattern = Vec<Vec<u8>>;

fn pattern(pattern: &str) -> Vec<Vec<u8>> {
    pattern
        .lines()
        .map(str::as_bytes)
        .map(Vec::from)
        .collect_vec()
}

fn symmetries<T: Eq>(sequence: &[T]) -> BTreeSet<usize> {
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

fn nested_symmetries<T: Eq>(sequence: &[Vec<T>]) -> BTreeSet<usize> {
    sequence
        .iter()
        .map(|nested_sequence| symmetries(nested_sequence))
        .reduce(|possible_symmetries, nested_sequence_symmetries| {
            possible_symmetries
                .intersection(&nested_sequence_symmetries)
                .copied()
                .collect::<BTreeSet<usize>>()
        })
        .expect("elements should not be empty")
}

fn folds(pattern: &Pattern) -> (BTreeSet<usize>, BTreeSet<usize>) {
    (symmetries(pattern), nested_symmetries(pattern))
}

fn summary_from_folds(
    horizontal_fold: Option<usize>,
    vertical_fold: Option<usize>,
) -> Option<usize> {
    match (horizontal_fold, vertical_fold) {
        (None, None) => None,
        (Some(horizontal_fold), None) => Some(100 * horizontal_fold),
        (None, Some(vertical_fold)) => Some(vertical_fold),
        _ => unreachable!("pattern should never have both a horizontal fold and a vertical fold"),
    }
}

fn pattern_summary(pattern: &mut Pattern) -> usize {
    let (mut horizontal_folds, mut vertical_folds) = folds(pattern);
    summary_from_folds(horizontal_folds.pop_first(), vertical_folds.pop_first())
        .expect("pattern should have a horizontal fold or a vertical fold")
}

fn opposite_type(kind: u8) -> u8 {
    if kind == b'#' {
        b'.'
    } else {
        b'#'
    }
}

fn correct_pattern_summary(pattern: &mut Pattern) -> usize {
    let original_folds = folds(pattern);
    for row_index in 0..pattern.len() {
        for column_index in 0..pattern[row_index].len() {
            pattern[row_index][column_index] = opposite_type(pattern[row_index][column_index]);
            let folds = folds(pattern);
            let horizontal_fold = folds
                .0
                .difference(&original_folds.0)
                .exactly_one()
                .ok()
                .copied();
            let vertical_fold = folds
                .1
                .difference(&original_folds.1)
                .exactly_one()
                .ok()
                .copied();
            if let Some(summary) = summary_from_folds(horizontal_fold, vertical_fold) {
                return summary;
            }
            pattern[row_index][column_index] = opposite_type(pattern[row_index][column_index]);
        }
    }
    unreachable!("pattern should be corrected by exactly one change");
}

fn sum_of_pattern_summaries(
    input: &str,
    mut pattern_summary: impl FnMut(&mut Pattern) -> usize,
) -> usize {
    input
        .split("\n\n")
        .map(pattern)
        .map(|mut pattern| pattern_summary(&mut pattern))
        .sum()
}

pub fn first(input: String) -> String {
    sum_of_pattern_summaries(&input, pattern_summary).to_string()
}

pub fn second(input: String) -> String {
    sum_of_pattern_summaries(&input, correct_pattern_summary).to_string()
}

#[cfg(test)]
mod tests {
    use crate::{tests::*, Input, Puzzle};

    const DAY: usize = 13;

    #[test]
    fn first_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 405);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::Real, 29130);
    }

    #[test]
    fn second_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 400);
    }

    #[test]
    fn second_input() {
        test_on_input(DAY, Puzzle::Second, Input::Real, 33438);
    }
}
