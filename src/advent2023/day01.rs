use std::cmp;

use itertools::Itertools;

type CalibrationValue = usize;

const NUMERALS: [(&str, CalibrationValue); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn first_and_last_digit(line: &str) -> [Option<(usize, CalibrationValue)>; 2] {
    [str::find, str::rfind].map(|finder| {
        let index = finder(line, char::is_numeric);
        index.map(|index| (index, (line.as_bytes()[index] - b'0') as CalibrationValue))
    })
}

fn first_and_last_numeral(line: &str) -> [Option<(usize, CalibrationValue)>; 2] {
    let numerals = NUMERALS
        .into_iter()
        .flat_map(|(numeral, value)| {
            line.match_indices(numeral)
                .map(move |(index, _)| (index, value))
        })
        .collect_vec();
    [Iterator::min, Iterator::max].map(|extremum| extremum(numerals.iter().copied()))
}

fn calibration_value(consider_numerals: bool, line: &str) -> CalibrationValue {
    let [mut first_value, mut last_value] = first_and_last_digit(line);
    if consider_numerals {
        let [first_numeral, last_numeral] = first_and_last_numeral(line);
        first_value = cmp::min(first_value.or(first_numeral), first_numeral.or(first_value));
        last_value = cmp::max(last_value.or(last_numeral), last_numeral.or(last_value));
    }
    let [first_value, last_value] = [first_value, last_value]
        .map(|value| value.expect("line should contain at least one digit").1);
    first_value * 10 + last_value
}

fn sum_of_calibration_values(consider_numerals: bool, input: &str) -> CalibrationValue {
    input
        .lines()
        .map(|line| calibration_value(consider_numerals, line))
        .sum()
}

pub fn first(input: &str) -> String {
    sum_of_calibration_values(false, input).to_string()
}

pub fn second(input: &str) -> String {
    sum_of_calibration_values(true, input).to_string()
}

#[cfg(test)]
mod tests {
    use super::super::tests::test_on_input;
    use crate::{Input, Puzzle};

    const DAY: usize = 1;

    #[test]
    fn first_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 142);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 56042);
    }

    #[test]
    fn second_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(1), 281);
    }

    #[test]
    fn second_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 55358);
    }
}
