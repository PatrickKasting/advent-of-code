use std::collections::HashMap;

use crate::strings::char_at;

type Monkey<'input> = (&'input str, Expression<'input>);

enum Expression<'input> {
    Addition(&'input str, &'input str),
    Subtraction(&'input str, &'input str),
    Multiplication(&'input str, &'input str),
    Division(&'input str, &'input str),
    Constant(Number),
}

type Number = isize;

pub fn first(input: &str) -> String {
    let monkeys = monkeys(input);
    number(&monkeys, "root").to_string()
}

pub fn second(_input: &str) -> String {
    todo!();
}

fn number(monkeys: &HashMap<&str, Expression>, name: &str) -> Number {
    match monkeys[name] {
        Expression::Addition(left, right) => number(monkeys, left) + number(monkeys, right),
        Expression::Subtraction(left, right) => number(monkeys, left) - number(monkeys, right),
        Expression::Multiplication(left, right) => number(monkeys, left) * number(monkeys, right),
        Expression::Division(left, right) => number(monkeys, left) / number(monkeys, right),
        Expression::Constant(constant) => constant,
    }
}

fn monkeys(input: &str) -> HashMap<&str, Expression> {
    input.lines().map(monkey).collect()
}

fn monkey(line: &str) -> Monkey {
    let (name, expression) = (&line[0..4], &line[6..]);
    (name, self::expression(expression))
}

fn expression(str: &str) -> Expression {
    if let Ok(constant) = str.parse() {
        Expression::Constant(constant)
    } else {
        let operator = match char_at(str, 5) {
            '+' => Expression::Addition,
            '-' => Expression::Subtraction,
            '*' => Expression::Multiplication,
            '/' => Expression::Division,
            _ => panic!("operator should be '+', '-', '*', ot '/'"),
        };
        operator(&str[0..4], &str[7..11])
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::test_on_input;
    use crate::{Input, Puzzle};

    const DAY: usize = 21;

    #[test]
    fn first_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 152);
    }

    #[test]
    fn first_input() {
        test_on_input(
            DAY,
            Puzzle::First,
            Input::PuzzleInput,
            291_425_799_367_130_usize,
        );
    }

    // #[test]
    // fn second_example() {
    //     test_on_input(DAY, Puzzle::Second, Input::Example(0), 24_933_642);
    // }

    // #[test]
    // fn second_input() {
    //     test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 404_395);
    // }
}
