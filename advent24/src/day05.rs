use std::vec::Vec;

use ahash::AHashMap;
use itertools::Itertools;
use shared::{graph::topological_sort, string::usizes};

type PageOrderingRule = [Page; 2];
type Update = Vec<Page>;
type Page = usize;

pub fn first_answer(input: &str) -> String {
    let (rules, updates) = page_ordering_rules_and_updates(input);
    updates
        .into_iter()
        .filter(|update| is_correctly_ordered(&rules, update))
        .map(|update| middle_page_number(&update))
        .sum::<Page>()
        .to_string()
}

pub fn second_answer(input: &str) -> String {
    let (rules, updates) = page_ordering_rules_and_updates(input);
    updates
        .into_iter()
        .filter(|update| !is_correctly_ordered(&rules, update))
        .map(|update| correct_order(&rules, &update))
        .map(|update| middle_page_number(&update))
        .sum::<Page>()
        .to_string()
}

fn is_correctly_ordered(rules: &[PageOrderingRule], update: &Update) -> bool {
    rules.iter().all(|&rule| is_rule_not_violated(rule, update))
}

fn is_rule_not_violated(rule: PageOrderingRule, update: &Update) -> bool {
    let rule_indices = rule.map(|rule_page| update.iter().position(|&page| page == rule_page));
    if let [Some(before_index), Some(after_index)] = rule_indices {
        before_index < after_index
    } else {
        true
    }
}

fn correct_order(rules: &[PageOrderingRule], update: &Update) -> Vec<Page> {
    let non_ignored_rules = rules
        .iter()
        .copied()
        .filter(|[before, after]| update.contains(before) && update.contains(after));

    let mut graph = AHashMap::new();
    for [before, after] in non_ignored_rules {
        graph.entry(before).or_insert_with(Vec::new).push(after);
    }
    topological_sort(&graph).expect("topological sort should exist")
}

fn middle_page_number(update: &Update) -> Page {
    update[update.len() / 2]
}

fn page_ordering_rules_and_updates(input: &str) -> (Vec<PageOrderingRule>, Vec<Update>) {
    let (rules, updates) = input
        .split_once("\n\n")
        .expect("ordering rules and updates should be separated by an empty line");
    (self::page_ordering_rules(rules), self::updates(updates))
}

fn page_ordering_rules(str: &str) -> Vec<PageOrderingRule> {
    str.lines()
        .map(|line| {
            usizes(line)
                .try_into()
                .expect("ordering rule should contain two page numbers")
        })
        .collect_vec()
}

fn updates(str: &str) -> Vec<Update> {
    str.lines().map(usizes).collect_vec()
}

#[cfg(test)]
mod tests {
    use infrastructure::{test, Input, Puzzle};

    use super::*;
    use crate::tests::test_on_input;

    const DAY: usize = 5;

    #[test]
    fn first_answer_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 143);
    }

    #[test]
    fn first_answer_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 5208);
    }

    #[test]
    fn second_answer_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 123);
    }

    #[test]
    fn second_answer_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 6732);
    }

    #[test]
    fn is_rule_not_violated() {
        let update: Update = usizes("75,47,61,53,29");
        let function = |rule| super::is_rule_not_violated(rule, &update);
        let cases = [
            ([75, 47], true),
            ([75, 61], true),
            ([75, 53], true),
            ([75, 29], true),
            ([53, 29], true),
            ([47, 75], false),
            ([29, 53], false),
            ([53, 61], false),
            ([61, 47], false),
            ([29, 75], false),
            ([76, 47], true),
            ([75, 48], true),
            ([76, 48], true),
        ];
        test::cases(function, cases);
    }
}
