use ahash::AHashMap;
use itertools::Itertools;

type Rules<'input> = AHashMap<Index, Pattern<'input>>;
type Pattern<'input> = &'input str;
type Message<'input> = &'input str;
type Index = usize;

pub fn first_answer(input: &str) -> String {
    let (rules, messages) = rules_and_messages(input);
    messages
        .filter(|message| completely_matches_rule_zero(&rules, message))
        .count()
        .to_string()
}

pub fn second_answer(input: &str) -> String {
    let (mut rules, messages) = rules_and_messages(input);
    rules.extend([(8, "42 | 42 8"), (11, "42 31 | 42 11 31")]);
    messages
        .filter(|message| completely_matches_rule_zero(&rules, message))
        .count()
        .to_string()
}

fn completely_matches_rule_zero(rules: &Rules, message: Message) -> bool {
    matches_rule(rules, 0, message).contains(&"")
}

fn matches_rule<'input>(
    rules: &'input Rules,
    index: usize,
    message: Message<'input>,
) -> Vec<Message<'input>> {
    let rule = rules[&index];
    if &rule[0..1] == r#"""# {
        return Vec::from_iter(message.strip_prefix(&rule[1..2]));
    }
    rule.split(" | ")
        .flat_map(|branch| matches_sequence(rules, branch, message))
        .collect_vec()
}

fn matches_sequence<'input>(
    rules: &'input Rules,
    sequence: Pattern<'input>,
    message: Message<'input>,
) -> Vec<Message<'input>> {
    sequence
        .split_whitespace()
        .fold(vec![message], |remaining, index| {
            let index = index.parse().expect("index should be numeric");
            remaining
                .into_iter()
                .flat_map(|remaining| matches_rule(rules, index, remaining))
                .collect_vec()
        })
}

fn rules_and_messages(input: &str) -> (Rules, impl Iterator<Item = Message>) {
    let (rules, messages) = input
        .split_once("\n\n")
        .expect("rules and messages should be separated by a blank line");
    (self::rules(rules), messages.lines())
}

fn rules(str: &str) -> Rules {
    str.lines().map(rule).collect()
}

fn rule(line: &str) -> (usize, Pattern) {
    let (index, pattern) = line
        .split_once(": ")
        .expect("index and pattern should be separated by a colon");
    let index = index.parse().expect("index should be numeric");
    (index, pattern)
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 19;

    #[test]
    fn first_answer_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 2);
    }

    #[test]
    fn first_answer_puzzle_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 124);
    }

    #[test]
    fn second_answer_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(1), 12);
    }

    #[test]
    fn second_answer_puzzle_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 228);
    }
}
