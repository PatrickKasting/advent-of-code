pub fn first(input: &str) -> String {
    let numbers: Vec<&str> = input.lines().collect();
    let mut gamma_rate = 0;
    let number_of_bits = numbers[0].chars().count();
    for position in 0..number_of_bits {
        if most_frequent_bit(&numbers, position) {
            gamma_rate += 1 << (number_of_bits - 1 - position)
        }
    }
    let epsilon_rate = (1 << number_of_bits) - 1 - gamma_rate;
    (gamma_rate * epsilon_rate).to_string()
}

pub fn second(input: &str) -> String {
    let o2_generator_rating = find_rating(input.lines().collect(), true);
    let co2_scrubber_rating = find_rating(input.lines().collect(), false);
    (o2_generator_rating * co2_scrubber_rating).to_string()
}

fn most_frequent_bit(numbers: &[&str], position: usize) -> bool {
    let mut number_of_ones = 0;
    for number in numbers {
        let bit = number
            .chars()
            .nth(position)
            .expect("position should be within number");
        if bit == '1' {
            number_of_ones += 1;
        }
    }
    number_of_ones >= numbers.len() - number_of_ones
}

fn find_rating(mut numbers: Vec<&str>, most_common: bool) -> usize {
    let mut position = 0;
    while numbers.len() > 1 {
        let expected = most_frequent_bit(&numbers, position) ^ !most_common;
        filter_matching_numbers(&mut numbers, position, expected);
        position += 1;
    }
    binary(numbers[0])
}

fn filter_matching_numbers(numbers: &mut Vec<&str>, position: usize, expected: bool) {
    let expected = if expected { '1' } else { '0' };
    let mut index = 0;
    while index < numbers.len() {
        let actual = numbers[index]
            .chars()
            .nth(position)
            .expect("position should be within number");
        if actual != expected {
            numbers.swap_remove(index);
        } else {
            index += 1;
        }
    }
}

fn binary(number: &str) -> usize {
    let mut result = 0;
    for (index, bit) in number.chars().rev().enumerate() {
        if bit == '1' {
            result += 1 << index;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 3;

    #[test]
    fn first_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 198);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 3847100);
    }

    #[test]
    fn second_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 230);
    }

    #[test]
    fn second_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 4105235);
    }
}
