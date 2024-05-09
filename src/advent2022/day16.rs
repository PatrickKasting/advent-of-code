use std::{cmp, sync::OnceLock};

use itertools::Itertools;
use regex::Regex;

use crate::{search::distances, strings::usizes, HashMap, HashSet};

type ContractedCave<'input> = HashMap<Valve<'input>, (Pressure, Vec<(Valve<'input>, Time)>)>;
type Cave<'input> = HashMap<Valve<'input>, (Pressure, Vec<Valve<'input>>)>;
type Valve<'input> = &'input str;
type Pressure = usize;
type Time = usize;
type Distance = usize;

pub fn first(input: &str) -> String {
    maximum_release_from_input::<1>(input, "AA", 30).to_string()
}

pub fn second(input: &str) -> String {
    maximum_release_from_input::<2>(input, "AA", 26).to_string()
}

fn maximum_release_from_input<const NUM_AGENTS: usize>(
    input: &str,
    start: Valve,
    time: Time,
) -> Pressure {
    let cave = cave(input);
    let contracted_cave = contracted_cave(&cave, start);
    let mut closed_valves = contracted_cave
        .iter()
        .filter_map(|(&valve, &(flow, _))| (flow != 0).then_some(valve))
        .collect();
    maximum_release(
        &contracted_cave,
        &mut closed_valves,
        [(start, time); NUM_AGENTS],
    )
}

fn maximum_release<'input, const NUM_AGENTS: usize>(
    cave: &ContractedCave<'input>,
    closed_valves: &mut HashSet<Valve<'input>>,
    mut agents: [(Valve<'input>, Time); NUM_AGENTS],
) -> Pressure {
    agents.sort_unstable_by_key(|&(_, time)| Time::MAX - time);
    let (current_valve, current_time) = agents[0];
    if current_time == 0 {
        return 0;
    }

    let mut maximum_release = 0;
    for &(valve, distance) in &cave[current_valve].1 {
        if !closed_valves.contains(valve) {
            continue;
        }

        let Some(time) = current_time.checked_sub(distance + 1) else {
            continue;
        };
        let (flow, _) = cave[valve];
        let valve_release = time * flow;

        agents[0] = (valve, time);
        closed_valves.remove(valve);
        let maximum_remaining_release = self::maximum_release(cave, closed_valves, agents);
        closed_valves.insert(valve);

        maximum_release = cmp::max(maximum_release, valve_release + maximum_remaining_release);
    }

    agents[0] = (current_valve, 0);
    let stop_release = self::maximum_release(cave, closed_valves, agents);
    maximum_release = cmp::max(maximum_release, stop_release);

    maximum_release
}

fn contracted_cave<'input>(cave: &Cave<'input>, start: Valve<'input>) -> ContractedCave<'input> {
    cave.iter()
        .filter(|(&valve, &(flow, _))| flow != 0 || valve == start)
        .map(|(&source, &(flow, _))| {
            let distances = distances_to_functioning_valves(cave, source);
            (source, (flow, distances))
        })
        .collect()
}

fn distances_to_functioning_valves<'input>(
    cave: &Cave<'input>,
    source: Valve<'input>,
) -> Vec<(Valve<'input>, Distance)> {
    let successors = |name| cave[name].1.iter().copied();
    distances(source, successors)
        .into_iter()
        .filter(|&(valve, _)| cave[valve].0 != 0 && valve != source)
        .collect_vec()
}

fn cave(input: &str) -> Cave {
    input.lines().map(valve).collect()
}

fn valve(line: &str) -> (&str, (Pressure, Vec<&str>)) {
    static REGEX: OnceLock<Regex> = OnceLock::new();
    let regex = REGEX.get_or_init(|| Regex::new("[A-Z]{2}").expect("regex should be valid"));
    let mut valves = regex.find_iter(line).map(|mat| mat.as_str()).collect_vec();
    let valve = valves.remove(0);
    (valve, (usizes(line)[0], valves))
}

#[cfg(test)]
mod tests {
    use super::{super::tests::test_on_input, maximum_release_from_input};
    use crate::{input, Input, Puzzle};

    const DAY: usize = 16;

    #[test]
    fn first_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 1651);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 1584);
    }

    #[test]
    fn second_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 1707);
    }

    // #[test]
    // fn second_input() {
    //     test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 2052);
    // }

    #[test]
    fn farther_agent_should_not_close_last_valve() {
        let input = input(2022, 16, Input::Example(1));
        let actual = maximum_release_from_input::<2>(&input, "AA", 10);
        assert_eq!(actual, 80 + 70 + 50);
    }
}
