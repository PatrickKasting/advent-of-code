use ahash::AHashSet;

type Person = AHashSet<Question>;
type Question = char;

pub fn first_answer(input: &str) -> String {
    groups(input)
        .map(number_of_questions_with_at_least_one_yes)
        .sum::<usize>()
        .to_string()
}

pub fn second_answer(input: &str) -> String {
    groups(input)
        .map(number_of_questions_with_all_yes)
        .sum::<usize>()
        .to_string()
}

fn number_of_questions_with_at_least_one_yes(group: impl Iterator<Item = Person>) -> usize {
    group
        .reduce(|questions, person| questions.union(&person).copied().collect())
        .expect("group should contain at least one person")
        .len()
}

fn number_of_questions_with_all_yes(group: impl Iterator<Item = Person>) -> usize {
    group
        .reduce(|questions, person| questions.intersection(&person).copied().collect())
        .expect("group should contain at least one person")
        .len()
}

fn groups(input: &str) -> impl Iterator<Item = impl Iterator<Item = Person> + '_> + '_ {
    input.split("\n\n").map(group)
}

fn group(str: &str) -> impl Iterator<Item = Person> + '_ {
    str.lines().map(person)
}

fn person(line: &str) -> Person {
    line.chars().collect()
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 6;

    #[test]
    fn first_answer_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 11);
    }

    #[test]
    fn first_answer_puzzle_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 6530);
    }

    #[test]
    fn second_answer_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 6);
    }

    #[test]
    fn second_answer_puzzle_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 3323);
    }
}
