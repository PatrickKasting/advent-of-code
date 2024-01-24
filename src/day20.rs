use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

type Pulse = bool;
const LOW: Pulse = false;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Module<'input> {
    FlipFlop(bool),
    Conjunction(HashMap<&'input str, Pulse>),
    Broadcaster,
}

impl<'input> Module<'input> {
    fn receive(&mut self, pulse: Pulse, emitter: &str) -> Option<Pulse> {
        match self {
            Module::FlipFlop(is_on) => {
                if pulse {
                    None
                } else {
                    *is_on = !*is_on;
                    Some(*is_on)
                }
            }
            Module::Conjunction(most_recent_pulse_from_each_emitter) => {
                *most_recent_pulse_from_each_emitter
                    .get_mut(emitter)
                    .expect("emitter should be recognized") = pulse;
                let all_high = most_recent_pulse_from_each_emitter
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

    for emitter in configuration.keys().copied().collect_vec() {
        for receiver in configuration[&emitter].1.clone() {
            if let Some((Module::Conjunction(most_recent_pulse_from_each_emitter), _)) =
                configuration.get_mut(&receiver)
            {
                most_recent_pulse_from_each_emitter.insert(emitter, LOW);
            }
        }
    }

    configuration
}

fn press_button(configuration: &mut Configuration) -> (usize, usize) {
    let (mut number_of_low_pulses, mut number_of_high_pulses) = (0, 0);
    let mut pulses = VecDeque::from([("button", LOW, "broadcaster")]);
    while let Some((emitter, pulse, receiver)) = pulses.pop_front() {
        if pulse {
            number_of_high_pulses += 1;
        } else {
            number_of_low_pulses += 1;
        }

        if let Some((receiver_module, output_pulse_receivers)) = configuration.get_mut(&receiver) {
            if let Some(output_pulse) = receiver_module.receive(pulse, emitter) {
                for output_pulse_receiver in output_pulse_receivers {
                    pulses.push_back((receiver, output_pulse, output_pulse_receiver));
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
    todo!()
    // let mut configuration = configuration(input);
    // for num_presses in 1usize.. {
    //     if let (_, (1, 0)) = press_button(&mut configuration) {
    //         return num_presses.to_string();
    //     }
    // }
    // unreachable!("a single low pulse should reach 'rx' in a finite number of button presses");
}

#[cfg(test)]
mod tests {
    use crate::{tests::*, InputType, Puzzle};

    const DAY: usize = 20;

    #[test]
    fn first_examples() {
        test_on_input(DAY, Puzzle::First, InputType::Example(0), 32_000_000);
        test_on_input(DAY, Puzzle::First, InputType::Example(1), 11_687_500);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, InputType::PuzzleInput, 1_020_211_150);
    }

    // #[test]
    // fn second_input() {
    //     test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 8754);
    // }
}
