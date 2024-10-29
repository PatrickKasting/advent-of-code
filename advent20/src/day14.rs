use ahash::AHashMap;
use bitvec::{order::Msb0, view::BitView};
use easy_cast::Cast;
use itertools::izip;
use shared::string::usizes;

type Address = u64;
type Value = u64;

pub fn first_answer(input: &str) -> String {
    let memory = execute_program_without_floating_bits(input);
    memory.values().sum::<Value>().to_string()
}

pub fn second_answer(input: &str) -> String {
    todo!()
}

fn execute_program_without_floating_bits(instructions: &str) -> AHashMap<Address, Value> {
    let mut mask = Default::default();
    let mut memory = AHashMap::new();
    for instruction in instructions.lines() {
        if let Some(next_mask) = instruction.strip_prefix("mask = ") {
            mask = ["0", "1"].map(|bit| {
                let replaced = next_mask.replace('X', bit);
                Value::from_str_radix(&replaced, 2).expect("mask should consist of '0's and '1's")
            });
        } else {
            let (address, value) = address_and_value(instruction);
            let masked_value = (value | mask[0]) & mask[1];
            memory.insert(address, masked_value);
        }
    }
    memory
}

fn execute_program_with_floating_bits(instructions: &str) -> [[Value; 2]; 32] {
    let mut mask: &[u8] = b"";
    let mut memory: [[Value; 2]; 32] = Default::default();
    for instruction in instructions.lines() {
        if let Some(next_mask) = instruction.strip_prefix("mask = ") {
            mask = next_mask.as_bytes();
        } else {
            let (address, value) = address_and_value(instruction);
            for (memory_bit, &mask_bit, address_bit) in
                izip!(&mut memory, mask, address.view_bits::<Msb0>())
            {
                match mask_bit {
                    b'0' => memory_bit[usize::from(*address_bit)] = value,
                    b'1' => memory_bit[1] = value,
                    b'X' => {
                        memory_bit[0] = value;
                        memory_bit[1] = value;
                    }
                    _ => panic!("mask bit should be '0', '1', or 'X'"),
                }
            }
        }
    }
    memory
}

fn address_and_value(instruction: &str) -> (Address, Value) {
    let (address, value) = instruction
        .split_once(" = ")
        .expect("address and value should be separated by and equal sign");
    let address = usizes(address)[0].cast();
    let value: Value = value.parse().expect("value should be numeric");
    (address, value)
}

#[cfg(test)]
mod tests {
    use infrastructure::{test, Input, Puzzle};

    use crate::tests::{input, test_on_input};

    const DAY: usize = 14;

    #[test]
    fn first_answer_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 165);
    }

    #[test]
    fn first_answer_puzzle_input() {
        test_on_input(
            DAY,
            Puzzle::First,
            Input::PuzzleInput,
            5_902_420_735_773_usize,
        );
    }

    #[test]
    fn second_answer_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 208);
    }

    #[test]
    fn second_answer_puzzle_input() {
        test_on_input(
            DAY,
            Puzzle::Second,
            Input::PuzzleInput,
            741_745_043_105_674_usize,
        );
    }

    #[test]
    fn execute_program_with_floating_bits() {
        let input = input(DAY, Input::Example(1));
        let actual = super::execute_program_with_floating_bits(&input);
        let expected = [
            [0, 0],
            [0, 0],
            [0, 0],
            [0, 0],
            [0, 0],
            [0, 0],
            [0, 0],
            [0, 0],
            [0, 0],
            [0, 0],
            [0, 0],
            [0, 0],
            [0, 0],
            [0, 0],
            [0, 0],
            [0, 0],
            [0, 0],
            [0, 0],
            [0, 0],
            [0, 0],
            [0, 0],
            [0, 0],
            [0, 0],
            [0, 0],
            [0, 0],
            [0, 0],
            [100, 100],
            [0, 1],
            [1, 1],
            [0, 0],
            [1, 1],
            [1, 1],
        ];
        debug_assert_eq!(actual, expected);
    }
}
