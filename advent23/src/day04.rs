use ahash::AHashSet;

type Score = usize;
type Numbers = AHashSet<Number>;
type Number = usize;

pub fn first_answer(input: &str) -> String {
    total_score_of_original_scratchcards(input).to_string()
}

pub fn second_answer(input: &str) -> String {
    final_number_of_scratchcards(input).to_string()
}

fn total_score_of_original_scratchcards(input: &str) -> Score {
    input.lines().map(scratchcard_score).sum()
}

fn scratchcard_score(scratchcard: &str) -> Score {
    match number_of_matches(scratchcard) {
        0 => 0,
        number_of_matches => {
            let number_of_matches = u32::try_from(number_of_matches)
                .expect("number of matches should be less than 'u32::MAX'");
            2_usize.pow(number_of_matches - 1)
        }
    }
}

fn number_of_matches(scratchcard: &str) -> usize {
    let (winning, yours) = scratchcard_numbers(scratchcard);
    winning.intersection(&yours).count()
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

fn numbers_from_list(list: &str) -> Numbers {
    let number = |str: &str| str.parse().expect("number should be numerical");
    list.split_whitespace().map(number).collect()
}

fn final_number_of_scratchcards(input: &str) -> usize {
    let mut number_of_copies = vec![1_usize; input.lines().count()];
    for (index, scratchcard) in input.lines().enumerate() {
        let number_of_copies_of_current_scratchcard = number_of_copies[index];
        let range_of_scratchcards_won = index + 1..index + 1 + number_of_matches(scratchcard);
        for number_of_copies in &mut number_of_copies[range_of_scratchcards_won] {
            *number_of_copies += number_of_copies_of_current_scratchcard;
        }
    }
    number_of_copies.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 4;

    #[test]
    fn first_answer_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 13);
    }

    #[test]
    fn first_answer_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 15205);
    }

    #[test]
    fn second_answer_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 30);
    }

    #[test]
    fn second_answer_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 6_189_740);
    }
}
