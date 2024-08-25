type ModelNumber = isize;

// Determined using the test function 'monad', modular arithmetic, and spreadsheets
const GREATEST_MODEL_NUMBER: ModelNumber = 99429795993929;
const SMALLEST_MODEL_NUMBER: ModelNumber = 18113181571611;

pub fn first(_input: &str) -> String {
    GREATEST_MODEL_NUMBER.to_string()
}

pub fn second(_input: &str) -> String {
    SMALLEST_MODEL_NUMBER.to_string()
}

#[cfg(test)]
mod tests {
    use std::ops::{Range, RangeFrom};

    use rand::Rng;

    use infrastructure::Input;

    use super::*;

    const DAY: usize = 24;

    type Value = isize;
    const NUM_MODEL_NUMBER_DIGITS: usize = 14;
    const NUM_VARIABLES: usize = 4;
    type Alu = [Value; NUM_VARIABLES];

    const OPCODE: Range<usize> = 0..3;
    const FIRST: Range<usize> = 4..5;
    const SECOND: RangeFrom<usize> = 6..;
    const Z: usize = 3;

    #[test]
    fn first_input() {
        assert_eq!(monad(GREATEST_MODEL_NUMBER), 0);
    }

    #[test]
    fn second_input() {
        assert_eq!(monad(SMALLEST_MODEL_NUMBER), 0);
    }

    fn random_model_number() -> ModelNumber {
        let random: usize = rand::thread_rng().gen();
        (random % 10usize.pow(NUM_MODEL_NUMBER_DIGITS as u32)) as ModelNumber
    }

    fn index(operand: &str) -> usize {
        let variable = operand
            .chars()
            .next()
            .expect("operand should be digits or a variable");
        (variable as u32 - 'w' as u32) as usize
    }

    fn read(alu: &Alu, operand: &str) -> Value {
        if let Ok(number) = operand.parse() {
            number
        } else {
            alu[index(operand)]
        }
    }

    fn write(alu: &mut Alu, operand: &str, value: Value) {
        alu[index(operand)] = value
    }

    fn input(alu: &mut Alu, instruction: &str, inputs: &mut impl Iterator<Item = Value>) {
        write(
            alu,
            &instruction[FIRST],
            inputs.next().expect("input should not run out"),
        )
    }

    fn binary_operation(
        alu: &mut Alu,
        instruction: &str,
        operation: impl FnOnce(Value, Value) -> Value,
    ) {
        let left = read(alu, &instruction[FIRST]);
        let right = read(alu, &instruction[SECOND]);
        let value = operation(left, right);
        write(alu, &instruction[FIRST], value);
    }

    fn run(program: &str, mut inputs: impl Iterator<Item = Value>) -> Alu {
        let mut alu = [0, 0, 0, 0];
        for instruction in program.lines() {
            match &instruction[OPCODE] {
                "inp" => input(&mut alu, instruction, &mut inputs),
                "add" => binary_operation(&mut alu, instruction, |left, right| left + right),
                "mul" => binary_operation(&mut alu, instruction, |left, right| left * right),
                "div" => binary_operation(&mut alu, instruction, |left, right| left / right),
                "mod" => binary_operation(&mut alu, instruction, |left, right| left % right),
                "eql" => {
                    let operation = |left, right| if left == right { 1 } else { 0 };
                    binary_operation(&mut alu, instruction, operation)
                }
                _ => panic!("opcode should be one of the supported six"),
            }
        }
        alu
    }

    fn assert_execution(program: &str, inputs: impl IntoIterator<Item = Value>, expected: Alu) {
        let actual = run(program, inputs.into_iter());
        assert_eq!(actual, expected);
    }

    #[test]
    fn negation() {
        let program = "\
            inp x\n\
            mul x -1\n\
        ";
        assert_execution(program, [6], [0, -6, 0, 0]);
    }

    #[test]
    fn equality() {
        let program = "\
            inp z\n\
            inp x\n\
            mul z 3\n\
            eql z x\n\
        ";
        assert_execution(program, [2, 6], [0, 6, 0, 1]);
    }

    #[test]
    fn arithmetic() {
        let program = "\
            inp w\n\
            add z w\n\
            mod z 2\n\
            div w 2\n\
            add y w\n\
            mod y 2\n\
            div w 2\n\
            add x w\n\
            mod x 2\n\
            div w 2\n\
            mod w 2\n\
        ";
        assert_execution(program, [8], [1, 0, 0, 0]);
    }

    fn digits(mut model_number: ModelNumber) -> [Value; NUM_MODEL_NUMBER_DIGITS] {
        let mut inputs: [Value; NUM_MODEL_NUMBER_DIGITS] = Default::default();
        for digit_index in (0..NUM_MODEL_NUMBER_DIGITS).rev() {
            inputs[digit_index] = model_number % 10;
            model_number /= 10;
        }
        inputs
    }

    #[test]
    fn to_digits() {
        let model_number = random_model_number();
        let actual = digits(model_number);
        let expected: Vec<Value> = format!("{model_number:014}")
            .chars()
            .map(|digit| digit.to_digit(10).expect("only digits should be converted") as Value)
            .collect();
        assert_eq!(actual[..], expected[..], "model_number: {model_number}");
    }

    fn monad(model_number: ModelNumber) -> Value {
        let inputs: [Value; NUM_MODEL_NUMBER_DIGITS] = digits(model_number);
        let accumulator_divisors: [Value; NUM_MODEL_NUMBER_DIGITS] =
            [1, 1, 1, 1, 1, 26, 26, 1, 26, 1, 26, 26, 26, 26];
        let modulus_summands: [Value; NUM_MODEL_NUMBER_DIGITS] =
            [10, 10, 14, 11, 14, -14, 0, 10, -10, 13, -12, -3, -11, -2];
        let input_summands: [Value; NUM_MODEL_NUMBER_DIGITS] =
            [2, 4, 8, 7, 12, 7, 10, 14, 2, 6, 8, 11, 5, 11];
        let mut z = 0;
        for index in 0..NUM_MODEL_NUMBER_DIGITS {
            let x = z % 26 + modulus_summands[index];
            let x = if x != inputs[index] { 1 } else { 0 };
            z /= accumulator_divisors[index];
            z *= 25 * x + 1;
            z += (inputs[index] + input_summands[index]) * x;
        }
        z
    }

    #[test]
    fn equivalence() {
        let program = crate::tests::input(DAY, Input::PuzzleInput);
        for _ in 0..100 {
            let model_number = random_model_number();
            let inputs = digits(model_number).into_iter();
            let actual = monad(model_number);
            let expected = run(&program, inputs)[Z];
            assert_eq!(actual, expected, "model_number: {model_number}");
        }
    }
}
