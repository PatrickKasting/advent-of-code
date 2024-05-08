use itertools::Itertools;

use crate::{math::sets::intersection, HashSet};

type Item = char;
type Set = HashSet<Item>;
type Priority = u32;

pub fn first(input: &str) -> String {
    let compartments = compartments(input);
    sum_of_priorities(compartments).to_string()
}

pub fn second(input: &str) -> String {
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
        groups.push([first, second, third].map(|line| line.chars().collect::<HashSet<char>>()));
    }
    groups
}

#[cfg(test)]
mod tests {
    use super::super::tests::test_on_input;
    use crate::{Input, Puzzle};

    const DAY: usize = 3;

    #[test]
    fn first_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 157);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 7763);
    }

    #[test]
    fn second_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 70);
    }

    #[test]
    fn second_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 2569);
    }
}
