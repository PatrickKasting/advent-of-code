use std::collections::HashMap;

use itertools::Itertools;

type PolymerTemplate<'input> = &'input [Element];
type PairInsertionRules = HashMap<Pair, Element>;
type PairCounts = HashMap<Pair, usize>;
type Pair = [Element; 2];
type Element = u8;

pub fn first_answer(input: &str) -> String {
    let [minimum_count, maximum_count] = minimum_and_maximum_element_counts(input, 10);
    (maximum_count - minimum_count).to_string()
}

pub fn second_answer(input: &str) -> String {
    let [minimum_count, maximum_count] = minimum_and_maximum_element_counts(input, 40);
    (maximum_count - minimum_count).to_string()
}

fn minimum_and_maximum_element_counts(input: &str, number_of_steps: usize) -> [usize; 2] {
    let (template, rules) = polymer_template_and_pair_insertion_rules(input);
    let element_counts = elements_counts(template, &rules, number_of_steps);
    let (&min_count, &max_count) = element_counts
        .values()
        .minmax()
        .into_option()
        .expect("at least one element should have been counted");
    [min_count, max_count]
}

fn elements_counts(
    template: PolymerTemplate,
    rules: &PairInsertionRules,
    number_of_steps: usize,
) -> HashMap<Element, usize> {
    let &last_element = template.last().expect("template should not be empty");

    let pair_counts = pair_counts(template, rules, number_of_steps);
    let mut element_counts = HashMap::new();
    for ([left, _], count) in pair_counts {
        *element_counts.entry(left).or_default() += count;
    }
    *element_counts.entry(last_element).or_default() += 1;
    element_counts
}

fn pair_counts(
    template: PolymerTemplate,
    rules: &PairInsertionRules,
    number_of_steps: usize,
) -> PairCounts {
    let mut counts = template.windows(2).map(|pair| [pair[0], pair[1]]).counts();
    for _ in 0..number_of_steps {
        counts = pair_counts_after_one_step(rules, counts);
    }
    counts
}

fn pair_counts_after_one_step(rules: &PairInsertionRules, pair_counts: PairCounts) -> PairCounts {
    let mut result = PairCounts::default();
    for (pair @ [left, right], count) in pair_counts {
        let middle = rules[&pair];
        *result.entry([left, middle]).or_default() += count;
        *result.entry([middle, right]).or_default() += count;
    }
    result
}

fn polymer_template_and_pair_insertion_rules(input: &str) -> (PolymerTemplate, PairInsertionRules) {
    let (template, rules) = input
        .split_once("\n\n")
        .expect("template and rules should be separated by an empty line");
    (template.as_bytes(), pair_insertion_rules(rules))
}

fn pair_insertion_rules(str: &str) -> PairInsertionRules {
    str.lines()
        .map(|line| {
            let bytes = line.as_bytes();
            ([bytes[0], bytes[1]], bytes[6])
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 14;

    #[test]
    fn first_answer_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 1588);
    }

    #[test]
    fn first_answer_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 3009);
    }

    #[test]
    fn second_answer_example() {
        test_on_input(
            DAY,
            Puzzle::Second,
            Input::Example(0),
            2_188_189_693_529_usize,
        );
    }

    #[test]
    fn second_answer_input() {
        test_on_input(
            DAY,
            Puzzle::Second,
            Input::PuzzleInput,
            3_459_822_539_451_usize,
        );
    }
}
