use std::{cmp::Ordering, ops::Range};

use ahash::AHashMap;
use itertools::Itertools;

use shared::string::usizes;

type Workflows<'input> = AHashMap<Name<'input>, Rules<'input>>;
type Workflow<'input> = (Name<'input>, Rules<'input>);
type Rules<'input> = (Vec<Condition<'input>>, Name<'input>);
type Condition<'input> = (Category, Ordering, Rating, Name<'input>);
type Category = usize;
type Rating = usize;
type Name<'input> = &'input str;
type Part = [Rating; 4];

type RatingRanges = [RatingRange; 4];
type RatingRange = Range<Rating>;

pub fn first(input: &str) -> String {
    let (workflows, parts) = input
        .split_once("\n\n")
        .expect("input should contain workflows and parts separated by an empty line");
    let acceptable_rating_ranges = acceptable_rating_ranges(workflows);
    let parts = parts.lines().map(part).collect_vec();
    let acceptable_parts = acceptable_parts(&acceptable_rating_ranges, parts);
    sum_of_ratings(acceptable_parts).to_string()
}

pub fn second(input: &str) -> String {
    let (workflows, _) = input
        .split_once("\n\n")
        .expect("input should contain workflows and parts separated by an empty line");
    let acceptable_rating_ranges = acceptable_rating_ranges(workflows);
    number_of_combinations(&acceptable_rating_ranges).to_string()
}

fn acceptable_rating_ranges(str: &str) -> Vec<RatingRanges> {
    let workflows = workflows(str);
    acceptable_ranges(&workflows, allowed_ranges(), "in")
}

fn allowed_ranges() -> RatingRanges {
    vec![(1..4001).clone(); 4]
        .try_into()
        .expect("'vec' should have correct length")
}

fn acceptable_ranges(
    workflows: &Workflows,
    mut possible_ranges: RatingRanges,
    workflow_name: &str,
) -> Vec<RatingRanges> {
    if workflow_name == "A" {
        return vec![possible_ranges];
    }
    if workflow_name == "R" {
        return vec![];
    }

    let mut acceptable_ranges = Vec::new();
    let (conditions, default) = &workflows[workflow_name];
    for &(category, comparison, limit, destination) in conditions {
        let [true_range, false_range] =
            split_range(possible_ranges[category].clone(), comparison, limit);
        if !true_range.is_empty() {
            possible_ranges[category] = true_range;
            let acceptable_ranges_from_true_range =
                self::acceptable_ranges(workflows, possible_ranges.clone(), destination);
            acceptable_ranges.extend(acceptable_ranges_from_true_range);
        }
        if false_range.is_empty() {
            return acceptable_ranges;
        };
        possible_ranges[category] = false_range;
    }
    let acceptable_ranges_from_default_range =
        self::acceptable_ranges(workflows, possible_ranges.clone(), default);
    acceptable_ranges.extend(acceptable_ranges_from_default_range);
    acceptable_ranges
}

fn split_range(range: RatingRange, comparison: Ordering, limit: Rating) -> [Range<Rating>; 2] {
    let split = if comparison == Ordering::Less {
        limit
    } else {
        limit + 1
    };
    let mut split = [range.start..split, split..range.end];
    if comparison == Ordering::Greater {
        split.reverse();
    }
    split
}

fn acceptable_parts(
    acceptable_rating_ranges: &[RatingRanges],
    parts: Vec<Part>,
) -> impl Iterator<Item = Part> + '_ {
    parts
        .into_iter()
        .filter(|&part| is_part_acceptable(acceptable_rating_ranges, part))
}

fn is_part_acceptable(acceptable_rating_ranges: &[RatingRanges], part: Part) -> bool {
    acceptable_rating_ranges.iter().any(|ranges| {
        ranges
            .iter()
            .zip(part)
            .all(|(range, rating)| range.contains(&rating))
    })
}

fn sum_of_ratings(parts: impl Iterator<Item = Part>) -> Rating {
    parts.map(|part| part.into_iter().sum::<Rating>()).sum()
}

fn number_of_combinations(ranges: &[RatingRanges]) -> usize {
    ranges
        .iter()
        .map(|ranges| {
            ranges
                .iter()
                .map(std::iter::ExactSizeIterator::len)
                .product::<usize>()
        })
        .sum::<usize>()
}

fn workflows(str: &str) -> Workflows {
    str.lines().map(workflow).collect()
}

fn workflow(line: &str) -> Workflow {
    let (name, rules) = line
        .split_once('{')
        .expect("workflow should contain opening brackets");
    (name, self::rules(&rules[..rules.len() - 1]))
}

fn rules(str: &str) -> Rules {
    let mut conditions = vec![];
    for rule in str.split(',') {
        let comparison = match rule.chars().nth(1) {
            Some('<') => Ordering::Less,
            Some('>') => Ordering::Greater,
            _ => return (conditions, rule),
        };
        let category = category(rule.chars().next().expect("rule should not be empty"));
        let (rating, destination) = rule[2..]
            .split_once(':')
            .expect("comparison and destination should be separated by colon");
        let rating = rating.parse().expect("rating should be numerical");
        conditions.push((category, comparison, rating, destination));
    }
    panic!("last rule should be unconditional")
}

fn category(char: char) -> Category {
    ['x', 'm', 'a', 's']
        .into_iter()
        .position(|category| category == char)
        .expect("category should be 'x', 'm', 'a', or 's'")
}

fn part(line: &str) -> Part {
    usizes(line)
        .try_into()
        .expect("each part should have four ratings")
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

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
