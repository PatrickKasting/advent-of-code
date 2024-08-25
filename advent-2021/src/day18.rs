use std::{cmp::max, fmt::Display};

type Number = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum FishNumberToken {
    Open,
    Close,
    Number(Number),
}

impl Display for FishNumberToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FishNumberToken::Open => write!(f, "["),
            FishNumberToken::Close => write!(f, "]"),
            FishNumberToken::Number(number) => write!(f, "{number}"),
        }
    }
}

type FishNumber = Vec<FishNumberToken>;

pub fn first(input: &str) -> String {
    let sum = input
        .lines()
        .map(parse_fish_number)
        .reduce(|sum, addend| add(&sum, &addend))
        .expect("at least one fish number should be present");
    magnitude(&sum).to_string()
}

pub fn second(input: &str) -> String {
    let fish_numbers: Vec<FishNumber> = input.lines().map(parse_fish_number).collect();
    largest_magnitude_of_two_term_sum(&fish_numbers).to_string()
}

fn parse_fish_number(fish_number: &str) -> FishNumber {
    fish_number
        .chars()
        .filter(|&char| char != ',')
        .map(|char| match char {
            '[' => FishNumberToken::Open,
            ']' => FishNumberToken::Close,
            '0'..='9' => FishNumberToken::Number(
                char.to_digit(10)
                    .expect("only digits should be converted to numbers") as usize,
            ),
            _ => panic!("input should only contain brackets, commas, digits, and not {char:?}"),
        })
        .collect()
}

type Index = usize;

enum Reduction {
    Explotion(Index),
    Split(Index),
}

fn reduction(fish_number: &[FishNumberToken]) -> Option<Reduction> {
    let mut depth = 0;
    let mut large_number_index = None;
    for (index, &token) in fish_number.iter().enumerate() {
        match token {
            FishNumberToken::Open => {
                if depth == 4 {
                    return Some(Reduction::Explotion(index));
                }
                depth += 1;
            }
            FishNumberToken::Close => {
                depth -= 1;
            }
            FishNumberToken::Number(number) => {
                if large_number_index.is_none() && number >= 10 {
                    large_number_index = Some(Reduction::Split(index));
                }
            }
        }
    }
    large_number_index
}

fn add_to_first_number<'tokens>(
    tokens: impl IntoIterator<Item = &'tokens mut FishNumberToken>,
    addend: Number,
) {
    for token in tokens {
        if let FishNumberToken::Number(number) = token {
            *number += addend;
            return;
        }
    }
}

fn explode(fish_number: &mut FishNumber, index: usize) {
    let FishNumberToken::Number(left) = fish_number[index + 1] else {
        panic!("first token after an open bracket should be the left number of a pair");
    };
    add_to_first_number(fish_number[..index].iter_mut().rev(), left);

    let FishNumberToken::Number(right) = fish_number[index + 2] else {
        panic!("second token after an open bracket should be the right number of a pair");
    };
    add_to_first_number(fish_number[index + 4..].iter_mut(), right);

    fish_number.splice(index..index + 4, [FishNumberToken::Number(0)]);
}

fn split(fish_number: &mut FishNumber, index: usize) {
    let FishNumberToken::Number(number) = fish_number[index] else {
        panic!("only numbers should be split");
    };
    let (left, right) = (number / 2, (number + 1) / 2);
    debug_assert_eq!(left + right, number);
    let pair = [
        FishNumberToken::Open,
        FishNumberToken::Number(left),
        FishNumberToken::Number(right),
        FishNumberToken::Close,
    ];
    fish_number.splice(index..index + 1, pair);
}

fn reduce(fish_number: &mut FishNumber) {
    while let Some(reduction) = reduction(fish_number) {
        match reduction {
            Reduction::Explotion(index) => explode(fish_number, index),
            Reduction::Split(index) => split(fish_number, index),
        }
    }
}

fn add(left: &[FishNumberToken], right: &[FishNumberToken]) -> FishNumber {
    let mut sum = Vec::from([FishNumberToken::Open]);
    sum.extend_from_slice(left);
    sum.extend_from_slice(right);
    sum.push(FishNumberToken::Close);
    reduce(&mut sum);
    sum
}

fn magnitude(tokens: &[FishNumberToken]) -> Number {
    fn magnitude(tokens: &[FishNumberToken]) -> (&[FishNumberToken], Number) {
        match tokens[0] {
            FishNumberToken::Open => {
                let (remaining, left) = magnitude(&tokens[1..]);
                let (remaining, right) = magnitude(remaining);
                (&remaining[1..], 3 * left + 2 * right)
            }
            FishNumberToken::Number(number) => (&tokens[1..], number),
            FishNumberToken::Close => {
                panic!("close bracket should not be the first token of a fish number")
            }
        }
    }

    let (remaining, magnitude) = magnitude(tokens);
    debug_assert!(remaining.is_empty());
    magnitude
}

fn largest_magnitude_of_two_term_sum(fish_numbers: &[FishNumber]) -> Number {
    let mut largest_magnitude = Number::MIN;
    for left_index in 0..fish_numbers.len() {
        for right_index in 0..fish_numbers.len() {
            if left_index == right_index {
                continue;
            }
            largest_magnitude = max(
                largest_magnitude,
                magnitude(&add(&fish_numbers[left_index], &fish_numbers[right_index])),
            );
        }
    }
    largest_magnitude
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use super::*;
    use crate::tests::{input, test_on_input};

    const DAY: usize = 18;

    #[test]
    fn first_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 4140);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 2501);
    }

    #[test]
    fn second_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 3993);
    }

    #[test]
    fn second_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 4935);
    }

    #[test]
    fn parse() {
        let numbers = input(DAY, Input::PuzzleInput);
        for number in numbers.lines() {
            assert_eq!(display_fish_number(&parse_fish_number(number)), number);
        }
    }

    fn display_fish_number(fish_number: &[FishNumberToken]) -> String {
        let mut result = String::new();
        for index in 0..fish_number.len() {
            let current_token = fish_number[index];
            result.push_str(&current_token.to_string());
            if current_token != FishNumberToken::Open {
                if let Some(&token) = fish_number.get(index + 1) {
                    if token != FishNumberToken::Close {
                        result.push(',');
                    }
                }
            }
        }
        result
    }
}
