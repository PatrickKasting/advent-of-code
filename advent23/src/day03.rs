use std::ops::Range;

use ahash::AHashMap;
use itertools::Itertools;
use regex::Regex;

type GearRatio = usize;
type PartNumber = usize;
type Coordinate = usize;

pub fn first_answer(input: &str) -> String {
    let mut sum: PartNumber = 0;
    for_each_part_number(input, |part_number, _, _| sum += part_number);
    sum.to_string()
}

pub fn second_answer(input: &str) -> String {
    gear_ratios(&part_numbers_next_to_stars(input))
        .sum::<GearRatio>()
        .to_string()
}

fn part_numbers_next_to_stars(input: &str) -> AHashMap<(Coordinate, Coordinate), Vec<PartNumber>> {
    let mut part_numbers_next_to_stars = AHashMap::new();
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

fn for_each_part_number(
    schematic: &str,
    mut action: impl FnMut(PartNumber, char, (Coordinate, Coordinate)),
) {
    let number_regex = Regex::new(r"\d+").expect("regex should be valid");
    let lines = schematic.lines().collect_vec();
    let [schematic_height, schematic_width] = [lines.len(), lines[0].len()];
    for (line_index, &line) in lines.iter().enumerate() {
        for mat in number_regex.find_iter(line) {
            let horizontal_range = range_extended_by_one(0..schematic_width, mat.range());
            #[expect(
                clippy::range_plus_one,
                reason = "type of this range matches return type of 'Match::range'"
            )]
            let vertical_range =
                range_extended_by_one(0..schematic_height, line_index..line_index + 1);
            for line_index in vertical_range {
                let line = &lines[line_index][horizontal_range.clone()];
                if let Some(symbol_position_within_range) = line.find(is_symbol) {
                    let part_number = mat
                        .as_str()
                        .parse()
                        .expect("part number should be numerical");
                    let symbol = line.as_bytes()[symbol_position_within_range] as char;
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

fn gear_ratios(
    part_numbers_next_to_stars: &AHashMap<(Coordinate, Coordinate), Vec<PartNumber>>,
) -> impl Iterator<Item = GearRatio> + '_ {
    part_numbers_next_to_stars
        .values()
        .filter(|part_numbers| part_numbers.len() == 2)
        .map(|part_numbers| gear_ratio(part_numbers))
}

fn gear_ratio(part_numbers: &[PartNumber]) -> GearRatio {
    debug_assert_eq!(
        part_numbers.len(),
        2,
        "a gear ratio should come from two part numbers"
    );
    part_numbers.iter().copied().product()
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 3;

    #[test]
    fn first_answer_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 4361);
    }

    #[test]
    fn first_answer_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 521_515);
    }

    #[test]
    fn second_answer_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 467_835);
    }

    #[test]
    fn second_answer_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 69_527_306);
    }
}
