use std::ops::{Add, Div, Mul, Rem};

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct Alu {
    w: Number,
    x: Number,
    y: Number,
    z: Number,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    Inp(Variable),
    Add(Variable, Value),
    Mul(Variable, Value),
    Div(Variable, Value),
    Mod(Variable, Value),
    Eql(Variable, Value),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Variable {
    W,
    X,
    Y,
    Z,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Value {
    Variable(Variable),
    Number(Number),
}

type Digit = Number;
type Number = isize;

pub fn first(input: &str) -> String {
    let monad = monad(input);
    largest_valid_model_number(&monad).to_string()
}

pub fn second(_input: &str) -> String {
    todo!()
}

fn largest_valid_model_number(monad: &[Instruction]) -> Number {
    model_numbers()
        .find(|&number| is_valid(number, monad))
        .expect("at least one model number should be accepted")
}

fn model_numbers() -> impl Iterator<Item = Number> {
    (11111111111111..=99999999999999)
        .rev()
        .filter(|&number| !contains_zero(number))
}

fn contains_zero(mut model_number: Number) -> bool {
    while model_number > 0 {
        if model_number % 10 == 0 {
            return true;
        }
        model_number /= 10;
    }
    false
}

fn is_valid(model_number: Number, monad: &[Instruction]) -> bool {
    let mut inputs = digits(model_number).into_iter();
    let mut alu = Alu::default();
    for &instruction in monad {
        execute(&mut inputs, instruction, &mut alu);
    }
    debug_assert_eq!(inputs.next(), None, "all 14 inputs should be used");
    alu.z == 0
}

fn digits(mut model_number: Number) -> Vec<Number> {
    let mut digits = vec![];
    while model_number > 0 {
        digits.push(model_number % 10);
        model_number /= 10;
    }
    digits.reverse();
    digits
}

fn execute(inputs: &mut impl Iterator<Item = Digit>, instruction: Instruction, alu: &mut Alu) {
    match instruction {
        Instruction::Inp(variable) => {
            let number = inputs.next().expect("inputs should not run out");
            assign(alu, variable, number)
        }
        Instruction::Add(variable, value) => binary(<Number as Add>::add, alu, variable, value),
        Instruction::Mul(variable, value) => binary(<Number as Mul>::mul, alu, variable, value),
        Instruction::Div(variable, value) => binary(<Number as Div>::div, alu, variable, value),
        Instruction::Mod(variable, value) => binary(<Number as Rem>::rem, alu, variable, value),
        Instruction::Eql(variable, value) => {
            binary(|lhs, rhs| (lhs == rhs).into(), alu, variable, value)
        }
    }
}

fn binary(
    operation: impl FnOnce(Number, Number) -> Number,
    alu: &mut Alu,
    variable: Variable,
    value: Value,
) {
    let rhs = match value {
        Value::Variable(variable) => read(alu, variable),
        Value::Number(number) => number,
    };
    let lhs = read(alu, variable);
    assign(alu, variable, operation(lhs, rhs));
}

fn read(alu: &Alu, variable: Variable) -> Number {
    match variable {
        Variable::W => alu.w,
        Variable::X => alu.x,
        Variable::Y => alu.y,
        Variable::Z => alu.z,
    }
}

fn assign(alu: &mut Alu, variable: Variable, number: Number) {
    match variable {
        Variable::W => alu.w = number,
        Variable::X => alu.x = number,
        Variable::Y => alu.y = number,
        Variable::Z => alu.z = number,
    }
}

fn monad(input: &str) -> Vec<Instruction> {
    input.lines().map(instruction).collect_vec()
}

fn instruction(line: &str) -> Instruction {
    let mut tokens = line.split_whitespace();
    let mut token = || {
        tokens
            .next()
            .expect("instruction should contain correct number of tokens")
    };
    match token() {
        "inp" => Instruction::Inp(variable(token())),
        "add" => Instruction::Add(variable(token()), value(token())),
        "mul" => Instruction::Mul(variable(token()), value(token())),
        "div" => Instruction::Div(variable(token()), value(token())),
        "mod" => Instruction::Mod(variable(token()), value(token())),
        "eql" => Instruction::Eql(variable(token()), value(token())),
        _ => panic!("instruction should be 'inp', 'add', 'mul', 'div', 'mod', 'eql'"),
    }
}

fn value(str: &str) -> Value {
    if let Ok(number) = str.parse() {
        Value::Number(number)
    } else {
        Value::Variable(variable(str))
    }
}

fn variable(str: &str) -> Variable {
    match str {
        "w" => Variable::W,
        "x" => Variable::X,
        "y" => Variable::Y,
        "z" => Variable::Z,
        _ => panic!("variable should be 'w', 'x', 'y', or 'z'"),
    }
}

#[cfg(test)]
mod tests {
    use infrastructure::{test, Input, Puzzle};

    use super::*;
    use crate::tests::test_on_input;

    const DAY: usize = 24;

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 99429795993929isize);
    }

    #[test]
    fn second_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 26984457539usize);
    }

    #[test]
    fn second_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 1681503251694usize);
    }

    #[test]
    fn contains_zero() {
        let cases = [
            (1, false),
            (40, true),
            (99, false),
            (209736, true),
            (90977998992999isize, true),
            (91999899931199isize, false),
        ];
        test::cases(super::contains_zero, cases);
    }

    #[test]
    fn monad() {
        let input = "\
            inp z\n\
            inp x\n\
            mul z 3\n\
            eql z x\n\
        ";
        let actual = super::monad(input);
        let expected = vec![
            Instruction::Inp(Variable::Z),
            Instruction::Inp(Variable::X),
            Instruction::Mul(Variable::Z, Value::Number(3)),
            Instruction::Eql(Variable::Z, Value::Variable(Variable::X)),
        ];
        assert_eq!(actual, expected);
    }
}
