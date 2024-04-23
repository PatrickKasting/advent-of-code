use std::cmp;

use easy_cast::Cast;
use itertools::Itertools;

type CalibrationValue = usize;

pub fn first(input: &str) -> String {
    sum_of_calibration_values(input, false).to_string()
}

pub fn second(input: &str) -> String {
    sum_of_calibration_values(input, true).to_string()
}

fn sum_of_calibration_values(input: &str, consider_numerals: bool) -> CalibrationValue {
    input
        .lines()
        .map(|line| calibration_value(line, consider_numerals))
        .sum()
}

fn calibration_value(line: &str, consider_numerals: bool) -> CalibrationValue {
    let [mut first_value, mut last_value] = first_and_last_digit(line);
    if consider_numerals {
        if let Some([first_numeral, last_numeral]) = first_and_last_numeral(line) {
            first_value = cmp::min(first_value, first_numeral);
            last_value = cmp::max(last_value, last_numeral);
        }
    }
    first_value.1 * 10 + last_value.1
}

fn first_and_last_digit(line: &str) -> [(usize, CalibrationValue); 2] {
    [str::find, str::rfind].map(|finder| {
        let index = finder(line, char::is_numeric).expect("line should contain a digit");
        (index, (line.as_bytes()[index] - b'0').cast())
    })
}

fn first_and_last_numeral(line: &str) -> Option<[(usize, CalibrationValue); 2]> {
    let numerals = NUMERALS
        .into_iter()
        .flat_map(|(numeral, value)| {
            line.match_indices(numeral)
                .map(move |(index, _)| (index, value))
        })
        .collect_vec();
    (!numerals.is_empty()).then(|| {
        [Iterator::min, Iterator::max].map(|extremum| {
            extremum(numerals.iter().copied()).expect("'numerals' should not be empty")
        })
    })
}

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
        test_on_input(DAY, Puzzle::Second, Input::Example(1), 281 - 83);
    }

    #[test]
    fn second_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 55358);
    }
}
