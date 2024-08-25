use std::{collections::BTreeMap, convert::identity};

type Integer = i32;

pub fn first(input: &str) -> String {
    min_total_fuel_cost(input, identity).to_string()
}

pub fn second(input: &str) -> String {
    let fuel_cost = |distance| distance * (distance + 1) / 2;
    min_total_fuel_cost(input, fuel_cost).to_string()
}

fn min_total_fuel_cost(input: &str, fuel_cost: impl Fn(Integer) -> Integer) -> Integer {
    const AT_LEAST_ONE_CRAB: &str = "input should contain at least one crab";
    let crabs = parse_input(input);
    let &min_pos = crabs.first_key_value().expect(AT_LEAST_ONE_CRAB).0;
    let &max_pos = crabs.last_key_value().expect(AT_LEAST_ONE_CRAB).0;
    (min_pos..=max_pos)
        .map(|position| total_fuel_cost(&crabs, &fuel_cost, position))
        .min()
        .expect(AT_LEAST_ONE_CRAB)
}

fn total_fuel_cost(
    crabs: &BTreeMap<Integer, Integer>,
    fuel_cost: &impl Fn(Integer) -> Integer,
    position: Integer,
) -> Integer {
    crabs
        .iter()
        .map(|(&crab_position, &num_crabs)| num_crabs * fuel_cost((position - crab_position).abs()))
        .sum()
}

fn parse_input(input: &str) -> BTreeMap<Integer, Integer> {
    let mut crabs = BTreeMap::new();
    for horizontal_position in input
        .trim()
        .split(',')
        .map(|pos| pos.parse().expect("input should only contain numbers"))
    {
        *crabs.entry(horizontal_position).or_default() += 1;
    }
    crabs
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 7;

    #[test]
    fn first_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 37);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 329389);
    }

    #[test]
    fn second_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 168);
    }

    #[test]
    fn second_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 86397080);
    }
}
