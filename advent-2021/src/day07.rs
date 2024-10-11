use std::convert::identity;

use itertools::Itertools;
use shared::string::isizes;

type Position = isize;
type Distance = isize;
type FuelConsumption = isize;

pub fn first(input: &str) -> String {
    let crabs = crabs(input);
    minimum_fuel_comsumption(&crabs, identity).to_string()
}

pub fn second(input: &str) -> String {
    let crabs = crabs(input);
    minimum_fuel_comsumption(&crabs, triangular_fuel_comsumption).to_string()
}

fn minimum_fuel_comsumption(
    crabs: &[Position],
    fuel_consumption: fn(Distance) -> FuelConsumption,
) -> FuelConsumption {
    let (&minimum_position, &maximum_position) = crabs
        .iter()
        .minmax()
        .into_option()
        .expect("at least one crab should exist");
    (minimum_position..=maximum_position)
        .map(|position| total_fuel_consumption(crabs, position, fuel_consumption))
        .min()
        .expect("at least one position should be in the range")
}

fn total_fuel_consumption(
    crabs: &[Position],
    destination: Position,
    fuel_consumption: fn(Distance) -> FuelConsumption,
) -> FuelConsumption {
    crabs
        .iter()
        .map(|crab| fuel_consumption((crab - destination).abs()))
        .sum()
}

fn triangular_fuel_comsumption(distance: Distance) -> FuelConsumption {
    distance * (distance + 1) / 2
}

fn crabs(input: &str) -> Vec<Position> {
    isizes(input)
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
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 329_389);
    }

    #[test]
    fn second_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 168);
    }

    #[test]
    fn second_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 86_397_080);
    }
}
