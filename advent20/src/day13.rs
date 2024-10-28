use easy_cast::{Cast, Conv};
use itertools::Itertools;
use shared::number_theory::chinese_remainder_theorem;

type Timestamp = Time;
type Bus = Time;
type Time = usize;

pub fn first_answer(input: &str) -> String {
    let (timestamp, buses) = timestamp_and_buses(input);
    let (wait, earliest_bus) = wait_and_earliest_bus(timestamp, &buses);
    (wait * earliest_bus).to_string()
}

pub fn second_answer(input: &str) -> String {
    let (_, buses) = timestamp_and_buses(input);
    let congruences = buses.into_iter().enumerate().filter_map(|(index, bus)| {
        bus.map(|bus| (isize::conv(bus) - isize::conv(index), bus.cast()))
    });
    chinese_remainder_theorem(congruences).to_string()
}

fn wait_and_earliest_bus(timestamp: Timestamp, buses: &[Option<Bus>]) -> (Time, Bus) {
    let buses = buses.iter().copied().flatten();
    buses
        .map(|bus| (wait(timestamp, bus), bus))
        .min()
        .expect("at least one bus should be in service")
}

fn wait(timestamp: Timestamp, bus: Bus) -> Time {
    bus - (timestamp % bus)
}

fn timestamp_and_buses(input: &str) -> (Timestamp, Vec<Option<Bus>>) {
    let mut lines = input.lines();

    let timestamp = lines.next().expect("first line should contain timestamp");
    let timestamp = timestamp.parse().expect("timestamp should be numeric");

    let buses = lines.next().expect("second line should contain buses");

    (timestamp, self::buses(buses))
}

fn buses(line: &str) -> Vec<Option<Bus>> {
    line.split(',').map(|bus| bus.parse().ok()).collect_vec()
}

#[cfg(test)]
mod tests {
    use infrastructure::{test, Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 13;

    #[test]
    fn first_answer_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 295);
    }

    #[test]
    fn first_answer_puzzle_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 4808);
    }

    #[test]
    fn second_answer_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 1_068_781);
    }

    #[test]
    fn second_answer_puzzle_input() {
        test_on_input(
            DAY,
            Puzzle::Second,
            Input::PuzzleInput,
            741_745_043_105_674_usize,
        );
    }

    #[test]
    fn wait() {
        let function = |(timestamp, bus)| super::wait(timestamp, bus);
        let cases = [((939, 7), 6), ((939, 13), 10), ((939, 59), 5)];
        test::cases(function, cases);
    }
}
