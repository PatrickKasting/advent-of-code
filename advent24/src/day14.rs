use std::{cmp::Ordering, path::Path};

use easy_cast::Cast;
use image::ImageBuffer;
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

const SPACE_DIMENSIONS: [Coordinate; 2] = [101, 103];

pub fn first_answer(input: &str) -> String {
    safety_factor(robots(input), 100).to_string()
}

pub fn second_answer(input: &str) -> String {
    let image_directory = std::env::temp_dir().join("advent24/14");
    std::fs::create_dir_all(&image_directory).expect("image directory should be created");

    let robots = robots(input).collect_vec();
    for seconds in 0..9999 {
        save_image(&robots, seconds, &image_directory);
    }
    format!(
        "The first 10,000 images are saved to '{}'.",
        image_directory.display()
    )
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

fn save_image(robots: &[Robot], seconds: usize, directory: impl AsRef<Path>) {
    let path = directory.as_ref().join(format!("{seconds}.png"));
    image(robots, seconds)
        .save(path)
        .expect("image should be save");
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
}
