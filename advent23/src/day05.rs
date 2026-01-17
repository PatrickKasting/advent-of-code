use std::{collections::BTreeMap, ops::Range};

use itertools::Itertools;

use shared::string::isizes;

type Map = BTreeMap<Number, Offset>;
type Number = isize;
type Offset = isize;

pub fn first_answer(input: &str) -> String {
    minimum_location_from_input(input, singleton_seed_ranges).to_string()
}

pub fn second_answer(input: &str) -> String {
    minimum_location_from_input(input, seed_ranges).to_string()
}

fn minimum_location_from_input(input: &str, seed_ranges: fn(&str) -> Vec<Range<Number>>) -> Number {
    let blocks = input.split("\n\n").collect_vec();
    let seed_ranges = seed_ranges(blocks[0]);
    let maps = blocks[1..].iter().map(|&block| map(block)).collect_vec();
    minimum_location(&maps, seed_ranges)
}

fn minimum_location(maps: &[Map], seed_ranges: Vec<Range<Number>>) -> Number {
    seed_ranges
        .into_iter()
        .flat_map(|seed_range| location(maps, seed_range))
        .map(|location_range| location_range.start)
        .min()
        .expect("there should be at least one seed range")
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

fn mapped_number_range(map: &Map, number_range: Range<Number>) -> Vec<Range<Number>> {
    let predcecessor = map
        .range(0..=number_range.start)
        .next_back()
        .expect("every map should have a map range starting at zero");
    let mut subranges_ends_and_translations = Vec::from([(number_range.start, *predcecessor.1)]);
    for (&start, &translation) in map.range(number_range.clone()) {
        subranges_ends_and_translations.push((start, translation));
    }
    subranges_ends_and_translations.push((number_range.end, 0)); // translation irrelevant
    subranges_ends_and_translations.dedup_by_key(|(endpoint, _)| *endpoint);

    subranges_ends_and_translations
        .windows(2)
        .map(|pair| {
            let translation = pair[0].1;
            pair[0].0 + translation..pair[1].0 + translation
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

fn map_numbers(block: &str) -> Vec<[Number; 3]> {
    block
        .lines()
        .skip(1)
        .map(|line| {
            isizes(line)
                .try_into()
                .expect("every line of a map should contain three numbers")
        })
        .collect_vec()
}

fn singleton_seed_ranges(seeds: &str) -> Vec<Range<Number>> {
    isizes(seeds)
        .into_iter()
        .map(|number| number..number + 1)
        .collect_vec()
}

fn seed_ranges(seeds: &str) -> Vec<Range<Number>> {
    isizes(seeds)
        .chunks(2)
        .map(|pair| pair[0]..pair[0] + pair[1])
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 5;

    #[test]
    fn first_answer_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 35);
    }

    #[test]
    fn first_answer_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 265_018_614);
    }

    #[test]
    fn second_answer_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 46);
    }

    #[test]
    fn second_answer_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 63_179_500);
    }
}
