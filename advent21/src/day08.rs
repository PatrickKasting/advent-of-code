use std::collections::{BTreeMap, BTreeSet};

use itertools::Itertools;

type Wiring<'digits> = BTreeMap<&'digits SevenSegmentDigit, usize>;
type Entry = ([SevenSegmentDigit; 10], [SevenSegmentDigit; 4]);
type SevenSegmentDigit = BTreeSet<char>;
type Number = usize;

pub fn first_answer(input: &str) -> String {
    let entries = entries(input);
    entries
        .flat_map(|(_, output)| output)
        .filter(is_one_seven_four_or_eight)
        .count()
        .to_string()
}

pub fn second_answer(input: &str) -> String {
    let entries = entries(input);
    entries
        .map(|entry| output_value(&entry))
        .sum::<Number>()
        .to_string()
}

fn is_one_seven_four_or_eight(digit: &SevenSegmentDigit) -> bool {
    [2, 3, 4, 7].contains(&digit.len())
}

fn output_value((signal_patterns, outputs): &Entry) -> Number {
    let wiring = wiring(signal_patterns);

    let mut number = 0;
    for digit in outputs {
        number *= 10;
        number += wiring[digit];
    }
    number
}

fn wiring(signal_patterns: &[SevenSegmentDigit; 10]) -> Wiring {
    let [one, seven, four, eight] = [2, 3, 4, 7].map(|number_of_segments| {
        digit_satisfying(signal_patterns.each_ref().each_ref(), |digit| {
            digit.len() == number_of_segments
        })
    });
    let [two_three_five, zero_six_nine] = [5, 6].map(|number_of_segments| {
        digits_with_number_of_segments(signal_patterns, number_of_segments)
    });

    let nine = digit_satisfying(&zero_six_nine, |digit| digit.difference(four).count() == 2);
    let two = digit_satisfying(&two_three_five, |digit| digit.difference(nine).count() == 1);
    let five = digit_satisfying(&two_three_five, |digit| digit.difference(two).count() == 2);
    let zero = digit_satisfying(&zero_six_nine, |digit| digit.difference(five).count() == 2);
    let three = digit_satisfying(&two_three_five, |digit| ![two, five].contains(&digit));
    let six = digit_satisfying(&zero_six_nine, |digit| ![zero, nine].contains(&digit));

    [zero, one, two, three, four, five, six, seven, eight, nine]
        .into_iter()
        .zip_eq(0..=9)
        .collect()
}

fn digit_satisfying<'collection, 'digits: 'collection>(
    digits: impl IntoIterator<Item = &'collection &'digits SevenSegmentDigit>,
    predicate: impl Fn(&SevenSegmentDigit) -> bool,
) -> &'digits SevenSegmentDigit {
    digits
        .into_iter()
        .find(|digit| predicate(digit))
        .expect("digit with number of segments should exist")
}

fn digits_with_number_of_segments(
    digits: &[SevenSegmentDigit],
    number_of_segments: usize,
) -> Vec<&SevenSegmentDigit> {
    digits
        .iter()
        .filter(|digit| digit.len() == number_of_segments)
        .collect_vec()
}

fn entries(input: &str) -> impl Iterator<Item = Entry> + '_ {
    input.lines().map(entry)
}

fn entry(line: &str) -> Entry {
    let (signal_patterns, outputs) = line
        .split_once(" | ")
        .expect("every line should contain a pipe separating signal patterns and outputs");
    let signal_patterns = signal_patterns
        .split_whitespace()
        .map(seven_segment_digit)
        .collect_vec()
        .try_into()
        .expect("line should contain 10 signal patterns");
    let outputs = outputs
        .split_whitespace()
        .map(seven_segment_digit)
        .collect_vec()
        .try_into()
        .expect("line should contain four outputs");
    (signal_patterns, outputs)
}

fn seven_segment_digit(str: &str) -> SevenSegmentDigit {
    str.chars().collect()
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 8;

    #[test]
    fn first_answer_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 26);
    }

    #[test]
    fn first_answer_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 534);
    }

    #[test]
    fn second_examples() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 61229);
    }

    #[test]
    fn second_answer_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 1_070_188);
    }
}
