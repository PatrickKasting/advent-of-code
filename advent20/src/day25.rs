use itertools::Itertools;

pub fn first_answer(input: &str) -> String {
    let [card_public_key, door_public_key] = public_keys(input);
    let card_loop_size = loop_size(7, card_public_key);
    let encryption_key = encryption_key(door_public_key, card_loop_size);
    encryption_key.to_string()
}

pub fn second_answer(_input: &str) -> String {
    "There is no second puzzle on the 25th. Merry Christmas!".to_owned()
}

fn loop_size(subject_number: usize, public_key: usize) -> usize {
    let mut value = 1;
    for loop_size in 0.. {
        if value == public_key {
            return loop_size;
        }
        value = transform_once(subject_number, value);
    }
    unreachable!("loop should only break on value matching public key");
}

fn encryption_key(subject_number: usize, loop_size: usize) -> usize {
    let mut value = 1;
    for _ in 0..loop_size {
        value = transform_once(subject_number, value);
    }
    value
}

fn transform_once(subject_number: usize, value: usize) -> usize {
    (value * subject_number) % 20_201_227
}

fn public_keys(input: &str) -> [usize; 2] {
    input
        .lines()
        .map(|public_key| public_key.parse().expect("public key should be numeric"))
        .collect_vec()
        .try_into()
        .expect("input should contain exactly two public keys")
}

#[cfg(test)]
mod tests {
    use infrastructure::{test, Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 25;

    #[test]
    fn first_answer_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 14_897_079);
    }

    #[test]
    fn first_answer_puzzle_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 16_902_792);
    }

    #[test]
    fn loop_size() {
        let function = |public_key| super::loop_size(7, public_key);
        let cases = [(5_764_801, 8), (17_807_724, 11)];
        test::cases(function, cases);
    }
}
