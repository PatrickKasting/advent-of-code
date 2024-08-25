use std::collections::{BTreeMap, BTreeSet};

type SignalPattern<'input> = &'input str;
type OutputDigit<'input> = &'input str;
type Display<'input> = ([SignalPattern<'input>; 10], [OutputDigit<'input>; 4]);

pub fn first(input: &str) -> String {
    num_output_digits_with_segment_counts(&[2, 3, 4, 7], &parse_input(input)).to_string()
}

pub fn second(input: &str) -> String {
    parse_input(input)
        .into_iter()
        .map(|(signal_patterns, output_digits)| {
            let signal_patterns: [BTreeSet<char>; 10] =
                signal_patterns.map(|patterns| patterns.chars().collect());
            let digits = digits(&signal_patterns);
            let mut output_value = 0;
            for output_digit in output_digits {
                let output_digit: BTreeSet<char> = output_digit.chars().collect();
                output_value = output_value * 10 + digits[&output_digit];
            }
            output_value
        })
        .sum::<usize>()
        .to_string()
}

fn num_output_digits_with_segment_counts(segment_counts: &[usize], displays: &[Display]) -> usize {
    displays
        .iter()
        .map(|(_, output_digits)| {
            output_digits
                .iter()
                .filter(|&digit| segment_counts.contains(&digit.len()))
                .count()
        })
        .sum::<usize>()
}

fn digits(signal_patterns: &[BTreeSet<char>]) -> BTreeMap<&BTreeSet<char>, usize> {
    let mut digits: Vec<&BTreeSet<char>> = signal_patterns.iter().collect();
    for (digit, segment_count) in [(1, 2), (7, 3), (4, 4), (8, 7)] {
        digits[digit] = signal_patterns
            .iter()
            .find(|pattern| pattern.len() == segment_count)
            .expect("signal pattern with given segment count should exist");
    }

    let all_signal_patterns_with_segment_count = |segment_count| {
        signal_patterns
            .iter()
            .filter(|pattern| pattern.len() == segment_count)
            .collect::<Vec<_>>()
    };
    let two_three_five = all_signal_patterns_with_segment_count(5);
    let zero_six_nine = all_signal_patterns_with_segment_count(6);

    digits[6] = find_signal_pattern(&zero_six_nine, |pattern| (pattern - digits[7]).len() == 4);
    digits[5] = find_signal_pattern(&two_three_five, |pattern| (digits[6] - pattern).len() == 1);
    digits[3] = find_signal_pattern(&two_three_five, |pattern| (digits[5] - pattern).len() == 1);
    digits[0] = find_signal_pattern(&zero_six_nine, |pattern| (pattern - digits[5]).len() == 2);
    digits[9] = find_signal_pattern(&zero_six_nine, |pattern| (pattern - digits[3]).len() == 1);
    digits[2] = find_signal_pattern(&two_three_five, |pattern| {
        ![digits[3], digits[5]].contains(&pattern)
    });

    digits
        .into_iter()
        .enumerate()
        .map(|(digit, pattern)| (pattern, digit))
        .collect()
}

fn find_signal_pattern<'patterns>(
    patterns: &[&'patterns BTreeSet<char>],
    predicate: impl Fn(&BTreeSet<char>) -> bool,
) -> &'patterns BTreeSet<char> {
    patterns
        .iter()
        .find(|&pattern| predicate(pattern))
        .expect("exactly one signal patterns should match the pattern")
}

fn parse_input(input: &str) -> Vec<Display> {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> Display {
    let mut tokens = line.split(' ');
    let mut next_token = || {
        tokens
            .next()
            .expect("each input line should contain exactly 15 tokens")
    };
    let mut signal_patterns = [""; 10];
    for signal_pattern in signal_patterns.iter_mut() {
        *signal_pattern = next_token();
    }
    next_token(); // skip delimiter
    let mut output_digits = [""; 4];
    for output_digit in output_digits.iter_mut() {
        *output_digit = next_token();
    }
    (signal_patterns, output_digits)
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 7;

    #[test]
    fn first_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 26);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 534);
    }

    #[test]
    fn second_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 61229);
    }

    #[test]
    fn second_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 1070188);
    }
}
