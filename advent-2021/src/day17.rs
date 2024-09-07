use std::ops::RangeInclusive;

use easy_cast::{Cast, CastFloat};
use shared::{string::isizes, vector::Vector};

type Velocity = [Coordinate; 2];
type TargetArea = [RangeInclusive<Coordinate>; 2];
type Coordinate = isize;

pub fn first(input: &str) -> String {
    let target_area = target_area(input);
    let probe_hit_velocities = probe_hit_velocities(&target_area);
    let greatest_y_velocity = probe_hit_velocities
        .into_iter()
        .map(|[_, y_velocity]| y_velocity)
        .max()
        .expect("at least one velocity should hit the target");
    let highest_y_position = (greatest_y_velocity * (greatest_y_velocity + 1)) / 2;
    highest_y_position.to_string()
}

pub fn second(input: &str) -> String {
    let target_area = target_area(input);
    probe_hit_velocities(&target_area).len().to_string()
}

fn probe_hit_velocities(
    target_area @ [target_x_range, target_y_range]: &TargetArea,
) -> Vec<Velocity> {
    let mut hits = vec![];
    for x_velocity in x_velocity_range(target_x_range) {
        for y_velocity in y_velocity_range(target_y_range) {
            let velocity = [x_velocity, y_velocity];
            if probe_hits_target_area(target_area, velocity) {
                hits.push(velocity);
            }
        }
    }
    hits
}

fn x_velocity_range(target_x_range: &RangeInclusive<Coordinate>) -> RangeInclusive<Coordinate> {
    let d: f64 = (1 + 8 * target_x_range.start()).cast();
    let minimum_x_velocity = ((-1.0 + d.sqrt()) / 2.0).cast_ceil();
    let &maximum_x_velocity = target_x_range.end();
    minimum_x_velocity..=maximum_x_velocity
}

fn y_velocity_range(target_y_range: &RangeInclusive<Coordinate>) -> RangeInclusive<Coordinate> {
    *target_y_range.start()..=-*target_y_range.start()
}

fn probe_hits_target_area(
    [target_x_range, target_y_range]: &TargetArea,
    mut velocity: Velocity,
) -> bool {
    let mut position = [0, 0];
    while position[0] <= *target_x_range.end() && *target_y_range.start() <= position[1] {
        if *target_x_range.start() <= position[0] && position[1] <= *target_y_range.end() {
            return true;
        }
        position = position.add(velocity);
        velocity[0] -= velocity[0].signum();
        velocity[1] -= 1;
    }
    false
}

fn target_area(input: &str) -> TargetArea {
    let coordinates = isizes(input);
    [
        coordinates[0]..=coordinates[1],
        coordinates[2]..=coordinates[3],
    ]
}

#[cfg(test)]
mod tests {
    use infrastructure::{test, Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 17;

    #[test]
    fn first_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 45);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 3655);
    }

    #[test]
    fn second_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 112);
    }

    #[test]
    fn second_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 1447);
    }

    #[test]
    fn probe_hits_target_area() {
        let target_area = [20..=30, -10..=-5];
        let function = |velocity| super::probe_hits_target_area(&target_area, velocity);
        let cases = [
            ([7, 2], true),
            ([6, 3], true),
            ([9, 0], true),
            ([17, -4], false),
        ];
        test::cases(function, cases);
    }

    #[test]
    fn x_velocity_range() {
        let actual = super::x_velocity_range(&(20..=30));
        let expected = 6..=30;
        assert_eq!(actual, expected);
    }

    #[test]
    fn y_velocity_range() {
        let actual = super::y_velocity_range(&(-10..=-5));
        let expected = -10..=10;
        assert_eq!(actual, expected);
    }
}
