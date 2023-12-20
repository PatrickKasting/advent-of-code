use std::{collections::HashMap, ops::Range};

use itertools::Itertools;
use regex::Regex;

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
                    let part_number = mat
                        .as_str()
                        .parse::<usize>()
                        .expect("substring of digits should be parsable");
                    let symbol = adjacent_range
                        .chars()
                        .nth(symbol_position_within_range)
                        .expect("symbol should be found within adjacent range");
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

pub fn first(input: String) -> String {
    let mut sum: usize = 0;
    for_each_part_number(&input, |part_number, _, _| sum += part_number);
    sum.to_string()
}

pub fn second(input: String) -> String {
    let mut gears = HashMap::new();
    let on_part_number = |part_number, symbol, location| {
        if symbol == '*' {
            gears
                .entry(location)
                .or_insert_with(Vec::new)
                .push(part_number);
        }
    };
    for_each_part_number(&input, on_part_number);

    gears
        .values()
        .filter(|gear| gear.len() == 2)
        .map(|gear| gear.iter().product::<usize>())
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use crate::{tests::*, Input, Puzzle};

    const DAY: usize = 3;

    #[test]
    fn first_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 4361);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::Real, 521515);
    }

    #[test]
    fn second_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 467835);
    }

    #[test]
    fn second_input() {
        test_on_input(DAY, Puzzle::Second, Input::Real, 69527306);
    }
}
