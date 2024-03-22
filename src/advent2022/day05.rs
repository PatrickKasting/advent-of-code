use regex::Regex;

use crate::strings::{matches, parse};

type Crate = char;
type Stack = Vec<Crate>;
type Stacks = Vec<Stack>;
type ProcedureStep = (usize, usize, usize);

pub fn first(input: &str) -> String {
    let (mut stacks, procedure) = stacks_and_procedure(input);
    rearrange(true, &mut stacks, procedure);
    message(&stacks)
}

pub fn second(input: &str) -> String {
    let (mut stacks, procedure) = stacks_and_procedure(input);
    rearrange(false, &mut stacks, procedure);
    message(&stacks)
}

fn message(stacks: &Stacks) -> String {
    stacks
        .iter()
        .map(|stack| stack.last().expect("no stack should be empty"))
        .collect()
}

fn rearrange(
    one_by_one: bool,
    stacks: &mut Stacks,
    procedure: impl Iterator<Item = ProcedureStep>,
) {
    for (amount, from, to) in procedure {
        for _ in 0..amount {
            let package = stacks[from - 1].pop().expect("stack should not be empty");
            stacks[to - 1].push(package);
        }
        if !one_by_one {
            let new_packages = stacks[to - 1].len() - amount..;
            stacks[to - 1][new_packages].reverse();
        }
    }
}

fn stacks_and_procedure(input: &str) -> (Stacks, impl Iterator<Item = ProcedureStep> + '_) {
    let (stacks, procedure) = input
        .split_once("\n\n")
        .expect("stacks and procedure should be separated by a blank line");
    (self::stacks(stacks), self::procedure(procedure))
}

fn stacks(str: &str) -> Stacks {
    let mut stacks = Stacks::new();
    for line in str.lines() {
        let packages = line
            .chars()
            .enumerate()
            .filter(|(_, char)| char.is_ascii_uppercase());
        for (index, char) in packages {
            let stack_index = index / 4;
            if stack_index >= stacks.len() {
                stacks.resize_with(stack_index + 1, Vec::new);
            }
            stacks[stack_index].push(char);
        }
    }
    stacks.iter_mut().for_each(|stack| stack.reverse());
    stacks
}

fn procedure(str: &str) -> impl Iterator<Item = ProcedureStep> + '_ {
    let number_regex = Regex::new(r"\d+").expect("regex should be valid");
    str.lines().map(move |line| {
        let mut numbers = matches(&number_regex, line).map(parse);
        let amount = numbers.next().expect("line should contain amount");
        let from = numbers.next().expect("line should contain source stack");
        let to = numbers.next().expect("line should contain target stack");
        (amount, from, to)
    })
}

#[cfg(test)]
mod tests {
    use super::super::tests::test_on_input;
    use crate::{Input, Puzzle};

    const DAY: usize = 5;

    #[test]
    fn first_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), "CMZ");
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, "FCVRLMVQP");
    }

    #[test]
    fn second_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), "MCD");
    }

    #[test]
    fn second_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, "RWLWGJGFD");
    }
}
