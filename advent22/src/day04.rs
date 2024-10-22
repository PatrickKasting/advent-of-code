use std::ops::RangeInclusive;

use shared::string::usizes;

type Pair = [Sections; 2];
type Sections = RangeInclusive<IdNumber>;
type IdNumber = usize;

pub fn first_answer(input: &str) -> String {
    number_of_pairs(input, contains).to_string()
}

pub fn second_answer(input: &str) -> String {
    number_of_pairs(input, overlaps).to_string()
}

fn number_of_pairs(input: &str, predicate: fn(Pair) -> bool) -> usize {
    input.lines().filter(|line| predicate(pair(line))).count()
}

fn contains([left, right]: Pair) -> bool {
    outer_contains_inner(left.clone(), right.clone()) || outer_contains_inner(right, left)
}

fn outer_contains_inner(outer: Sections, inner: Sections) -> bool {
    outer.contains(inner.start()) && outer.contains(inner.end())
}

fn overlaps([left, right]: Pair) -> bool {
    left.contains(right.start())
        || left.contains(right.end())
        || right.contains(left.start())
        || right.contains(left.end())
}

fn pair(line: &str) -> Pair {
    let (left, right) = line
        .split_once(',')
        .expect("sections should be delimited by a comma");
    [sections(left), sections(right)]
}

fn sections(str: &str) -> Sections {
    let ids = usizes(str);
    ids[0]..=ids[1]
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 4;

    #[test]
    fn first_answer_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 2);
    }

    #[test]
    fn first_answer_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 515);
    }

    #[test]
    fn second_answer_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 4);
    }

    #[test]
    fn second_answer_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 883);
    }
}
