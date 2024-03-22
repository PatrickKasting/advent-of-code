use std::{collections::BTreeMap, ops::Range};

use itertools::Itertools;

use crate::utilities::number;

type Number = isize;
type Offset = isize;
type Map = BTreeMap<Number, Offset>;

fn seed_numbers(seeds: &str) -> Vec<Number> {
    seeds
        .split_ascii_whitespace()
        .skip(1)
        .map(number)
        .collect_vec()
}

fn singleton_seed_ranges(seeds: &str) -> Vec<Range<Number>> {
    #![allow(clippy::range_plus_one)]
    seed_numbers(seeds)
        .into_iter()
        .map(|number| number..number + 1)
        .collect_vec()
}

fn seed_ranges(seeds: &str) -> Vec<Range<Number>> {
    seed_numbers(seeds)
        .chunks(2)
        .map(|pair| pair[0]..pair[0] + pair[1])
        .collect_vec()
}

fn map_numbers(block: &str) -> Vec<[Number; 3]> {
    block
        .lines()
        .skip(1)
        .map(|line| {
            line.split_ascii_whitespace()
                .map(number)
                .collect_vec()
                .try_into()
                .expect("every line of a map should contain three numbers")
        })
        .collect_vec()
}

fn map(block: &str) -> Map {
    let mut numbers = map_numbers(block);
    numbers.sort_unstable_by_key(|&[_, source, _]| source);

    let mut map = BTreeMap::from([(0, 0)]);
    for [destination, source, length] in numbers {
        map.insert(source, destination - source);
        map.insert(source + length, 0);
    }
    map
}

fn mapped_number_range(map: &Map, number_range: Range<Number>) -> Vec<Range<Number>> {
    let predcecessor = map
        .range(0..=number_range.start)
        .next_back()
        .expect("every map should have a map range starting at zero");
    let mut subrange_endpoints = Vec::from([(number_range.start, *predcecessor.1)]);
    for (&start, &translation) in map.range(number_range.clone()) {
        subrange_endpoints.push((start, translation));
    }
    subrange_endpoints.push((number_range.end, 0)); // translation irrelevant
    subrange_endpoints.dedup_by_key(|(endpoint, _)| *endpoint);

    subrange_endpoints
        .windows(2)
        .map(|pair| {
            let translation = pair[0].1;
            pair[0].0 + translation..pair[1].0 + translation
        })
        .collect_vec()
}

fn location(maps: &[Map], seed_range: Range<Number>) -> Vec<Range<Number>> {
    let mut number_ranges = Vec::from([seed_range]);
    for map in maps {
        number_ranges = number_ranges
            .into_iter()
            .flat_map(|number_range| mapped_number_range(map, number_range))
            .collect_vec();
    }
    number_ranges
}

fn minimum_location(maps: &[Map], seed_ranges: Vec<Range<Number>>) -> Number {
    seed_ranges
        .into_iter()
        .flat_map(|seed_range| location(maps, seed_range))
        .map(|location_range| location_range.start)
        .min()
        .expect("there should be at least one seed range")
}

pub fn first(input: &str) -> String {
    let blocks = input.split("\n\n").collect_vec();
    let seed_ranges = singleton_seed_ranges(blocks[0]);
    let maps = blocks[1..].iter().map(|&block| map(block)).collect_vec();
    minimum_location(&maps, seed_ranges).to_string()
}

pub fn second(input: &str) -> String {
    let blocks = input.split("\n\n").collect_vec();
    let seed_ranges = seed_ranges(blocks[0]);
    let maps = blocks[1..].iter().map(|&block| map(block)).collect_vec();
    minimum_location(&maps, seed_ranges).to_string()
}

#[cfg(test)]
mod tests {
    use super::super::tests::test_on_input;
    use crate::{Input, Puzzle};

    const DAY: usize = 5;

    #[test]
    fn first_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 35);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 265_018_614);
    }

    #[test]
    fn second_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 46);
    }

    #[test]
    fn second_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 63_179_500);
    }
}
