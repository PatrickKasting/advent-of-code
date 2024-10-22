use ahash::AHashSet;
use itertools::Itertools;

type Set = AHashSet<Item>;
type Item = char;
type Priority = u32;

pub fn first_answer(input: &str) -> String {
    let compartments = compartments(input);
    sum_of_priorities(compartments).to_string()
}

pub fn second_answer(input: &str) -> String {
    let groups = groups(input);
    sum_of_priorities(groups).to_string()
}

fn sum_of_priorities(sets: impl IntoIterator<Item = impl IntoIterator<Item = Set>>) -> Priority {
    sets.into_iter().map(common_item).map(priority).sum()
}

fn priority(item: Item) -> Priority {
    match item {
        'a'..='z' => item as Priority - 'a' as Priority + 1,
        'A'..='Z' => item as Priority - 'A' as Priority + 27,
        _ => panic!("item should be represented by a letter"),
    }
}

fn common_item(sets: impl IntoIterator<Item = Set>) -> Item {
    intersection(sets)
        .into_iter()
        .exactly_one()
        .expect("only one item should be in common")
}

fn intersection(sets: impl IntoIterator<Item = Set>) -> Set {
    let mut sets = sets.into_iter();
    let mut intersection = sets
        .next()
        .expect("intersection should not be computed from empty collections of sets");
    for set in sets {
        intersection.retain(|element| set.contains(element));
    }
    intersection
}

fn compartments(input: &str) -> impl Iterator<Item = [Set; 2]> + '_ {
    input.lines().map(|line| {
        let number_of_items = line.chars().count();
        let split = number_of_items / 2;
        [0..split, split..number_of_items].map(|range| line[range].chars().collect())
    })
}

fn groups(input: &str) -> Vec<[Set; 3]> {
    let mut lines = input.lines();
    let mut groups = vec![];
    while let Some(first) = lines.next() {
        let second = lines.next().expect("lines should come in triplets");
        let third = lines.next().expect("lines should come in triplets");
        groups.push([first, second, third].map(|line| line.chars().collect::<AHashSet<char>>()));
    }
    groups
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 3;

    #[test]
    fn first_answer_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 157);
    }

    #[test]
    fn first_answer_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 7763);
    }

    #[test]
    fn second_answer_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 70);
    }

    #[test]
    fn second_answer_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 2569);
    }
}
