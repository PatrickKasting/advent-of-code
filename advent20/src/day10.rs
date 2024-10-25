use std::iter;

use itertools::Itertools;

type Joltage = usize;

pub fn first_answer(input: &str) -> String {
    let mut joltage_ratings = joltage_ratings(input);
    let joltage_differences = joltage_differences(&mut joltage_ratings).counts();
    (joltage_differences[&1] * joltage_differences[&3]).to_string()
}

pub fn second_answer(input: &str) -> String {
    let mut joltage_ratings = joltage_ratings(input);
    number_of_adapter_arrangements(&mut joltage_ratings).to_string()
}

fn joltage_differences(joltage_ratings: &mut [Joltage]) -> impl Iterator<Item = Joltage> + '_ {
    joltage_ratings.sort_unstable();

    let charging_outlet = iter::once(joltage_ratings[0]);
    let joltage_differences = joltage_ratings.windows(2).map(|pair| pair[1] - pair[0]);
    let built_in_adapter = iter::once(3);
    charging_outlet
        .chain(joltage_differences)
        .chain(built_in_adapter)
}

fn number_of_adapter_arrangements(joltage_ratings: &mut Vec<Joltage>) -> usize {
    joltage_ratings.push(0); // charging outlet
    joltage_ratings.sort_unstable();

    let mut number_of_arrangements = vec![0; joltage_ratings.len()];
    number_of_arrangements[0] = 1; // charging outlet
    for (from_index, &from_rating) in joltage_ratings.iter().enumerate() {
        let connections = joltage_ratings
            .iter()
            .enumerate()
            .skip(from_index + 1)
            .take_while(|(_, &to_rating)| to_rating - from_rating <= 3);
        for (to_index, _) in connections {
            number_of_arrangements[to_index] += number_of_arrangements[from_index];
        }
    }
    *number_of_arrangements
        .last()
        .expect("at least the charging outlet should be in list")
}

fn joltage_ratings(input: &str) -> Vec<Joltage> {
    input
        .lines()
        .map(|line| line.parse().expect("joltage should be numeric"))
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 10;

    #[test]
    fn first_answer_examples() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 35);
        test_on_input(DAY, Puzzle::First, Input::Example(1), 220);
    }

    #[test]
    fn first_answer_puzzle_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 2346);
    }

    #[test]
    fn second_answer_examples() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 8);
        test_on_input(DAY, Puzzle::Second, Input::Example(1), 19208);
    }

    #[test]
    fn second_answer_puzzle_input() {
        test_on_input(
            DAY,
            Puzzle::Second,
            Input::PuzzleInput,
            6_044_831_973_376_usize,
        );
    }
}
