use std::ops::{Add, Sub};

use itertools::Itertools;

use crate::utilities::number;

type Number = isize;

#[derive(Debug, Clone, Copy)]
struct Extrapolation {
    combination: fn(Number, Number) -> Number,
    prediction: fn(Number, Number) -> Number,
}

fn history(history: &str) -> Vec<Number> {
    history.split_ascii_whitespace().map(number).collect_vec()
}

fn row(preceding_row: Vec<Number>, combination: fn(Number, Number) -> Number) -> Vec<Number> {
    preceding_row
        .windows(2)
        .map(|pair| combination(pair[0], pair[1]))
        .collect_vec()
}

fn extrapolation(history: Vec<Number>, extrapolation: Extrapolation) -> Number {
    if history.iter().all(|&number| number == 0) {
        return 0;
    }
    let &last_number = history.last().expect("history should not be empty");
    let succeeding_row = row(history, extrapolation.combination);
    let succeeding_prediction = self::extrapolation(succeeding_row, extrapolation);
    (extrapolation.prediction)(last_number, succeeding_prediction)
}

fn prediction(history: &str, reverse: bool) -> Number {
    let mut history = self::history(history);
    if reverse {
        history.reverse();
    }

    let extrapolation = if reverse {
        Extrapolation {
            combination: Number::sub,
            prediction: Number::sub,
        }
    } else {
        Extrapolation {
            combination: |left, right| right - left,
            prediction: Number::add,
        }
    };

    self::extrapolation(history, extrapolation)
}

fn sum_of_predictions(input: &str, reverse: bool) -> Number {
    input
        .lines()
        .map(|history| prediction(history, reverse))
        .sum()
}

pub fn first(input: String) -> String {
    sum_of_predictions(&input, false).to_string()
}

pub fn second(input: String) -> String {
    sum_of_predictions(&input, true).to_string()
}

#[cfg(test)]
mod tests {
    use crate::{tests::*, Input, Puzzle};

    const DAY: usize = 9;

    #[test]
    fn first_examples() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 114);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::Real, 1995001648);
    }

    #[test]
    fn second_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 2);
    }

    #[test]
    fn second_input() {
        test_on_input(DAY, Puzzle::Second, Input::Real, 988);
    }
}
