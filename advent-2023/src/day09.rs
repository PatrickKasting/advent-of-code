use std::ops::{Add, Sub};

use itertools::Itertools;

use shared::string::isizes;

type Combination = fn(Number, Number) -> Number;
type Prediction = fn(Number, Number) -> Number;
type Number = isize;

pub fn first(input: &str) -> String {
    sum_of_predictions(input, false).to_string()
}

pub fn second(input: &str) -> String {
    sum_of_predictions(input, true).to_string()
}

fn sum_of_predictions(input: &str, reverse: bool) -> Number {
    input
        .lines()
        .map(|history| prediction(history, reverse))
        .sum()
}

fn prediction(history: &str, reverse: bool) -> Number {
    let mut history = isizes(history);
    if reverse {
        history.reverse();
    }
    if reverse {
        extrapolation(&history, Number::sub, Number::sub)
    } else {
        extrapolation(&history, |left, right| right - left, Number::add)
    }
}

fn extrapolation(history: &[Number], combination: Combination, prediction: Prediction) -> Number {
    if history.iter().all(|&number| number == 0) {
        return 0;
    }
    let &last_number = history.last().expect("history should not be empty");
    let succeeding_row = row(history, combination);
    let succeeding_prediction = extrapolation(&succeeding_row, combination, prediction);
    prediction(last_number, succeeding_prediction)
}

fn row(preceding_row: &[Number], combination: Combination) -> Vec<Number> {
    preceding_row
        .windows(2)
        .map(|pair| combination(pair[0], pair[1]))
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 9;

    #[test]
    fn first_examples() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 114);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 1_995_001_648);
    }

    #[test]
    fn second_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 2);
    }

    #[test]
    fn second_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 988);
    }
}
