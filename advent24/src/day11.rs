use ahash::AHashMap;
use shared::{number_theory::number_of_decimal_digits, string::isizes};

type Line = Vec<Stone>;
type Stone = isize;

pub fn first_answer(input: &str) -> String {
    final_number_of_stones(line(input), 25).to_string()
}

pub fn second_answer(input: &str) -> String {
    final_number_of_stones(line(input), 75).to_string()
}

fn final_number_of_stones(line: Line, number_of_steps: usize) -> usize {
    let mut cache = AHashMap::new();
    line.into_iter()
        .map(|stone| number_of_stones(&mut cache, stone, number_of_steps))
        .sum::<usize>()
}

fn number_of_stones(
    cache: &mut AHashMap<(Stone, usize), usize>,
    stone: Stone,
    number_of_steps: usize,
) -> usize {
    if number_of_steps == 0 {
        return 1;
    }
    if let Some(&cached) = cache.get(&(stone, number_of_steps)) {
        return cached;
    }

    let number_of_digits = number_of_decimal_digits(stone);
    let number_of_stones = if stone == 0 {
        number_of_stones(cache, 1, number_of_steps - 1)
    } else if number_of_digits % 2 == 0 {
        let half_number_of_digits = number_of_digits / 2;
        let divisor = 10_isize.pow(half_number_of_digits);
        let mut stones = [stone / divisor, stone % divisor];
        stones.sort_unstable();
        let small = number_of_stones(cache, stones[0], number_of_steps - 1);
        let large = number_of_stones(cache, stones[1], number_of_steps - 1);
        small + large
    } else {
        number_of_stones(cache, stone * 2024, number_of_steps - 1)
    };
    cache.insert((stone, number_of_steps), number_of_stones);
    number_of_stones
}

fn line(input: &str) -> Line {
    let mut line = isizes(input);
    line.sort_unstable();
    line
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 11;

    #[test]
    fn first_answer_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 55312);
    }

    #[test]
    fn first_answer_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 216_042);
    }

    #[test]
    fn second_answer_input() {
        test_on_input(
            DAY,
            Puzzle::Second,
            Input::PuzzleInput,
            255_758_646_442_399_usize,
        );
    }
}
