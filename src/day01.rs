use itertools::Itertools;
use regex::Regex;

use crate::utilities::number;

type CalibrationValue = u32;
type DigitPattern<'pattern> = (&'pattern str, fn(&str) -> CalibrationValue);

fn calibration_value(
    digit_patterns: &[(Regex, impl Fn(&str) -> CalibrationValue)],
    line: &str,
) -> CalibrationValue {
    let first_and_last_matches = digit_patterns
        .iter()
        .flat_map(|(pattern, value)| {
            let matches = pattern.find_iter(line).collect_vec();
            if matches.is_empty() {
                None
            } else {
                let first_and_last_match = [matches[0], matches[matches.len() - 1]];
                Some(first_and_last_match.map(|m| (m.start(), value(m.as_str()))))
            }
        })
        .collect_vec();

    let first_digit = first_and_last_matches
        .iter()
        .map(|first_and_last_match| first_and_last_match[0])
        .min()
        .expect("every line should contain at least one digit")
        .1;
    let last_digit = first_and_last_matches
        .iter()
        .map(|first_and_last_match| first_and_last_match[1])
        .max()
        .expect("every line should contain at least one digit")
        .1;

    first_digit * 10 + last_digit
}

fn sum_of_calibration_values<'word>(
    digit_patterns: impl IntoIterator<Item = (&'word str, fn(&str) -> CalibrationValue)>,
    input: &str,
) -> CalibrationValue {
    let digit_patterns = digit_patterns
        .into_iter()
        .map(|(word, value)| (Regex::new(word).expect("regex should be valid"), value))
        .collect_vec();
    input
        .lines()
        .map(|line| calibration_value(&digit_patterns, line))
        .sum()
}

pub fn first(input: String) -> String {
    let digit_patterns: [DigitPattern; 1] = [(r"\d", |str| number(str))];
    sum_of_calibration_values(digit_patterns, &input).to_string()
}

fn one(_: &str) -> CalibrationValue {
    1
}

fn two(_: &str) -> CalibrationValue {
    2
}

fn three(_: &str) -> CalibrationValue {
    3
}

fn four(_: &str) -> CalibrationValue {
    4
}

fn five(_: &str) -> CalibrationValue {
    5
}

fn six(_: &str) -> CalibrationValue {
    6
}

fn seven(_: &str) -> CalibrationValue {
    7
}

fn eight(_: &str) -> CalibrationValue {
    8
}

fn nine(_: &str) -> CalibrationValue {
    9
}

pub fn second(input: String) -> String {
    let digit_patterns: [DigitPattern; 10] = [
        (r"\d", |str| number(str)),
        ("one", one),
        ("two", two),
        ("three", three),
        ("four", four),
        ("five", five),
        ("six", six),
        ("seven", seven),
        ("eight", eight),
        ("nine", nine),
    ];
    sum_of_calibration_values(digit_patterns, &input).to_string()
}

#[cfg(test)]
mod tests {
    use crate::{tests::*, Input, Puzzle};

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
