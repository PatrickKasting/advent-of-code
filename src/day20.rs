use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

type Pulse = bool;

const LOW: Pulse = false;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Module {
    FlipFlop(bool),
    Conjunction(HashMap<String, Pulse>),
    Broadcaster,
}

impl Module {
    fn receive(&mut self, pulse: Pulse, from: &str) -> Option<Pulse> {
        match self {
            Module::FlipFlop(is_on) => {
                if pulse {
                    None
                } else {
                    *is_on = !*is_on;
                    Some(*is_on)
                }
            }
            Module::Conjunction(most_recent_pulses_received) => {
                *most_recent_pulses_received
                    .get_mut(from)
                    .expect("source should be recognized") = pulse;
                let all_high = most_recent_pulses_received.values().all(|&pulse| pulse);
                Some(!all_high)
            }
            Module::Broadcaster => Some(pulse),
        }
    }
}

type Configuration = HashMap<String, (Module, Vec<String>)>;

fn module(line: &str) -> (String, (Module, Vec<String>)) {
    let (module, destinations) = line
        .split_once(" -> ")
        .expect("every line should contain ' -> '");
    let (name, module) = if module == "broadcaster" {
        ("broadcaster", Module::Broadcaster)
    } else if let Some(name) = module.strip_prefix('%') {
        (name, Module::FlipFlop(LOW))
    } else if let Some(name) = module.strip_prefix('&') {
        (name, Module::Conjunction(HashMap::new()))
    } else {
        unreachable!("module should be 'broadcaster', '%', or '&'")
    };
    let destinations = destinations.split(", ").map(String::from).collect_vec();
    (String::from(name), (module, destinations))
}

fn configuration(str: &str) -> Configuration {
    let mut configuration: Configuration = str.lines().map(module).collect();

    for source in configuration.keys().cloned().collect_vec() {
        for destination in configuration[&source].1.clone() {
            if let Some((Module::Conjunction(most_recent_pulses_received), _)) =
                configuration.get_mut(&destination)
            {
                most_recent_pulses_received.insert(source.clone(), LOW);
            }
        }
    }

    configuration
}

fn button_press(configuration: &mut Configuration) -> (usize, usize) {
    let (mut number_of_low_pulses, mut number_of_high_pulses) = (0, 0);
    let mut pulses = VecDeque::from([(String::from("button"), LOW, String::from("broadcaster"))]);
    while let Some((source, pulse, destination)) = pulses.pop_front() {
        if pulse {
            number_of_high_pulses += 1;
        } else {
            number_of_low_pulses += 1;
        }
        let Some((destination_module, next_destinations)) = configuration.get_mut(&destination)
        else {
            continue;
        };
        if let Some(output_pulse) = destination_module.receive(pulse, &source) {
            for next_destination in next_destinations {
                pulses.push_back((destination.clone(), output_pulse, next_destination.clone()));
            }
        }
    }
    (number_of_low_pulses, number_of_high_pulses)
}

pub fn first(input: &str) -> String {
    let mut configuration = configuration(&input);
    let (number_of_low_pulses, number_of_high_pulses): (Vec<_>, Vec<_>) =
        (0..1000).map(|_| button_press(&mut configuration)).unzip();
    let total_number_of_low_pulses: usize = number_of_low_pulses.into_iter().sum();
    let total_number_of_high_pulses: usize = number_of_high_pulses.into_iter().sum();
    (total_number_of_low_pulses * total_number_of_high_pulses).to_string()
}

pub fn second(_input: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::{tests::*, Input, Puzzle};

    const DAY: usize = 20;

    #[test]
    fn first_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 32_000_000);
        test_on_input(DAY, Puzzle::First, Input::Example(1), 11_687_500);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 1_020_211_150);
    }

    // #[test]
    // fn second_example() {
    //     test_on_input(DAY, Puzzle::Second, Input::Example(0), 51);
    // }

    // #[test]
    // fn second_input() {
    //     test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 8754);
    // }
}
