use std::ops::RangeInclusive;

use shared::string::usizes;

type Password<'input> = &'input [Character];
type Policy = (RangeInclusive<usize>, Character);
type Character = u8;

pub fn first_answer(input: &str) -> String {
    let policies_and_passwords = policies_and_passwords(input);
    policies_and_passwords
        .filter(|(policy, password)| is_password_valid_based_on_occurances(policy, password))
        .count()
        .to_string()
}

pub fn second_answer(input: &str) -> String {
    let policies_and_passwords = policies_and_passwords(input);
    policies_and_passwords
        .filter(|(policy, password)| is_password_valid_based_on_indices(policy, password))
        .count()
        .to_string()
}

fn is_password_valid_based_on_occurances((range, character): &Policy, password: Password) -> bool {
    let occurances = bytecount::count(password, *character);
    range.contains(&occurances)
}

fn is_password_valid_based_on_indices((range, character): &Policy, password: Password) -> bool {
    let [first, second] = [range.start(), range.end()].map(|&index| password[index - 1]);
    (first == *character) != (second == *character)
}

fn policies_and_passwords(input: &str) -> impl Iterator<Item = (Policy, Password)> {
    input.lines().map(policy_and_password)
}

fn policy_and_password(line: &str) -> (Policy, Password) {
    let (policy, password) = line
        .split_once(": ")
        .expect("policy and password should be separated by a colon");
    (self::policy(policy), password.as_bytes())
}

fn policy(policy: &str) -> Policy {
    let (range, character) = policy
        .split_once(' ')
        .expect("range and character should be separated by a space");
    let range = usizes(range);
    (range[0]..=range[1], character.as_bytes()[0])
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 2;

    #[test]
    fn first_answer_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 2);
    }

    #[test]
    fn first_answer_puzzle_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 591);
    }

    #[test]
    fn second_answer_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 1);
    }

    #[test]
    fn second_answer_puzzle_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 335);
    }
}
