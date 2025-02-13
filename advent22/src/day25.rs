type Decimal = isize;

pub fn first_answer(input: &str) -> String {
    let sum: Decimal = input.lines().map(decimal).sum();
    snafu(sum)
}

pub fn second_answer(_input: &str) -> String {
    "There is no second puzzle on the 25th. Merry Christmas!".to_owned()
}

fn snafu(mut decimal: Decimal) -> String {
    let mut snafu = vec![];
    while decimal > 0 {
        let mut symbol = decimal % 5;
        if symbol > 2 {
            symbol -= 5;
            decimal -= symbol;
        }
        snafu.push(snafu_symbol(symbol));
        decimal /= 5;
    }
    snafu.reverse();
    if snafu.is_empty() {
        snafu.push(b'0');
    }
    String::from_utf8(snafu).expect("snafu number should consist of only ascii characters")
}

fn snafu_symbol(value: Decimal) -> u8 {
    match value {
        -2 => b'=',
        -1 => b'-',
        0 => b'0',
        1 => b'1',
        2 => b'2',
        _ => panic!("value should be '-2', '-1', '0', '1', or '2'"),
    }
}

fn decimal(line: &str) -> Decimal {
    let mut decimal = 0;
    for char in line.chars() {
        decimal = 5 * decimal + digit(char);
    }
    decimal
}

fn digit(char: char) -> Decimal {
    match char {
        '=' => -2,
        '-' => -1,
        '0' => 0,
        '1' => 1,
        '2' => 2,
        _ => panic!("digit should be '=', '-', '0', '1', or '2'"),
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use infrastructure::{test, Input, Puzzle};

    use super::*;
    use crate::tests::test_on_input;

    const DAY: usize = 25;

    #[test]
    fn first_answer_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), "2=-1=0");
    }

    #[test]
    fn first_answer_input() {
        test_on_input(
            DAY,
            Puzzle::First,
            Input::PuzzleInput,
            "2=--=0000-1-0-=1=0=2",
        );
    }

    #[test]
    fn small_decimal_to_snafu() {
        test::cases(
            snafu,
            SMALL_DECIMAL
                .into_iter()
                .zip_eq(SMALL_SNAFU.map(String::from)),
        );
    }

    #[test]
    fn small_snafu_to_decimal() {
        test::cases(decimal, SMALL_SNAFU.into_iter().zip_eq(SMALL_DECIMAL));
    }

    #[test]
    fn large_decimal_to_snafu() {
        test::cases(
            snafu,
            LARGE_DECIMAL
                .into_iter()
                .zip_eq(LARGE_SNAFU.map(String::from)),
        );
    }

    #[test]
    fn large_snafu_to_decimal() {
        test::cases(decimal, LARGE_SNAFU.into_iter().zip_eq(LARGE_DECIMAL));
    }

    const SMALL_DECIMAL: [Decimal; 11] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    const SMALL_SNAFU: [&str; 11] = [
        "0", "1", "2", "1=", "1-", "10", "11", "12", "2=", "2-", "20",
    ];

    const LARGE_DECIMAL: [Decimal; 11] = [1747, 906, 198, 11, 201, 31, 1257, 32, 353, 107, 37];
    const LARGE_SNAFU: [&str; 11] = [
        "1=-0-2", "12111", "2=0=", "21", "2=01", "111", "20012", "112", "1=-1=", "1-12", "122",
    ];
}
