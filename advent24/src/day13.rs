use itertools::Itertools;
use shared::string::usizes;

struct Machine {
    a: Translation,
    b: Translation,
    prize: Position,
}
type Translation = Position;
type Position = [usize; 2];

pub fn first_answer(input: &str) -> String {
    todo!()
}

pub fn second_answer(input: &str) -> String {
    todo!()
}

fn machines(input: &str) -> impl Iterator<Item = Machine> + use<'_> {
    input.split("\n\n").map(machine)
}

fn machine(str: &str) -> Machine {
    let lines = str.lines().collect_vec();
    let a: Translation = usizes(lines[0])
        .try_into()
        .expect("first line of machine should contain two integers");
    let b: Translation = usizes(lines[0])
        .try_into()
        .expect("second line of machine should contain two integers");
    let prize: Translation = usizes(lines[0])
        .try_into()
        .expect("third line of machine should contain two integers");
    Machine { a, b, prize }
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 13;

    #[test]
    fn first_answer_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 14);
    }

    #[test]
    fn first_answer_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 396);
    }

    #[test]
    fn second_answer_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 34);
    }

    #[test]
    fn second_answer_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 1200);
    }
}
