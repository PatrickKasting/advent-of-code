use itertools::Itertools;

type Race = (Number, Number);
type Number = f64;

pub fn first(input: &str) -> String {
    multiple_races(input)
        .map(number_of_ways_to_beat_record)
        .product::<f64>()
        .to_string()
}

pub fn second(input: &str) -> String {
    number_of_ways_to_beat_record(single_race(input)).to_string()
}

fn number_of_ways_to_beat_record((time, distance): Race) -> f64 {
    let (min, max) = roots(1.0, -time, distance);
    max.ceil() - min.floor() - 1.0
}

fn roots(a: Number, b: Number, c: Number) -> (Number, Number) {
    let d = b * b - 4.0 * a * c;
    ((-b - d.sqrt()) / (2.0 * a), (-b + d.sqrt()) / (2.0 * a))
}

fn multiple_races(input: &str) -> impl Iterator<Item = Race> + '_ {
    let lines = input.lines().collect_vec();
    numbers(lines[0]).zip(numbers(lines[1]))
}

fn numbers(line: &str) -> impl Iterator<Item = Number> + '_ {
    line.split_whitespace().skip(1).map(number)
}

fn single_race(input: &str) -> Race {
    input
        .lines()
        .map(number_ignoring_whitespaces)
        .collect_tuple()
        .expect("input should contain two lines")
}

fn number_ignoring_whitespaces(line: &str) -> Number {
    number(&line.split_whitespace().skip(1).collect::<String>())
}

fn number(str: &str) -> Number {
    str.parse::<Number>().expect("number should be numerical")
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 6;

    #[test]
    fn first_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 288);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 3_316_275);
    }

    #[test]
    fn second_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 71503);
    }

    #[test]
    fn second_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 27_102_791);
    }
}
