use itertools::Itertools;

use crate::string::usizes;

struct Monkey {
    items: Vec<Worry>,
    operation: Operation,
    divisor: Worry,
    receivers: [usize; 2],
}

type Worry = usize;
type Operation = Box<dyn Fn(Worry) -> Worry>;

pub fn first(input: &str) -> String {
    monkey_business(&mut monkeys(input), 20, 3).to_string()
}

pub fn second(input: &str) -> String {
    monkey_business(&mut monkeys(input), 10000, 1).to_string()
}

fn monkey_business(monkeys: &mut [Monkey], number_of_rounds: usize, worry_divisor: Worry) -> Worry {
    let worry_modulo = monkeys.iter().map(|monkey| monkey.divisor).product();
    let mut number_of_inspections = vec![0; monkeys.len()];
    for _ in 0..number_of_rounds {
        round(
            monkeys,
            worry_divisor,
            worry_modulo,
            &mut number_of_inspections,
        );
    }
    number_of_inspections.sort_unstable_by_key(|inspections| Worry::MAX - inspections);
    number_of_inspections[0] * number_of_inspections[1]
}

fn round(
    monkeys: &mut [Monkey],
    worry_divisor: Worry,
    worry_modulo: Worry,
    number_of_inspections: &mut [usize],
) {
    for monkey_index in 0..monkeys.len() {
        for item_index in 0..monkeys[monkey_index].items.len() {
            number_of_inspections[monkey_index] += 1;

            let monkey = &monkeys[monkey_index];
            let worry = monkey.items[item_index];
            let worry = ((monkey.operation)(worry) / worry_divisor) % worry_modulo;
            let destination = if worry % monkey.divisor == 0 {
                monkey.receivers[0]
            } else {
                monkey.receivers[1]
            };
            monkeys[destination].items.push(worry);
        }
        monkeys[monkey_index].items.clear();
    }
}

fn monkeys(input: &str) -> Vec<Monkey> {
    input.split("\n\n").map(monkey).collect_vec()
}

fn monkey(str: &str) -> Monkey {
    let lines = str.lines().collect_vec();

    let items = usizes(lines[1]);
    let (_, rhs) = lines[2]
        .split_once(" = ")
        .expect("operation should contain an equal sign");
    let operation = operation(rhs);
    let divisor = usizes(lines[3])[0];
    let receivers = [4, 5].map(|index| usizes(lines[index])[0]);
    Monkey {
        items,
        operation,
        divisor,
        receivers,
    }
}

fn operation(rhs: &str) -> Operation {
    let tokens = rhs.split_whitespace().collect_vec();
    let [left, right] = [tokens[0], tokens[2]].map(value);
    match *tokens.get(1).expect("operation should have three tokens") {
        "+" => Box::new(move |old| left(old) + right(old)),
        "*" => Box::new(move |old| left(old) * right(old)),
        _ => panic!("operator should be '+' or '*'"),
    }
}

fn value(str: &str) -> Box<dyn Fn(Worry) -> Worry> {
    if str == "old" {
        Box::new(|old| old)
    } else {
        let constant = str.parse().expect("value should be numerical");
        Box::new(move |_| constant)
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::test_on_input;
    use crate::{Input, Puzzle};

    const DAY: usize = 11;

    #[test]
    fn first_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 10605);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 90294);
    }

    #[test]
    fn second_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 2_713_310_158_usize);
    }

    #[test]
    fn second_input() {
        test_on_input(
            DAY,
            Puzzle::Second,
            Input::PuzzleInput,
            18_170_818_354_usize,
        );
    }
}
