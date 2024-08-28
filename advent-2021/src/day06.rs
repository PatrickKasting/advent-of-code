use itertools::Itertools;
use shared::string::usizes;

const CYCLE_LENGTH: usize = 7;
const FIRST_CYCLE_LENGTH: usize = CYCLE_LENGTH + 2;

type Fish = [usize; FIRST_CYCLE_LENGTH];

pub fn first(input: &str) -> String {
    let fish = fish(input);
    number_of_fish_after(80, fish).to_string()
}

pub fn second(input: &str) -> String {
    let fish = fish(input);
    number_of_fish_after(256, fish).to_string()
}

fn number_of_fish_after(number_of_days: usize, mut fish: [usize; FIRST_CYCLE_LENGTH]) -> usize {
    for _ in 0..number_of_days {
        advance_one_day(&mut fish);
    }
    fish.into_iter().sum()
}

fn advance_one_day(fish: &mut Fish) {
    let number_of_spawning_fish = fish[0];
    fish.rotate_left(1);
    fish[CYCLE_LENGTH - 1] += number_of_spawning_fish;
}

fn fish(input: &str) -> Fish {
    let mut fish = [0; FIRST_CYCLE_LENGTH];
    for (timer, count) in usizes(input).into_iter().counts() {
        fish[timer] = count;
    }
    fish
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 6;

    #[test]
    fn first_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 5934);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 372984);
    }

    #[test]
    fn second_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 26984457539usize);
    }

    #[test]
    fn second_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 1681503251694usize);
    }
}
