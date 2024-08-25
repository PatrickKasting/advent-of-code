use std::iter::{self, once};

use itertools::Itertools;

use shared::grid::{self, Grid, Position};

type Pixel = bool;
type EnhancementAlgorithm = Vec<Pixel>;
type Image = Grid<Pixel>;

const ENHANCEMENT_ALGORITHM_LEN: usize = 512;
const ENHANCEMENT_ALGORITHM_STRING: &str = "enhancement algorithm string should not be empty";

pub fn first(input: &str) -> String {
    let enhanced_image = enhanced_image(input, 2);
    num_light_pixels(&enhanced_image).to_string()
}

pub fn second(input: &str) -> String {
    let enhanced_image = enhanced_image(input, 50);
    num_light_pixels(&enhanced_image).to_string()
}

fn parse_pixel(char: char) -> Pixel {
    char == '#'
}

fn parse_enhancement_algorithm(enhancement_algorithm: &str) -> EnhancementAlgorithm {
    let enhancement_algorithm: EnhancementAlgorithm =
        enhancement_algorithm.chars().map(parse_pixel).collect();
    debug_assert_eq!(enhancement_algorithm.len(), ENHANCEMENT_ALGORITHM_LEN);
    enhancement_algorithm
}

fn parse_image<'lines>(lines: impl Iterator<Item = &'lines str>, border: usize) -> Image {
    let mut lines = lines.peekable();
    let image_size = lines.peek().expect("image should not be empty").len() + 2 * border;
    let mut image = Grid::new(image_size, image_size, |_| false);
    for (row, line) in lines.enumerate() {
        for (col, char) in line.chars().enumerate() {
            let position = [(row + border) as isize, (col + border) as isize];
            image[position] = parse_pixel(char);
        }
    }
    image
}

fn parse_input(input: &str, border: usize) -> (EnhancementAlgorithm, Image) {
    let mut lines = input.lines();
    let enhancement_algorithm = parse_enhancement_algorithm(
        lines
            .next()
            .expect("first line should contain the enhancement algorithm"),
    );
    lines.next().expect("second line should be empty");
    let image = parse_image(lines, border);
    (enhancement_algorithm, image)
}

fn outside_pixels(enhancement_algorithm: &[Pixel]) -> impl Iterator<Item = Pixel> {
    let &all_dark = enhancement_algorithm
        .first()
        .expect(ENHANCEMENT_ALGORITHM_STRING);
    let &all_light = enhancement_algorithm
        .last()
        .expect(ENHANCEMENT_ALGORITHM_STRING);
    let repeating = match [all_dark, all_light] {
        [false, _] => [false, false],
        pattern @ [true, _] => pattern,
    };
    once(false).chain(repeating.into_iter().cycle())
}

fn pixel_value(outside_pixel: Pixel, image: &Image, position: Position) -> usize {
    let mut value = 0;
    for (index, pos) in (0..9)
        .rev()
        .zip_eq(iter::once(position).chain(grid::neighbors(position)))
    {
        let &pixel = image.get(pos).unwrap_or(&outside_pixel);
        if pixel {
            value += 1 << index;
        }
    }
    value
}

fn enhance(enhancement_algorithm: &[Pixel], outside_pixel: Pixel, image: &Image) -> Image {
    let mut enhanced_image = Image::new(image.height(), image.width(), |_| false);
    for row in 0..image.height() {
        for col in 0..image.width() {
            let position = [row as isize, col as isize];
            enhanced_image[position] =
                enhancement_algorithm[pixel_value(outside_pixel, image, position)];
        }
    }
    enhanced_image
}

fn enhanced_image(input: &str, num_enhancements: usize) -> Image {
    let (enhancement_algorithm, mut image) = parse_input(input, num_enhancements);
    let mut outside_pixels = outside_pixels(&enhancement_algorithm);
    for _ in 0..num_enhancements {
        let outside_pixel = outside_pixels.next().expect("outside pixels should cycle");
        image = enhance(&enhancement_algorithm, outside_pixel, &image);
    }
    image
}

fn num_light_pixels(image: &Image) -> usize {
    image.iter_row_major().filter(|&(_, &pixel)| pixel).count()
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use super::*;
    use crate::tests::{input, test_on_input};

    const DAY: usize = 20;

    #[test]
    fn first_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 35);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 5391);
    }

    #[test]
    fn second_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 3351);
    }

    #[test]
    fn second_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 16383);
    }

    #[test]
    fn value() {
        let (_, image) = parse_input(&input(DAY, Input::Example(0)), 0);
        assert_eq!(pixel_value(false, &image, [2, 2]), 34);
        assert_eq!(pixel_value(false, &image, [0, 0]), 18);
        assert_eq!(pixel_value(false, &image, [4, 3]), 312);
        assert_eq!(pixel_value(true, &image, [2, 2]), 34);
        assert_eq!(pixel_value(true, &image, [0, 0]), 502);
        assert_eq!(pixel_value(true, &image, [4, 3]), 319);
    }

    fn assert_first_outside_pixels(enhancement_algorithm: &[Pixel], expected: &[Pixel]) {
        let pixels = outside_pixels(enhancement_algorithm);
        assert_eq!(&pixels.take(expected.len()).collect::<Vec<_>>(), expected);
    }

    #[test]
    fn outside() {
        assert_first_outside_pixels(&[false, true], &[false, false, false, false, false]);
        assert_first_outside_pixels(&[true, false], &[false, true, false, true, false]);
        assert_first_outside_pixels(&[true, true], &[false, true, true, true, true]);
    }

    #[test]
    fn enhancements() {
        let actual = enhanced_image(&input(DAY, Input::Example(0)), 2)
            .map(|_, &pixel| if pixel { '#' } else { '.' })
            .to_string();
        let expected = "\
            .......#.\n\
            .#..#.#..\n\
            #.#...###\n\
            #...##.#.\n\
            #.....#.#\n\
            .#.#####.\n\
            ..#.#####\n\
            ...##.##.\n\
            ....###..\n\
        ";
        assert_eq!(actual, expected);
    }
}
