use std::{collections::HashMap, ops::Range};

use itertools::Itertools;
use regex::Regex;

use crate::utilities::{char_at, number};

type PartNumber = usize;

fn range_extended_by_one(bounds: Range<usize>, mut range: Range<usize>) -> Range<usize> {
    if range.start != bounds.start {
        range.start -= 1;
    }
    if range.end != bounds.end {
        range.end += 1;
    }
    range
}

fn is_symbol(char: char) -> bool {
    char != '.' && char.is_ascii_punctuation()
}

fn for_each_part_number(schematic: &str, mut action: impl FnMut(PartNumber, char, (usize, usize))) {
    #![allow(clippy::range_plus_one)]

    let number_regex = Regex::new(r"\d+").expect("regex should be valid");
    let lines = schematic.lines().collect_vec();
    let [schematic_height, schematic_width] = [lines.len(), lines[0].len()];
    for (line_index, &line) in lines.iter().enumerate() {
        let vertical_range = range_extended_by_one(0..schematic_height, line_index..line_index + 1);
        for mat in number_regex.find_iter(line) {
            let horizontal_range = range_extended_by_one(0..schematic_width, mat.range());
            for line_index in vertical_range.clone() {
                let adjacent_range = &lines[line_index][horizontal_range.clone()];
                if let Some(symbol_position_within_range) = adjacent_range.find(is_symbol) {
                    let part_number = number(mat.as_str());
                    let symbol = char_at(adjacent_range, symbol_position_within_range);
                    let location = (
                        line_index,
                        symbol_position_within_range + horizontal_range.start,
                    );
                    action(part_number, symbol, location);
                    break;
                }
            }
        }
    }
}

fn part_numbers_next_to_stars(input: &str) -> HashMap<(usize, usize), Vec<usize>> {
    let mut part_numbers_next_to_stars = HashMap::new();
    let add_part_number_if_next_to_star = |part_number, symbol, location| {
        if symbol == '*' {
            part_numbers_next_to_stars
                .entry(location)
                .or_insert_with(Vec::new)
                .push(part_number);
        }
    };
    for_each_part_number(input, add_part_number_if_next_to_star);
    part_numbers_next_to_stars
}

fn gear_ratio(part_numbers: &[usize]) -> usize {
    debug_assert_eq!(
        part_numbers.len(),
        2,
        "a gear ratio should come from two part numbers"
    );
    part_numbers.iter().copied().product()
}

fn gear_ratios(
    part_numbers_next_to_stars: &HashMap<(usize, usize), Vec<usize>>,
) -> impl Iterator<Item = usize> + '_ {
    part_numbers_next_to_stars
        .values()
        .filter(|part_numbers| part_numbers.len() == 2)
        .map(|part_numbers| gear_ratio(part_numbers))
}

pub fn first(input: &str) -> String {
    let mut sum: usize = 0;
    for_each_part_number(input, |part_number, _, _| sum += part_number);
    sum.to_string()
}

pub fn second(input: &str) -> String {
    gear_ratios(&part_numbers_next_to_stars(input))
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use crate::{tests::*, InputType, Puzzle};

    const DAY: usize = 3;

    #[test]
    fn first_example() {
        test_on_input(DAY, Puzzle::First, InputType::Example(0), 4361);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, InputType::PuzzleInput, 521515);
    }

    #[test]
    fn second_example() {
        test_on_input(DAY, Puzzle::Second, InputType::Example(0), 467835);
    }

    #[test]
    fn second_input() {
        test_on_input(DAY, Puzzle::Second, InputType::PuzzleInput, 69527306);
    }
}
