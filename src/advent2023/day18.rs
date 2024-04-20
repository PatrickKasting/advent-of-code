// use std::cmp::Ordering;

// use itertools::Itertools;

// use crate::{
//     convert::as_isize,
//     grid::{Coordinate, Curvature, Direction, Position},
//     strings::parse,
// };

// type DigStep = (Direction, Coordinate);

// fn dig_plan_step_from_directions_and_distances(line: &str) -> DigStep {
//     let direction = Direction::from_up_down_left_or_right(line);
//     let (distance, _) = line[2..]
//         .split_once(' ')
//         .expect("line should contain distance");
//     (direction, parse(distance))
// }

// fn dig_plan_step_from_color_codes(line: &str) -> DigStep {
//     let (_, color_code) = line
//         .split_once('#')
//         .expect("line should have a color code starting with '#'");
//     let direction = match &color_code[5..6] {
//         "0" => Direction::East,
//         "1" => Direction::South,
//         "2" => Direction::West,
//         "3" => Direction::North,
//         _ => panic!("direction should be '0', '1', '2', or '3'"),
//     };
//     let distance = Coordinate::from_str_radix(&color_code[0..5], 16)
//         .expect("first five digits of color code should be parsable");
//     (direction, distance)
// }

// fn dig_plan(parser: fn(&str) -> DigStep, input: &str) -> Vec<DigStep> {
//     input.lines().map(parser).collect_vec()
// }

// type Corner = (Position, Curvature);

// fn trench(dig_plan: &[DigStep]) -> Vec<Corner> {
//     let mut position = Position::new(0, 0);
//     dig_plan
//         .iter()
//         .circular_tuple_windows::<(_, _)>()
//         .map(|(&(towards, distance), &(away, _))| {
//             position = position.addition(towards.as_unit_vector().scalar_product(distance));
//             (position, Curvature::from((towards, away)))
//         })
//         .collect_vec()
// }

// // fn draw(trench: &[Corner], width: usize) {
// //     let mut trench: std::collections::BTreeSet<_> = trench.iter().copied().collect();
// //     let [mut row, mut column] = [0, 0];
// //     while let Some((position, curvature)) = trench.pop_first() {
// //         while row < position.row() {
// //             while column < width.try_into().unwrap() {
// //                 print!(".");
// //                 column += 1;
// //             }
// //             println!();
// //             row += 1;
// //             column = 0;
// //         }
// //         while column < position.column() {
// //             print!(".");
// //             column += 1;
// //         }
// //         match curvature {
// //             Curvature::LeftTurn => print!("L"),
// //             Curvature::RightTurn => print!("R"),
// //             _ => panic!("trench should only have left turns or right turns"),
// //         }
// //         column += 1;
// //     }
// //     while column < width.try_into().unwrap() {
// //         print!(".");
// //         column += 1;
// //     }
// //     println!();
// // }

// fn is_clockwise(trench: &[Corner]) -> bool {
//     let curvature_counts = trench.iter().map(|(_, curvature)| *curvature).counts();
//     let difference = as_isize(curvature_counts[&Curvature::RightTurn])
//         - as_isize(curvature_counts[&Curvature::LeftTurn]);
//     debug_assert_eq!(
//         difference.abs(),
//         4,
//         "turn count difference should be four or negative four",
//     );
//     difference.is_positive()
// }

// fn reverse(trench: &mut [Corner]) {
//     trench.reverse();
//     for (_, curvature) in trench.iter_mut() {
//         *curvature = curvature.as_seen_from_opposite_direction();
//     }
// }

// fn area_of_bounding_rectangle(first_corner: Position, second_corner: Position) -> Coordinate {
//     let height = (first_corner.row() - second_corner.row()).abs() + 1;
//     let width = (first_corner.column() - second_corner.column()).abs() + 1;
//     height * width
// }

// fn remove_protrusion(trench: &mut Vec<Corner>) -> Coordinate {
//     for index in (0..=trench.len() - 4).rev() {
//         let mut corners @ [left_neighbor, left_corner, right_corner, right_neighbor] =
//             [0, 1, 2, 3].map(|offset| index + offset);

//         let is_bend = trench[left_corner].1 == Curvature::RightTurn
//             && trench[right_corner].1 == Curvature::RightTurn;
//         if !is_bend {
//             continue;
//         }

//         let direction = Direction::try_from((trench[left_neighbor].0, trench[left_corner].0))
//             .expect("two successive corners should share a row or a column");
//         let unit_vector = direction.as_unit_vector();
//         let [left_side_length, right_side_length] =
//             [[left_neighbor, left_corner], [right_neighbor, right_corner]].map(
//                 |[neighbor, corner]| {
//                     (unit_vector.dot_product(trench[neighbor].0)
//                         - unit_vector.dot_product(trench[corner].0))
//                     .abs()
//                 },
//             );
//         let [left_turns_away, right_turns_away] = [left_neighbor, right_neighbor]
//             .map(|neighbor| trench[neighbor].1 == Curvature::LeftTurn);

//         match left_side_length.cmp(&right_side_length) {
//             Ordering::Less if !left_turns_away => continue,
//             Ordering::Equal if !left_turns_away || !right_turns_away => continue,
//             Ordering::Greater if !right_turns_away => continue,
//             Ordering::Greater => corners.reverse(),
//             _ => (),
//         }
//         let [short_neighbor, short_corner, long_corner, long_neighbor] = corners;

//         let protrusion_area = area_of_bounding_rectangle(
//             trench[short_neighbor].0.neighbor(direction),
//             trench[long_corner].0,
//         );

//         let short_neighbor_position = trench[short_neighbor].0;
//         let long_corner_position = &mut trench[long_corner].0;
//         match direction {
//             Direction::North | Direction::South => {
//                 long_corner_position.set_row(short_neighbor_position.row());
//             }
//             Direction::West | Direction::East => {
//                 long_corner_position.set_column(short_neighbor_position.column());
//             }
//         }
//         let mut corners_to_be_removed = vec![short_neighbor, short_corner];
//         if left_side_length == right_side_length {
//             corners_to_be_removed.extend([long_corner, long_neighbor]);
//         }
//         let [minimum_index, maximum_index] = [Iterator::min, Iterator::max].map(|extremum| {
//             *extremum(corners_to_be_removed.iter()).expect("at least two corners should be removed")
//         });
//         trench.drain(minimum_index..=maximum_index);

//         return protrusion_area;
//     }
//     // draw(trench, 80);
//     panic!("trench should contain a protrusion");
// }

// fn area(dig_plan: &[DigStep]) -> Coordinate {
//     let mut trench = trench(dig_plan);
//     if !is_clockwise(&trench) {
//         reverse(&mut trench);
//     }

//     let mut area = 0;
//     while trench.len() > 4 {
//         area += remove_protrusion(&mut trench);
//     }
//     area + area_of_bounding_rectangle(trench[0].0, trench[2].0)
// }

pub fn first(_input: &str) -> String {
    // let dig_plan = dig_plan(dig_plan_step_from_directions_and_distances, input);
    // area(&dig_plan).to_string()
    unimplemented!()
}

pub fn second(_input: &str) -> String {
    // let dig_plan = dig_plan(dig_plan_step_from_color_codes, input);
    // area(&dig_plan).to_string()
    unimplemented!()
}

// #[cfg(test)]
// mod tests {
//     use super::super::tests::test_on_input;
//     use crate::{Input, Puzzle};

//     const DAY: usize = 18;

//     #[test]
//     fn first_example() {
//         test_on_input(DAY, Puzzle::First, Input::Example(0), 62);
//     }

//     // #[test]
//     // fn first_input() {
//     //     test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 70253);
//     // }

//     #[test]
//     fn second_example() {
//         test_on_input(DAY, Puzzle::Second, Input::Example(0), 952_408_144_115_usize);
//     }
// }
