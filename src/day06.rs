use itertools::Itertools;

use crate::utilities::number;

type Number = f64;
type Race = (Number, Number);

fn numbers(line: &str) -> impl Iterator<Item = Number> + '_ {
    line.split_ascii_whitespace().skip(1).map(number)
}

fn multiple_races(input: &str) -> impl Iterator<Item = Race> + '_ {
    let lines = input.lines().collect_vec();
    numbers(lines[0]).zip(numbers(lines[1]))
}

fn number_ignoring_whitespaces(line: &str) -> Number {
    number(line.split_ascii_whitespace().skip(1).collect::<String>())
}

fn single_race(input: &str) -> Race {
    input
        .lines()
        .map(number_ignoring_whitespaces)
        .collect_tuple()
        .expect("input should contain two lines")
}

fn roots(a: Number, b: Number, c: Number) -> (Number, Number) {
    let d = b * b - 4.0 * a * c;
    ((-b - d.sqrt()) / (2.0 * a), (-b + d.sqrt()) / (2.0 * a))
}

fn number_of_ways_to_beat_record((time, distance): Race) -> f64 {
    let (min, max) = roots(1.0, -time, distance);
    max.ceil() - min.floor() - 1.0
}

pub fn first(input: &str) -> String {
    multiple_races(input)
        .map(number_of_ways_to_beat_record)
        .product::<f64>()
        .to_string()
}

pub fn second(input: &str) -> String {
    number_of_ways_to_beat_record(single_race(input)).to_string()
}

#[cfg(test)]
mod tests {
    use crate::{tests::*, InputType, Puzzle};

    const DAY: usize = 6;

    #[test]
    fn first_example() {
        test_on_input(DAY, Puzzle::First, InputType::Example(0), 288);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, InputType::PuzzleInput, 3316275);
    }

    #[test]
    fn second_example() {
        test_on_input(DAY, Puzzle::Second, InputType::Example(0), 71503);
    }

    #[test]
    fn second_input() {
        test_on_input(DAY, Puzzle::Second, InputType::PuzzleInput, 27102791);
    }
}
