use std::{cmp, collections::HashMap, sync::OnceLock};

use itertools::Itertools;
use regex::Regex;

use crate::{search::shortest_path_length, strings::isizes};

type ContractedCave<'input> = HashMap<Valve<'input>, (Pressure, Vec<(Time, Valve<'input>)>)>;
type Cave<'input> = HashMap<Valve<'input>, (Pressure, Vec<Valve<'input>>)>;
type Valve<'input> = &'input str;
type Pressure = isize;
type Time = isize;

pub fn first(input: &str) -> String {
    let cave = cave(input);
    let start = "AA";
    let mut contracted_cave = contracted_cave(&cave, start);
    maximum_cave_pressure_release(&mut contracted_cave, start, 30).to_string()
}

pub fn second(_input: &str) -> String {
    todo!();
}

fn maximum_cave_pressure_release<'input>(
    cave: &mut ContractedCave<'input>,
    valve: Valve<'input>,
    time: Time,
) -> Pressure {
    let (flow, other_valves) = cave.remove(valve).expect("valve should be present in cave");
    let mut maximum_release = 0;
    for &(distance, other_valve) in &other_valves {
        let Some((flow, _)) = cave.get(other_valve) else {
            continue;
        };
        let time = time - distance - 1;
        if time <= 0 {
            continue;
        }
        let valve_release = time * flow;
        let cave_release = maximum_cave_pressure_release(cave, other_valve, time);
        maximum_release = cmp::max(maximum_release, valve_release + cave_release);
    }
    cave.insert(valve, (flow, other_valves));
    maximum_release
}

fn contracted_cave<'input>(cave: &Cave<'input>, source: Valve<'input>) -> ContractedCave<'input> {
    cave.iter()
        .filter(|(&valve, &(flow, _))| flow != 0 || valve == source)
        .map(|(&source, &(flow, _))| {
            let distances = distances_to_functioning_valves(cave, source);
            (source, (flow, distances))
        })
        .collect()
}

fn distances_to_functioning_valves<'input>(
    cave: &Cave<'input>,
    source: Valve<'input>,
) -> Vec<(isize, Valve<'input>)> {
    let mut other_valves = Vec::new();
    let add_valve_maybe = |valve, distance: usize| {
        let (flow, _) = cave[valve];
        if flow != 0 && valve != source {
            let distance = distance
                .try_into()
                .expect("distance should be less than 'isize::MAX'");
            other_valves.push((distance, valve));
        }
    };
    let successors = |name| cave[name].1.iter().copied();
    shortest_path_length(source, add_valve_maybe, successors, |_| false);
    other_valves
}

fn cave(input: &str) -> Cave {
    input.lines().map(valve).collect()
}

fn valve(line: &str) -> (&str, (Pressure, Vec<&str>)) {
    static REGEX: OnceLock<Regex> = OnceLock::new();
    let regex = REGEX.get_or_init(|| Regex::new(r"[A-Z]{2}").expect("regex should be valid"));
    let mut valves = regex.find_iter(line).map(|mat| mat.as_str()).collect_vec();
    let valve = valves.remove(0);
    (valve, (isizes(line)[0], valves))
}

#[cfg(test)]
mod tests {
    use super::super::tests::test_on_input;
    use crate::{Input, Puzzle};

    const DAY: usize = 16;

    #[test]
    fn first_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 1651);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 1584);
    }

    // #[test]
    // fn second_example() {
    //     test_on_input(DAY, Puzzle::Second, Input::Example(0), 24_933_642);
    // }

    // #[test]
    // fn second_input() {
    //     test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 404_395);
    // }
}
