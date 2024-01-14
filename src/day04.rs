use std::collections::HashSet;

use crate::utilities::number;

type Numbers = HashSet<usize>;

fn numbers_from_list(list: &str) -> Numbers {
    list.split_ascii_whitespace().map(number).collect()
}

fn scratchcard_numbers(scratchcard: &str) -> (Numbers, Numbers) {
    let (_, numbers) = scratchcard
        .split_once(':')
        .expect("every line should contain a colon");
    let (winning, yours) = numbers
        .split_once('|')
        .expect("every line should contain a bar");
    (numbers_from_list(winning), numbers_from_list(yours))
}

fn number_of_matches(scratchcard: &str) -> usize {
    let (winning, yours) = scratchcard_numbers(scratchcard);
    winning.intersection(&yours).count()
}

fn scratchcard_score(scratchcard: &str) -> usize {
    match number_of_matches(scratchcard) {
        0 => 0,
        number_of_matches => 2usize.pow(number_of_matches as u32 - 1),
    }
}

pub fn first(input: &str) -> String {
    input
        .lines()
        .map(scratchcard_score)
        .sum::<usize>()
        .to_string()
}

pub fn second(input: &str) -> String {
    let mut number_of_copies = vec![1usize; input.lines().count()];
    for (index, scratchcard) in input.lines().enumerate() {
        let number_of_copies_of_current_scratchcard = number_of_copies[index];
        let range_of_scratchcards_won = index + 1..index + 1 + number_of_matches(scratchcard);
        for number_of_copies in &mut number_of_copies[range_of_scratchcards_won] {
            *number_of_copies += number_of_copies_of_current_scratchcard;
        }
    }
    number_of_copies.into_iter().sum::<usize>().to_string()
}

#[cfg(test)]
mod tests {
    use crate::{tests::*, Input, Puzzle};

    const DAY: usize = 4;

    #[test]
    fn first_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 13);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 15205);
    }

    #[test]
    fn second_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 30);
    }

    #[test]
    fn second_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 6189740);
    }
}
