use std::{
    cmp::Ordering,
    collections::HashMap,
    ops::{Index, IndexMut},
};

use itertools::Itertools;

use crate::strings::{parse, usizes};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Category {
    X,
    M,
    A,
    S,
}

impl From<&str> for Category {
    fn from(str: &str) -> Self {
        match str {
            "x" => Self::X,
            "m" => Self::M,
            "a" => Self::A,
            "s" => Self::S,
            _ => panic!("category should be 'x', 'm', 'a', or 's'"),
        }
    }
}

type Rating = usize;

type WorkflowName<'name> = &'name str;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Condition<'condition> {
    category: Category,
    comparison: Ordering,
    limit: Rating,
    destination: WorkflowName<'condition>,
}

impl<'condition> From<&'condition str> for Condition<'condition> {
    fn from(str: &'condition str) -> Self {
        let category = Category::from(&str[0..1]);
        let comparison = if &str[1..2] == "<" {
            Ordering::Less
        } else {
            Ordering::Greater
        };
        let colon_index = str
            .find(':')
            .expect("condition should use ':' to separate condition and destination");
        let limit = parse(&str[2..colon_index]);
        let destination = &str[colon_index + 1..];
        Self {
            category,
            comparison,
            limit,
            destination,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct WorkflowRules<'rules> {
    conditions: Vec<Condition<'rules>>,
    default: &'rules str,
}

impl<'rules> From<&'rules str> for WorkflowRules<'rules> {
    fn from(str: &'rules str) -> Self {
        let rules = str.split(',').collect_vec();
        let (&default, conditions) = rules
            .split_last()
            .expect("workflow should contain at least one rule");
        let conditions = conditions.iter().copied().map(Condition::from).collect();
        Self {
            conditions,
            default,
        }
    }
}

type Workflow<'workflow> = (WorkflowName<'workflow>, WorkflowRules<'workflow>);
type WorkflowSystem<'system> = HashMap<WorkflowName<'system>, WorkflowRules<'system>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Part([Rating; 4]);

impl Part {
    fn sum_of_ratings(self) -> Rating {
        self.0.into_iter().sum()
    }
}

impl From<&str> for Part {
    fn from(str: &str) -> Self {
        let ratings = usizes(str)
            .try_into()
            .expect("part should consist of four ratings");
        Self(ratings)
    }
}

impl Index<Category> for Part {
    type Output = Rating;

    fn index(&self, category: Category) -> &Self::Output {
        match category {
            Category::X => &self.0[0],
            Category::M => &self.0[1],
            Category::A => &self.0[2],
            Category::S => &self.0[3],
        }
    }
}

type RatingRange = [Rating; 2];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct RatingRanges([RatingRange; 4]);

impl Index<Category> for RatingRanges {
    type Output = RatingRange;

    fn index(&self, category: Category) -> &Self::Output {
        match category {
            Category::X => &self.0[0],
            Category::M => &self.0[1],
            Category::A => &self.0[2],
            Category::S => &self.0[3],
        }
    }
}

impl IndexMut<Category> for RatingRanges {
    fn index_mut(&mut self, category: Category) -> &mut Self::Output {
        match category {
            Category::X => &mut self.0[0],
            Category::M => &mut self.0[1],
            Category::A => &mut self.0[2],
            Category::S => &mut self.0[3],
        }
    }
}

impl RatingRanges {
    fn all_possible_combinations() -> Self {
        Self([[1, 4001]; 4])
    }

    fn number_of_combinations(self) -> usize {
        self.0.into_iter().map(|[start, end]| end - start).product()
    }

    fn contains(self, part: Part) -> bool {
        self.0
            .into_iter()
            .zip(part.0)
            .all(|([min, max], rating)| min <= rating && rating < max)
    }
}

fn workflow(str: &str) -> Workflow {
    let bracket_indices = ['{', '}'].map(|bracket| {
        str.find(bracket)
            .expect("workflow should contain both brackets")
    });
    let (name, rules) = (
        &str[0..bracket_indices[0]],
        &str[bracket_indices[0] + 1..bracket_indices[1]],
    );
    (name, WorkflowRules::from(rules))
}

fn workflow_system(str: &str) -> WorkflowSystem {
    str.lines().map(workflow).collect()
}

fn split_rating_range(
    possible_rating_range: RatingRange,
    condition: &Condition<'_>,
) -> [Option<[usize; 2]>; 2] {
    let splitting_point = if condition.comparison == Ordering::Less {
        condition.limit
    } else {
        condition.limit + 1
    };
    let left = (possible_rating_range[0] < splitting_point)
        .then_some([possible_rating_range[0], splitting_point]);
    let right = (splitting_point < possible_rating_range[1])
        .then_some([splitting_point, possible_rating_range[1]]);
    let mut split = [left, right];
    if condition.comparison == Ordering::Greater {
        split.reverse();
    }
    split
}

fn acceptable_ranges(
    workflow_system: &WorkflowSystem,
    mut possible_ranges: RatingRanges,
    workflow_name: &str,
) -> Vec<RatingRanges> {
    if workflow_name == "A" {
        return vec![possible_ranges];
    }
    if workflow_name == "R" {
        return vec![];
    }

    let workflow_rules = &workflow_system[workflow_name];
    let mut accepted_ranges = Vec::new();
    for condition in &workflow_rules.conditions {
        let [true_range, false_range] =
            split_rating_range(possible_ranges[condition.category], condition);
        if let Some(true_range) = true_range {
            possible_ranges[condition.category] = true_range;
            let acceptable_ranges_from_true_range =
                acceptable_ranges(workflow_system, possible_ranges, condition.destination);
            accepted_ranges.extend(acceptable_ranges_from_true_range);
        }
        let Some(false_range) = false_range else {
            return accepted_ranges;
        };
        possible_ranges[condition.category] = false_range;
    }
    let acceptable_ranges_from_default_range =
        acceptable_ranges(workflow_system, possible_ranges, workflow_rules.default);
    accepted_ranges.extend(acceptable_ranges_from_default_range);
    accepted_ranges
}

fn acceptable_rating_ranges(workflows: &str) -> Vec<RatingRanges> {
    let workflow_system = workflow_system(workflows);
    acceptable_ranges(
        &workflow_system,
        RatingRanges::all_possible_combinations(),
        "in",
    )
}

fn sum_of_sum_of_ratings(accepted_ranges: &[RatingRanges], parts: &[Part]) -> usize {
    parts
        .iter()
        .filter(|&&part| accepted_ranges.iter().any(|range| range.contains(part)))
        .map(|part| part.sum_of_ratings())
        .sum()
}

fn number_of_combinations(accepted_ranges: &[RatingRanges]) -> usize {
    accepted_ranges
        .iter()
        .map(|ranges| ranges.number_of_combinations())
        .sum::<usize>()
}

pub fn first(input: &str) -> String {
    let (workflows, parts) = input
        .split_once("\n\n")
        .expect("input should contain workflows and parts separated by an empty line");
    let acceptable_rating_ranges = acceptable_rating_ranges(workflows);
    let parts = parts.lines().map(Part::from).collect_vec();
    sum_of_sum_of_ratings(&acceptable_rating_ranges, &parts).to_string()
}

pub fn second(input: &str) -> String {
    let (workflows, _) = input
        .split_once("\n\n")
        .expect("input should contain workflows and parts separated by an empty line");
    let acceptable_rating_ranges = acceptable_rating_ranges(workflows);
    number_of_combinations(&acceptable_rating_ranges).to_string()
}

#[cfg(test)]
mod tests {
    use super::super::tests::test_on_input;
    use crate::{Input, Puzzle};

    const DAY: usize = 19;

    #[test]
    fn first_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 19114);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 332_145);
    }

    #[test]
    fn second_example() {
        test_on_input(
            DAY,
            Puzzle::Second,
            Input::Example(0),
            167_409_079_868_000_usize,
        );
    }

    #[test]
    fn second_input() {
        test_on_input(
            DAY,
            Puzzle::Second,
            Input::PuzzleInput,
            136_661_579_897_555_usize,
        );
    }
}
