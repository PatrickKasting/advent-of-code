use std::iter;

use easy_cast::Cast;
use itertools::Itertools;
use shared::grid::{Grid, Position};

type Image = Grid<Pixel>;
type EnhancementString<'input> = &'input [Pixel];
type Pixel = u8;

pub fn first_answer(input: &str) -> String {
    let (enhancement_string, image) = enhancement_string_and_image(input);
    let enhanced_image = enhanced_image(enhancement_string, image, 2);
    number_of_light_pixels(&enhanced_image).to_string()
}

pub fn second_answer(input: &str) -> String {
    let (enhancement_string, image) = enhancement_string_and_image(input);
    let enhanced_image = enhanced_image(enhancement_string, image, 50);
    number_of_light_pixels(&enhanced_image).to_string()
}

fn enhanced_image(
    enhancement_string: EnhancementString,
    mut image: Image,
    number_of_enhancements: usize,
) -> Image {
    for outside_pixel in outside_pixels(enhancement_string).take(number_of_enhancements) {
        image = enhanced_once(enhancement_string, &image, outside_pixel);
    }
    image
}

fn outside_pixels(enhancement_string: &[u8]) -> Box<dyn Iterator<Item = Pixel>> {
    let output_pixel_all_dark_pixels = enhancement_string[0];
    let output_pixel_all_light_pixels = enhancement_string[512 - 1];
    match [output_pixel_all_dark_pixels, output_pixel_all_light_pixels] {
        [b'.', _] => Box::new(iter::repeat(b'.')),
        [b'#', b'.'] => Box::new(iter::repeat(b'.').interleave(iter::repeat(b'#'))),
        [b'#', b'#'] => Box::new(iter::once(b'.').chain(iter::repeat(b'#'))),
        _ => panic!("enhancement string pixels should be '.' or '#'"),
    }
}

fn enhanced_once(
    enhancement_string: EnhancementString,
    image: &Image,
    outside_pixel: Pixel,
) -> Image {
    let positions = (-1..=image.height().cast()).cartesian_product(-1..=image.width().cast());
    let pixels = positions
        .map(|(row, column)| output_pixel(enhancement_string, image, outside_pixel, [row, column]))
        .collect_vec();
    Image::from_elements(pixels, image.width() + 2)
}

fn output_pixel(
    enhancement_string: EnhancementString,
    image: &Image,
    outside_pixel: Pixel,
    position: Position,
) -> Pixel {
    enhancement_string[output_pixel_index(image, outside_pixel, position)]
}

fn output_pixel_index(image: &Image, outside_pixel: Pixel, [row, column]: Position) -> usize {
    let mut output_pixel_index = 0;
    let positions = (row - 1..=row + 1).cartesian_product(column - 1..=column + 1);
    for (row, column) in positions {
        output_pixel_index <<= 1;
        if *image.get([row, column]).unwrap_or(&outside_pixel) == b'#' {
            output_pixel_index |= 1;
        }
    }
    output_pixel_index
}

fn number_of_light_pixels(image: &Image) -> usize {
    image
        .iter_row_major()
        .filter(|(_, &pixel)| pixel == b'#')
        .count()
}

fn enhancement_string_and_image(input: &str) -> (EnhancementString, Image) {
    let (enhancement_string, image) = input
        .split_once("\n\n")
        .expect("enhancement string and image should be separated by an empty line");
    (enhancement_string.as_bytes(), Image::from(image))
}

#[cfg(test)]
mod tests {
    use std::array;

    use infrastructure::{test, Input, Puzzle};

    use super::*;
    use crate::tests::{input, test_on_input};

    const DAY: usize = 20;

    #[test]
    fn first_answer_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 35);
    }

    #[test]
    fn first_answer_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 5391);
    }

    #[test]
    fn second_answer_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 3351);
    }

    #[test]
    fn second_answer_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 16383);
    }

    #[test]
    fn outside_pixels() {
        let mut enhancement_string: [u8; 512] = array::from_fn(|_| b'.');

        let function = |[all_dark_pixels, all_light_pixels]: [Pixel; 2]| {
            enhancement_string[0] = all_dark_pixels;
            enhancement_string[512 - 1] = all_light_pixels;
            super::outside_pixels(&enhancement_string)
                .take(6)
                .collect_vec()
        };
        let cases = [
            ([b'.', b'.'], vec![b'.', b'.', b'.', b'.', b'.', b'.']),
            ([b'.', b'#'], vec![b'.', b'.', b'.', b'.', b'.', b'.']),
            ([b'#', b'.'], vec![b'.', b'#', b'.', b'#', b'.', b'#']),
            ([b'#', b'#'], vec![b'.', b'#', b'#', b'#', b'#', b'#']),
        ];
        test::cases(function, cases);
    }

    #[test]
    fn enhanced_once() {
        let input = input(DAY, Input::Example(0));
        let (enhancement_string, image) = enhancement_string_and_image(&input);
        let actual = super::enhanced_once(enhancement_string, &image, b'.');
        let expected = "\
            .##.##.\n\
            #..#.#.\n\
            ##.#..#\n\
            ####..#\n\
            .#..##.\n\
            ..##..#\n\
            ...#.#.\n\
        ";
        assert_eq!(actual, Image::from(expected));
    }

    #[test]
    fn output_pixel_index() {
        let (_, image) = enhancement_string_and_image(&input(DAY, Input::Example(0)));
        let actual = super::output_pixel_index(&image, b'.', [2, 2]);
        let expected = 34;
        assert_eq!(actual, expected);
    }
}
