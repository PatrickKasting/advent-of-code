use std::{cmp::Ordering, fs};

use easy_cast::Cast;
use image::{imageops::FilterType, ImageBuffer};
use itertools::Itertools;
use shared::{string::isizes, vector::Vector};
use tflitec::{interpreter::Interpreter, model::Model};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Robot {
    position: Position,
    velocity: Velocity,
}
type Position = [Coordinate; 2];
type Velocity = [Coordinate; 2];
type Coordinate = isize;

const SPACE_DIMENSIONS: [Coordinate; 2] = [101, 103];

pub fn first_answer(input: &str) -> String {
    safety_factor(robots(input), 100).to_string()
}

pub fn second_answer(input: &str) -> String {
    let robots = robots(input).collect_vec();
    christmas_tree_time(&robots).to_string()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Quadrant {
    TopRight,
    BottomRight,
    BottomLeft,
    TopLeft,
}

const QUADRANT_BORDERS: [Coordinate; 2] = [SPACE_DIMENSIONS[0] / 2, SPACE_DIMENSIONS[1] / 2];

fn safety_factor(robots: impl Iterator<Item = Robot>, seconds: usize) -> usize {
    let counts = robots.filter_map(|robot| quadrant(robot, seconds)).counts();
    counts.values().copied().product()
}

fn quadrant(robot: Robot, seconds: usize) -> Option<Quadrant> {
    let [x, y] = position(robot, seconds);
    match [x.cmp(&QUADRANT_BORDERS[0]), y.cmp(&QUADRANT_BORDERS[1])] {
        [Ordering::Less, Ordering::Less] => Some(Quadrant::TopLeft),
        [Ordering::Less, Ordering::Greater] => Some(Quadrant::BottomLeft),
        [Ordering::Greater, Ordering::Less] => Some(Quadrant::TopRight),
        [Ordering::Greater, Ordering::Greater] => Some(Quadrant::BottomRight),
        _ => None,
    }
}

const MODEL_INPUT_DIMENSION: usize = 28;

fn christmas_tree_time(robots: &[Robot]) -> usize {
    let machine_learning_directory = [env!("CARGO_MANIFEST_DIR"), "/machine-learning/14"].join("");

    let model_path = [&machine_learning_directory, "/quickdraw_model_int8.tflite"].join("");
    let model = Model::new(&model_path).expect("model should load");
    let interpreter = Interpreter::new(&model, None).expect("tensorflow lite should not fail");
    interpreter
        .allocate_tensors()
        .expect("tensors should allocate");

    let labels_path = [&machine_learning_directory, "/labels.txt"].join("");
    let labels = fs::read_to_string(labels_path).expect("labels should be read");
    let fireplace = labels
        .lines()
        .position(|label| label == "fireplace")
        .expect("fireplace should be a label");

    for seconds in 0.. {
        let image = scaled_image(robots, seconds);
        if classification(&interpreter, &image) == fireplace {
            return seconds;
        }
    }
    unreachable!("robots should resemble a christmas tree");
}

fn classification(
    interpreter: &Interpreter<'_>,
    image: &ImageBuffer<image::Luma<u8>, Vec<u8>>,
) -> usize {
    interpreter
        .copy(image, 0)
        .expect("image data should copy to tensor");
    interpreter.invoke().expect("model invocation should work");
    let output_tensor = interpreter
        .output(0)
        .expect("output tensor should be accessable");
    let output_data: &[u8] = output_tensor.data();
    output_data
        .iter()
        .position_max()
        .expect("output data should be 345 values")
}

fn scaled_image(robots: &[Robot], seconds: usize) -> ImageBuffer<image::Luma<u8>, Vec<u8>> {
    image::imageops::resize(
        &image(robots, seconds),
        MODEL_INPUT_DIMENSION.cast(),
        MODEL_INPUT_DIMENSION.cast(),
        FilterType::Lanczos3,
    )
}

fn image(robots: &[Robot], seconds: usize) -> ImageBuffer<image::Luma<u8>, Vec<u8>> {
    let mut image = ImageBuffer::from_pixel(
        SPACE_DIMENSIONS[0].cast(),
        SPACE_DIMENSIONS[1].cast(),
        image::Luma([u8::MAX]),
    );
    for [x, y] in robots.iter().map(|&robot| position(robot, seconds)) {
        image.put_pixel(x.cast(), y.cast(), image::Luma([0]));
    }
    image
}

fn position(Robot { position, velocity }: Robot, seconds: usize) -> Position {
    let [x, y] = position.add(velocity.mul(seconds.cast()));
    [
        x.rem_euclid(SPACE_DIMENSIONS[0]),
        y.rem_euclid(SPACE_DIMENSIONS[1]),
    ]
}

fn robots(input: &str) -> impl Iterator<Item = Robot> + use<'_> {
    input.lines().map(robot)
}

fn robot(line: &str) -> Robot {
    let numbers = isizes(line);
    Robot {
        position: [numbers[0], numbers[1]],
        velocity: [numbers[2], numbers[3]],
    }
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 14;

    #[test]
    fn first_answer_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 211_692_000);
    }

    #[test]
    #[ignore = "this takes several minutes"]
    fn second_answer_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 6587);
    }
}
