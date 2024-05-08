use std::collections::VecDeque;

use itertools::Itertools;

use crate::HashMap;

type Pulse = bool;
const LOW: Pulse = false;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Module<'input> {
    FlipFlop(bool),
    Conjunction(HashMap<&'input str, Pulse>),
    Broadcaster,
}

impl<'input> Module<'input> {
    fn receive(&mut self, pulse: Pulse, source: &str) -> Option<Pulse> {
        match self {
            Module::FlipFlop(is_on) => {
                if pulse {
                    None
                } else {
                    *is_on = !*is_on;
                    Some(*is_on)
                }
            }
            Module::Conjunction(most_recent_pulse_from_each_source) => {
                *most_recent_pulse_from_each_source
                    .get_mut(source)
                    .expect("source should be recognized") = pulse;
                let all_high = most_recent_pulse_from_each_source
                    .values()
                    .all(|&pulse| pulse);
                Some(!all_high)
            }
            Module::Broadcaster => Some(pulse),
        }
    }
}

type Configuration<'input> = HashMap<&'input str, (Module<'input>, Vec<&'input str>)>;

fn module(line: &str) -> (&str, (Module, Vec<&str>)) {
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
        panic!("module should be 'broadcaster', '%', or '&'")
    };
    let destinations = destinations.split(", ").collect_vec();
    (name, (module, destinations))
}

fn configuration(str: &str) -> Configuration {
    let mut configuration: Configuration = str.lines().map(module).collect();

    for source in configuration.keys().copied().collect_vec() {
        for destination in configuration[&source].1.clone() {
            if let Some((Module::Conjunction(most_recent_pulse_from_each_source), _)) =
                configuration.get_mut(&destination)
            {
                most_recent_pulse_from_each_source.insert(source, LOW);
            }
        }
    }

    configuration
}

fn press_button(configuration: &mut Configuration) -> (usize, usize) {
    let (mut number_of_low_pulses, mut number_of_high_pulses) = (0, 0);
    let mut pulses = VecDeque::from([("button", LOW, "broadcaster")]);
    while let Some((source, pulse, destination)) = pulses.pop_front() {
        if pulse {
            number_of_high_pulses += 1;
        } else {
            number_of_low_pulses += 1;
        }

        if let Some((destination_module, output_pulse_destinations)) =
            configuration.get_mut(&destination)
        {
            if let Some(output_pulse) = destination_module.receive(pulse, source) {
                for output_pulse_destination in output_pulse_destinations {
                    pulses.push_back((destination, output_pulse, output_pulse_destination));
                }
            }
        }
    }
    (number_of_low_pulses, number_of_high_pulses)
}

fn total_number_of_low_and_high_pulses(configuration: &mut Configuration) -> (usize, usize) {
    let (number_of_low_pulses, number_of_high_pulses): (Vec<_>, Vec<_>) =
        (0..1000).map(|_| press_button(configuration)).unzip();
    let total_number_of_low_pulses = number_of_low_pulses.into_iter().sum();
    let total_number_of_high_pulses = number_of_high_pulses.into_iter().sum();
    (total_number_of_low_pulses, total_number_of_high_pulses)
}

pub fn first(input: &str) -> String {
    let mut configuration = configuration(input);
    let (total_number_of_low_pulses, total_number_of_high_pulses) =
        total_number_of_low_and_high_pulses(&mut configuration);
    (total_number_of_low_pulses * total_number_of_high_pulses).to_string()
}

pub fn second(_input: &str) -> String {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::super::tests::test_on_input;
    use crate::{Input, Puzzle};

    const DAY: usize = 20;

    #[test]
    fn first_examples() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 32_000_000);
        test_on_input(DAY, Puzzle::First, Input::Example(1), 11_687_500);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 1_020_211_150);
    }
}
