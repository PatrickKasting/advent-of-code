const CYCLE_LENGTH: usize = 7;
const FIRST_CYCLE_LENGTH: usize = CYCLE_LENGTH + 2;

type School = [usize; FIRST_CYCLE_LENGTH];

pub fn first(input: &str) -> String {
    number_of_fish(80, input).to_string()
}

pub fn second(input: &str) -> String {
    number_of_fish(256, input).to_string()
}

fn number_of_fish(number_of_days: usize, input: &str) -> usize {
    let mut school = parse_input(input);
    for _ in 0..number_of_days {
        advance_one_day(&mut school);
    }
    school.into_iter().sum()
}

fn advance_one_day(school: &mut School) {
    let number_of_spawning_fish = school[0];
    school.rotate_left(1);
    school[CYCLE_LENGTH - 1] += number_of_spawning_fish;
}

fn parse_input(input: &str) -> School {
    let input = input.trim();
    let mut school: School = [0; FIRST_CYCLE_LENGTH];
    let timers = input.split(',').map(|timer| {
        timer
            .parse::<usize>()
            .expect("input should contain only numbers")
    });
    for timer in timers {
        school[timer] += 1;
    }
    school
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
