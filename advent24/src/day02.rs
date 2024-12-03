use itertools::Itertools;
use shared::string::usizes;

type Level = usize;

pub fn first_answer(input: &str) -> String {
    let reports = reports(input);
    reports.filter(|report| is_safe(report)).count().to_string()
}

pub fn second_answer(input: &str) -> String {
    let reports = reports(input);
    reports
        .filter(|report| is_safe_with_problem_dampener(report))
        .count()
        .to_string()
}

fn is_safe_with_problem_dampener(report: &[Level]) -> bool {
    report
        .iter()
        .copied()
        .combinations(report.len() - 1)
        .any(|report| is_safe(&report))
}

fn is_safe(report: &[Level]) -> bool {
    monotonic(report) && gradually_changing(report)
}

fn monotonic(report: &[Level]) -> bool {
    all_increasing(report) || all_decreasing(report)
}

fn all_increasing(report: &[Level]) -> bool {
    report.windows(2).all(|pair| pair[0] < pair[1])
}

fn all_decreasing(report: &[Level]) -> bool {
    report.windows(2).all(|pair| pair[0] > pair[1])
}

fn gradually_changing(report: &[Level]) -> bool {
    report
        .windows(2)
        .all(|pair| (1..=3).contains(&pair[0].abs_diff(pair[1])))
}

fn reports(input: &str) -> impl Iterator<Item = Vec<Level>> + '_ {
    input.lines().map(usizes)
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 2;

    #[test]
    fn first_answer_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 2);
    }

    #[test]
    fn first_answer_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 660);
    }

    #[test]
    fn second_answer_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 4);
    }

    #[test]
    fn second_answer_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 689);
    }
}
