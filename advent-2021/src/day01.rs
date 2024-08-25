pub fn first(input: &str) -> String {
    let measurements = measurements(input);
    measurements
        .iter()
        .zip(measurements.iter().skip(1))
        .filter(|(current, next)| current < next)
        .count()
        .to_string()
}

pub fn second(input: &str) -> String {
    let window_size: usize = 3;
    let measurements = measurements(input);
    measurements
        .windows(window_size)
        .zip(measurements.windows(window_size).skip(1))
        .filter(|(current, next)| current.iter().sum::<usize>() < next.iter().sum::<usize>())
        .count()
        .to_string()
}

fn measurements(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|line| str::parse::<usize>(line).expect("every measurement should be an integer"))
        .collect()
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 1;

    #[test]
    fn first_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 7);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 1233);
    }

    #[test]
    fn second_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 5);
    }

    #[test]
    fn second_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 1275);
    }
}
