use regex::Regex;

pub fn first_answer(input: &str) -> String {
    sum_of_multiplications(input, false).to_string()
}

pub fn second_answer(input: &str) -> String {
    sum_of_multiplications(input, true).to_string()
}

fn sum_of_multiplications(input: &str, dos_and_donts: bool) -> usize {
    let regex = Regex::new(r"mul\((?<lhs>\d{1,3}),(?<rhs>\d{1,3})\)|do\(\)|don't\(\)")
        .expect("regex should be valid");
    let mut sum: usize = 0;
    let mut enabled = true;
    for capture in regex.captures_iter(input) {
        let mat = capture
            .get(0)
            .expect("capture group zero should always exist")
            .as_str();
        match mat {
            "do()" => enabled = true,
            "don't()" => enabled = !dos_and_donts,
            _ if enabled => {
                let [lhs, rhs]: [usize; 2] = ["lhs", "rhs"]
                    .map(|name| capture[name].parse().expect("captures should be numeric"));
                sum += lhs * rhs;
            }
            _ => (),
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 3;

    #[test]
    fn first_answer_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 161);
    }

    #[test]
    fn first_answer_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 182_619_815);
    }

    #[test]
    fn second_answer_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(1), 48);
    }

    #[test]
    fn second_answer_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 80_747_545);
    }
}
