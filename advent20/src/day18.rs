use std::ops::{Add, Mul};

type Value = usize;

pub fn first_answer(input: &str) -> String {
    input
        .lines()
        .map(|line| value(line, left_to_right))
        .sum::<Value>()
        .to_string()
}

pub fn second_answer(input: &str) -> String {
    input
        .lines()
        .map(|line| value(line, product))
        .sum::<Value>()
        .to_string()
}

fn value(line: &str, expression: fn(&str) -> (&str, Value)) -> Value {
    let (remaining, value) = expression(line);
    debug_assert!(remaining.is_empty(), "entire line should be parsed");
    value
}

fn left_to_right(str: &str) -> (&str, Value) {
    let (mut str, mut value) = term(str, left_to_right);
    while str.get(0..1) == Some(" ") {
        let (operator, remaining) = str.split_at(3);
        let operator = match operator {
            " + " => <Value as Add>::add,
            " * " => <Value as Mul>::mul,
            _ => panic!("operator should be ' + ' or ' * '"),
        };
        let (remaining, rhs) = term(remaining, left_to_right);
        value = operator(value, rhs);
        str = remaining;
    }
    (str, value)
}

fn product(str: &str) -> (&str, Value) {
    let (mut str, mut product) = sum(str);
    while str.get(0..3) == Some(" * ") {
        let (remaining, factor) = sum(&str[3..]);
        product *= factor;
        str = remaining;
    }
    (str, product)
}

fn sum(str: &str) -> (&str, Value) {
    let (mut str, mut sum) = term(str, product);
    while str.get(0..3) == Some(" + ") {
        let (remaining, term) = term(&str[3..], product);
        sum += term;
        str = remaining;
    }
    (str, sum)
}

fn term(str: &str, expression: fn(&str) -> (&str, Value)) -> (&str, Value) {
    if &str[0..1] == "(" {
        parenthesis(str, expression)
    } else {
        number(str)
    }
}

fn parenthesis(str: &str, expression: fn(&str) -> (&str, Value)) -> (&str, Value) {
    let content = str
        .strip_prefix('(')
        .expect("first character should be opening parenthesis");
    let (remaining, value) = expression(content);
    let remaining = remaining
        .strip_prefix(')')
        .expect("last character should be closing parenthesis");
    (remaining, value)
}

fn number(str: &str) -> (&str, Value) {
    let end = str
        .find(|char: char| !char.is_numeric())
        .unwrap_or(str.len());
    let (number, remaining) = str.split_at(end);
    let number = number.parse().expect("number should be numeric");
    (remaining, number)
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 18;

    #[test]
    fn first_answer_example() {
        let expected = 71 + 51 + 26 + 437 + 12240 + 13632;
        test_on_input(DAY, Puzzle::First, Input::Example(0), expected);
    }

    #[test]
    fn first_answer_puzzle_input() {
        test_on_input(
            DAY,
            Puzzle::First,
            Input::PuzzleInput,
            45_283_905_029_161_usize,
        );
    }

    #[test]
    fn second_answer_example() {
        let expected = 231 + 51 + 46 + 1445 + 669_060 + 23340;
        test_on_input(DAY, Puzzle::Second, Input::Example(0), expected);
    }

    #[test]
    fn second_answer_puzzle_input() {
        test_on_input(
            DAY,
            Puzzle::Second,
            Input::PuzzleInput,
            216_975_281_211_165_usize,
        );
    }
}
