use std::{
    collections::{BTreeMap, BTreeSet},
    mem,
};

const AT_LEAST_ONE_ELEMENT: &str = "polymer should contain at least one element";
const ELEMENT_IN_COUNT: &str = "every element should be present in every count";

type Element = char;
type Polymer<'input> = &'input str;
type InsertionRules = BTreeMap<(Element, Element), Element>;
type ElementCount = BTreeMap<Element, usize>;
type InsertionCounts = BTreeMap<(Element, Element), ElementCount>;

pub fn first(input: &str) -> String {
    polymer_score(input, 10).to_string()
}

pub fn second(input: &str) -> String {
    polymer_score(input, 40).to_string()
}

fn parse_input(input: &str) -> (Polymer, InsertionRules) {
    debug_assert!(input.is_ascii());
    let mut lines = input.lines();

    let template = lines
        .next()
        .expect("input should contain a polymer template");

    lines.next().expect("input should contain an empty line");

    let mut insertion_rules = InsertionRules::new();
    for line in lines.map(str::as_bytes) {
        insertion_rules.insert((line[0] as char, line[1] as char), line[6] as char);
    }

    (template, insertion_rules)
}

fn zero_count<'pairs>(pairs: impl Iterator<Item = &'pairs (Element, Element)>) -> ElementCount {
    let elements: BTreeSet<Element> = pairs
        .flat_map(|&(left, right)| [left, right].into_iter())
        .collect();
    elements.into_iter().map(|element| (element, 0)).collect()
}

fn zero_insertion_count(insertion_rules: &InsertionRules) -> InsertionCounts {
    insertion_rules
        .keys()
        .map(|&pair| (pair, zero_count(insertion_rules.keys())))
        .collect()
}

fn sum(left: &ElementCount, right: &ElementCount) -> ElementCount {
    left.iter()
        .map(|(&element, &left_count)| (element, left_count + right[&element]))
        .collect()
}

fn insertion_counts(num_steps: usize, insertion_rules: &InsertionRules) -> InsertionCounts {
    let mut current = zero_insertion_count(insertion_rules);
    let mut next = InsertionCounts::new();
    for _ in 0..num_steps {
        for (&pair @ (left, right), &new) in insertion_rules {
            let mut element_count = sum(&current[&(left, new)], &current[&(new, right)]);
            *element_count.get_mut(&new).expect(ELEMENT_IN_COUNT) += 1;
            next.insert(pair, element_count);
        }
        mem::swap(&mut current, &mut next);
    }
    current
}

fn element_count(insertion_counts: &InsertionCounts, polymer: Polymer) -> ElementCount {
    let mut element_count = zero_count(insertion_counts.keys());
    for element in polymer.chars() {
        *element_count.get_mut(&element).expect(ELEMENT_IN_COUNT) += 1;
    }
    for pair in polymer.as_bytes().windows(2) {
        element_count = sum(
            &element_count,
            &insertion_counts[&(pair[0] as char, pair[1] as char)],
        );
    }
    element_count
}

fn polymer_score(input: &str, num_steps: usize) -> usize {
    let (polymer, insertion_rules) = parse_input(input);
    let insertion_counts = insertion_counts(num_steps, &insertion_rules);
    let element_count = element_count(&insertion_counts, polymer);
    let &max_count = element_count.values().max().expect(AT_LEAST_ONE_ELEMENT);
    let &min_count = element_count.values().min().expect(AT_LEAST_ONE_ELEMENT);
    max_count - min_count
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 14;

    #[test]
    fn first_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 1588);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 3009);
    }

    #[test]
    fn second_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 2188189693529usize);
    }

    #[test]
    fn second_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 3459822539451usize);
    }
}
