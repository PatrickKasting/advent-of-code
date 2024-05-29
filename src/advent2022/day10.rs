use easy_cast::Conv;

use crate::grid::Grid;

type SignalStrength = isize;
type Register = isize;
type Image = Grid<char>;

pub fn first(input: &str) -> String {
    sum_of_signal_strengths(input).to_string()
}

pub fn second(input: &str) -> String {
    image(input).to_string()
}

fn image(input: &str) -> Image {
    let mut image = Image::new(6, 40, |_| '.');
    let draw_maybe = |register: Register, number_of_completed_cycles: usize| {
        let position @ [_, column] = [
            number_of_completed_cycles / image.width(),
            number_of_completed_cycles % image.width(),
        ];
        if (Register::conv(column) - register).abs() <= 1 {
            image[position.map(easy_cast::Cast::cast)] = '#';
        }
    };
    execute(input, draw_maybe);
    image
}

fn sum_of_signal_strengths(input: &str) -> SignalStrength {
    let mut sum_of_signal_strengths = 0;
    let mut next_sample_time = 20;
    let sample_maybe = |register, number_of_completed_cycles| {
        if number_of_completed_cycles + 1 == next_sample_time {
            sum_of_signal_strengths += register * Register::conv(next_sample_time);
            next_sample_time += 40;
        }
    };
    execute(input, sample_maybe);
    sum_of_signal_strengths
}

fn execute(input: &str, mut on_cycle: impl FnMut(Register, usize)) {
    let mut register = 1;
    let mut number_of_completed_cycles = 0;
    for line in input.lines() {
        let (next_register, execution_time): (Register, _) = match &line[0..4] {
            "noop" => (register, 1),
            "addx" => {
                let operand: Register = line[5..].parse().expect("operand should be numerical");
                (register + operand, 2)
            }
            _ => panic!("instruction shoul be 'noop' or 'addx'"),
        };
        for _ in 0..execution_time {
            on_cycle(register, number_of_completed_cycles);
            number_of_completed_cycles += 1;
        }
        register = next_register;
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::test_on_input;
    use crate::{Input, Puzzle};

    const DAY: usize = 10;

    #[test]
    fn first_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 13140);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 12560);
    }

    #[test]
    fn second_example() {
        let expected = "\
            ##..##..##..##..##..##..##..##..##..##..\n\
            ###...###...###...###...###...###...###.\n\
            ####....####....####....####....####....\n\
            #####.....#####.....#####.....#####.....\n\
            ######......######......######......####\n\
            #######.......#######.......#######.....\n\
        ";
        test_on_input(DAY, Puzzle::Second, Input::Example(0), expected);
    }

    #[test]
    fn second_input() {
        let expected = "\
            ###..#....###...##..####.###...##..#....\n\
            #..#.#....#..#.#..#.#....#..#.#..#.#....\n\
            #..#.#....#..#.#..#.###..###..#....#....\n\
            ###..#....###..####.#....#..#.#....#....\n\
            #....#....#....#..#.#....#..#.#..#.#....\n\
            #....####.#....#..#.#....###...##..####.\n\
        ";
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, expected);
    }
}
