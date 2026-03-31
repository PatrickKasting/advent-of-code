use easy_cast::Cast;
use itertools::Itertools;
use shared::string::usizes;

struct Computer {
    a: Register,
    b: Register,
    c: Register,

    ip: usize,

    outputs: Vec<Register>,
}
type Register = usize;

pub fn first_answer(input: &str) -> String {
    let (mut computer, program) = computer_and_program(input);
    execute(&mut computer, &program);
    computer.outputs.iter().join(",")
}

pub fn second_answer(input: &str) -> String {
    todo!()
}

fn execute(computer: &mut Computer, program: &[Register]) {
    while let Some(&[instruction, operand]) = program.get(computer.ip..computer.ip + 2) {
        match instruction {
            0 => adv(computer, operand),
            1 => bxl(computer, operand),
            2 => bst(computer, operand),
            3 => jnz(computer, operand),
            4 => bxc(computer, operand),
            5 => out(computer, operand),
            6 => bdv(computer, operand),
            7 => cdv(computer, operand),
            _ => panic!("instruction should be known"),
        };
    }
}

fn adv(computer: &mut Computer, operand: usize) {
    computer.a /= 2_usize.pow(combo(computer, operand).cast());
    computer.ip += 2;
}

fn bxl(computer: &mut Computer, operand: usize) {
    computer.b ^= operand;
    computer.ip += 2;
}

fn bst(computer: &mut Computer, operand: usize) {
    computer.b = combo(computer, operand) % 8;
    computer.ip += 2;
}

fn jnz(computer: &mut Computer, operand: usize) {
    if computer.a == 0 {
        computer.ip += 2;
    } else {
        computer.ip = operand;
    }
}

fn bxc(computer: &mut Computer, _operand: usize) {
    computer.b ^= computer.c;
    computer.ip += 2;
}

fn out(computer: &mut Computer, operand: usize) {
    computer.outputs.push(combo(computer, operand) % 8);
    computer.ip += 2;
}

fn bdv(computer: &mut Computer, operand: usize) {
    computer.b = computer.a / 2_usize.pow(combo(computer, operand).cast());
    computer.ip += 2;
}

fn cdv(computer: &mut Computer, operand: usize) {
    computer.c = computer.a / 2_usize.pow(combo(computer, operand).cast());
    computer.ip += 2;
}

fn combo(Computer { a, b, c, .. }: &Computer, operand: Register) -> Register {
    match operand {
        0..=3 => operand,
        4 => *a,
        5 => *b,
        6 => *c,
        7 => panic!("the combo operand 7 should not appear in valid programs"),
        _ => panic!("operand should be only three bits"),
    }
}

fn computer_and_program(input: &str) -> (Computer, Vec<Register>) {
    let numbers = usizes(input);
    let computer = Computer {
        a: numbers[0],
        b: numbers[1],
        c: numbers[2],
        ip: 0,
        outputs: vec![],
    };
    let program = Vec::from(&numbers[3..]);
    (computer, program)
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 17;

    #[test]
    fn first_answer_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn first_answer_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 396);
    }

    // #[test]
    // fn second_answer_example() {
    //     test_on_input(DAY, Puzzle::Second, Input::Example(0), 34);
    // }

    // #[test]
    // fn second_answer_input() {
    //     test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 1200);
    // }
}
