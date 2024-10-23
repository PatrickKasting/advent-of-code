use easy_cast::Conv;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    Acc(Number),
    Jmp(Number),
    Nop(Number),
}

type Number = isize;

pub fn first_answer(input: &str) -> String {
    let instructions = instructions(input);
    let (_, accumulator) = run(&instructions);
    accumulator.to_string()
}

pub fn second_answer(input: &str) -> String {
    let mut instructions = instructions(input);
    accumulator_after_termination_of_repaired_program(&mut instructions).to_string()
}

fn run(instructions: &[Instruction]) -> (bool, Number) {
    let mut accumulator = 0;
    let mut program_counter = 0;
    let mut executed_instructions = vec![false; instructions.len()];
    while executed_instructions.get(program_counter) == Some(&false) {
        executed_instructions[program_counter] = true;
        match instructions[program_counter] {
            Instruction::Acc(number) => {
                accumulator += number;
                program_counter += 1;
            }
            Instruction::Jmp(offset) => {
                program_counter = usize::conv(isize::conv(program_counter) + offset);
            }
            Instruction::Nop(_) => program_counter += 1,
        }
    }
    let normal_termination = program_counter == instructions.len();
    (normal_termination, accumulator)
}

fn accumulator_after_termination_of_repaired_program(instructions: &mut [Instruction]) -> Number {
    for index in 0..instructions.len() {
        let original_instruction = instructions[index];
        let replacement_instruction = match original_instruction {
            Instruction::Acc(_) => None,
            Instruction::Jmp(value) => Some(Instruction::Nop(value)),
            Instruction::Nop(value) => Some(Instruction::Jmp(value)),
        };
        if let Some(replacement_instruction) = replacement_instruction {
            instructions[index] = replacement_instruction;

            let (normal_termination, accumulator) = run(instructions);
            if normal_termination {
                return accumulator;
            }

            instructions[index] = original_instruction;
        }
    }
    unreachable!("one instruction swap should yield a terminating program")
}

fn instructions(input: &str) -> Vec<Instruction> {
    input.lines().map(instruction).collect_vec()
}

fn instruction(line: &str) -> Instruction {
    let (instruction, value) = line
        .split_once(' ')
        .expect("instruction and value should be separated by a space");
    let value = value.parse().expect("value should be numeric");
    match instruction {
        "nop" => Instruction::Nop(value),
        "acc" => Instruction::Acc(value),
        "jmp" => Instruction::Jmp(value),
        _ => panic!("instruction should be 'nop', 'add', or 'jmp'"),
    }
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 8;

    #[test]
    fn first_answer_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 5);
    }

    #[test]
    fn first_answer_puzzle_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 1489);
    }

    #[test]
    fn second_answer_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 8);
    }

    #[test]
    fn second_answer_puzzle_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 1539);
    }
}
