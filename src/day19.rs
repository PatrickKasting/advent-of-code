use std::{
    cmp::Ordering,
    collections::HashMap,
    ops::{Index, IndexMut},
    str::FromStr,
};

use itertools::Itertools;
use regex::Regex;

use crate::utilities::number;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Category {
    X,
    M,
    A,
    S,
}

impl FromStr for Category {
    type Err = ();

    fn from_str(category: &str) -> Result<Self, Self::Err> {
        match category {
            "x" => Ok(Category::X),
            "m" => Ok(Category::M),
            "a" => Ok(Category::A),
            "s" => Ok(Category::S),
            _ => Err(()),
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct WorkflowRules<'rules> {
    conditions: Vec<Condition<'rules>>,
    default: &'rules str,
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

impl FromStr for Part {
    type Err = &'static str;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let regex = Regex::new(r"\d+").expect("regex should be valid");
        let ratings = regex
            .find_iter(str)
            .map(|mat| number(mat.as_str()))
            .collect_vec()
            .try_into()
            .expect("part should consist of four ratings");
        Ok(Self(ratings))
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

fn condition(str: &str) -> Condition {
    let category = str[0..1]
        .parse()
        .expect("category should be 'x', 'm', 'a', or 's'");
    let comparison = if &str[1..2] == "<" {
        Ordering::Less
    } else {
        Ordering::Greater
    };
    let colon_index = str
        .find(':')
        .expect("condition should use ':' to separate condition and destination");
    let limit = str[2..colon_index]
        .parse()
        .expect("limit should be numerical");
    let destination = &str[colon_index + 1..];
    Condition {
        category,
        comparison,
        limit,
        destination,
    }
}

fn workflow_rules(str: &str) -> WorkflowRules {
    let rules = str.split(',').collect_vec();
    let (&default, conditions) = rules
        .split_last()
        .expect("workflow should contain at least one rule");
    let conditions = conditions.iter().map(|cond| condition(cond)).collect();
    WorkflowRules {
        conditions,
        default,
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
    (name, workflow_rules(rules))
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
    let parts = parts
        .lines()
        .map(|line| line.parse().expect("part should be parsable"))
        .collect_vec();
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
    use crate::{tests::*, InputType, Puzzle};

    const DAY: usize = 19;

    #[test]
    fn first_example() {
        test_on_input(DAY, Puzzle::First, InputType::Example(0), 19114);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, InputType::PuzzleInput, 332145);
    }

    #[test]
    fn second_example() {
        test_on_input(
            DAY,
            Puzzle::Second,
            InputType::Example(0),
            167_409_079_868_000_usize,
        );
    }

    #[test]
    fn second_input() {
        test_on_input(
            DAY,
            Puzzle::Second,
            InputType::PuzzleInput,
            136_661_579_897_555_usize,
        );
    }
}
