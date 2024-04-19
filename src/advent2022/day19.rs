use std::cmp;

use itertools::Itertools;

const NUMBER_OF_RESOURCES: usize = 4;

const ORE: usize = 0;
const CLAY: usize = 1;
const OBSIDIAN: usize = 2;
const GEODE: usize = 3;

type Blueprint = [Resources; NUMBER_OF_RESOURCES];
type RobotCost = Resources;
type Resources = [Count; NUMBER_OF_RESOURCES];
type RobotCounts = [Count; NUMBER_OF_RESOURCES];
type Time = Count;
type Count = i32;

pub fn first(input: &str) -> String {
    let blueprints = blueprints(input);
    (1..)
        .zip(blueprints)
        .map(|(id, blueprint)| quality_level(24, id, blueprint))
        .sum::<Count>()
        .to_string()
}

pub fn second(input: &str) -> String {
    let blueprints = blueprints(input).take(3);
    blueprints
        .map(|blueprint| maximum_geodes(32, blueprint))
        .product::<Count>()
        .to_string()
}

fn quality_level(time_limit: Time, id: Count, blueprint: Blueprint) -> Count {
    id * maximum_geodes(time_limit, blueprint)
}

type State = (Time, RobotCounts, Resources);

fn maximum_geodes(time_limit: Time, blueprint: Blueprint) -> Count {
    let maximum_needed_income = maximum_needed_income(blueprint);

    let mut maximum_geodes = 0;
    let mut states: Vec<State> = vec![(0, [1, 0, 0, 0], [0, 0, 0, 0])];
    while let Some((time, robot_counts, resources)) = states.pop() {
        let geodes_no_additional_robots =
            resource_after_time(GEODE, time_limit - time, robot_counts, resources);
        maximum_geodes = cmp::max(maximum_geodes, geodes_no_additional_robots);

        for (resource_type, robot_cost) in blueprint.into_iter().enumerate().rev() {
            if robot_counts[resource_type] == maximum_needed_income[resource_type] {
                continue;
            }

            let Some((additional_time, robot_counts, resources)) =
                robot_building(resource_type, robot_cost, robot_counts, resources)
            else {
                continue;
            };
            let time = time + additional_time;

            if time >= time_limit {
                continue;
            }
            let remaining_time = time_limit - time;
            let geodes_upper_bound = geodes_upper_bound(remaining_time, robot_counts, resources);
            if geodes_upper_bound <= maximum_geodes {
                continue;
            }

            states.push((time, robot_counts, resources));
        }
    }
    maximum_geodes
}

fn maximum_needed_income(blueprint: Blueprint) -> Resources {
    let mut maximum_needed_income = [0, 0, 0, Count::MAX];
    for robot_cost in blueprint {
        for resource_type in [ORE, CLAY, OBSIDIAN] {
            maximum_needed_income[resource_type] = cmp::max(
                maximum_needed_income[resource_type],
                robot_cost[resource_type],
            );
        }
    }
    maximum_needed_income
}

fn robot_building(
    resource_type: usize,
    robot_cost: Resources,
    mut robot_counts: RobotCounts,
    resources: Resources,
) -> Option<State> {
    let time_until_completion = time_until_completion(robot_cost, robot_counts, resources)?;
    let resources =
        resources_after_robot_building(time_until_completion, robot_cost, robot_counts, resources);
    robot_counts[resource_type] += 1;
    Some((time_until_completion, robot_counts, resources))
}

fn time_until_completion(
    robot_cost: Resources,
    robot_counts: RobotCounts,
    resources: Resources,
) -> Option<Time> {
    let needed_resources = [ORE, CLAY, OBSIDIAN].map(|resource_type| {
        (robot_cost[resource_type] - resources[resource_type]).clamp(0, Count::MAX)
    });
    let mut time_until_affordable = 0;
    for (needed_resource, robot_count) in needed_resources.into_iter().zip(robot_counts) {
        if needed_resource == 0 {
            continue;
        }
        if robot_count == 0 {
            return None;
        }
        time_until_affordable = cmp::max(
            time_until_affordable,
            div_ceil(needed_resource, robot_count),
        );
    }
    Some(time_until_affordable + 1)
}

fn resources_after_robot_building(
    elapsed_time: Time,
    robot_cost: Resources,
    robot_counts: RobotCounts,
    mut resources: Resources,
) -> Resources {
    for (resource_type, resource) in resources.iter_mut().enumerate() {
        *resource += elapsed_time * robot_counts[resource_type] - robot_cost[resource_type];
    }
    resources
}

fn geodes_upper_bound(
    remaining_time: Time,
    robot_counts: RobotCounts,
    resources: Resources,
) -> Count {
    let geodes_from_building_geode_robots_every_minute = remaining_time * (remaining_time - 1) / 2;
    geodes_from_building_geode_robots_every_minute
        + resource_after_time(GEODE, remaining_time, robot_counts, resources)
}

fn resource_after_time(
    resource_type: usize,
    time: Time,
    robot_counts: RobotCounts,
    resources: Resources,
) -> Count {
    time * robot_counts[resource_type] + resources[resource_type]
}

fn blueprints(input: &str) -> impl Iterator<Item = Blueprint> + '_ {
    input.lines().map(blueprint)
}

fn blueprint(line: &str) -> Blueprint {
    let (_, costs) = line.split_once(':').expect("line should contain a colon");
    costs
        .split('.')
        .filter(|str| !str.is_empty())
        .map(robot_cost)
        .collect_vec()
        .try_into()
        .expect("line should contain four robot costs")
}

fn robot_cost(str: &str) -> RobotCost {
    let mut costs: Resources = [0; NUMBER_OF_RESOURCES];
    let mut tokens = str.split_whitespace();
    while let Some(token) = tokens.next() {
        if let Ok(cost) = token.parse() {
            let resource = tokens
                .next()
                .expect("type of resource should follow amount");
            match resource {
                "ore" => costs[ORE] = cost,
                "clay" => costs[CLAY] = cost,
                "obsidian" => costs[OBSIDIAN] = cost,
                _ => panic!("robots should only cost ore, clay, or obsidian"),
            }
        }
    }
    costs
}

fn div_ceil(dividend: Count, divisor: Count) -> Count {
    debug_assert!(
        !dividend.is_negative() && !divisor.is_negative(),
        "dividend and divisor should be non-negative"
    );
    (dividend + divisor - 1) / divisor
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::{
        super::tests::{test_on_input, YEAR},
        *,
    };
    use crate::{input, tests::test_cases, Input, Puzzle};

    const DAY: usize = 19;

    #[test]
    fn first_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 33);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 1725);
    }

    #[test]
    fn second_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 15510);
    }

    #[test]
    fn largest_number_of_geodes_24_seconds() {
        let input = input(YEAR, DAY, Input::Example(0));
        let blueprints = super::blueprints(&input).collect_vec();
        assert_eq!(super::maximum_geodes(24, blueprints[0]), 9);
    }

    #[test]
    fn largest_number_of_geodes_32_seconds() {
        let input = input(YEAR, DAY, Input::Example(0));
        let blueprints = super::blueprints(&input).collect_vec();
        assert_eq!(super::maximum_geodes(32, blueprints[0]), 56);
    }

    #[test]
    fn robot_building_some_wait() {
        let actual = robot_building(GEODE, [2, 0, 7, 0], [1, 4, 2, 0], [1, 5, 4, 0]);
        let expected_elapsed_time = 3;
        let expected_robot_counts = [1, 4, 2, 1];
        let expected_resources = [2, 17, 3, 0];
        assert_eq!(
            actual,
            Some((
                expected_elapsed_time,
                expected_robot_counts,
                expected_resources,
            ))
        );
    }

    #[test]
    fn time_until_completion_some_wait() {
        let actual = time_until_completion([3, 14, 0, 0], [1, 3, 0, 0], [1, 6, 0, 0]);
        assert_eq!(actual, Some(4));
    }

    #[test]
    fn time_until_completion_no_wait() {
        let actual = time_until_completion([4, 0, 0, 0], [1, 4, 2, 2], [6, 41, 8, 9]);
        assert_eq!(actual, Some(1));
    }

    #[test]
    fn time_until_completion_impossible() {
        let actual = time_until_completion([3, 14, 0, 0], [1, 0, 0, 0], [1, 0, 0, 0]);
        assert_eq!(actual, None);
    }

    #[test]
    fn resources_after_robot_building() {
        let actual =
            super::resources_after_robot_building(2, [2, 0, 0, 0], [1, 1, 0, 0], [1, 0, 0, 0]);
        assert_eq!(actual, [1, 2, 0, 0]);
    }

    #[test]
    fn blueprints() {
        let input = input(YEAR, DAY, Input::Example(0));
        let actual = super::blueprints(&input).collect_vec();
        let expected = [
            [[4, 0, 0, 0], [2, 0, 0, 0], [3, 14, 0, 0], [2, 0, 7, 0]],
            [[2, 0, 0, 0], [3, 0, 0, 0], [3, 8, 0, 0], [3, 0, 12, 0]],
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn div_ceil() {
        let function = |(left, right)| super::div_ceil(left, right);
        let cases = [
            ((5, 2), 3),
            ((6, 3), 2),
            ((1, 17), 1),
            ((0, 4), 0),
            ((7, 4), 2),
        ];
        test_cases(function, cases);
    }
}
