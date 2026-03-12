use std::cmp::Ordering;

use easy_cast::Cast;
use itertools::Itertools;
use shared::{string::isizes, vector::Vector};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Robot {
    position: Position,
    velocity: Velocity,
}
type Position = [Coordinate; 2];
type Velocity = [Coordinate; 2];
type Coordinate = isize;

const DIMENSIONS: [Coordinate; 2] = [101, 103];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Quadrant {
    TopRight,
    BottomRight,
    BottomLeft,
    TopLeft,
}
const BOUNDARIES: [Coordinate; 2] = [DIMENSIONS[0] / 2, DIMENSIONS[1] / 2];

pub fn first_answer(input: &str) -> String {
    safety_factor(robots(input), 100).to_string()
}

pub fn second_answer(input: &str) -> String {
    let robots = robots(input).collect_vec();
    christmas_tree(&robots).to_string()
}

fn safety_factor(robots: impl Iterator<Item = Robot>, seconds: usize) -> usize {
    let counts = robots.filter_map(|robot| quadrant(robot, seconds)).counts();
    counts.values().copied().product()
}

fn quadrant(robot: Robot, seconds: usize) -> Option<Quadrant> {
    let [x, y] = position(robot, seconds);
    match [x.cmp(&BOUNDARIES[0]), y.cmp(&BOUNDARIES[1])] {
        [Ordering::Less, Ordering::Less] => Some(Quadrant::TopLeft),
        [Ordering::Less, Ordering::Greater] => Some(Quadrant::BottomLeft),
        [Ordering::Greater, Ordering::Less] => Some(Quadrant::TopRight),
        [Ordering::Greater, Ordering::Greater] => Some(Quadrant::BottomRight),
        _ => None,
    }
}

const INPUT_DIMENSION: usize = 28;

fn christmas_tree(robots: &[Robot]) -> usize {
    let machine_learning_directory = [env!("CARGO_MANIFEST_DIR"), "/machine-learning/14"].join("");

    let model_path = [&machine_learning_directory, "/quickdraw_model_int8.tflite"].join("");
    let model = tflitec::model::Model::new(&model_path).expect("model should load");
    let interpreter = tflitec::interpreter::Interpreter::new(&model, None)
        .expect("tensorflow lite should not fail");
    interpreter
        .allocate_tensors()
        .expect("tensors should allocate");

    let labels_path = [&machine_learning_directory, "/labels.txt"].join("");
    let labels = std::fs::read_to_string(labels_path).expect("labels should be read");
    let fireplace = labels
        .lines()
        .position(|label| label == "fireplace")
        .expect("fireplace should be a label");

    for second in 0.. {
        let image = scaled_image(robots, second);
        interpreter
            .copy(&image, 0)
            .expect("image data should copy to tensor");
        interpreter.invoke().expect("model invocation should work");
        let output_tensor = interpreter
            .output(0)
            .expect("output tensor should be accessable");
        let output_data: &[u8] = output_tensor.data();

        let cassification = output_data
            .iter()
            .position_max()
            .expect("output data should be 345 values");
        if cassification == fireplace {
            return second;
        }
    }
    unreachable!("robots should resemble a christmas tree");
}

fn scaled_image(robots: &[Robot], seconds: usize) -> image::ImageBuffer<image::Luma<u8>, Vec<u8>> {
    image::imageops::resize(
        &image(robots, seconds),
        INPUT_DIMENSION.cast(),
        INPUT_DIMENSION.cast(),
        image::imageops::FilterType::Lanczos3,
    )
}

fn image(robots: &[Robot], seconds: usize) -> image::ImageBuffer<image::Luma<u8>, Vec<u8>> {
    let mut image = image::ImageBuffer::from_pixel(
        DIMENSIONS[0].cast(),
        DIMENSIONS[1].cast(),
        image::Luma([u8::MAX]),
    );
    for [x, y] in robots.iter().map(|&robot| position(robot, seconds)) {
        image.put_pixel(x.cast(), y.cast(), image::Luma([0]));
    }
    image
}

fn position(Robot { position, velocity }: Robot, seconds: usize) -> Position {
    let [x, y] = position.add(velocity.mul(seconds.cast()));
    [x.rem_euclid(DIMENSIONS[0]), y.rem_euclid(DIMENSIONS[1])]
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
    fn second_answer_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 6587);
    }
}
