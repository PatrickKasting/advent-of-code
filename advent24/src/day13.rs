use std::array;

use easy_cast::{Cast, Conv};
use itertools::Itertools;
use num_traits::CheckedEuclid;
use shared::{string::isizes, vector::Vector};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Machine {
    a: [isize; 2],
    b: [isize; 2],
    prize: [isize; 2],
}

const COST: [usize; 2] = [3, 1];

pub fn first_answer(input: &str) -> String {
    total_number_of_tokens(machines(input)).to_string()
}

pub fn second_answer(input: &str) -> String {
    total_number_of_tokens(corrected_machines(input)).to_string()
}

fn total_number_of_tokens(machines: impl Iterator<Item = Machine>) -> usize {
    machines.filter_map(number_of_tokens).sum()
}

fn number_of_tokens(machine: Machine) -> Option<usize> {
    numbers_of_button_presses(machine).map(|n| n.dot(COST))
}

fn numbers_of_button_presses(
    machine @ Machine {
        a: a @ [ax, _],
        b: b @ [bx, _],
        prize: p @ [px, py],
    }: Machine,
) -> Option<[usize; 2]> {
    let an = determinant(p, b).checked_div_rem_euclid(&determinant(a, b));
    let an = match an {
        None if determinant(p, b) == 0 => {
            return [a, b]
                .into_iter()
                .enumerate()
                .filter_map(|(index, [vx, vy])| {
                    (px + py)
                        .checked_div_rem_euclid(&(vx + vy))
                        .and_then(|(div, rem)| (rem == 0).then_some((index, div)))
                })
                .min_by_key(|&(index, n)| COST[index] * usize::conv(n))
                .map(|(index, n)| array::from_fn(|i| if i == index { n.cast() } else { 0 }));
        }
        Some((an, 0)) => an,
        _ => return None,
    };
    let bn = (px - an * ax).checked_div_rem_euclid(&bx);
    let bn = match bn {
        None => {
            return numbers_of_button_presses(swapped(machine)).map(|[b, a]| [a, b]);
        }
        Some((bn, 0)) => bn,
        Some(_) => return None,
    };
    Some([an, bn].cast())
}

fn determinant(lhs: [isize; 2], rhs: [isize; 2]) -> isize {
    lhs[0] * rhs[1] - lhs[1] * rhs[0]
}

fn swapped(Machine { a, b, prize }: Machine) -> Machine {
    Machine { a: b, b: a, prize }
}

fn corrected_machines(input: &str) -> impl Iterator<Item = Machine> + use<'_> {
    machines(input).map(|machine| Machine {
        prize: machine.prize.map(|p| p + 10_000_000_000_000),
        ..machine
    })
}

fn machines(input: &str) -> impl Iterator<Item = Machine> + use<'_> {
    input.split("\n\n").map(machine)
}

fn machine(str: &str) -> Machine {
    let lines = str.lines().collect_vec();
    let a: [isize; 2] = isizes(lines[0])
        .try_into()
        .expect("first line of machine should contain two integers");
    let b: [isize; 2] = isizes(lines[1])
        .try_into()
        .expect("second line of machine should contain two integers");
    let prize: [isize; 2] = isizes(lines[2])
        .try_into()
        .expect("third line of machine should contain two integers");
    Machine { a, b, prize }
}

#[cfg(test)]
mod tests {
    use infrastructure::{test, Input, Puzzle};
    use itertools::Itertools;

    use super::*;
    use crate::tests::{input, test_on_input};

    const DAY: usize = 13;

    #[test]
    fn first_answer_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 480);
    }

    #[test]
    fn first_answer_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 37680);
    }

    #[test]
    fn second_answer_input() {
        test_on_input(
            DAY,
            Puzzle::Second,
            Input::PuzzleInput,
            87_550_094_242_995_usize,
        );
    }

    #[test]
    fn tokens() {
        let input = input(DAY, Input::Example(0));
        let cases = super::machines(&input).zip_eq([Some(280), None, Some(200), None]);
        test::cases(super::number_of_tokens, cases);
    }

    #[test]
    fn prize_is_zero() {
        let machine = Machine {
            a: [3, -7],
            b: [-4, 2],
            prize: [0, 0],
        };
        let actual = numbers_of_button_presses(machine);
        let expected = Some([0, 0]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn parallel_basis_with_no_solution() {
        let machine = Machine {
            a: [2, -8],
            b: [-1, 4],
            prize: [3, 3],
        };
        let actual = numbers_of_button_presses(machine);
        let expected = None;
        assert_eq!(actual, expected);
    }

    #[test]
    fn parallel_basis_with_solution() {
        let machine = Machine {
            a: [3, -12],
            b: [-2, 8],
            prize: [9, -36],
        };
        let actual = numbers_of_button_presses(machine);
        let expected = Some([3, 0]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn parallel_basis_fewer_presses_are_cheaper() {
        let machine = Machine {
            a: [4, 8],
            b: [1, 2],
            prize: [8, 16],
        };
        let actual = numbers_of_button_presses(machine);
        let expected = Some([2, 0]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn parallel_basis_additional_presses_are_cheaper() {
        let machine = Machine {
            a: [2, 4],
            b: [1, 2],
            prize: [2, 4],
        };
        let actual = numbers_of_button_presses(machine);
        let expected = Some([0, 2]);
        assert_eq!(actual, expected);
    }
}
