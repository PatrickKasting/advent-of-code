use itertools::Itertools;

use crate::strings::parse;

type Calories = usize;

pub fn first(input: &str) -> String {
    elves_in_descending_order(input)[0].to_string()
}

pub fn second(input: &str) -> String {
    elves_in_descending_order(input)[0..3]
        .iter()
        .sum::<Calories>()
        .to_string()
}

fn elves_in_descending_order(str: &str) -> Vec<Calories> {
    let mut elves = str.split("\n\n").map(elf).collect_vec();
    elves.sort_unstable_by_key(|&elf| Calories::MAX - elf);
    elves
}

fn elf(str: &str) -> Calories {
    str.lines().map(parse::<&str, Calories>).sum()
}

#[cfg(test)]
mod tests {
    use super::super::tests::test_on_input;
    use crate::{Input, Puzzle};

    const DAY: usize = 1;

    #[test]
    fn first_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 24000);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 69528);
    }

    #[test]
    fn second_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 45000);
    }

    #[test]
    fn second_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 206_152);
    }
}
