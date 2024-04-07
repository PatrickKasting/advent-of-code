use std::collections::HashMap;

use crate::strings::char_at;

type Monkey<'input> = (&'input str, Expression<'input>);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
enum Expression<'input> {
    Add(&'input str, &'input str),
    Sub(&'input str, &'input str),
    Mul(&'input str, &'input str),
    Div(&'input str, &'input str),
    Equal(&'input str, &'input str),
    Constant(Number),
    Unknown,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
enum Reduction {
    Number(Number),
    Rearrangements(Vec<Operation>),
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
enum Operation {
    Add(Number),
    Sub(Number),
    Mul(Number),
    Div(Number),
    Inverse,
}

type Number = f64;

pub fn first(input: &str) -> String {
    let monkeys = monkeys(input);
    let Reduction::Number(number) = reduce(&monkeys, "root") else {
        panic!("expression should contain no unknowns");
    };
    number.to_string()
}

pub fn second(input: &str) -> String {
    let mut monkeys = monkeys(input);
    correct_operations(&mut monkeys);
    let Reduction::Number(number) = reduce(&monkeys, "root") else {
        panic!("equation should reduce to a number");
    };
    number.to_string()
}

fn correct_operations(monkeys: &mut HashMap<&str, Expression>) {
    let (Expression::Add(left, right)
    | Expression::Sub(left, right)
    | Expression::Mul(left, right)
    | Expression::Div(left, right)) = monkeys["root"]
    else {
        panic!("root must be a binary operation");
    };
    monkeys.insert("root", Expression::Equal(left, right));
    monkeys.insert("humn", Expression::Unknown);
}

fn reduce(monkeys: &HashMap<&str, Expression>, name: &str) -> Reduction {
    match monkeys[name] {
        Expression::Add(left, right) => match (reduce(monkeys, left), reduce(monkeys, right)) {
            (Reduction::Number(left), Reduction::Number(right)) => Reduction::Number(left + right),
            (Reduction::Number(known), Reduction::Rearrangements(mut rearrangements))
            | (Reduction::Rearrangements(mut rearrangements), Reduction::Number(known)) => {
                rearrangements.push(Operation::Sub(known));
                Reduction::Rearrangements(rearrangements)
            }
            (Reduction::Rearrangements(_), Reduction::Rearrangements(_)) => {
                panic!("equation should contain at most one unknown");
            }
        },
        Expression::Sub(left, right) => match (reduce(monkeys, left), reduce(monkeys, right)) {
            (Reduction::Number(left), Reduction::Number(right)) => Reduction::Number(left - right),
            (Reduction::Number(known), Reduction::Rearrangements(mut rearrangements)) => {
                rearrangements.push(Operation::Mul(-1.0));
                rearrangements.push(Operation::Sub(known));
                Reduction::Rearrangements(rearrangements)
            }
            (Reduction::Rearrangements(mut rearrangements), Reduction::Number(known)) => {
                rearrangements.push(Operation::Add(known));
                Reduction::Rearrangements(rearrangements)
            }
            (Reduction::Rearrangements(_), Reduction::Rearrangements(_)) => {
                panic!("equation should contain at most one unknown");
            }
        },
        Expression::Mul(left, right) => match (reduce(monkeys, left), reduce(monkeys, right)) {
            (Reduction::Number(left), Reduction::Number(right)) => Reduction::Number(left * right),
            (Reduction::Number(known), Reduction::Rearrangements(mut rearrangements))
            | (Reduction::Rearrangements(mut rearrangements), Reduction::Number(known)) => {
                rearrangements.push(Operation::Div(known));
                Reduction::Rearrangements(rearrangements)
            }
            (Reduction::Rearrangements(_), Reduction::Rearrangements(_)) => {
                panic!("equation should contain at most one unknown");
            }
        },
        Expression::Div(left, right) => match (reduce(monkeys, left), reduce(monkeys, right)) {
            (Reduction::Number(left), Reduction::Number(right)) => Reduction::Number(left / right),
            (Reduction::Number(known), Reduction::Rearrangements(mut rearrangements)) => {
                rearrangements.push(Operation::Inverse);
                rearrangements.push(Operation::Div(known));
                Reduction::Rearrangements(rearrangements)
            }
            (Reduction::Rearrangements(mut rearrangements), Reduction::Number(known)) => {
                rearrangements.push(Operation::Mul(known));
                Reduction::Rearrangements(rearrangements)
            }
            (Reduction::Rearrangements(_), Reduction::Rearrangements(_)) => {
                panic!("equation should contain at most one unknown");
            }
        },
        Expression::Equal(left, right) => match (reduce(monkeys, left), reduce(monkeys, right)) {
            (Reduction::Number(_), Reduction::Number(_)) => {
                panic!("exactly one side of the equation should contain an unknown")
            }
            (Reduction::Number(mut known), Reduction::Rearrangements(mut rearrangements))
            | (Reduction::Rearrangements(mut rearrangements), Reduction::Number(mut known)) => {
                while let Some(operation) = rearrangements.pop() {
                    match operation {
                        Operation::Add(number) => known += number,
                        Operation::Sub(number) => known -= number,
                        Operation::Mul(number) => known *= number,
                        Operation::Div(number) => known /= number,
                        Operation::Inverse => known = 1.0 / known,
                    }
                }
                Reduction::Number(known)
            }
            (Reduction::Rearrangements(_), Reduction::Rearrangements(_)) => {
                panic!("equation should contain at most one unknown");
            }
        },
        Expression::Constant(constant) => Reduction::Number(constant),
        Expression::Unknown => Reduction::Rearrangements(vec![]),
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
            '+' => Expression::Add,
            '-' => Expression::Sub,
            '*' => Expression::Mul,
            '/' => Expression::Div,
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

    #[test]
    fn second_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 301);
    }

    #[test]
    fn second_input() {
        test_on_input(
            DAY,
            Puzzle::Second,
            Input::PuzzleInput,
            3_219_579_395_609_usize,
        );
    }
}
