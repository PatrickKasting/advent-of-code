use std::convert::identity;

use itertools::Itertools;

type BinaryNumber<'a> = &'a [Bit];
type Bit = u8;

pub fn first_answer(input: &str) -> String {
    let binary_numbers = binary_numbers(input);
    let rates = rates(&binary_numbers);
    let power_consumption = rates
        .into_iter()
        .map(|number| decimal(&number))
        .product::<usize>();
    power_consumption.to_string()
}

pub fn second_answer(input: &str) -> String {
    let oxygen_generator_rating = filtering(binary_numbers(input), identity);
    let co2_scrubber_rating = filtering(binary_numbers(input), negation);
    let life_support_rating = decimal(oxygen_generator_rating) * decimal(co2_scrubber_rating);
    life_support_rating.to_string()
}

fn filtering(
    mut binary_numbers: Vec<BinaryNumber>,
    most_common_bit_to_target_bit: fn(Bit) -> Bit,
) -> BinaryNumber {
    for index in 0..binary_numbers[0].len() {
        let most_common_bit = most_common_bit(&binary_numbers, index);
        let target_bit = most_common_bit_to_target_bit(most_common_bit);
        binary_numbers.retain(|number| number[index] == target_bit);

        if binary_numbers.len() == 1 {
            return binary_numbers[0];
        }
    }
    panic!("binary numbers should filter down to exactly one number");
}

fn rates(binary_numbers: &[BinaryNumber]) -> [Vec<Bit>; 2] {
    let mut gamma_rate = vec![];
    let mut epsilon_rate = vec![];
    for index in 0..binary_numbers[0].len() {
        let most_common_bit = most_common_bit(binary_numbers, index);
        gamma_rate.push(most_common_bit);
        epsilon_rate.push(negation(most_common_bit));
    }
    [gamma_rate, epsilon_rate]
}

fn most_common_bit(binary_numbers: &[BinaryNumber], index: usize) -> Bit {
    let number_of_high_bits = binary_numbers
        .iter()
        .filter(|number| number[index] == b'1')
        .count();
    if 2 * number_of_high_bits < binary_numbers.len() {
        b'0'
    } else {
        b'1'
    }
}

fn negation(bit: Bit) -> Bit {
    match bit {
        b'0' => b'1',
        b'1' => b'0',
        _ => panic!("bit should be '0' or '1'"),
    }
}

fn decimal(binary_number: BinaryNumber) -> usize {
    let mut decimal = 0;
    for &bit in binary_number {
        decimal <<= 1;
        if bit == b'1' {
            decimal += 1;
        }
    }
    decimal
}

fn binary_numbers(input: &str) -> Vec<BinaryNumber> {
    input.lines().map(str::as_bytes).collect_vec()
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 3;

    #[test]
    fn first_answer_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 198);
    }

    #[test]
    fn first_answer_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 3_847_100);
    }

    #[test]
    fn second_answer_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 230);
    }

    #[test]
    fn second_answer_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 4_105_235);
    }
}
