use std::ops::{Add, Div, Mul, Sub};

use ahash::AHashMap;

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
    Reciprocal,
}

type Number = f64;

pub fn first_answer(input: &str) -> String {
    let monkeys = monkeys(input);
    let Reduction::Number(number) = reduce(&monkeys, "root") else {
        panic!("expression should contain no unknowns");
    };
    number.to_string()
}

pub fn second_answer(input: &str) -> String {
    let mut monkeys = monkeys(input);
    correct_operations(&mut monkeys);
    let Reduction::Number(number) = reduce(&monkeys, "root") else {
        panic!("equation should reduce to a number");
    };
    number.to_string()
}

fn correct_operations(monkeys: &mut AHashMap<&str, Expression>) {
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

fn reduce<'input>(monkeys: &AHashMap<&'input str, Expression>, name: &'input str) -> Reduction {
    match monkeys[name] {
        Expression::Equal(left, right) => solve(monkeys, left, right),
        Expression::Add(left, right) => {
            let both_known = <f64 as Add>::add;
            let one_known = |known| vec![Operation::Sub(known)];
            reduce_binary_operator(monkeys, left, right, both_known, one_known, one_known)
        }
        Expression::Sub(left, right) => {
            let both_known = <f64 as Sub>::sub;
            let left_known = |known| vec![Operation::Sub(known), Operation::Mul(-1.0)];
            let right_known = |known| vec![Operation::Add(known)];
            reduce_binary_operator(monkeys, left, right, both_known, left_known, right_known)
        }
        Expression::Mul(left, right) => {
            let both_known = <f64 as Mul>::mul;
            let one_known = |known| vec![Operation::Div(known)];
            reduce_binary_operator(monkeys, left, right, both_known, one_known, one_known)
        }
        Expression::Div(left, right) => {
            let both_known = <f64 as Div>::div;
            let left_known = |known| vec![Operation::Div(known), Operation::Reciprocal];
            let right_known = |known| vec![Operation::Mul(known)];
            reduce_binary_operator(monkeys, left, right, both_known, left_known, right_known)
        }
        Expression::Constant(constant) => Reduction::Number(constant),
        Expression::Unknown => Reduction::Rearrangements(vec![]),
    }
}

fn solve<'input>(
    monkeys: &AHashMap<&'input str, Expression<'input>>,
    left: &'input str,
    right: &'input str,
) -> Reduction {
    match (reduce(monkeys, left), reduce(monkeys, right)) {
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
                    Operation::Reciprocal => known = known.recip(),
                }
            }
            Reduction::Number(known)
        }
        (Reduction::Rearrangements(_), Reduction::Rearrangements(_)) => {
            panic!("exactly one side of the equation should contain an unknown");
        }
    }
}

fn reduce_binary_operator<'input>(
    monkeys: &AHashMap<&'input str, Expression<'input>>,
    left: &'input str,
    right: &'input str,
    both_known: impl FnOnce(Number, Number) -> Number,
    left_known: impl FnOnce(Number) -> Vec<Operation>,
    right_known: impl FnOnce(Number) -> Vec<Operation>,
) -> Reduction {
    match (reduce(monkeys, left), reduce(monkeys, right)) {
        (Reduction::Number(left), Reduction::Number(right)) => {
            Reduction::Number(both_known(left, right))
        }
        (Reduction::Number(known), Reduction::Rearrangements(mut rearrangements)) => {
            rearrangements.extend(left_known(known).into_iter().rev());
            Reduction::Rearrangements(rearrangements)
        }
        (Reduction::Rearrangements(mut rearrangements), Reduction::Number(known)) => {
            rearrangements.extend(right_known(known).into_iter().rev());
            Reduction::Rearrangements(rearrangements)
        }
        (Reduction::Rearrangements(_), Reduction::Rearrangements(_)) => {
            panic!("equation should contain at most one unknown");
        }
    }
}

fn monkeys(input: &str) -> AHashMap<&str, Expression> {
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
        let operator = match &str[5..6] {
            "+" => Expression::Add,
            "-" => Expression::Sub,
            "*" => Expression::Mul,
            "/" => Expression::Div,
            _ => panic!("operator should be '+', '-', '*', ot '/'"),
        };
        operator(&str[0..4], &str[7..11])
    }
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 21;

    #[test]
    fn first_answer_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 152);
    }

    #[test]
    fn first_answer_input() {
        test_on_input(
            DAY,
            Puzzle::First,
            Input::PuzzleInput,
            291_425_799_367_130_usize,
        );
    }

    #[test]
    fn second_answer_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 301);
    }

    #[test]
    fn second_answer_input() {
        test_on_input(
            DAY,
            Puzzle::Second,
            Input::PuzzleInput,
            3_219_579_395_609_usize,
        );
    }
}
