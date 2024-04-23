use itertools::Itertools;

use crate::strings::isizes;

type File = Vec<Number>;
type Number = isize;

pub fn first(input: &str) -> String {
    let coordinates = grove_coordinates_from_input(input, 1, 1);
    coordinates.into_iter().sum::<Number>().to_string()
}

pub fn second(input: &str) -> String {
    let coordinates = grove_coordinates_from_input(input, 811_589_153, 10);
    coordinates.into_iter().sum::<Number>().to_string()
}

fn grove_coordinates_from_input(
    input: &str,
    decryption_key: Number,
    rounds_of_mixing: usize,
) -> [Number; 3] {
    let mut file = file(input);
    let decrypted_file = decrypt(&mut file, decryption_key, rounds_of_mixing);
    grove_coordinates(&decrypted_file)
}

fn decrypt(file: &mut File, decryption_key: Number, rounds_of_mixing: usize) -> File {
    apply_decryption_key(file, decryption_key);
    mixed_file(file, rounds_of_mixing)
}

fn apply_decryption_key(file: &mut [Number], key: Number) {
    file.iter_mut().for_each(|number| *number *= key);
}

fn mixed_file(file: &[Number], rounds: usize) -> File {
    let mut labels = (0..file.len()).collect_vec();
    for _ in 0..rounds {
        for (count, &number) in file.iter().enumerate() {
            let origin = labels
                .iter()
                .position(|&label| label == count)
                .expect("all integer labels up to amount of numbers should be present");
            let label = labels.remove(origin);
            #[allow(clippy::cast_possible_wrap)]
            let destination =
                (origin as Number + number).rem_euclid(labels.len() as Number) as usize;
            labels.insert(destination, label);
        }
    }
    labels.into_iter().map(|label| file[label]).collect_vec()
}

fn grove_coordinates(file: &[Number]) -> [Number; 3] {
    let zero_position = file
        .iter()
        .position(|&number| number == 0)
        .expect("zero should be in the list");
    [1000, 2000, 3000].map(|offset| {
        let position = (zero_position + offset) % file.len();
        file[position]
    })
}

fn file(input: &str) -> File {
    isizes(input)
}

#[cfg(test)]
mod tests {
    use super::{super::tests::test_on_input, *};
    use crate::{input, Input, Puzzle};

    const DAY: usize = 20;

    #[test]
    fn first_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 3);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 3466);
    }

    #[test]
    fn second_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 1_623_178_306);
    }

    #[test]
    fn second_input() {
        test_on_input(
            DAY,
            Puzzle::Second,
            Input::PuzzleInput,
            9_995_532_008_348_isize,
        );
    }

    #[test]
    fn ten_rounds_of_mixing() {
        let mut file = file(&input(2022, DAY, Input::Example(0)));
        let mut actual = decrypt(&mut file, 811_589_153, 10);
        let expected = vec![
            0,
            -2_434_767_459,
            1_623_178_306,
            3_246_356_612,
            -1_623_178_306,
            2_434_767_459,
            811_589_153,
        ];
        assert_circular_equality(&mut actual, &expected);
    }

    #[test]
    fn one_round_of_mixing() {
        let file = file(&input(2022, DAY, Input::Example(0)));
        let mut actual = super::mixed_file(&file, 1);
        let expected = vec![1, 2, -3, 4, 0, 3, -2];
        assert_circular_equality(&mut actual, &expected);
    }

    fn assert_circular_equality(left: &mut [Number], right: &[Number]) {
        for _ in 0..left.len() {
            if left == right {
                return;
            }
            left.rotate_right(1);
        }
        // guaranteed false assertion to get pretty printed output
        assert_eq!(left, right, "files should be circular equal");
    }
}
