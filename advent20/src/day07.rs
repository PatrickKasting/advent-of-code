use std::sync::OnceLock;

use ahash::AHashMap;
use regex::Regex;

type Rules<'input> = AHashMap<Color<'input>, Vec<(usize, Color<'input>)>>;
type Rule<'input> = (Color<'input>, Vec<(usize, Color<'input>)>);
type Color<'input> = &'input str;

const MY_BAG_COLOR: Color = "shiny gold";

pub fn first_answer(input: &str) -> String {
    rules(input)
        .iter()
        .filter(|(color, _)| contains_my_bag(&rules(input), color))
        .count()
        .to_string()
}

pub fn second_answer(input: &str) -> String {
    let rules = rules(input);
    number_of_contained_bags(&rules, MY_BAG_COLOR).to_string()
}

fn contains_my_bag(rules: &Rules, color: Color) -> bool {
    rules[color].iter().any(|&(_, contained_color)| {
        contained_color == MY_BAG_COLOR || contains_my_bag(rules, contained_color)
    })
}

fn number_of_contained_bags(rules: &Rules, color: Color) -> usize {
    rules[color]
        .iter()
        .map(|&(amount, contained_color)| {
            amount * (1 + number_of_contained_bags(rules, contained_color))
        })
        .sum()
}

fn rules(input: &str) -> Rules {
    input.lines().map(rule).collect()
}

fn rule(line: &str) -> Rule {
    let (color, contained_bags) = line
        .split_once(" bags contain ")
        .expect("rule should contain ' bags contain '");
    (color, self::contained_bags(contained_bags))
}

fn contained_bags(str: &str) -> Vec<(usize, Color)> {
    static REGEX: OnceLock<Regex> = OnceLock::new();
    let regex = REGEX.get_or_init(|| {
        Regex::new(r"(?<amount>\d+) (?<color>.+?) bags?").expect("regex should be valid")
    });
    let mut contains = vec![];
    for (_, [amount, color]) in regex.captures_iter(str).map(|capture| capture.extract()) {
        let amount = amount.parse().expect("amount should be numeric");
        contains.push((amount, color));
    }
    contains
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 7;

    #[test]
    fn first_answer_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 4);
    }

    #[test]
    fn first_answer_puzzle_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 257);
    }

    #[test]
    fn second_answer_examples() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 32);
        test_on_input(DAY, Puzzle::Second, Input::Example(1), 126);
    }

    #[test]
    fn second_answer_puzzle_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 1038);
    }
}
