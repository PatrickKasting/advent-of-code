use std::{cmp, collections::HashSet, fmt::Debug};

use itertools::Itertools;

use crate::{data_structures::grid::Position, strings::isizes};

type Sensor = Position;
type Beacon = Position;
type Range = [Coordinate; 2];
type Coordinate = isize;

pub fn first(input: &str) -> String {
    number_of_impossible_positions_from_input(input, 2_000_000).to_string()
}

pub fn second(input: &str) -> String {
    let distress_beacon = distress_beacon_from_input(input, 4_000_000);
    tuning_frequency(distress_beacon).to_string()
}

fn distress_beacon_from_input(input: &str, coordinate_max: Coordinate) -> Position {
    let (sensors, beacons) = sensors_and_closest_beacons(input);
    let distances = distances_to_closest_beacons(&sensors, &beacons);
    let (row, [column_start, column_end]) =
        possible_positions(&sensors, &distances, coordinate_max)
            .exactly_one()
            .expect("exactly one row should be a possible row for the beacon");
    debug_assert!(
        column_start + 1 == column_end,
        "exactly one column should be a possible column for the beacon",
    );
    Position::new(row, column_start)
}

fn possible_positions<'data>(
    sensors: &'data [Sensor],
    distances: &'data [Coordinate],
    coordinate_max: Coordinate,
) -> impl Iterator<Item = (Coordinate, Range)> + Debug + 'data {
    (0..=coordinate_max).flat_map(move |row| {
        let impossible_ranges = impossible_ranges(sensors, distances, row);
        complement_ranges(impossible_ranges, [0, coordinate_max + 1])
            .into_iter()
            .map(move |range| (row, range))
    })
}

fn complement_ranges(sorted_ranges: Vec<Range>, [min, max]: [Coordinate; 2]) -> Vec<Range> {
    let ranges_overlapping_universe = sorted_ranges
        .into_iter()
        .skip_while(move |&[_, end]| end <= min)
        .take_while(move |&[start, _]| start < max)
        .collect_vec();
    if ranges_overlapping_universe.is_empty() {
        return vec![[min, max]];
    }

    let mut complement_ranges = vec![];

    let &[first_start, _] = ranges_overlapping_universe
        .first()
        .expect("at least one range should overlap universe");
    if min < first_start {
        complement_ranges.push([min, first_start]);
    }

    let between_ranges = ranges_overlapping_universe
        .iter()
        .copied()
        .tuple_windows()
        .map(move |([_, left_end], [right_start, _])| [left_end, right_start]);
    complement_ranges.extend(between_ranges);

    let &[_, last_end] = ranges_overlapping_universe
        .last()
        .expect("at least one range should overlap universe");
    if last_end < max {
        complement_ranges.push([last_end, max]);
    }

    complement_ranges
}

fn tuning_frequency(beacon: Position) -> isize {
    beacon.column() * 4_000_000 + beacon.row()
}

fn number_of_impossible_positions_from_input(input: &str, row: Coordinate) -> usize {
    let (sensors, beacons) = sensors_and_closest_beacons(input);
    let distances = distances_to_closest_beacons(&sensors, &beacons);
    let impossible_ranges = impossible_ranges(&sensors, &distances, row);
    let beacons = HashSet::from_iter(beacons);
    number_of_impossible_positions(&beacons, &impossible_ranges, row)
}

#[allow(clippy::cast_sign_loss)]
fn number_of_impossible_positions(
    beacons: &HashSet<Beacon>,
    impossible_ranges: &[Range],
    row: Coordinate,
) -> usize {
    let beacons = beacons
        .iter()
        .filter(|&&beacon| beacon_within_ranges(beacon, impossible_ranges, row))
        .count();
    let impossible_positions: isize = impossible_ranges
        .iter()
        .map(|&[start, end]| end - start)
        .sum();
    impossible_positions as usize - beacons
}

fn beacon_within_ranges(beacon: Beacon, ranges: &[Range], row: Coordinate) -> bool {
    let correct_row = beacon.row() == row;
    let within_range = ranges
        .iter()
        .any(|&[start, end]| start <= beacon.column() && beacon.column() < end);
    correct_row && within_range
}

fn impossible_ranges(sensors: &[Sensor], distances: &[Coordinate], row: Coordinate) -> Vec<Range> {
    let mut overlapping_impossible_ranges = sensors
        .iter()
        .zip_eq(distances)
        .filter_map(|(&sensor, &distance)| impossible_range(sensor, distance, row))
        .collect_vec();
    overlapping_impossible_ranges.sort_unstable();

    let mut overlapping_impossible_ranges = &overlapping_impossible_ranges[0..];
    let mut merged_impossible_ranges = vec![];
    while !overlapping_impossible_ranges.is_empty() {
        let (remaining, merged_range) = merged_range(overlapping_impossible_ranges);
        overlapping_impossible_ranges = remaining;
        merged_impossible_ranges.push(merged_range);
    }
    merged_impossible_ranges
}

fn merged_range(sorted_ranges: &[Range]) -> (&[Range], Range) {
    let [start, mut end] = [sorted_ranges[0][0], sorted_ranges[0][1]];
    for (index, range) in sorted_ranges[0..].iter().enumerate().skip(1) {
        if end < range[0] {
            return (&sorted_ranges[index..], [start, end]);
        }
        end = cmp::max(end, range[1]);
    }
    (&[], [start, end])
}

fn impossible_range(sensor: Sensor, distance: Coordinate, row: Coordinate) -> Option<Range> {
    let perimeter_row_farthest_distance = distance - (sensor.row() - row).abs();
    (!perimeter_row_farthest_distance.is_negative()).then(|| {
        [
            sensor.column() - perimeter_row_farthest_distance,
            sensor.column() + perimeter_row_farthest_distance + 1,
        ]
    })
}

fn distances_to_closest_beacons(sensors: &[Sensor], beacons: &[Beacon]) -> Vec<Coordinate> {
    sensors
        .iter()
        .zip_eq(beacons)
        .map(|(&sensor, &beacon)| sensor.manhattan_distance(beacon))
        .collect_vec()
}

fn sensors_and_closest_beacons(input: &str) -> (Vec<Sensor>, Vec<Beacon>) {
    input
        .lines()
        .map(|line| {
            let coordinates = isizes(line);
            let sensor = Position::new(coordinates[1], coordinates[0]);
            let beacon = Position::new(coordinates[3], coordinates[2]);
            (sensor, beacon)
        })
        .unzip()
}

#[cfg(test)]
mod tests {
    use super::{super::tests::test_on_input, *};
    use crate::{input, tests::test_cases, Input, Puzzle};

    const DAY: usize = 15;

    #[test]
    fn first_example() {
        let input = input(super::super::tests::YEAR, DAY, Input::Example(0));
        let actual = number_of_impossible_positions_from_input(&input, 10);
        assert_eq!(actual, 26);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 4_502_208);
    }

    #[test]
    fn second_example() {
        let input = input(super::super::tests::YEAR, DAY, Input::Example(0));
        let actual = tuning_frequency(distress_beacon_from_input(&input, 20));
        assert_eq!(actual, 56_000_011);
    }

    #[test]
    fn second_input() {
        test_on_input(
            DAY,
            Puzzle::Second,
            Input::PuzzleInput,
            13_784_551_204_480_usize,
        );
    }

    #[test]
    fn complement_ranges() {
        let universe @ [min, max] = [0, 21];
        let function = |sorted_ranges| super::complement_ranges(sorted_ranges, universe);
        let cases = [
            (vec![[-10, 0]], vec![universe]),
            (vec![[7, 9]], vec![[min, 7], [9, max]]),
            (vec![[-3, 14], [15, 26]], vec![[14, 15]]),
        ];
        test_cases(function, cases);
    }

    #[test]
    fn impossible_beacon_range() {
        let function = |row| super::impossible_range(Sensor::new(7, 8), 9, row);
        let cases = [
            (-3, None),
            (-2, Some([8, 9])),
            (3, Some([3, 14])),
            (7, Some([-1, 18])),
            (10, Some([2, 15])),
            (14, Some([6, 11])),
            (17, None),
        ];
        test_cases(function, cases);
    }
}
