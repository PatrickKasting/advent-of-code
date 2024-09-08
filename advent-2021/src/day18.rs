use std::fmt::Display;

use easy_cast::Cast;
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq)]
enum SnailfishNumber {
    RegularNumber(RegularNumber),
    Pair(Box<SnailfishNumber>, Box<SnailfishNumber>),
}

impl Display for SnailfishNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SnailfishNumber::RegularNumber(regular_number) => write!(f, "{regular_number}"),
            SnailfishNumber::Pair(left, right) => {
                write!(f, "[{left},{right}]")
            }
        }
    }
}

type RegularNumber = usize;
type Magnitude = usize;

pub fn first(input: &str) -> String {
    let snailfish_numbers = snailfish_numbers(input);
    magnitude(&sum(snailfish_numbers)).to_string()
}

pub fn second(input: &str) -> String {
    let snailfish_numbers = snailfish_numbers(input);
    magnitudes_all_pairs(snailfish_numbers)
        .max()
        .expect("at least one pair should exist")
        .to_string()
}

fn sum(snailfish_numbers: impl Iterator<Item = SnailfishNumber>) -> SnailfishNumber {
    snailfish_numbers
        .reduce(add)
        .expect("at least one snailfish number should be present")
}

fn magnitudes_all_pairs(
    snailfish_numbers: impl Iterator<Item = SnailfishNumber>,
) -> impl Iterator<Item = Magnitude> {
    snailfish_numbers.permutations(2).map(|pair| {
        let [left, right] = pair.try_into().expect("pair should contain two elements");
        magnitude(&add(left, right))
    })
}

fn magnitude(snailfish_number: &SnailfishNumber) -> Magnitude {
    match snailfish_number {
        &SnailfishNumber::RegularNumber(regular_number) => regular_number,
        SnailfishNumber::Pair(left, right) => 3 * magnitude(left) + 2 * magnitude(right),
    }
}

fn add(left: SnailfishNumber, right: SnailfishNumber) -> SnailfishNumber {
    reduced(SnailfishNumber::Pair(Box::new(left), Box::new(right)))
}

fn reduced(mut snailfish_number: SnailfishNumber) -> SnailfishNumber {
    while reduce(&mut snailfish_number) {}
    snailfish_number
}

fn reduce(snailfish_number: &mut SnailfishNumber) -> bool {
    maybe_explode(snailfish_number, 0, None, None) || maybe_split(snailfish_number)
}

fn maybe_explode(
    snailfish_number: &mut SnailfishNumber,
    depth: usize,
    nearest_predecessor: Option<&mut SnailfishNumber>,
    nearest_successor: Option<&mut SnailfishNumber>,
) -> bool {
    match snailfish_number {
        SnailfishNumber::RegularNumber(_) => false,
        SnailfishNumber::Pair(left, right) => {
            if depth >= 4 {
                explode(snailfish_number, nearest_predecessor, nearest_successor);
                true
            } else {
                maybe_explode(left, depth + 1, nearest_predecessor, Some(right))
                    || maybe_explode(right, depth + 1, Some(left), nearest_successor)
            }
        }
    }
}

fn maybe_split(snailfish_number: &mut SnailfishNumber) -> bool {
    match snailfish_number {
        &mut SnailfishNumber::RegularNumber(regular_number) => {
            let splits = regular_number >= 10;
            if splits {
                split(snailfish_number);
            }
            splits
        }
        SnailfishNumber::Pair(left, right) => maybe_split(left) || maybe_split(right),
    }
}

fn explode(
    snailfish_number: &mut SnailfishNumber,
    nearest_predecessor: Option<&mut SnailfishNumber>,
    nearest_successor: Option<&mut SnailfishNumber>,
) {
    let SnailfishNumber::Pair(left, right) = snailfish_number else {
        panic!("only pairs should explode");
    };
    let (SnailfishNumber::RegularNumber(left), SnailfishNumber::RegularNumber(right)) =
        (left.as_ref(), right.as_ref())
    else {
        panic!("members of exploding pair should be regular numbers");
    };

    if let Some(nearest_predecessor) = nearest_predecessor {
        let SnailfishNumber::RegularNumber(left_neighbor) = rightmost_child(nearest_predecessor)
        else {
            panic!("left neighbor should be a regular number");
        };
        *left_neighbor += left;
    }
    if let Some(nearest_successor) = nearest_successor {
        let SnailfishNumber::RegularNumber(right_neighbor) = leftmost_child(nearest_successor)
        else {
            panic!("right neighbor should be a regular number");
        };
        *right_neighbor += right;
    }

    *snailfish_number = SnailfishNumber::RegularNumber(0);
}

fn leftmost_child(mut snailfish_number: &mut SnailfishNumber) -> &mut SnailfishNumber {
    while let SnailfishNumber::Pair(left, _) = snailfish_number {
        snailfish_number = left.as_mut();
    }
    snailfish_number
}

fn rightmost_child(mut snailfish_number: &mut SnailfishNumber) -> &mut SnailfishNumber {
    while let SnailfishNumber::Pair(_, right) = snailfish_number {
        snailfish_number = right.as_mut();
    }
    snailfish_number
}

fn split(snailfish_number: &mut SnailfishNumber) {
    let &mut SnailfishNumber::RegularNumber(regular_number) = snailfish_number else {
        panic!("only regular numbers should split");
    };
    let [left, right] = [half_floor, half_ceil]
        .map(|half| Box::new(SnailfishNumber::RegularNumber(half(regular_number))));
    *snailfish_number = SnailfishNumber::Pair(left, right);
}

fn half_floor(number: RegularNumber) -> RegularNumber {
    number / 2
}

fn half_ceil(number: RegularNumber) -> RegularNumber {
    let odd = number & 1;
    number / 2 + odd
}

fn snailfish_numbers(input: &str) -> impl Iterator<Item = SnailfishNumber> + '_ {
    input
        .lines()
        .map(|line| snailfish_number(line.as_bytes()).1)
}

fn snailfish_number(bytes: &[u8]) -> (&[u8], SnailfishNumber) {
    match bytes[0] {
        b'[' => {
            let (remaining, left) = snailfish_number(&bytes[1..]);
            debug_assert_eq!(remaining[0], b',', "pair should be separated by ','");
            let (remaining, right) = snailfish_number(&remaining[1..]);
            debug_assert_eq!(remaining[0], b']', "pair should end with ']'");
            let pair = SnailfishNumber::Pair(Box::new(left), Box::new(right));
            (&remaining[1..], pair)
        }
        b'0'..=b'9' => {
            let regular_number = (bytes[0] - b'0').cast();
            (&bytes[1..], SnailfishNumber::RegularNumber(regular_number))
        }
        _ => panic!("first character in pair should be '[' or digit"),
    }
}

#[cfg(test)]
mod tests {
    use infrastructure::{test, Input, Puzzle};

    use super::*;
    use crate::tests::{input, test_on_input};

    const DAY: usize = 18;

    #[test]
    fn first_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 4140);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 2501);
    }

    #[test]
    fn second_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 3993);
    }

    #[test]
    fn second_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 4935);
    }

    #[test]
    fn sum() {
        let function = |example| {
            let input = input(DAY, Input::Example(example));
            super::sum(snailfish_numbers(&input)).to_string()
        };
        let cases = [
            (1, "[[[[1,1],[2,2]],[3,3]],[4,4]]".to_owned()),
            (2, "[[[[3,0],[5,3]],[4,4]],[5,5]]".to_owned()),
            (3, "[[[[5,0],[7,4]],[5,5]],[6,6]]".to_owned()),
            (
                4,
                "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]".to_owned(),
            ),
        ];
        test::cases(function, cases);
    }

    #[test]
    fn reduced() {
        let (remaining, snailfish_number) =
            snailfish_number(b"[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");
        assert!(remaining.is_empty(), "entire input should be parsed");
        let actual = super::reduced(snailfish_number);
        let expected = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]";
        assert_eq!(actual.to_string(), expected);
    }
}
